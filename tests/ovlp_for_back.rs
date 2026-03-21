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



// Idea 1: Full-matrix function — both i,j loops inside, forward differentiate.
// One call computes the entire overlap matrix. Forward mode: env2_len calls total.
#[no_mangle]
#[autodiff_forward(dovlpp_matrix_for, Dual, Const, Const, Const, Dual)]
pub fn ovlpp_matrix(
    out: &mut Vec<f64>,      // full nshells×nshells flattened matrix
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env1: &mut Vec<f64>,
    env2: &mut Vec<f64>,
) {
    let (natm, nbas) = nmol(atm, bas);
    let nshells = angl(bas, 0);
    let mut env: Vec<f64> = combine(&env1, &env2);
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
            cint1e_ovlp_cart(&mut buf, &mut shls_buf, atm, natm as i32, bas, nbas as i32, &mut env, std::ptr::null_mut());

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

// Idea 2: Row-level function — j loop inside, reverse differentiate.
// One call computes one row of the overlap matrix (all j for fixed i).
// i_shell[0] holds the shell index i.
#[no_mangle]
#[autodiff_reverse(dovlpp_row_rev, Duplicated, Const, Const, Const, Const, Duplicated)]
pub fn ovlpp_row(
    out: &mut Vec<f64>,      // one row: di*nshells elements
    i_shell: &mut Vec<i32>,  // length-1 vec holding shell index i (Const)
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env1: &mut Vec<f64>,
    env2: &mut Vec<f64>,
) {
    let (natm, nbas) = nmol(atm, bas);
    let i = i_shell[0] as usize;
    let di = CINTcgto_cart(i, &bas) as usize;
    let mut env: Vec<f64> = combine(&env1, &env2);
    let mut shls = vec![0i32; 4];
    shls[0] = i as i32;

    let mut nu = 0;
    for j in 0..nbas {
        shls[1] = j as i32;
        let dj = CINTcgto_cart(j, &bas) as usize;

        let mut buf = vec![0.0; di * dj];
        cint1e_ovlp_cart(&mut buf, &mut shls, atm, natm as i32, bas, nbas as i32, &mut env, std::ptr::null_mut());

        let mut c: usize = 0;
        for nuj in nu..(nu + dj) {
            for mui in 0..di {
                out[nuj * di + mui] = buf[c];
                c += 1;
            }
        }
        nu += dj;
    }
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
            denv = vec![0.0; env2.len()];

            let mut c: usize = 0;
            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    dbuf.fill(0.0);
                    dbuf[c] = 1.0;

                    denv.fill(0.0);
                    dovlpp(&mut buf, &mut dbuf, &mut shls, atm, bas, &mut env1, &mut env2, &mut denv);
                    for l in 0..env2.len() {
                        dS[(nuj * nshells + mui) * env2.len() + l] = denv[l];
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
            denv = vec![0.0; env2.len()];

            for l in 0..env2.len() {
                // We use fwd mode, buf is the output, so init it to 0.0. autodiff will
                // overwrite it with the derivative
                buf.fill(0.0);
                dbuf.fill(0.0);

                denv.fill(0.0);
                // We use fwd mode, env2 is the input, so seed the shadow denv to 1.0
                denv[l] = 1.0;
                dovlppfor(&mut buf, &mut dbuf, &mut shls, atm, bas, &mut env1, &mut env2, &mut denv);

                let mut c: usize = 0;
                for nuj in nu..(nu + dj) {
                    for mui in mu..(mu + di) {
                        dS[(nuj * nshells + mui) * env2.len() + l] = dbuf[c];
                        c += 1;
                    }
                }
            }
            nu += dj;
        }
        mu += di;
    }
    
    return dS;
}


// Idea 1 wrapper: full-matrix forward mode
// Total AD calls: env2_len (one per env parameter)
#[no_mangle]
pub fn dS_uncontracted_matrix_for(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
) -> Vec<f64> {
    let nshells = angl(&bas, 0);

    let (s1, s2) = split(bas);

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();
    let env2_len = env2.len();

    let mut dS = vec![0.0; nshells * nshells * env2_len];

    // One forward call per env2 parameter
    for l in 0..env2_len {
        let mut out = vec![0.0; nshells * nshells];
        let mut dout = vec![0.0; nshells * nshells];

        let mut denv2 = vec![0.0; env2_len];
        denv2[l] = 1.0;

        dovlpp_matrix_for(
            &mut out, &mut dout,
            atm, bas,
            &mut env1,
            &mut env2, &mut denv2,
        );

        // dout[nuj * nshells + mui] = d(S[nuj,mui]) / d(env2[l])
        for idx in 0..(nshells * nshells) {
            dS[idx * env2_len + l] = dout[idx];
        }
    }

    return dS;
}


// Idea 2 wrapper: row-level reverse mode
// Total AD calls: sum_i (di * nshells) = nshells^2 (one per output element)
#[no_mangle]
pub fn dS_uncontracted_row_rev(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
) -> Vec<f64> {
    let (_, nbas) = nmol(&atm, &bas);
    let nshells = angl(&bas, 0);

    let (s1, s2) = split(bas);

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();
    let env2_len = env2.len();

    let mut dS = vec![0.0; nshells * nshells * env2_len];

    let mut mu = 0;
    for i in 0..nbas {
        let di = CINTcgto_cart(i, &bas) as usize;
        let mut i_shell = vec![i as i32];

        // out has di * nshells elements (di rows, nshells columns)
        // Layout: out[nuj * di + mui_local] where mui_local is 0..di
        let row_size = di * nshells;

        // Seed each output element one at a time
        for c in 0..row_size {
            let mut out = vec![0.0; row_size];
            let mut dout = vec![0.0; row_size];
            dout[c] = 1.0;

            let mut denv2 = vec![0.0; env2_len];

            dovlpp_row_rev(
                &mut out, &mut dout,
                &mut i_shell,
                atm, bas,
                &mut env1,
                &mut env2, &mut denv2,
            );

            // c = nuj * di + mui_local
            let mui_local = c % di;
            let nuj = c / di;
            let mui = mu + mui_local;

            for l in 0..env2_len {
                dS[(nuj * nshells + mui) * env2_len + l] = denv2[l];
            }
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

    println!("=== Computing dS (reverse mode, per shell pair) ===");
    let dS = dS_uncontracted(&mut atm, &mut bas, &mut env);

    println!("=== Computing dS_for (forward mode, per shell pair) ===");
    let dS_for = dS_uncontracted_for(&mut atm, &mut bas, &mut env);

    println!("=== Computing dS_matrix_for (forward mode, full matrix) ===");
    let dS_matrix_for = dS_uncontracted_matrix_for(&mut atm, &mut bas, &mut env);

    println!("=== Computing dS_row_rev (reverse mode, per row) ===");
    let dS_row_rev = dS_uncontracted_row_rev(&mut atm, &mut bas, &mut env);

    // Compare all against the reference (dS from reverse mode)
    let results = [
        ("dS_for", &dS_for),
        ("dS_matrix_for", &dS_matrix_for),
        ("dS_row_rev", &dS_row_rev),
    ];

    for (name, other) in &results {
        let mut mismatches = 0;
        for i in 0..dS.len() {
            if mismatches > 5 {
                break;
            }
            if (dS[i] - other[i]).abs() > 1e-10 {
                println!("Mismatch {} at index {}: dS = {}, {} = {}", name, i, dS[i], name, other[i]);
                mismatches += 1;
            }
        }
        if mismatches == 0 {
            println!("dS and {} match ✓", name);
        } else {
            println!("dS and {} have {} mismatches ✗", name, mismatches);
        }
    }
}