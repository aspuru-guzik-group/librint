#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]

use crate::cint::CINTOpt;
use crate::cint1e::{cint1e_nuc_cart, cint1e_nuc_sph, cint1e_ovlp_cart, cint1e_ovlp_sph};
use crate::cint2e::{cint2e_cart, cint2e_cart_optimizer, cint2e_sph};
use crate::cint_bas::{CINTcgto_cart, CINTcgto_spheric};
use crate::intor1::{cint1e_kin_cart, cint1e_kin_sph};
use crate::optimizer::CINTdel_optimizer;

use crate::linalg::{dcopya, matmult, sort, transpose};

use faer::{linalg::solvers::SelfAdjointEigendecomposition, mat, Side};

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

/// An out-of-range `coord`/`typec` code is a caller bug: the C boundary passes
/// them as bare ints with no type safety. Aborting preserves the historical
/// behaviour (and the rule in `p2c.rs` about never returning a placeholder
/// result), but now says why first instead of exiting silently.
///
/// TODO: these should become a `Result` propagated out through `p2c.rs`, the way
/// `density`/`scf` already report failure in band. A library should not call
/// `process::exit` on its host.
fn invalid_code(what: &str, code: i32) -> ! {
    eprintln!("librint: invalid {} code {}", what, code);
    std::process::exit(1);
}

/// Which angular-momentum convention the AO matrices are built in. Selected by
/// the `coord` code at the C boundary: 0 cartesian, 1 spherical.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Basis {
    Cartesian,
    Spherical,
}

/// Which one-electron operator to integrate. Selected by the `typec` code at
/// the C boundary: 0 overlap, 1 kinetic, 2 nuclear attraction.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Op1e {
    Overlap,
    Kinetic,
    Nuclear,
}

impl Basis {
    pub fn from_code(coord: i32) -> Self {
        match coord {
            0 => Basis::Cartesian,
            1 => Basis::Spherical,
            _ => invalid_code("coord", coord),
        }
    }

    /// Contracted gaussians per shell in this convention.
    fn cgto(self, bas_id: usize, bas: &[i32]) -> i32 {
        match self {
            Basis::Cartesian => CINTcgto_cart(bas_id, bas),
            Basis::Spherical => CINTcgto_spheric(bas_id, bas),
        }
    }

    /// One-electron integral block for a single shell pair.
    #[allow(clippy::too_many_arguments)]
    fn int1e(
        self,
        op: Op1e,
        buf: &mut [f64],
        shls: &mut [i32],
        atm: &mut [i32],
        natm: i32,
        bas: &mut [i32],
        nbas: i32,
        env: &mut [f64],
    ) -> i32 {
        let opt = std::ptr::null_mut();
        match (self, op) {
            (Basis::Cartesian, Op1e::Overlap) => {
                cint1e_ovlp_cart(buf, shls, atm, natm, bas, nbas, env, opt)
            }
            (Basis::Cartesian, Op1e::Kinetic) => {
                cint1e_kin_cart(buf, shls, atm, natm, bas, nbas, env, opt)
            }
            (Basis::Cartesian, Op1e::Nuclear) => {
                cint1e_nuc_cart(buf, shls, atm, natm, bas, nbas, env, opt)
            }
            (Basis::Spherical, Op1e::Overlap) => {
                cint1e_ovlp_sph(buf, shls, atm, natm, bas, nbas, env, opt)
            }
            (Basis::Spherical, Op1e::Kinetic) => {
                cint1e_kin_sph(buf, shls, atm, natm, bas, nbas, env, opt)
            }
            (Basis::Spherical, Op1e::Nuclear) => {
                cint1e_nuc_sph(buf, shls, atm, natm, bas, nbas, env, opt)
            }
        }
    }

    /// Two-electron integral block for a single shell quartet.
    #[allow(clippy::too_many_arguments)]
    fn int2e(
        self,
        buf: &mut [f64],
        shls: &mut [i32],
        atm: &mut [i32],
        natm: i32,
        bas: &mut [i32],
        nbas: i32,
        env: &mut [f64],
        opt: *mut CINTOpt,
    ) -> i32 {
        match self {
            Basis::Cartesian => cint2e_cart(buf, shls, atm, natm, bas, nbas, env, opt),
            Basis::Spherical => cint2e_sph(buf, shls, atm, natm, bas, nbas, env, opt),
        }
    }
}

impl Op1e {
    pub fn from_code(typec: i32) -> Self {
        match typec {
            0 => Op1e::Overlap,
            1 => Op1e::Kinetic,
            2 => Op1e::Nuclear,
            _ => invalid_code("typec", typec),
        }
    }
}

pub fn nmol(atm: &[i32], bas: &[i32]) -> (usize, usize) {
    let natm: usize = atm.len() / ATM_SLOTS;
    let nbas: usize = bas.len() / BAS_SLOTS;
    (natm, nbas)
}

pub fn angl(bas: &[i32], coord: i32) -> usize {
    let mut nshells: usize = 0;
    for i in (0..bas.len()).step_by(BAS_SLOTS) {
        let l = bas[i + 1] as usize;
        // nctr contracted functions per shell (general contraction); for
        // segmented bases bas[i+3] == 1 and this reduces to the old count.
        let nctr = bas[i + 3] as usize;
        if coord == 0 {
            nshells += (l + 1) * (l + 2) / 2 * nctr;
        } else if coord == 1 {
            nshells += (2 * l + 1) * nctr;
        }
    }
    nshells
}

pub fn integral1e(
    atm: &mut [i32],
    bas: &mut [i32],
    env: &mut [f64],
    coord: i32,
    typec: i32,
) -> Vec<f64> {
    let (natm, nbas) = nmol(atm, bas);
    let nshells = angl(bas, coord);

    let mut R = vec![0.0; nshells * nshells];

    let mut buf: Vec<f64>;
    let mut shls: [i32; 4] = [0, 0, 0, 0];

    let mut mu;
    let mut nu;

    let mut di;
    let mut dj;

    let basis = Basis::from_code(coord);
    let op = Op1e::from_code(typec);

    mu = 0;
    for i in 0..nbas {
        shls[0] = i as i32;
        di = basis.cgto(i, bas) as usize;

        nu = 0;
        for j in 0..nbas {
            shls[1] = j as i32;
            dj = basis.cgto(j, bas) as usize;

            buf = vec![0.0; di * dj];

            basis.int1e(
                op,
                &mut buf,
                &mut shls,
                atm,
                natm as i32,
                bas,
                nbas as i32,
                env,
            );
            let mut c: usize = 0;
            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    R[mui * nshells + nuj] = buf[c];
                    c += 1;
                }
            }

            nu += dj;
        }
        mu += di;
    }

    R
}

pub fn integral2e(atm: &mut [i32], bas: &mut [i32], env: &mut [f64], coord: i32) -> Vec<f64> {
    let (natm, nbas) = nmol(atm, bas);
    let nshells = angl(bas, coord);

    let mut R = vec![0.0; nshells * nshells * nshells * nshells];

    let mut buf: Vec<f64>;
    let mut shls: [i32; 4] = [0, 0, 0, 0];

    let mut mu;
    let mut nu;
    let mut sig;
    let mut lam;

    let mut di;
    let mut dj;
    let mut dk;
    let mut dl;

    let basis = Basis::from_code(coord);

    // CINTOpt built once and shared by every quartet (primal only -- see
    // integral2e_sym for why it must not enter differentiated calls)
    let mut opt: *mut CINTOpt = std::ptr::null_mut();
    unsafe {
        cint2e_cart_optimizer(
            &mut opt,
            atm.as_mut_ptr(),
            natm as i32,
            bas.as_mut_ptr(),
            nbas as i32,
            env.as_mut_ptr(),
        );
    }

    mu = 0;
    for i in 0..nbas {
        shls[0] = i as i32;
        di = basis.cgto(i, bas) as usize;

        nu = 0;
        for j in 0..nbas {
            shls[1] = j as i32;
            dj = basis.cgto(j, bas) as usize;

            sig = 0;
            for k in 0..nbas {
                shls[2] = k as i32;
                dk = basis.cgto(k, bas) as usize;

                lam = 0;
                for l in 0..nbas {
                    shls[3] = l as i32;
                    dl = basis.cgto(l, bas) as usize;

                    buf = vec![0.0; di * dj * dk * dl];

                    basis.int2e(
                        &mut buf,
                        &mut shls,
                        atm,
                        natm as i32,
                        bas,
                        nbas as i32,
                        env,
                        opt,
                    );
                    let mut c: usize = 0;
                    for laml in lam..(lam + dl) {
                        for sigk in sig..(sig + dk) {
                            for nuj in nu..(nu + dj) {
                                for mui in mu..(mu + di) {
                                    R[mui * nshells.pow(3)
                                        + nuj * nshells.pow(2)
                                        + sigk * nshells
                                        + laml] = buf[c];
                                    c += 1;
                                }
                            }
                        }
                    }

                    lam += dl;
                }
                sig += dk;
            }
            nu += dj;
        }
        mu += di;
    }

    unsafe {
        CINTdel_optimizer(&mut opt);
    }

    R
}

// G = two-electron Fock matrix, built directly in O(n^2) memory -- the full
// n^4 ERI tensor is never allocated, so the frozen-P gradient's Q=PFP build is
// O(n^2) not O(n^4). F = H + G at the caller. Primal only (reused CINTOpt fine).
pub fn integral2e_fock(
    atm: &mut [i32],
    bas: &mut [i32],
    env: &mut [f64],
    P: &[f64],
    coord: i32,
) -> Vec<f64> {
    let (natm, nbas) = nmol(atm, bas);
    let n = angl(bas, coord);

    let mut G = vec![0.0; n * n];

    let mut buf: Vec<f64>;
    let mut shls: [i32; 4] = [0, 0, 0, 0];

    let basis = Basis::from_code(coord);

    let mut opt: *mut CINTOpt = std::ptr::null_mut();
    unsafe {
        cint2e_cart_optimizer(
            &mut opt,
            atm.as_mut_ptr(),
            natm as i32,
            bas.as_mut_ptr(),
            nbas as i32,
            env.as_mut_ptr(),
        );
    }

    // Full nbas^4 quartet loop (no permutational symmetry): every ERI slot is
    // visited exactly once, so accumulating the per-slot Coulomb/exchange
    // contributions gives G with no double-counting. (8-fold canonical would
    // need per-shell degeneracy factors; not worth it for this once-per-
    // gradient primal build -- minor vs the dRf reverse.)
    let mut mu = 0usize;
    for i in 0..nbas {
        shls[0] = i as i32;
        let di = basis.cgto(i, bas) as usize;
        let mut nu = 0usize;
        for j in 0..nbas {
            shls[1] = j as i32;
            let dj = basis.cgto(j, bas) as usize;
            let mut sig = 0usize;
            for k in 0..nbas {
                shls[2] = k as i32;
                let dk = basis.cgto(k, bas) as usize;
                let mut lam = 0usize;
                for l in 0..nbas {
                    shls[3] = l as i32;
                    let dl = basis.cgto(l, bas) as usize;

                    buf = vec![0.0; di * dj * dk * dl];
                    basis.int2e(
                        &mut buf,
                        &mut shls,
                        atm,
                        natm as i32,
                        bas,
                        nbas as i32,
                        env,
                        opt,
                    );

                    let mut c: usize = 0;
                    for laml in lam..(lam + dl) {
                        for sigk in sig..(sig + dk) {
                            for nuj in nu..(nu + dj) {
                                for mui in mu..(mu + di) {
                                    let v = buf[c];
                                    c += 1;
                                    // slot (mui nuj | sigk laml):
                                    //   Coulomb  -> G[mui,nuj] += P[laml,sigk]*v
                                    //   exchange -> G[mui,laml] += -0.5*P[nuj,sigk]*v
                                    G[mui * n + nuj] += P[laml * n + sigk] * v;
                                    G[mui * n + laml] += -0.5 * P[nuj * n + sigk] * v;
                                }
                            }
                        }
                    }

                    lam += dl;
                }
                sig += dk;
            }
            nu += dj;
        }
        mu += di;
    }

    unsafe {
        CINTdel_optimizer(&mut opt);
    }

    G
}

fn integrals(
    natm: usize,
    nbas: usize,
    nshells: usize,
    atm: &mut [i32],
    bas: &mut [i32],
    env: &mut [f64],
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut S = vec![0.0; nshells * nshells];
    let mut H = vec![0.0; nshells * nshells];
    let mut two = vec![0.0; nshells * nshells * nshells * nshells];

    let mut buf: Vec<f64>;

    let mut shls: [i32; 4] = [0, 0, 0, 0];

    let mut mu;
    let mut nu;
    let mut sig;
    let mut lam;

    let mut di;
    let mut dj;
    let mut dk;
    let mut dl;

    let mut T = vec![0.0; nshells * nshells];
    let mut V = vec![0.0; nshells * nshells];

    mu = 0;
    for i in 0..nbas {
        shls[0] = i as i32;
        di = CINTcgto_cart(i, bas) as usize;

        nu = 0;
        for j in 0..nbas {
            sig = 0;

            shls[1] = j as i32;
            dj = CINTcgto_cart(j, bas) as usize;

            buf = vec![0.0; di * dj];

            cint1e_ovlp_cart(
                &mut buf,
                &mut shls,
                atm,
                natm as i32,
                bas,
                nbas as i32,
                env,
                std::ptr::null_mut(),
            );
            let mut c: usize = 0;
            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    S[mui * nshells + nuj] = buf[c];
                    c += 1;
                }
            }

            cint1e_kin_cart(
                &mut buf,
                &mut shls,
                atm,
                natm as i32,
                bas,
                nbas as i32,
                env,
                std::ptr::null_mut(),
            );
            let mut c: usize = 0;
            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    T[mui * nshells + nuj] = buf[c];
                    c += 1;
                }
            }

            cint1e_nuc_cart(
                &mut buf,
                &mut shls,
                atm,
                natm as i32,
                bas,
                nbas as i32,
                env,
                std::ptr::null_mut(),
            );
            let mut c: usize = 0;
            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    V[mui * nshells + nuj] = buf[c];
                    c += 1;
                }
            }

            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    H[mui * nshells + nuj] = T[mui * nshells + nuj] + V[mui * nshells + nuj];
                }
            }

            for k in 0..nbas {
                shls[2] = k as i32;
                dk = CINTcgto_cart(k, bas) as usize;

                lam = 0;
                for l in 0..nbas {
                    shls[3] = l as i32;
                    dl = CINTcgto_cart(l, bas) as usize;

                    buf = vec![0.0; di * dj * dk * dl];

                    cint2e_cart(
                        &mut buf,
                        &mut shls,
                        atm,
                        natm as i32,
                        bas,
                        nbas as i32,
                        env,
                        std::ptr::null_mut(),
                    );
                    let mut c: usize = 0;
                    for laml in lam..(lam + dl) {
                        for sigk in sig..(sig + dk) {
                            for nuj in nu..(nu + dj) {
                                for mui in mu..(mu + di) {
                                    two[mui * nshells.pow(3)
                                        + nuj * nshells.pow(2)
                                        + sigk * nshells
                                        + laml] = buf[c];
                                    c += 1;
                                }
                            }
                        }
                    }

                    lam += dl;
                }
                sig += dk;
            }
            nu += dj;
        }
        mu += di;
    }

    (S, H, two)
}

// Smallest overlap eigenvalue still considered a usable basis in find_X.
const S_EIG_MIN: f64 = 1e-8;
// Tolerance on tr(PS) = nelec and on the S-metric idempotency PSP = 2P.
const P_TOL: f64 = 1e-6;

// A converged fixed point is not automatically a valid density: if the
// orthogonalizer loses C^dag S C = 1, the iteration can settle on a P that is
// neither N-electron nor idempotent, and its energy is then not variational
// (CH4/sto-3g returned tr(PS) = 10.69, E = -40.305 vs the true -39.727).
fn check_density(n: usize, nelec: usize, P: &[f64], S: &[f64]) -> Result<(), String> {
    let mut trace: f64 = 0.0;
    for mu in 0..n {
        for nu in 0..n {
            trace += P[mu * n + nu] * S[nu * n + mu];
        }
    }
    if (trace - nelec as f64).abs() > P_TOL {
        return Err(format!(
            "converged density has tr(PS) = {:.6}, expected {} electrons",
            trace, nelec
        ));
    }

    let PS = matmult(n, P, S);
    let PSP = matmult(n, &PS, P);
    let mut idem: f64 = 0.0;
    for k in 0..(n * n) {
        idem = idem.max((PSP[k] - 2.0 * P[k]).abs());
    }
    if idem > P_TOL {
        return Err(format!(
            "converged density is not idempotent: max|PSP - 2P| = {:.3e}",
            idem
        ));
    }

    Ok(())
}

// S is symmetric, so the eigendecomposition MUST use the self-adjoint solver.
// faer's general Eigendecomposition returns a basis that is not orthonormal
// across a degenerate eigenvalue, which silently breaks C^dag S C = 1 further
// down: CH4/sto-3g (Td, 4 degenerate pairs) converged to tr(PS) = 10.69 with
// |PSP - 2P| = 1.95, and CH4/def2-svp (18 pairs) never converged at all.
fn find_X(n: usize, S: &[f64]) -> Result<(Vec<f64>, Vec<f64>), String> {
    let s_mat = mat::from_column_major_slice::<f64>(S, n, n);
    let eig_decomp = SelfAdjointEigendecomposition::<f64>::new(s_mat, Side::Lower);
    let eigenvalues = eig_decomp.s();
    let eigenvectors = eig_decomp.u();

    let mut U = vec![0.0; n * n];

    for i in 0..eigenvectors.nrows() {
        for j in 0..eigenvectors.ncols() {
            U[i * n + j] = eigenvectors.read(i, j);
        }
    }

    let eign = eigenvalues.column_vector();
    let mut eig = vec![0.0; n];
    for i in 0..n {
        eig[i] = eign.read(i);
    }

    sort(n, &mut eig, &mut U);

    // s^-1/2 diverges on a (near-)linearly dependent basis. Dropping those
    // vectors -- canonical orthogonalization -- would change the dimension of
    // every downstream matrix, so refuse rather than return a garbage X.
    if eig[0] < S_EIG_MIN {
        return Err(format!(
            "overlap matrix is near-singular at this geometry: smallest \
             eigenvalue {:.3e} < {:.0e} (linearly dependent basis)",
            eig[0], S_EIG_MIN
        ));
    }

    let mut lamb = vec![0.0; n * n];
    for i in 0..n {
        lamb[i * n + i] = eig[i].powf(-0.5);
    }

    let X = matmult(n, &U, &lamb);
    let Xdag = transpose(n, &X);

    Ok((X, Xdag))
}

pub fn calc_F(n: usize, P: &[f64], two: &[f64], H: &[f64]) -> Vec<f64> {
    let mut G = vec![0.0; n * n];
    let mut F = vec![0.0; n * n];

    for mu in 0..n {
        for nu in 0..n {
            for la in 0..n {
                for sig in 0..n {
                    G[mu * n + nu] += P[la * n + sig]
                        * (two[mu * n.pow(3) + nu * n.pow(2) + sig * n + la]
                            - 0.5 * two[mu * n.pow(3) + la * n.pow(2) + sig * n + nu]);
                }
            }

            F[mu * n + nu] += G[mu * n + nu] + H[mu * n + nu];
        }
    }

    F
}

fn calc_Fprime(n: usize, F: &[f64], X: &[f64], Xdag: &[f64]) -> Vec<f64> {
    let inter = matmult(n, Xdag, F);

    matmult(n, &inter, X)
}

// F' = X^dag F X is symmetric; same self-adjoint requirement as find_X. With
// the general solver the occupied block of a degenerate Fock matrix comes back
// non-orthonormal, so calc_P's C C^dag is no longer a projector.
fn diag_F(n: usize, Fprime: &[f64], X: &[f64]) -> Vec<f64> {
    let fprime_mat = mat::from_column_major_slice::<f64>(Fprime, n, n);
    let eig_decomp = SelfAdjointEigendecomposition::<f64>::new(fprime_mat, Side::Lower);
    let eigenvalues = eig_decomp.s();
    let eigenvectors = eig_decomp.u();

    let mut U = vec![0.0; n * n];
    for i in 0..eigenvectors.nrows() {
        for j in 0..eigenvectors.ncols() {
            U[i * n + j] = eigenvectors.read(i, j);
        }
    }

    let eign = eigenvalues.column_vector();
    let mut eig = vec![0.0; n];
    for i in 0..n {
        eig[i] = eign.read(i);
    }

    // ascending eigenvalues -> calc_P's first nelec/2 columns are the aufbau
    // occupied set
    sort(n, &mut eig, &mut U);

    matmult(n, X, &U)
}

fn calc_P(n: usize, nelec: usize, C: &mut [f64]) -> Vec<f64> {
    let mut P = vec![0.0; n * n];
    for mu in 0..n {
        for nu in 0..n {
            for i in 0..(nelec / 2) {
                P[mu * n + nu] += 2.0 * C[mu * n + i] * C[nu * n + i];
            }
        }
    }
    P
}

fn f_delta(n: usize, P: &mut [f64], Pold: &mut [f64]) -> f64 {
    let mut delta: f64 = 0.0;
    for mu in 0..n {
        for nu in 0..n {
            delta += (P[mu * n + nu] - Pold[mu * n + nu]).powf(2.0);
        }
    }
    delta = delta.powf(0.5) / 2.0;
    delta
}

pub fn norm(atm: &mut [i32], env: &mut [f64], i: usize, j: usize) -> f64 {
    let xi: f64 = env[(atm[i * 6 + 1]) as usize];
    let xj: f64 = env[(atm[j * 6 + 1]) as usize];

    let yi: f64 = env[(atm[i * 6 + 1] + 1) as usize];
    let yj: f64 = env[(atm[j * 6 + 1] + 1) as usize];

    let zi: f64 = env[(atm[i * 6 + 1] + 2) as usize];
    let zj: f64 = env[(atm[j * 6 + 1] + 2) as usize];

    ((xi - xj).powf(2.0) + (yi - yj).powf(2.0) + (zi - zj).powf(2.0)).powf(0.5)
}

pub fn density(
    atm: &mut [i32],
    bas: &mut [i32],
    env: &mut [f64],
    nelec: usize,
    imax: i32,
    conv: f64,
) -> Result<Vec<f64>, String> {
    let (natm, nbas) = nmol(atm, bas);
    let nshells = angl(bas, 0);

    let (S, H, two) = integrals(natm, nbas, nshells, atm, bas, env);
    let (X, Xdag) = find_X(nshells, &S)?;

    let mut P = vec![0.0; nshells * nshells];
    for i in 0..nshells {
        for j in 0..nshells {
            if i == j {
                P[i * nshells + j] = 1.0;
            }
        }
    }

    let mut i: i32 = 0;
    let mut delta: f64 = 1.0;

    let mut Pold;
    let mut F;
    let mut Fprime;
    let mut C;

    while delta > conv && i < imax {
        Pold = dcopya(nshells, &P);

        F = calc_F(nshells, &P, &two, &H);
        Fprime = calc_Fprime(nshells, &F, &X, &Xdag);
        C = diag_F(nshells, &Fprime, &X);

        P = calc_P(nshells, nelec, &mut C);
        delta = f_delta(nshells, &mut P, &mut Pold);
        i += 1;
    }

    // Returning a zeroed P here (the old behaviour) is indistinguishable from
    // success at the ctypes boundary: the benchmark recorded gradients of the
    // zero density as ok. Fail instead.
    if delta > conv {
        return Err(format!(
            "SCF did not converge in {} cycles: |dP| = {:.3e} > {:.3e}",
            imax, delta, conv
        ));
    }

    check_density(nshells, nelec, &P, &S)?;

    Ok(P)
}

pub fn energy(atm: &mut [i32], bas: &mut [i32], env: &mut [f64], P: &[f64]) -> f64 {
    let (natm, nbas) = nmol(atm, bas);
    let nshells = angl(bas, 0);

    let (_, H, two) = integrals(natm, nbas, nshells, atm, bas, env);
    let F = calc_F(nshells, P, &two, &H);

    let mut E0: f64 = 0.0;
    for mu in 0..nshells {
        for nu in 0..nshells {
            E0 += 0.5 * P[mu * nshells + nu] * (H[mu * nshells + nu] + F[mu * nshells + nu]);
        }
    }

    let mut Enuc: f64 = 0.0;
    for i in 0..natm {
        for j in 0..natm {
            if i > j {
                Enuc += (atm[i * 6] * atm[j * 6]) as f64 / (norm(atm, env, i, j));
            }
        }
    }

    E0 + Enuc
}

pub fn energyfast(atm: &mut [i32], bas: &mut [i32], env: &mut [f64], P: &[f64]) -> f64 {
    let (natm, nbas) = nmol(atm, bas);
    let nshells = angl(bas, 0);

    let mut buf;
    let mut shls = vec![0; 4];

    let mut mu;
    let mut nu;
    let mut sig;
    let mut lam;

    let mut E0: f64 = 0.0;

    mu = 0;
    for i in 0..nbas {
        shls[0] = i as i32;
        let di = CINTcgto_cart(i, bas) as usize;
        nu = 0;
        for j in 0..nbas {
            shls[1] = j as i32;
            let dj = CINTcgto_cart(j, bas) as usize;

            buf = vec![0.0; di * dj];

            cint1e_kin_cart(
                &mut buf,
                &mut shls,
                atm,
                natm as i32,
                bas,
                nbas as i32,
                env,
                std::ptr::null_mut(),
            );
            let mut c: usize = 0;
            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    E0 += P[mui * nshells + nuj] * buf[c];
                    c += 1;
                }
            }

            cint1e_nuc_cart(
                &mut buf,
                &mut shls,
                atm,
                natm as i32,
                bas,
                nbas as i32,
                env,
                std::ptr::null_mut(),
            );
            let mut c: usize = 0;
            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    E0 += P[mui * nshells + nuj] * buf[c];
                    c += 1;
                }
            }
            nu += dj;
        }
        mu += di;
    }

    mu = 0;
    for i in 0..nbas {
        shls[0] = i as i32;
        let di = CINTcgto_cart(i, bas) as usize;
        nu = 0;
        for j in 0..nbas {
            shls[1] = j as i32;
            let dj = CINTcgto_cart(j, bas) as usize;
            sig = 0;
            for k in 0..nbas {
                shls[2] = k as i32;
                let dk = CINTcgto_cart(k, bas) as usize;
                lam = 0;
                for l in 0..nbas {
                    shls[3] = l as i32;
                    let dl = CINTcgto_cart(l, bas) as usize;

                    buf = vec![0.0; di * dj * dk * dl];

                    cint2e_cart(
                        &mut buf,
                        &mut shls,
                        atm,
                        natm as i32,
                        bas,
                        nbas as i32,
                        env,
                        std::ptr::null_mut(),
                    );
                    let mut c: usize = 0;
                    for laml in lam..(lam + dl) {
                        for sigk in sig..(sig + dk) {
                            for nuj in nu..(nu + dj) {
                                for mui in mu..(mu + di) {
                                    E0 += 0.5
                                        * (P[mui * nshells + nuj] * P[sigk * nshells + laml]
                                            - 0.5
                                                * P[mui * nshells + sigk]
                                                * P[nuj * nshells + laml])
                                        * buf[c];
                                    c += 1;
                                }
                            }
                        }
                    }
                    lam += dl;
                }
                sig += dk;
            }
            nu += dj;
        }
        mu += di;
    }

    let mut Enuc: f64 = 0.0;
    for i in 0..natm {
        for j in 0..natm {
            if i > j {
                Enuc += (atm[i * 6] * atm[j * 6]) as f64 / (norm(atm, env, i, j));
            }
        }
    }

    E0 + Enuc
}

pub fn scf(
    atm: &mut [i32],
    bas: &mut [i32],
    env: &mut [f64],
    nelec: usize,
    imax: i32,
    conv: f64,
) -> Result<f64, String> {
    let mut P = density(atm, bas, env, nelec, imax, conv)?;
    let Etot = energyfast(atm, bas, env, &mut P);
    Ok(Etot)
}
