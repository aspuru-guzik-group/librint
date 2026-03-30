#![allow(non_snake_case, non_upper_case_globals,unused_variables,improper_ctypes_definitions,static_mut_refs)]
#![feature(autodiff)]

use std::autodiff::*;
use librint::utils::{read_basis, load_expected, write_expected};
use librint::scf::{nmol, angl};

use librint::cint_bas::CINTcgto_cart;
use librint::cint1e::cint1e_ovlp_cart;

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

pub const epsilon: f64 = 1e-12;

#[no_mangle]
#[autodiff_reverse(dS_rev, Duplicated, Const, Const, Duplicated)]
// #[autodiff_forward(dS_for, Dual, Const, Const, Dual)]
pub fn S_matrix(
    out: &mut Vec<f64>,
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
) {
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
            cint1e_ovlp_cart(&mut buf, &mut shls_buf, atm, natm as i32, bas, nbas as i32, env, std::ptr::null_mut());

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

fn set_molecule(
    path: &str,
) -> (usize, Vec<i32>, Vec<i32>, Vec<f64>) {
    let mut atm: Vec<i32> = Vec::new();
    let mut bas: Vec<i32> = Vec::new();
    let mut env: Vec<f64> = Vec::new();

    _ = read_basis(&path.to_string(), &mut atm, &mut bas, &mut env);

    let nshells = angl(&bas, 0);

    return (nshells, atm, bas, env);
}

pub fn test_path(
    path: &str,
    exp: &str,
) {
    let (nshells, mut atm, mut bas, mut env) = set_molecule(path);
    let S_exp = load_expected(exp);

    let mut S = vec![0.0; nshells * nshells];
    S_matrix(&mut S, &mut atm, &mut bas, &mut env);

    if S_exp.len() != S.len() {
        panic!("Different sizes exp={} actual={}", S_exp.len(), S.len());
    }

    for (a, b) in S.iter().zip(S_exp.iter()) {
        if (a - b).abs() > epsilon {
            panic!("Test failed for molecule '{}' exp={} actual={}", path, b, a);
        }
    }
}

pub fn test_path_rev(
    path: &str,
    exp: &str,
) {
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

        dS_rev(&mut out, &mut dout, &mut atm, &mut bas, &mut env, &mut denv);

        for l in 0..m {
            J[k * m + l] = denv[l];
        }
        dout[k] = 0.0;
    }

    if J_exp.len() != J.len() {
        write_expected(exp, &J);
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
    fn test_overlap_h2() {
        let path = "molecules/h2/sto3g.txt".to_string();
        let exp = "checks/truth/h2/sto3g_S.txt".to_string();

        test_path(&path, &exp);
    }

    #[test]
    fn test_overlap_h2o() {
        let path = "molecules/h2o/sto3g.txt".to_string();
        let exp = "checks/truth/h2o/sto3g_S.txt".to_string();

        test_path(&path, &exp);
    }

    #[test]
    fn test_overlap_c6h6() {
        let path = "molecules/c6h6/631g.txt".to_string();
        let exp = "checks/truth/c6h6/631g_S.txt".to_string();

        test_path(&path, &exp);
    }

    #[test]
    fn test_doverlap_h2() {
        let path = "molecules/h2/sto3g.txt".to_string();
        let exp = "checks/truth/h2/sto3g_dS.txt".to_string();

        test_path_rev(&path, &exp);
    }
}
