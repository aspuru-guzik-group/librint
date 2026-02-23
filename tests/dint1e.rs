#![feature(autodiff)]
use std::io;

use std::autodiff::autodiff_reverse;

use librint::cint_bas::CINTcgto_cart;
use librint::cint1e::cint1e_ovlp_cart;
use librint::scf::nmol;
use librint::utils::read_basis;
use std::env;
use std::time::Instant;
use librint::utils::combine;
use librint::utils::split;

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

#[no_mangle]
#[autodiff_reverse(dovlpp_, Duplicated, Const, Const, Const, Const, Duplicated)]
pub fn ovlp_(
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

// #[no_mangle]
// pub extern "C" fn dint1e_ovlp_c(
//     i: i32,
//     j: i32,
//     atm_p: *mut i32,
//     atm_l: usize,
//     bas_p: *mut i32,
//     bas_l: usize,
//     env_p: *mut f64,
//     env_l: usize,
// ) -> *mut f64 {
//     let (mut atm, mut bas, mut env) = c2r_arr(atm_p, atm_l, bas_p, bas_l, env_p, env_l);

//     let (s1, s2) = split(&mut bas);

//     let mut env1: Vec<f64> = env[0..s1].to_vec();
//     let mut env2: Vec<f64> = env[s1..s2].to_vec();

//     let mut denv = vec![0.0; env2.len()];

//     let mut shls = vec![0; 4];

//     shls[0] = i as i32; let di = CINTcgto_cart(i as usize, &bas) as usize;
//     shls[1] = j as i32; let dj = CINTcgto_cart(j as usize, &bas) as usize;

//     let mut buf = vec![0.0; di * dj];
//     let mut dbuf = vec![0.0; di * dj];

//     dovlpp(&mut buf, &mut dbuf, &mut shls, &mut atm, &mut bas, &mut env1, &mut env2, &mut denv);

//     let denv_ptr = denv.as_mut_ptr();
//     std::mem::forget(denv);
//     return denv_ptr;
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

    let (s1, s2) = split(&mut bas);

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let mut denv = vec![0.0; env2.len()];

    let mut shls = vec![0; 4];

    let i = 0;
    let j = 1;

    shls[0] = i as i32; let di = CINTcgto_cart(i as usize, &bas) as usize;
    shls[1] = j as i32; let dj = CINTcgto_cart(j as usize, &bas) as usize;

    let mut buf = vec![0.0; di * dj];
    let mut dbuf = vec![0.0; di * dj];

    dbuf[0] = 1.0;
    dovlpp_(&mut buf, &mut dbuf, &mut shls, &mut atm, &mut bas, &mut env1, &mut env2, &mut denv);

    dbg!(&denv);
}