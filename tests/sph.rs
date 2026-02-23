#![feature(autodiff)]
use std::io;

use std::autodiff::autodiff_reverse;

use librint::cint_bas::{CINTcgto_cart, CINTcgtos_spheric};
use librint::cint1e::{cint1e_ovlp_cart, cint1e_ovlp_sph};
use librint::scf::nmol;
use librint::utils::read_basis;
use std::env;
use std::time::Instant;
use librint::utils::combine;
use librint::utils::split;

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

#[no_mangle]
#[autodiff_reverse(dovlp_cart, Duplicated, Const, Const, Const, Duplicated)]
pub fn ovlp_cart(
    out: &mut Vec<f64>, 
    shls: &mut Vec<i32>, 
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>, 
    env: &mut Vec<f64>,
) {
    let (natm, nbas) = nmol(atm, bas);
    cint1e_ovlp_cart(out, shls, atm, natm as i32, bas, nbas as i32, env, std::ptr::null_mut());
}


#[no_mangle]
#[autodiff_reverse(dovlp_sph, Duplicated, Const, Const, Const, Duplicated)]
pub fn ovlp_sph(
    out: &mut Vec<f64>, 
    shls: &mut Vec<i32>, 
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>, 
    env: &mut Vec<f64>,
) {
    let (natm, nbas) = nmol(atm, bas);
    cint1e_ovlp_sph(out, shls, atm, natm as i32, bas, nbas as i32, env, std::ptr::null_mut());
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

    let mut denv = vec![0.0; env.len()];

    let mut shls = vec![0; 4];

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

            ovlp_cart(
                &mut buf,
                &mut shls,
                &mut atm,
                &mut bas,
                &mut env,
            );

            dovlp_cart(
                &mut buf,
                &mut dbuf,
                &mut shls,
                &mut atm,
                &mut bas,
                &mut env,
                &mut denv,
            );

            dbg!(&buf);
            dbg!(&denv[s1..s2]);
            denv.fill(0.0);
        }
    }

    for i in 0..nbas {
        for j in 0..nbas {
            shls[0] = i as i32;
            shls[1] = j as i32;

            let di = unsafe { CINTcgtos_spheric(i as i32, bas.as_mut_ptr()) };
            let dj = unsafe { CINTcgtos_spheric(j as i32, bas.as_mut_ptr()) };

            let size = (di * dj) as usize;
            let mut buf = vec![0.0; size];
            let mut dbuf = vec![0.0; size];
            
            dbuf[0] = 1.0;

            ovlp_sph(
                &mut buf,
                &mut shls,
                &mut atm,
                &mut bas,
                &mut env,
            );

            dovlp_sph(
                &mut buf,
                &mut dbuf,
                &mut shls,
                &mut atm,
                &mut bas,
                &mut env,
                &mut denv,
            );

            dbg!(&denv[s1..s2]);
            denv.fill(0.0);
        }
    }
}