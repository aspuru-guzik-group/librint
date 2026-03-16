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
#[autodiff_reverse(dovlpp, Duplicated, Const, Const, Const, Const, Const, Duplicated)]
#[autodiff_forward(dovlppfor, Dual, Const, Const, Const, Const, Const, Dual)]
fn ovlpp(
    out: &mut [f64], 
    shls: &mut [i32], 
    atm: &mut [i32],
    natm: usize, 
    bas: &mut [i32], 
    nbas: usize, 
    env: &mut [f64]
) {
    cint1e_ovlp_cart(
        out, 
        shls, 
        atm, 
        natm as i32, 
        bas, 
        nbas as i32, 
        env,
        std::ptr::null_mut(),
    );
}

// #[no_mangle]
// #[autodiff(dovlppfor, Forward, Dual, Const, Const, Const, Const, Const, Dual)]
// fn ovlpp(
//     out: &mut [f64], 
//     shls: &mut [i32], 
//     atm: &mut [i32],
//     natm: usize, 
//     bas: &mut [i32], 
//     nbas: usize, 
//     env: &mut [f64]
// ) {
//     cint1e_ovlp_cart(
//         out, 
//         shls, 
//         atm, 
//         natm as i32, 
//         bas, 
//         nbas as i32, 
//         env,
//         std::ptr::null_mut(),
//     );
// }

// #[no_mangle]
// #[autodiff(drepp, Reverse, Duplicated, Const, Const, Const, Const, Const, Duplicated)]
// fn repp(
//     out: &mut [f64], 
//     shls: &mut [i32], 
//     atm: &mut [i32],
//     natm: usize, 
//     bas: &mut [i32], 
//     nbas: usize, 
//     env: &mut [f64]
// ) {
//     cint2e_cart(
//         out, 
//         shls, 
//         atm, 
//         natm as i32, 
//         bas, 
//         nbas as i32, 
//         env,
//         std::ptr::null_mut(),
//     );
// }


fn time_reverse_mode(
    atm: &mut [i32],
    natm: usize,
    bas: &mut [i32],
    nbas: usize,
    env: &mut [f64],
    env2_len: usize,
    dS: &mut [f64],
) {
    println!("--- Timing Reverse Mode dS ---");
    let mut shls: [i32; 4] = [0, 0, 0, 0];

    let max_size = (nbas * nbas) as usize; // Preallocate with an upper bound
    let mut buf = vec![0.0; max_size];
    let mut dbuf = vec![0.0; max_size];
    let mut denv = vec![0.0f64; env.len()];

    let mut total_ovlp_time = 0.0;
    let mut total_dovlp_time = 0.0;
    let mut count = 0;

    for i in 0..nbas {
        for j in 0..nbas {
            shls[0] = i as i32;
            shls[1] = j as i32;

            let di = CINTcgto_cart(i, bas);
            let dj = CINTcgto_cart(j, bas);

            let size = (di * dj) as usize;
            
            buf[..size].fill(0.0);
            dbuf[..size].fill(0.0);
            dbuf[0] = 1.0;

            let start_ovlp = std::time::Instant::now();
            ovlpp(
                &mut buf,
                &mut shls,
                atm,
                natm,
                bas,
                nbas,
                env,
            );
            let duration_ovlp = start_ovlp.elapsed().as_secs_f64();
            total_ovlp_time += duration_ovlp;

            denv.fill(0.0);
            let start_dovlp = std::time::Instant::now();
            dovlpp(
                &mut buf,
                &mut dbuf,
                &mut shls,
                atm,
                natm,
                bas,
                nbas,
                env,
                &mut denv,
            );
            let duration_dovlp = start_dovlp.elapsed().as_secs_f64();
            total_dovlp_time += duration_dovlp;
            for l in 0..env2_len {
                dS[(i * nbas + j) * env2_len + l] = denv[l];
            }
            count += 1;
        }
    }    

    println!("count {}", count);
    println!("total ovlp time:    {:.6} sec", total_ovlp_time);
    println!("total dovlp time:   {:.6} sec", total_dovlp_time);
    println!("average ovlp time:  {:.6} sec", total_ovlp_time / count as f64);
    println!("average dovlp time: {:.6} sec", total_dovlp_time / count as f64);
    println!("avg overhead:       {:.6}", total_dovlp_time / total_ovlp_time);
}

fn time_forward_mode(
    atm: &mut [i32],
    natm: usize,
    bas: &mut [i32],
    nbas: usize,
    env: &mut [f64],
    env2_len: usize,
    dS_for: &mut [f64],
) {
    println!("--- Timing Forward Mode dS ---");
    let mut shls: [i32; 4] = [0, 0, 0, 0];

    let max_size = (nbas * nbas) as usize; // Preallocate with an upper bound
    let mut buf = vec![0.0; max_size];
    let mut dbuf = vec![0.0; max_size];
    let mut denv = vec![0.0f64; env.len()];

    let mut total_ovlp_time_for = 0.0;
    let mut total_dovlp_time_for = 0.0;
    let mut count = 0;

    for i in 0..nbas {
        for j in 0..nbas {
            shls[0] = i as i32;
            shls[1] = j as i32;

            let di = CINTcgto_cart(i, bas);
            let dj = CINTcgto_cart(j, bas);

            let size = (di * dj) as usize;
            
            buf[..size].fill(0.0);
            dbuf[..size].fill(0.0);

            let start_ovlp = std::time::Instant::now();
            ovlpp(
                &mut buf,
                &mut shls,
                atm,
                natm,
                bas,
                nbas,
                env,
            );
            let duration_ovlp = start_ovlp.elapsed().as_secs_f64();
            total_ovlp_time_for += duration_ovlp;

            for l in 0..env2_len {
                buf[..size].fill(0.0);
                dbuf[..size].fill(0.0);
                
                denv.fill(0.0);
                denv[l] = 1.0;

                let start_dovlp = std::time::Instant::now();
                dovlppfor(
                    &mut buf,
                    &mut dbuf,
                    &mut shls,
                    atm,
                    natm,
                    bas,
                    nbas,
                    env,
                    &mut denv,
                );
                let duration_dovlp = start_dovlp.elapsed().as_secs_f64();
                total_dovlp_time_for += duration_dovlp;

                dS_for[(i * nbas + j) * env2_len + l] = dbuf[0];
            }

            count += 1;
        }
    }    

    println!("count {}", count);
    println!("total ovlp time:    {:.6} sec", total_ovlp_time_for);
    println!("total dovlp time:   {:.6} sec", total_dovlp_time_for);
    println!("average ovlp time:  {:.6} sec", total_ovlp_time_for / count as f64);
    println!("average dovlp time: {:.6} sec", total_dovlp_time_for / count as f64);
    println!("avg overhead:       {:.6}", total_dovlp_time_for / total_ovlp_time_for);
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

    let (natm, nbas) = nmol(&mut atm, &mut bas);

    let (s1, s2) = split(&mut bas);
    let nshells = angl(&bas, 0);

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let mut dS = vec![0.0; nbas * nbas * env2.len()];
    let mut dS_for = vec![0.0; nbas * nbas * env2.len()];


    time_reverse_mode(&mut atm, natm, &mut bas, nbas, &mut env, env2.len(), &mut dS);
    time_forward_mode(&mut atm, natm, &mut bas, nbas, &mut env, env2.len(), &mut dS_for);


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

    println!("env2.len() {}", env2.len());

    // let mut total_repp_time = 0.0;
    // let mut total_drepp_time = 0.0;
    // count = 0;

    // for i in 0..nbas {
    //     for j in 0..nbas {
    //         for k in 0..nbas {
    //             for l in 0..nbas {
    //                 // Set shell quartet indices
    //                 let mut shls: [i32; 4] = [0; 4];
    //                 shls[0] = i as i32;
    //                 shls[1] = j as i32;
    //                 shls[2] = k as i32;
    //                 shls[3] = l as i32;

    //                 // Compute basis function counts for each shell
    //                 let di = CINTcgto_cart(i, &bas);
    //                 let dj = CINTcgto_cart(j, &bas);
    //                 let dk = CINTcgto_cart(k, &bas);
    //                 let dl = CINTcgto_cart(l, &bas);

    //                 // Compute size of output array: product of all basis function counts
    //                 let size = (di * dj * dk * dl) as usize;

    //                 let mut buf = vec![0.0f64; size];
    //                 let mut dbuf = vec![0.0f64; size];
    //                 dbuf[0] = 1.0;

    //                 // Time primal function
    //                 let start_repp = Instant::now();
    //                 repp(
    //                     &mut buf,
    //                     &mut shls,
    //                     &mut atm,
    //                     natm,
    //                     &mut bas,
    //                     nbas,
    //                     &mut env,
    //                 );
    //                 let duration_repp = start_repp.elapsed().as_secs_f64();
    //                 total_repp_time += duration_repp;

    //                 // Time autodiff function
    //                 let mut denv = vec![0.0f64; env.len()];
    //                 let start_drepp = Instant::now();
    //                 drepp(
    //                     &mut buf,
    //                     &mut dbuf,
    //                     &mut shls,
    //                     &mut atm,
    //                     natm,
    //                     &mut bas,
    //                     nbas,
    //                     &mut env,
    //                     &mut denv,
    //                 );
    //                 let duration_drepp = start_drepp.elapsed().as_secs_f64();
    //                 total_drepp_time += duration_drepp;

    //                 count += 1;
    //             }
    //         }
    //     }
    // }

    // println!("count {}", count);
    // println!("total rep time:     {:.6} sec", total_repp_time);
    // println!("total drep time:    {:.6} sec", total_drepp_time);
    // println!("average rep time:   {:.6} sec", total_repp_time / count as f64);
    // println!("average drep time:  {:.6} sec", total_drepp_time / count as f64);
    // println!("avg overhead:       {:.6}", total_drepp_time / total_repp_time);
}