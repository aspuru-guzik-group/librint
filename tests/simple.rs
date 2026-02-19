#![feature(autodiff)]
use std::io;

use std::autodiff::autodiff_reverse;

use librint::cint_bas::CINTcgto_cart;
use librint::cint1e::cint1e_ovlp_cart;
use librint::scf::nmol;
use librint::utils::read_basis;
//use std::autodiff; //::autodiff_reverse;
use std::env;
use std::time::Instant;


pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

#[no_mangle]
#[autodiff_reverse(dovlpp, Duplicated, Const, Const, Const, Const, Const, Duplicated)]
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
            dovlpp(
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
}
