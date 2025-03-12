#![allow(non_snake_case, non_upper_case_globals)]
#![feature(autodiff)]
use std::autodiff::autodiff;
use librint::utils::read_basis;

use librint::cint_bas::CINTcgto_cart;
use librint::cint1e::cint1e_ovlp_cart;

use librint::scf::nmol;

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

#[no_mangle]
#[autodiff(cint_diff, Reverse, Duplicated, Const, Const, Const, Const, Const, Duplicated)]
fn cint_wrap(
    out: &mut [f64], 
    shls: &mut [i32], 
    atm: &mut [i32],
    natm: i32, 
    bas: &mut [i32], 
    nbas: i32, 
    env: &mut [f64]
) {
    cint1e_ovlp_cart(out, shls, atm, natm, bas, nbas, env);
}

fn main() {
    let mut atm = Vec::new();
    let mut bas = Vec::new();
    let mut env = Vec::new();

    let path = librint::get_path();
    read_basis(&path, &mut atm, &mut bas, &mut env)?;

    let (natm, nbas) = nmol(&atm, &bas);

    let mut shls: [i32; 4] = [0, 0, 0, 0];

    let mut buf;
    let mut dbuf;

    let i = 2;
    let j = 2;

    shls[0] = i as i32;
    shls[1] = j as i32;

    let di = CINTcgto_cart(i, &bas);
    let dj = CINTcgto_cart(j, &bas);

    buf = vec![0.0; (di * dj) as usize];
    dbuf = vec![0.0; (di * dj) as usize];

    println!("di dj {} {}", di, dj);

    let size: usize = 42;

    let mut denv: [f64; 42] = [0.0; 42];
    println!("denv:");

    for i in 0..(di * dj) as usize {
        for k in 0..42 {
            denv[k] = 0.0;
        }

        for k in 0..(di * dj) as usize {
            dbuf[k] = 0.0;
        }

        dbuf[i] = 1.0;

        cint_diff(&mut buf, &mut dbuf, &mut shls, &mut atm, natm as i32, &mut bas, nbas as i32, &mut env, &mut denv);
        for i in 41..42 {
            print!("{:.6} ", denv[i]);
        }
    }
    println!();
}
