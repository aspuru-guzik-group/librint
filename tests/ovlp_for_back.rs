#![allow(non_snake_case, non_upper_case_globals,unused_variables,improper_ctypes_definitions,static_mut_refs)]
#![feature(autodiff)]

use std::env;

use std::time::Instant;

use std::autodiff::*;
use librint::utils::{read_basis, split, combine};
use librint::scf::{nmol, angl};

use librint::cint_bas::CINTcgto_cart;
use librint::cint1e::cint1e_ovlp_cart;
// use librint::cint2e::cint2e_cart;


pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

#[no_mangle]
#[autodiff_reverse(dovlpp, Duplicated, Const, Const, Const, Const, Duplicated)]
#[autodiff_forward(dovlppfor, Dual, Const, Const, Const, Const, Dual)]
pub fn ovlpp(
    out: &mut Vec<f64>, 
    shls: &mut Vec<i32>, 
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>, 
    env1: &mut Vec<f64>,
    env2: &mut Vec<f64>,
) {
    let (natm, nbas) = nmol(atm, bas);
    let mut env: Vec<f64> = combine(&env1, &env2);
    cint1e_ovlp_cart(out, shls, atm, natm as i32, bas, nbas as i32, &mut env, std::ptr::null_mut());
}

#[no_mangle]
pub fn dS_uncontracted(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
) -> Vec<f64> {
    let (_, nbas) = nmol(&atm, &bas);
    let nshells = angl(&bas, 0);

    let (s1, s2) = split(bas);

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let mut dS = vec![0.0; nshells * nshells * env2.len()];

    let mut buf;
    let mut dbuf;
    let mut denv;
    let mut shls = vec![0; 4];

    let mut mu;
    let mut nu;

    mu = 0;
    for i in 0..nbas {
        shls[0] = i as i32; let di = CINTcgto_cart(i, &bas) as usize;
        nu = 0;
        for j in 0..nbas {
            shls[1] = j as i32; let dj = CINTcgto_cart(j, &bas) as usize;

            buf = vec![0.0; di * dj];
            dbuf = vec![0.0; di * dj];

            let mut c: usize = 0;
            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    dbuf[c] = 1.0;

                    denv = vec![0.0; env2.len()];
                    dovlpp(&mut buf, &mut dbuf, &mut shls, atm, bas, &mut env1, &mut env2, &mut denv);
                    for l in 0..env2.len() {
                        dS[(nuj * nshells + mui) * env2.len() + l] = denv[l];
                        // dS[l * nshells * nshells + nuj * nshells + mui] = denv[l];
                    }
                    
                    dbuf[c] = 0.0;
                    c += 1;
                }
            }
            nu += dj;
        }
        mu += di;
    }
    
    return dS;
}


#[no_mangle]
pub fn dS_uncontracted_for(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
) -> Vec<f64> {
    let (_, nbas) = nmol(&atm, &bas);
    let nshells = angl(&bas, 0);

    let (s1, s2) = split(bas);

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let mut dS = vec![0.0; nshells * nshells * env2.len()];

    let mut buf;
    let mut dbuf;
    let mut denv;
    let mut shls = vec![0; 4];

    let mut mu;
    let mut nu;

    mu = 0;
    for i in 0..nbas {
        shls[0] = i as i32; let di = CINTcgto_cart(i, &bas) as usize;
        nu = 0;
        for j in 0..nbas {
            shls[1] = j as i32; let dj = CINTcgto_cart(j, &bas) as usize;

            buf = vec![0.0; di * dj];
            dbuf = vec![0.0; di * dj];

            let mut c: usize = 0;
            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    // We use fwd mode, buf is the output, so init it to 0.0. autodiff will
                    // overwrite it with the derivative
                    dbuf = vec![0.0; di * dj];

                    denv = vec![0.0; env2.len()];
                    // We use fwd mode, env2 is the input, so seed the shadow denv to 1.0
                    denv[c] = 1.0;
                    dovlppfor(&mut buf, &mut dbuf, &mut shls, atm, bas, &mut env1, &mut env2, &mut denv);
                    for l in 0..dbuf.len() {
                        dS[(nuj * nshells + mui) * env2.len() + l] = dbuf[l];
                    }
                    
                    denv[c] = 0.0;
                    c += 1;
                }
            }
            nu += dj;
        }
        mu += di;
    }
    
    return dS;
}



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} file", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];

    let mut atm: Vec<i32> = Vec::new();
    let mut bas: Vec<i32> = Vec::new();
    let mut env: Vec<f64> = Vec::new();

    _ = read_basis(&path, &mut atm, &mut bas, &mut env);

    let dS = dS_uncontracted(&mut atm, &mut bas, &mut env);
    let dS_for = dS_uncontracted_for(&mut atm, &mut bas, &mut env);

    // assert_eq!(dS, dS_for);

    // now compare dS and dS_for
    let mut mismatches = 0;
    for i in 0..dS.len() {
        if mismatches > 10 {
            break;
        }
        if (dS[i] - dS_for[i]).abs() > 1e-10 {
            println!("Mismatch at index {}: dS = {}, dS_for = {}", i, dS[i], dS_for[i]);
            mismatches += 1;
        }
    }
    if (mismatches == 0) {
        println!("dS and dS_for match");
    }

}
