#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#![feature(autodiff)]

use std::io;

use librint::utils::read_basis;

use librint::cint_bas::CINTcgto_cart;
use librint::cint1e::cint1e_ovlp_cart;
use librint::cint1e::cint1e_nuc_cart;
use librint::intor1::cint1e_kin_cart;
use librint::cint2e::cint2e_cart;

use librint::cint_bas::CINTcgto_spheric;
use librint::cint1e::cint1e_ovlp_sph;
use librint::cint1e::cint1e_nuc_sph;
use librint::intor1::cint1e_kin_sph;
use librint::cint2e::cint2e_sph;

use faer::mat;
use faer::linalg::solvers::Eigendecomposition;
use faer::complex_native::c64;

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

type inte = fn(buf: &mut [f64], shls: &mut [i32], 
    atm: &mut [i32], natm: i32, 
    bas: &mut [i32], nbas: i32, 
    env: &mut [f64]) -> i32;

type cgto = fn(bas_id: usize, bas: &[i32],) -> i32;

fn matmult(
    n: usize,
    A: &[f64],
    B: &[f64],
) -> Vec<f64> {
    // cij = aik bkj
    let mut C = vec![0.0; n * n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                C[i * n + j] += A[i * n + k] * B[k * n + j];
            }
        }
    }
    return C;
}

fn transpose(
    n: usize,
    C: &[f64],
) -> Vec<f64> {
    let mut Ct = vec![0.0; n * n];
    for i in 0..n {
        for j in 0..n {
            Ct[i * n + j] = C[j * n + i]
        }
    }
    return Ct;
}

fn dcopy(
    n: usize,
    A: &[f64],
) -> Vec<f64> {
    let mut Ac = vec![0.0; n * n];
    for i in 0..n {
        for j in 0..n {
            Ac[i * n + j] = A[i * n + j];
        }
    }
    return Ac;
}

fn sort(
    n: usize,
    eig: &mut [f64],
    U: &mut [f64],
) {
    for i in (0..n).rev() {
        for j in 0..i {
            if eig[j] > eig[j+1] {
                let tmp = eig[j];
                eig[j] = eig[j+1];
                eig[j+1] = tmp;

                for k in 0..n {
                    let tmp = U[k*n + j];
                    U[k*n + j] = U[k*n + j+1];
                    U[k*n + j+1] = tmp;
                }
            }
        }
    }
}

#[no_mangle]
pub fn nparams(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
) 
-> (usize, usize, usize) {
    let natm: usize = atm.len() / ATM_SLOTS;
    let nbas: usize = bas.len() / BAS_SLOTS;

    let mut nshells: usize = 0;
    for i in (1..bas.len()).step_by(BAS_SLOTS) {
        nshells += (2*bas[i] + 1) as usize;
    }

    return (natm, nbas, nshells);
}

#[no_mangle]
pub fn split(
    bas: &mut Vec<i32>,
) -> (usize, usize) {
    let mut min = -1;
    let mut max = -1;

    for b in (0..bas.len()).step_by(BAS_SLOTS) {
        let ngto = bas[b + 2];
        let exp = bas[b + 5];
        let cont = bas[b + 6];

        if min == -1 {
            min = exp;
        } else if min > exp {
            min = exp;
        }

        if max == -1 {
            max = cont + ngto;
        } else if max < cont {
            max = cont + ngto;
        }
    }

    return (min as usize, max as usize);
}

#[no_mangle]
pub fn integral1e(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    coord: i32,
    typec: i32,
) -> Vec<f64> {
    let (natm, nbas, nshells) = nparams(atm, bas);
    
    let mut R = vec![0.0; nshells * nshells];

    let mut buf: Vec<f64>;
    let mut shls: [i32; 4] = [0, 0, 0, 0];

    let mut mu;
    let mut nu;

    let mut di;
    let mut dj;

    let intcgto: cgto;
    let func: inte;

    if coord == 0 {
        intcgto = CINTcgto_cart;

        if typec == 0 {
            func = cint1e_ovlp_cart;
        } else if typec == 1 {
            func = cint1e_kin_cart;
        } else if typec == 2 {
            func = cint1e_nuc_cart;
        } else {
            std::process::exit(1);
        }
    } else if coord == 1 {
        intcgto = CINTcgto_spheric;

        if typec == 0 {
            func = cint1e_ovlp_sph;
        } else if typec == 1 {
            func = cint1e_kin_sph;
        } else if typec == 2 {
            func = cint1e_nuc_sph;
        } else {
            std::process::exit(1);
        }
    } else {
        std::process::exit(1);
    }

    mu = 0;
    for i in 0..nbas {
        shls[0] = i as i32;
        di = intcgto(i, &bas) as usize;

        nu = 0;
        for j in 0..nbas {
            shls[1] = j as i32;
            dj = intcgto(j, &bas) as usize;

            buf = vec![0.0; di * dj];

            func(&mut buf, &mut shls, atm, natm as i32, bas, nbas as i32, env);
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
    
    return R;
}

#[no_mangle]
pub fn integral2e(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    coord: i32,
) -> Vec<f64> {
    let (natm, nbas, nshells) = nparams(atm, bas);

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

    let intcgto: cgto;
    let func: inte;

    if coord == 0 {
        intcgto = CINTcgto_cart;
        func = cint2e_cart;
    } else if coord == 1 {
        intcgto = CINTcgto_spheric;
        func = cint2e_sph;
    } else {
        std::process::exit(1);
    }

    mu = 0;
    for i in 0..nbas {
        shls[0] = i as i32;
        di = intcgto(i, &bas) as usize;

        nu = 0;
        for j in 0..nbas {
            shls[1] = j as i32;
            dj = intcgto(j, &bas) as usize;

            sig = 0;
            for k in 0..nbas {
                shls[2] = k as i32;
                dk = intcgto(k, &bas) as usize;
                
                lam = 0;
                for l in 0..nbas {
                    shls[3] = l as i32;
                    dl = intcgto(l, &bas) as usize;
                    
                    buf = vec![0.0; di * dj * dk * dl];
        
                    func(&mut buf, &mut shls, atm, natm as i32, bas, nbas as i32, env);
                    let mut c: usize = 0;
                    for laml in lam..(lam + dl) {
                        for sigk in sig..(sig + dk) {
                            for nuj in nu..(nu + dj) {
                                for mui in mu..(mu + di) {
                                    R[mui*nshells.pow(3) + nuj*nshells.pow(2) + sigk*nshells + laml] = buf[c];
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

    return R;
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
        di = CINTcgto_cart(i, &bas) as usize;

        nu = 0;
        for j in 0..nbas {
            sig = 0;

            shls[1] = j as i32;
            dj = CINTcgto_cart(j, &bas) as usize;

            buf = vec![0.0; di * dj];

            cint1e_ovlp_cart(&mut buf, &mut shls, atm, natm as i32, bas, nbas as i32, env);
            let mut c: usize = 0;
            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    S[mui * nshells + nuj] = buf[c];
                    c += 1;
                }
            }

            cint1e_kin_cart(&mut buf, &mut shls, atm, natm as i32, bas, nbas as i32, env);
            let mut c: usize = 0;
            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    T[mui * nshells + nuj] = buf[c];
                    c += 1;
                }
            }

            cint1e_nuc_cart(&mut buf, &mut shls, atm, natm as i32, bas, nbas as i32, env);
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
                dk = CINTcgto_cart(k, &bas) as usize;

                lam = 0;
                for l in 0..nbas {
                    shls[3] = l as i32;
                    dl = CINTcgto_cart(l, &bas) as usize;
                    
                    buf = vec![0.0; di * dj * dk * dl];
        
                    cint2e_cart(&mut buf, &mut shls, atm, natm as i32, bas, nbas as i32, env);
                    let mut c: usize = 0;
                    for laml in lam..(lam + dl) {
                        for sigk in sig..(sig + dk) {
                            for nuj in nu..(nu + dj) {
                                for mui in mu..(mu + di) {
                                    two[mui*nshells.pow(3) + nuj*nshells.pow(2) + sigk*nshells + laml] = buf[c];
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
    
    return (S, H, two);
}

fn find_X(
    n: usize,
    S: &[f64],
) -> (Vec<f64>, Vec<f64>) {
    let s_mat = mat::from_column_major_slice::<f64>(&S, n, n);
    let eig_decomp: Eigendecomposition<c64> = Eigendecomposition::new_from_real(s_mat);
    let eigenvalues = eig_decomp.s();
    let eigenvectors = eig_decomp.u();

    let mut U = vec![0.0; n * n];

    for i in 0..eigenvectors.nrows() {
        for j in 0..eigenvectors.ncols() {
            U[i * n + j] = eigenvectors.read(i, j).re();
        }
    }

    let eign = eigenvalues.column_vector();
    let mut eig = vec![0.0; n];
    for i in 0..n {
        eig[i] = eign.read(i).re();
    }

    sort(n, &mut eig, &mut U);
    
    let mut lamb = vec![0.0; n * n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                let li: f64 = eig[i];
                lamb[i * n + j] = li.powf(-0.5);
            }
        }
    }

    let X = matmult(n, &U, &lamb);
    let Xdag = transpose(n, &X);

    return (X, Xdag);
}

fn calc_F(
    n: usize,
    P: &[f64],
    two: &[f64],
    H: &[f64],
) -> Vec<f64> {
    let mut G = vec![0.0; n * n];
    let mut F = vec![0.0; n * n];

    for mu in 0..n {
        for nu in 0..n {
            for la in 0..n {
                for sig in 0..n {
                    G[mu*n + nu] += P[la*n + sig]
                        * (two[mu*n.pow(3) + nu*n.pow(2) + sig*n + la]
                        - 0.5 * two[mu*n.pow(3) + la*n.pow(2) + sig*n + nu]);
                }
            }

            F[mu*n + nu] += G[mu*n + nu] + H[mu*n + nu];
        }
    }

    return F;
}

fn calc_Fprime(
    n: usize,
    F: &[f64],
    X: &[f64],
    Xdag: &[f64],
) -> Vec<f64> {
    let inter = matmult(n, &Xdag, &F);
    let Fprime = matmult(n, &inter, X);
    return Fprime;
}

fn diag_F(
    n: usize,
    Fprime: &[f64],
    X: &[f64],
) -> Vec<f64> {
    let fprime_mat = mat::from_column_major_slice::<f64>(&Fprime, n, n);
    let eig_decomp: Eigendecomposition<c64> = Eigendecomposition::new_from_real(fprime_mat);
    let eigenvalues = eig_decomp.s();
    let eigenvectors = eig_decomp.u();

    let mut U = vec![0.0; n * n];
    for i in 0..eigenvectors.nrows() {
        for j in 0..eigenvectors.ncols() {
            U[i * n + j] = eigenvectors.read(i, j).re();
        }
    }

    let eign = eigenvalues.column_vector();
    let mut eig = vec![0.0; n];
    for i in 0..n {
        eig[i] = eign.read(i).re();
    }

    sort(n, &mut eig, &mut U);

    let Udag = transpose(n, &U);
    let inter = matmult(n, &Udag, Fprime); 
    let f = matmult(n, &inter, &U);
    let Cprime = dcopy(n, &U);

    let _epsilon = dcopy(n, &f);
    let C = matmult(n, X, &Cprime);

    return C;
}

fn calc_P(
    n: usize,
    nelec: usize,
    C: &mut [f64],
) -> Vec<f64> {
    let mut P = vec![0.0; n * n];
    for mu in 0..n {
        for nu in 0..n {
            for i in 0..(nelec/2) {
                P[mu * n + nu] += 2.0 * C[mu * n + i] * C[nu * n + i];
            }
        }
    }
    return P;
}

fn f_delta(
    n: usize,
    P: &mut [f64],
    Pold: &mut [f64],
) -> f64 {
    let mut delta: f64 = 0.0;
    for mu in 0..n {
        for nu in 0..n {
            delta += (P[mu * n + nu] - Pold[mu * n + nu]).powf(2.0);
        }
    }
    delta = delta.powf(0.5) / 2.0;
    return delta;
}

fn norm(
    atm: &mut [i32],
    env: &mut [f64],
    i: usize,
    j: usize,
) -> f64 {
    let xi: f64 = env[(atm[i*6 + 1]) as usize];
    let xj: f64 = env[(atm[j*6 + 1]) as usize];

    let yi: f64 = env[(atm[i*6 + 1] + 1) as usize];
    let yj: f64 = env[(atm[j*6 + 1] + 1) as usize];

    let zi: f64 = env[(atm[i*6 + 1] + 2) as usize];
    let zj: f64 = env[(atm[j*6 + 1] + 2) as usize];

    return ((xi - xj).powf(2.0) + (yi - yj).powf(2.0) + (zi - zj).powf(2.0)).powf(0.5);
}

#[no_mangle]
#[autodiff(dscf, Reverse, Const, Const, Duplicated, Const, Const, Const, Active)]
pub fn SCF(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    nelec: usize,
    imax: i32,
    conv: f64,
) -> f64 {
    let (natm, nbas, nshells) = nparams(atm, bas);

    let (S, H, two) = integrals(natm, nbas, nshells, atm, bas, env);

    let (X, Xdag) = find_X(nshells, &S);

    let mut P = vec![0.0; nshells * nshells];
    for i in 0..nshells {
        for j in 0..nshells {
            if i == j {
                P[i*nshells + j] = 1.0;
            }
        }
    }

    let mut i: i32 = 0;
    let mut delta: f64 = 1.0;

    let mut Pold;
    let mut F = vec![0.0; nshells * nshells];
    let mut Fprime; 
    let mut C;

    for i in 0..imax {
        Pold = dcopy(nshells, &P);

        F = calc_F(nshells, &P, &two, &H);
        Fprime = calc_Fprime(nshells, &F, &X, &Xdag);
        C = diag_F(nshells, &Fprime, &X);

        P = calc_P(nshells, nelec, &mut C);
        delta = f_delta(nshells, &mut P, &mut Pold);

        if delta <= conv {
            break;
        }
    }

    if delta > conv && i == imax {
        println!("did not converge");
        for i in 0..nshells {
            for j in 0..nshells {
                P[i*nshells + j] = 0.0;
            }
        }
    }

    let mut E0: f64 = 0.0;
    for mu in 0..nshells {
        for nu in 0..nshells {
            E0 += 0.5 * P[mu*nshells + nu] * (H[mu*nshells + nu]  + F[mu*nshells + nu]);
        }
    }

    let mut Enuc: f64 = 0.0;
    for i in 0..natm {
        for j in 0..natm {
            if i > j {
                Enuc += (atm[i*6 + 0] * atm[j*6 + 0]) as f64 / (norm(atm, env, i, j));
            }
        }
    }

    return E0 + Enuc;
}


fn main() -> io::Result<()> {
    let mut atm = Vec::new();
    let mut bas = Vec::new();
    let mut env = Vec::new();

    let path = "/u/jpmedina/libcint/librint/molecules/h2/sto3g.txt";
    read_basis(path, &mut atm, &mut bas, &mut env)?;

    let nelec = 2;

    let E = SCF(&mut atm, &mut bas, &mut env, nelec, 20, 1e-6);
    println!("E: {}", E);

    let mut denv = vec![0.0; env.len()];
    let _ = dscf(&mut atm, &mut bas, &mut env, &mut denv, nelec, 20, 1e-6, 1.0);

    println!("{:.5?}", denv);

    Ok(())
}