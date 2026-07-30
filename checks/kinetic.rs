#![allow(
    non_snake_case,
    non_upper_case_globals,
    unused_variables,
    improper_ctypes_definitions,
    static_mut_refs
)]
#![feature(autodiff)]

use librint::scf::{angl, nmol};
use librint::utils::{load_expected, read_basis, write_expected};
use std::autodiff::*;

use librint::cint_bas::CINTcgto_cart;
use librint::intor1::cint1e_kin_cart;

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

pub const epsilon: f64 = 1e-12;

#[no_mangle]
#[autodiff_reverse(dT_rev, Duplicated, Const, Const, Duplicated)]
#[autodiff_forward(dT_for, Dual, Const, Const, Dual)]
pub fn matrix(out: &mut Vec<f64>, atm: &mut Vec<i32>, bas: &mut Vec<i32>, env: &mut Vec<f64>) {
    let (natm, nbas) = nmol(atm, bas);
    let nshells = angl(bas, 0);

    let mut shls_buf = vec![0i32; 4];

    let mut mu = 0;
    for i in 0..nbas {
        shls_buf[0] = i as i32;
        let di = CINTcgto_cart(i, &bas) as usize;
        let mut nu = 0;
        for j in 0..nbas {
            shls_buf[1] = j as i32;
            let dj = CINTcgto_cart(j, &bas) as usize;

            let mut buf = vec![0.0; di * dj];
            cint1e_kin_cart(
                &mut buf,
                &mut shls_buf,
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
                    out[nuj * nshells + mui] = buf[c];
                    c += 1;
                }
            }
            nu += dj;
        }
        mu += di;
    }
}

fn set_molecule(path: &str) -> (usize, Vec<i32>, Vec<i32>, Vec<f64>) {
    let mut atm: Vec<i32> = Vec::new();
    let mut bas: Vec<i32> = Vec::new();
    let mut env: Vec<f64> = Vec::new();

    _ = read_basis(&path.to_string(), &mut atm, &mut bas, &mut env);

    let nshells = angl(&bas, 0);

    return (nshells, atm, bas, env);
}

pub fn test_path(path: &str, exp: &str) {
    let (nshells, mut atm, mut bas, mut env) = set_molecule(path);
    let M_exp = load_expected(exp);

    let mut M = vec![0.0; nshells * nshells];
    matrix(&mut M, &mut atm, &mut bas, &mut env);

    if M_exp.len() != M.len() {
        // write_expected(exp, &M);
        panic!("Different sizes exp={} actual={}", M_exp.len(), M.len());
    }

    for (a, b) in M.iter().zip(M_exp.iter()) {
        if (a - b).abs() > epsilon {
            panic!("Test failed for molecule '{}' exp={} actual={}", path, b, a);
        }
    }
}

pub fn test_path_rev(path: &str, exp: &str) {
    let (nshells, mut atm, mut bas, mut env) = set_molecule(path);
    let J_exp = load_expected(exp);

    let n = nshells * nshells;
    let m = env.len();

    let mut J = vec![0.0; n * m];

    let mut out = vec![0.0; n];
    let mut dout = vec![0.0; n];
    let mut denv = vec![0.0; m];

    for k in 0..n {
        denv.fill(0.0);
        dout[k] = 1.0;

        let mut denv = vec![0.0; m];

        dT_rev(&mut out, &mut dout, &mut atm, &mut bas, &mut env, &mut denv);

        for l in 0..m {
            J[k * m + l] = denv[l];
        }
        dout[k] = 0.0;
    }

    if J_exp.len() != J.len() {
        // write_expected(exp, &J);
        panic!("Different sizes exp={} actual={}", J_exp.len(), J.len());
    }

    for (a, b) in J.iter().zip(J_exp.iter()) {
        if (a - b).abs() > epsilon {
            panic!("Test failed for molecule '{}' exp={} actual={}", path, b, a);
        }
    }
}

pub fn test_path_for(path: &str, exp: &str) {
    let (nshells, mut atm, mut bas, mut env) = set_molecule(path);
    let J_exp = load_expected(exp);

    let n = nshells * nshells;
    let m = env.len();

    let mut J = vec![0.0; n * m];

    let mut out = vec![0.0; n];
    let mut dout = vec![0.0; n];
    let mut denv = vec![0.0; m];

    for l in 0..m {
        dout.fill(0.0);
        denv[l] = 1.0;

        dT_for(&mut out, &mut dout, &mut atm, &mut bas, &mut env, &mut denv);

        for k in 0..n {
            J[k * m + l] = dout[k];
        }
        denv[l] = 0.0;
    }

    if J_exp.len() != J.len() {
        // write_expected(exp, &J);
        panic!("Different sizes exp={} actual={}", J_exp.len(), J.len());
    }

    for (a, b) in J.iter().zip(J_exp.iter()) {
        if (a - b).abs() > epsilon {
            panic!("Test failed for molecule '{}' exp={} actual={}", path, b, a);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kinetic_h2() {
        let path = "molecules/h2/sto3g.txt".to_string();
        let exp = "checks/truth/h2/sto3g_T.txt".to_string();

        test_path(&path, &exp);
    }

    #[test]
    fn test_kinetic_h2o() {
        let path = "molecules/h2o/sto3g.txt".to_string();
        let exp = "checks/truth/h2o/sto3g_T.txt".to_string();

        test_path(&path, &exp);
    }

    #[test]
    fn test_kinetic_c6h6() {
        let path = "molecules/c6h6/631g.txt".to_string();
        let exp = "checks/truth/c6h6/631g_T.txt".to_string();

        test_path(&path, &exp);
    }

    #[test]
    fn test_kinetic_rev_h2() {
        let path = "molecules/h2/sto3g.txt".to_string();
        let exp = "checks/truth/h2/sto3g_dT_rev.txt".to_string();

        test_path_rev(&path, &exp);
    }

    #[test]
    fn test_kinetic_rev_h2o() {
        let path = "molecules/h2o/sto3g.txt".to_string();
        let exp = "checks/truth/h2o/sto3g_dT_rev.txt".to_string();

        test_path_rev(&path, &exp);
    }

    // #[test]
    // fn tests_kinetic_rev_c6h6() {
    //     let path = "molecules/c6h6/631g.txt".to_string();
    //     let exp = "checks/truth/c6h6/631g_dT_rev.txt".to_string();
    //
    //     test_path_rev(&path, &exp);
    // }

    #[test]
    fn test_kinetic_for_h2() {
        let path = "molecules/h2/sto3g.txt".to_string();
        let exp = "checks/truth/h2/sto3g_dT_for.txt".to_string();

        test_path_for(&path, &exp);
    }

    #[test]
    fn test_kinetic_for_h2o() {
        let path = "molecules/h2o/sto3g.txt".to_string();
        let exp = "checks/truth/h2o/sto3g_dT_for.txt".to_string();

        test_path_for(&path, &exp);
    }

    #[test]
    fn test_kinetic_for_c6h6() {
        let path = "molecules/c6h6/631g.txt".to_string();
        let exp = "checks/truth/c6h6/631g_dT_for.txt".to_string();

        test_path_for(&path, &exp);
    }
}
