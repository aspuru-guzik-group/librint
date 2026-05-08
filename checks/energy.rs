#![allow(non_snake_case, non_upper_case_globals,unused_variables,improper_ctypes_definitions,static_mut_refs)]
#![feature(autodiff)]

use std::autodiff::*;
use librint::utils::{read_basis, load_expected, write_expected};
use librint::scf::{nmol, angl};

use librint::cint_bas::CINTcgto_cart;
use librint::cint1e::{cint1e_ovlp_cart, cint1e_nuc_cart};
use librint::intor1::cint1e_kin_cart;
use librint::cint2e::cint2e_cart;
use librint::scf::{norm, density};

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

pub const epsilon: f64 = 1e-12;

#[no_mangle]
pub fn nelec(
    atm: &Vec<i32>
) -> usize {
    // dbg!(atm);
    let mut nelec = 0;
    for i in 0..(atm.len() / ATM_SLOTS) {
        nelec += atm[i * ATM_SLOTS];
    }
    return nelec as usize;
}

#[no_mangle]
// #[autodiff_reverse(dE_rev, Const, Const, Duplicated, Const, Active)]
// #[autodiff_forward(dE_for, Const, Const, Dual, Const, Active)]
pub fn E(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    P: &mut Vec<f64>,
) -> f64 {
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
        shls[0] = i as i32; let di = CINTcgto_cart(i, &bas) as usize;
        nu = 0;
        for j in 0..nbas {
            shls[1] = j as i32; let dj = CINTcgto_cart(j, &bas) as usize;

            buf = vec![0.0; di * dj];

            cint1e_kin_cart(&mut buf, &mut shls, atm, natm as i32, bas, nbas as i32, env, std::ptr::null_mut());
            let mut c: usize = 0;
            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    E0 += P[mui*nshells + nuj] * buf[c];
                    c += 1;
                }
            }

            cint1e_nuc_cart(&mut buf, &mut shls, atm, natm as i32, bas, nbas as i32, env, std::ptr::null_mut());
            let mut c: usize = 0;
            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    E0 += P[mui*nshells + nuj] * buf[c];
                    c += 1;
                }
            }
            nu += dj;
        }
        mu += di;
    }

    mu = 0;
    for i in 0..nbas {
        shls[0] = i as i32; let di = CINTcgto_cart(i, &bas) as usize;
        nu = 0;
        for j in 0..nbas {
            shls[1] = j as i32; let dj = CINTcgto_cart(j, &bas) as usize;
            sig = 0;
            for k in 0..nbas {
                shls[2] = k as i32; let dk = CINTcgto_cart(k, &bas) as usize;
                lam = 0;
                for l in 0..nbas {
                    shls[3] = l as i32; let dl = CINTcgto_cart(l, &bas) as usize;

                    buf = vec![0.0; di * dj * dk * dl];

                    cint2e_cart(&mut buf, &mut shls, atm, natm as i32, bas, nbas as i32, env, std::ptr::null_mut());
                    let mut c: usize = 0;
                    for laml in lam..(lam + dl) {
                        for sigk in sig..(sig + dk) {
                            for nuj in nu..(nu + dj) {
                                for mui in mu..(mu + di) {
                                    E0 += 0.5 * (P[mui*nshells + nuj] * P[sigk*nshells + laml] - 0.5 * P[mui*nshells + sigk] * P[nuj*nshells + laml]) * buf[c];
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
                Enuc += (atm[i*6 + 0] * atm[j*6 + 0]) as f64 / (norm(atm, env, i, j));
            }
        }
    }

    return E0 + Enuc;
}

fn set_molecule(
    path: &str,
) -> (usize, usize, Vec<i32>, Vec<i32>, Vec<f64>) {
    let mut atm: Vec<i32> = Vec::new();
    let mut bas: Vec<i32> = Vec::new();
    let mut env: Vec<f64> = Vec::new();

    _ = read_basis(&path.to_string(), &mut atm, &mut bas, &mut env);

    let nshells = angl(&bas, 0);
    let nelec = nelec(&atm);

    return (nshells, nelec, atm, bas, env);
}

pub fn test_path(
    path: &str,
    exp: &str,
) {
    let (nshells, nelec, mut atm, mut bas, mut env) = set_molecule(path);
    let exp = load_expected(exp);
    let E_exp = exp.get(0).unwrap();

    let mut P = density(&mut atm, &mut bas, &mut env, nelec, 200, 1e-6);
    let mut E = E(&mut atm, &mut bas, &mut env, &mut P);

    if (E - E_exp).abs() > epsilon {
        panic!("Test failed for molecule '{}' exp={} actual={}", path, E_exp, E);
    }
}

// pub fn test_path_rev(
//     path: &str,
//     exp: &str,
// ) {
//     let (nshells, mut atm, mut bas, mut env) = set_molecule(path);
//     let J_exp = load_expected(exp);
//
//     let n = nshells * nshells;
//     let m = env.len();
//
//     let mut J = vec![0.0; n * m];
//
//     let mut out = vec![0.0; n];
//     let mut dout = vec![0.0; n];
//     let mut denv = vec![0.0; m];
//
//     for k in 0..n {
//         denv.fill(0.0);
//         dout[k] = 1.0;
//
//         let mut denv = vec![0.0; m];
//
//         dS_rev(&mut out, &mut dout, &mut atm, &mut bas, &mut env, &mut denv);
//
//         for l in 0..m {
//             J[k * m + l] = denv[l];
//         }
//         dout[k] = 0.0;
//     }
//
//     if J_exp.len() != J.len() {
//         // write_expected(exp, &J);
//         panic!("Different sizes exp={} actual={}", J_exp.len(), J.len());
//     }
//
//     for (a, b) in J.iter().zip(J_exp.iter()) {
//         if (a - b).abs() > epsilon {
//             panic!("Test failed for molecule '{}' exp={} actual={}", path, b, a);
//         }
//     }
// }
//
// pub fn test_path_for(
//     path: &str,
//     exp: &str,
// ) {
//     let (nshells, mut atm, mut bas, mut env) = set_molecule(path);
//     let J_exp = load_expected(exp);
//
//     let n = nshells * nshells;
//     let m = env.len();
//
//     let mut J = vec![0.0; n * m];
//
//     let mut out = vec![0.0; n];
//     let mut dout = vec![0.0; n];
//     let mut denv = vec![0.0; m];
//
//     for l in 0..m {
//         dout.fill(0.0);
//         denv[l] = 1.0;
//
//         dS_for(&mut out, &mut dout, &mut atm, &mut bas, &mut env, &mut denv);
//
//         for k in 0..n {
//             J[k * m + l] = dout[k];
//         }
//         denv[l] = 0.0;
//     }
//
//     if J_exp.len() != J.len() {
//         // write_expected(exp, &J);
//         panic!("Different sizes exp={} actual={}", J_exp.len(), J.len());
//     }
//
//     for (a, b) in J.iter().zip(J_exp.iter()) {
//         if (a - b).abs() > epsilon {
//             panic!("Test failed for molecule '{}' exp={} actual={}", path, b, a);
//         }
//     }
// }
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_h2() {
        let path = "molecules/h2/sto3g.txt".to_string();
        let exp = "checks/truth/h2/sto3g_E.txt".to_string();

        test_path(&path, &exp);
    }

    #[test]
    fn test_energy_h2o() {
        let path = "molecules/h2o/sto3g.txt".to_string();
        let exp = "checks/truth/h2o/sto3g_E.txt".to_string();

        test_path(&path, &exp);
    }

    // #[test]
    // fn test_energy_c6h6() {
    //     let path = "molecules/c6h6/631g.txt".to_string();
    //     let exp = "checks/truth/c6h6/631g_E.txt".to_string();
    //
    //     test_path(&path, &exp);
    // }

    // #[test]
    // fn test_energy_rev_h2() {
    //     let path = "molecules/h2/sto3g.txt".to_string();
    //     let exp = "checks/truth/h2/sto3g_dS_rev.txt".to_string();
    //
    //     test_path_rev(&path, &exp);
    // }
    //
    // #[test]
    // fn test_energy_rev_h2o() {
    //     let path = "molecules/h2o/sto3g.txt".to_string();
    //     let exp = "checks/truth/h2o/sto3g_dS_rev.txt".to_string();
    //
    //     test_path_rev(&path, &exp);
    // }

    // #[test]
    // fn tests_energy_rev_c6h6() {
    //     let path = "molecules/c6h6/631g.txt".to_string();
    //     let exp = "checks/truth/c6h6/631g_dS_rev.txt".to_string();
    //
    //     test_path_rev(&path, &exp);
    // }

    // #[test]
    // fn test_energy_for_h2() {
    //     let path = "molecules/h2/sto3g.txt".to_string();
    //     let exp = "checks/truth/h2/sto3g_dS_for.txt".to_string();
    //
    //     test_path_for(&path, &exp);
    // }
    //
    // #[test]
    // fn test_energy_for_h2o() {
    //     let path = "molecules/h2o/sto3g.txt".to_string();
    //     let exp = "checks/truth/h2o/sto3g_dS_for.txt".to_string();
    //
    //     test_path_for(&path, &exp);
    // }
    //
    // #[test]
    // fn test_energy_for_c6h6() {
    //     let path = "molecules/c6h6/631g.txt".to_string();
    //     let exp = "checks/truth/c6h6/631g_dS_for.txt".to_string();
    //
    //     test_path_for(&path, &exp);
    // }
}
