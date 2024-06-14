#![feature(autodiff)]

use librint::cint_bas::CINTcgto_cart;
use librint::cint1e::cint1e_ovlp_cart;
use librint::cint1e::cint1e_nuc_cart;
use librint::intor1::cint1e_kin_cart;
use librint::cint2e::cint2e_cart;

use faer::mat;
use faer::linalg::solvers::Eigendecomposition;
use faer::complex_native::c64;

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

pub const N_STO: i32 = 3;

fn matmult(
    n: usize,
    A: &mut [f64],
    B: &mut [f64],
    C: &mut [f64],
) {
    // cij = aik bkj
    C.fill(0.0);
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                C[i * n + j] += A[i * n + k] * B[k * n + j];
            }
        }
    }
}

fn transpose(
    n: usize,
    C: &mut [f64],
    Ct: &mut [f64],
) {
    for i in 0..n {
        for j in 0..n {
            Ct[i * n + j] = C[j * n + i]
        }
    }
}

fn dcopy(
    n: usize,
    A: &mut [f64],
    Ac: &mut [f64],
) {
    for i in 0..n {
        for j in 0..n {
            Ac[i * n + j] = A[i * n + j];
        }
    }
}

fn zero(

) {

}

fn print_mat(
    n: usize,
    C: &mut [f64],
) {
    for i in 0..n {
        for j in 0..n {
            print!("{} ", C[i * n + j]);
        }
        println!();
    }
}

fn integrals(
    natm: usize,
    nbas: usize,
    atm: &mut [i32],
    bas: &mut [i32],
    env: &mut [f64],
    S: &mut [f64],
    T: &mut [f64],
    V: &mut [f64],
    H: &mut [f64],
    two: &mut [f64],
) {
    let mut buf;

    let mut shls: [i32; 4] = [0, 0, 0, 0];

    for i in 0..nbas {
        for j in 0..nbas {
            shls[0] = i as i32;
            shls[1] = j as i32;
            
            let di = CINTcgto_cart(i as usize, &bas);
            let dj = CINTcgto_cart(j as usize, &bas);

            buf = vec![0.0; (di * dj) as usize];
            cint1e_ovlp_cart(&mut buf, &mut shls, atm, natm as i32, bas, nbas as i32, env);
            S[i*nbas + j] = buf[0];

            cint1e_kin_cart(&mut buf, &mut shls, atm, natm as i32, bas, nbas as i32, env);
            T[i*nbas + j] = buf[0];

            cint1e_nuc_cart(&mut buf, &mut shls, atm, natm as i32, bas, nbas as i32, env);
            V[i*nbas + j] = buf[0];

            H[i*nbas + j] = T[i*nbas + j] + V[i*nbas + j];

            for k in 0..nbas {
                for l in 0..nbas {
                    shls[2] = k as i32;
                    shls[3] = l as i32;
        
                    let dk = CINTcgto_cart(k as usize, &bas);
                    let dl = CINTcgto_cart(l as usize, &bas);

                    buf = vec![0.0; (di * dj * dk * dl) as usize];
                    cint2e_cart(&mut buf, &mut shls, atm, natm as i32, bas, nbas as i32, env);
                    two[i*nbas.pow(3) + j*nbas.pow(2) + k*nbas.pow(1) + l] = buf[0];
                }
            }
        }
    }
}

fn find_X(
    nbas: usize,
    S: &mut [f64],
    X: &mut [f64],
    Xdag: &mut [f64],
) {
    let s_mat = mat::from_column_major_slice::<f64>(&S, nbas, nbas);
    let eig_decomp: Eigendecomposition<c64> = Eigendecomposition::new_from_real(s_mat);
    let eigenvalues = eig_decomp.s();
    let eigenvectors = eig_decomp.u();

    let mut U = vec![0.0; nbas * nbas];

    for i in 0..eigenvectors.nrows() {
        for j in 0..eigenvectors.ncols() {
            U[i * nbas + j] = eigenvectors.read(i, j).re();
        }
    }

    let eign = eigenvalues.column_vector();
    let mut lamb = vec![0.0; nbas * nbas];
    for i in 0..nbas {
        for j in 0..nbas {
            if i == j {
                let li: f64 = eign.read(i).re();
                lamb[i * nbas + j] = li.powf(-0.5);
            }
        }
    }

    matmult(nbas, &mut U, &mut lamb, X);
    transpose(nbas, X, Xdag);
}

fn calc_F(
    nbas: usize,
    P: &mut [f64],
    two: &mut [f64],
    H: &mut [f64],
    G: &mut [f64],
    F: &mut [f64],
) {
    G.fill(0.0);
    F.fill(0.0);

    for mu in 0..nbas {
        for nu in 0..nbas {
            for la in 0..nbas {
                for sig in 0..nbas {
                    G[mu*nbas + nu] += P[la*nbas + sig]
                        * (two[mu*nbas.pow(3) + nu*nbas.pow(2) + sig*nbas + la]
                        - 0.5 * two[mu*nbas.pow(3) + la*nbas.pow(2) + sig*nbas + nu]);
                }
            }

            F[mu*nbas + nu] += G[mu*nbas + nu] + H[mu*nbas + nu];
        }
    }
}

fn calc_Fprime(
    nbas: usize,
    F: &mut [f64],
    X: &mut [f64],
    Xdag: &mut [f64],
    mut Fprime: &mut [f64],
) {
    let mut inter = vec![0.0; nbas * nbas];
    matmult(nbas, Xdag, F, &mut inter);
    matmult(nbas, &mut inter, X, &mut Fprime);
}

fn diag_F(
    nbas: usize,
    Fprime: &mut [f64],
    X: &mut [f64],
    C: &mut [f64],
    epsilon: &mut [f64],
) {
    // diagonalize Fprime
    let fprime_mat = mat::from_column_major_slice::<f64>(&Fprime, nbas, nbas);
    let eig_decomp: Eigendecomposition<c64> = Eigendecomposition::new_from_real(fprime_mat);
    let eigenvalues = eig_decomp.s();
    let eigenvectors = eig_decomp.u();

    let mut U = vec![0.0; nbas * nbas];
    for i in 0..eigenvectors.nrows() {
        for j in 0..eigenvectors.ncols() {
            U[i * nbas + j] = eigenvectors.read(i, j).re();
        }
    }

    let mut Udag = vec![0.0; nbas * nbas];
    transpose(nbas, &mut U, &mut Udag);

    let mut inter = vec![0.0; nbas * nbas];
    matmult(nbas, &mut Udag, Fprime, &mut inter); 

    let mut f = vec![0.0; nbas * nbas];
    matmult(nbas, &mut inter, &mut U, &mut f);

    let mut Cprime = vec![0.0; nbas * nbas];
    dcopy(nbas, &mut U, &mut Cprime);

    dcopy(nbas, &mut f, epsilon);

    matmult(nbas, X, &mut Cprime, C);
}

fn calc_P(
    nbas: usize,
    nelec: usize,
    C: &mut [f64],
) -> Vec<f64> {
    let mut P = vec![0.0; nbas * nbas];
    for mu in 0..nbas {
        for nu in 0..nbas {
            for i in 0..(nelec/2) {
                P[mu * nbas + nu] += 2.0 * C[mu * nbas + i] * C[nu * nbas + i];
            }
        }
    }
    return P;
}

fn f_delta(
    nbas: usize,
    P: &mut [f64],
    Pold: &mut [f64],
) -> f64 {
    let mut delta: f64 = 0.0;
    for mu in 0..nbas {
        for nu in 0..nbas {
            delta += (P[mu * nbas + nu] - Pold[mu * nbas + nu]).powf(2.0);
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

fn RHF(
    natm: usize,
    nbas: usize,
    nelec: usize,
    atm: &mut [i32],
    bas: &mut [i32],
    env: &mut [f64],
    imax: i32,
    conv: f64,
) -> Vec<f64> {
    let mut S = vec![0.0; nbas * nbas];
    let mut T = vec![0.0; nbas * nbas];
    let mut V = vec![0.0; nbas * nbas];
    let mut H = vec![0.0; nbas * nbas];
    let mut two = vec![0.0; nbas * nbas * nbas * nbas];
    integrals(natm, nbas, atm, bas, env, &mut S, &mut T, &mut V, &mut H, &mut two);

    let mut X = vec![0.0; nbas * nbas];
    let mut Xdag = vec![0.0; nbas * nbas];
    find_X(nbas, &mut S, &mut X, &mut Xdag);

    let mut P = vec![0.0; nbas * nbas];
    for i in 0..nbas {
        for j in 0..nbas {
            if i == j {
                P[i*nbas + j] = 1.0;
            }
        }
    }

    let mut i: i32 = 0;
    let mut delta: f64 = 1.0;

    let mut Pold = vec![0.0; nbas * nbas];
    let mut G = vec![0.0; nbas * nbas];
    let mut F = vec![0.0; nbas * nbas];
    let mut Fprime = vec![0.0; nbas * nbas];
    let mut C = vec![0.0; nbas * nbas];
    let mut epsilon = vec![0.0; nbas * nbas];

    while delta > conv && i < imax {
        dcopy(nbas, &mut P, &mut Pold);

        calc_F(nbas, &mut P, &mut two, &mut H, &mut G, &mut F);
        calc_Fprime(nbas, &mut F, &mut X, &mut Xdag, &mut Fprime);
        diag_F(nbas, &mut Fprime, &mut X, &mut C, &mut epsilon);

        P = calc_P(nbas, nelec, &mut C);
        delta = f_delta(nbas, &mut P, &mut Pold);
        i += 1;
    }

    println!("Conv P:");
    for i in 0..nbas {
        for j in 0..nbas {
            print!("{} ", P[i*nbas + j]);
        }
        println!();
    }

    if delta > conv && i == imax {
        println!("did not converge");
    }

    return P;
}

#[autodiff(denergy, Reverse, Duplicated, Const, Const, Const, Const, Duplicated, Const)]
fn energy(
    E: &mut f64,
    natm: usize,
    nbas: usize,
    atm: &mut [i32],
    bas: &mut [i32],
    env: &mut [f64],
    P: &mut [f64],
) {
    let mut S = vec![0.0; nbas * nbas];
    let mut T = vec![0.0; nbas * nbas];
    let mut V = vec![0.0; nbas * nbas];
    let mut H = vec![0.0; nbas * nbas];
    let mut two = vec![0.0; nbas * nbas * nbas * nbas];
    integrals(natm, nbas, atm, bas, env, &mut S, &mut T, &mut V, &mut H, &mut two);

    let mut G = vec![0.0; nbas * nbas];
    let mut F = vec![0.0; nbas * nbas];
    calc_F(nbas, P, &mut two, &mut H, &mut G, &mut F);

    let mut E0: f64 = 0.0;
    for mu in 0..nbas {
        for nu in 0..nbas {
            E0 += 0.5 * P[mu*nbas + nu] * (H[mu*nbas + nu]  + F[mu*nbas + nu]);
        }
    }

    let mut Enuc: f64 = 0.0;
    for i in 0..nbas {
        for j in 0..nbas {
            if i > j {
                Enuc += (atm[i*6 + 0] * atm[j*6 + 0]) as f64 / (norm(atm, env, i, j));
            }
        }
    }

    *E = Enuc + E0;
}

fn main() {
    const natm: usize = 2;
    const nbas: usize = 2;

    const nelec: usize = 2;

    let mut atm: [i32; natm * ATM_SLOTS] = [1, 20, 1, 23, 0, 0, 1, 24, 1, 27, 0, 0];
    let mut bas: [i32; nbas * BAS_SLOTS] = [0, 0, 3, 1, 0, 28, 31, 0, 1, 0, 3, 1, 0, 28, 31, 0];
    let mut env: [f64; 34] = [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 
        -1.5117809, 0., 0., 0., 1.5117809, 0., 3.42525091, 0.62391373, 0.1688554, 0.98170675, 0.94946401, 0.29590645];
    let mut denv: [f64; 34] = [0.0; 34];

    const imax: i32 = 2;
    const conv: f64 = 0.000001;

    let mut P = RHF(natm, nbas, nelec, &mut atm, &mut bas, &mut env, imax, conv);

    let mut Etot: f64 = 0.0;
    let mut dEtot: f64 = 1.0;
    denergy(&mut Etot, &mut dEtot, natm, nbas, &mut atm, &mut bas, &mut env, &mut denv, &mut P);
    println!("Etot: {}", Etot);

    println!("denv:");
    for i in 28..36 {
        print!("{} ", denv[i]);
    }
}