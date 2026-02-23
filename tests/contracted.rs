#![feature(autodiff)]
use std::io;

use std::autodiff::autodiff_reverse;

use librint::cint_bas::CINTcgto_cart;
use librint::cint1e::cint1e_ovlp_cart;
use librint::scf::nmol;
use librint::utils::read_basis;
use librint::utils::combine;
use librint::utils::split;
use librint::scf::angl;

use std::env;
use std::time::Instant;

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

#[no_mangle]
#[autodiff_reverse(dovlp_single, Duplicated, Const, Const, Const, Const, Duplicated)]
pub fn ovlp_single(
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

    let mut shls = vec![0, 0, 0, 0];

    let nshells = angl(&bas, 0);

    let (s1, s2) = split(&mut bas);

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let mut denv = vec![0.0; env2.len()];

    let mut dS = vec![0.0; nshells * nshells * env2.len()];

    let mut buf;
    let mut dbuf;
    let mut denv;

    let mut mu;
    let mut nu;

    let start_ovlp = Instant::now();
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
                    dovlp_single(&mut buf, &mut dbuf, &mut shls, &mut atm, &mut bas, &mut env1, &mut env2, &mut denv);

                    // dS[nu * nshells + mu] = denv;
                    
                    // dS[l * nshells * nshells + nuj * nshells + mui] = denv;

                    for l in 0..env2.len() {
                        dS[l * nshells * nshells + nuj * nshells + mui] = denv[l];

                        // dS[nuj * nshells * env2.len() + mui * env2.len() + l] = denv[l];

                        // dS[l] += Q[nuj * nshells + mui] * denv[l];
                    }
                    
                    dbuf[c] = 0.0;
                    c += 1;
                }
            }
            nu += dj;
        }
        mu += di;
    }
    let duration_ovlp = start_ovlp.elapsed().as_secs_f64();
    println!("total dovlp time:   {:.6} sec", duration_ovlp);
    
    // dbg!(&dS);
}
