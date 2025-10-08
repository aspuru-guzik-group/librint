#![allow(non_snake_case, non_upper_case_globals,unused_variables,improper_ctypes_definitions,static_mut_refs)]
#![feature(autodiff)]

use std::env;

use std::time::Instant;

use std::autodiff::*; //::autodiff;
use librint::utils::read_basis;
//use librint::scf::nmol;
#[no_mangle]
pub fn nmol(
    atm: &Vec<i32>,
    bas: &Vec<i32>,
) 
-> (usize, usize) {
    let natm: usize = atm.len() / ATM_SLOTS;
    let nbas: usize = bas.len() / BAS_SLOTS;
    return (natm, nbas);
}

use librint::cint_bas::CINTcgto_cart;
use librint::cint1e::cint1e_ovlp_cart;
// use librint::cint2e::cint2e_cart;

// use librint::reduc::{nmol, CINTcgto_cart, cint1e_ovlp_cart};

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

#[no_mangle]
// #[autodiff(dovlppfor, Forward, Dual, Const, Const, Const, Const, Const, Dual)]
#[autodiff_forward(dovlppfor, Dual, Const, Const, Const, Const, Const, Dual)]
// #[autodiff_reverse(dovlpp, Duplicated, Const, Const, Const, Const, Const, Duplicated)]
// #[autodiff(dovlpp, Reverse, Duplicated, Const, Const, Const, Const, Const, Duplicated)]
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

    let mut shls: [i32; 4] = [0, 0, 0, 0];

    let mut total_ovlp_time = 0.0;
    let mut total_dovlp_time = 0.0;
    let mut count = 0;

    println!("{} {}", natm, nbas);

    for i in 0..nbas {
        for j in 0..nbas {
            shls[0] = i as i32;
            shls[1] = j as i32;

            let di = CINTcgto_cart(i, &bas);
            let dj = CINTcgto_cart(j, &bas);

            let size = (di * dj) as usize;
            let mut buf = vec![0.0; size];
            let mut dbuf = vec![0.0; size];
            
            dbuf[0] = 1.0;

            // fix
            // loop through di * dj

            // Time primal function
            let start_ovlp = Instant::now();
            ovlpp(
                &mut buf,
                &mut shls,
                &mut atm,
                natm,
                &mut bas,
                nbas,
                &mut env,
            );
            let duration_ovlp = start_ovlp.elapsed().as_secs_f64();
            total_ovlp_time += duration_ovlp;

            // Time autodiff function
            let mut denv = vec![0.0f64; env.len()];
            let start_dovlp = Instant::now();
            // dovlpp(
            //     &mut buf,
            //     &mut dbuf,
            //     &mut shls,
            //     &mut atm,
            //     natm,
            //     &mut bas,
            //     nbas,
            //     &mut env,
            //     &mut denv,
            // );
            let duration_dovlp = start_dovlp.elapsed().as_secs_f64();
            total_dovlp_time += duration_dovlp;

            count += 1;
        }
    }    

    println!("count {}", count);
    println!("total ovlp time:    {:.6} sec", total_ovlp_time);
    println!("total dovlp time:   {:.6} sec", total_dovlp_time);
    println!("average ovlp time:  {:.6} sec", total_ovlp_time / count as f64);
    println!("average dovlp time: {:.6} sec", total_dovlp_time / count as f64);
    println!("avg overhead:       {:.6}", total_dovlp_time / total_ovlp_time);


    // FORWARD MODE

    let mut total_ovlp_time_for = 0.0;
    let mut total_dovlp_time_for = 0.0;
    count = 0;

    for i in 0..nbas {
        for j in 0..nbas {
            shls[0] = i as i32;
            shls[1] = j as i32;

            let di = CINTcgto_cart(i, &bas);
            let dj = CINTcgto_cart(j, &bas);

            let size = (di * dj) as usize;
            let mut buf = vec![0.0; size];
            let mut dbuf = vec![0.0; size];

            // Time primal function
            let start_ovlp = Instant::now();
            ovlpp(
                &mut buf,
                &mut shls,
                &mut atm,
                natm,
                &mut bas,
                nbas,
                &mut env,
            );
            let duration_ovlp = start_ovlp.elapsed().as_secs_f64();
            total_ovlp_time_for += duration_ovlp;

            // Time autodiff function
            let mut denv = vec![0.0f64; env.len()];

            denv[0] = 1.0;

            // loop through 0..denv loop through ROI in denv only
            // denv[k] = 1.0;

            let start_dovlp = Instant::now();
            dovlppfor(
                &mut buf,
                &mut dbuf,
                &mut shls,
                &mut atm,
                natm,
                &mut bas,
                nbas,
                &mut env,
                &mut denv,
            );
            let duration_dovlp = start_dovlp.elapsed().as_secs_f64();
            total_dovlp_time_for += duration_dovlp;

            count += 1;

            // buf 1x1
            // env [x, y, z]
            // denv [dx, dy, dz]

            // buf w
            // dbuf [dw] denv[x] = 1.0
            // dbuf [dw] denv[y] = 1.0
            // dbuf [dw] denv[z] = 1.0
        }
    }    

    println!("count {}", count);
    println!("total ovlp time:    {:.6} sec", total_ovlp_time_for);
    println!("total dovlp time:   {:.6} sec", total_dovlp_time_for);
    println!("average ovlp time:  {:.6} sec", total_ovlp_time_for / count as f64);
    println!("average dovlp time: {:.6} sec", total_dovlp_time_for / count as f64);
    println!("avg overhead:       {:.6}", total_dovlp_time_for / total_ovlp_time_for);

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
