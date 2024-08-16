#![allow(non_snake_case, non_upper_case_globals)]
#![feature(autodiff)]

use std::io;

use librint::utils::read_basis;

use librint::cint_bas::CINTcgto_cart;
use librint::cint1e::cint1e_ovlp_cart;
use librint::cint2e::cint2e_cart;

// use librint::scf::{density, energyfast};
use librint::scf::nmol;
use librint::utils::{split, combine};

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

#[no_mangle]
#[autodiff(dovlp, Reverse, Duplicated, Const, Const, Const, Const, Duplicated)]
pub fn ovlp(
    out: &mut Vec<f64>, 
    shls: &mut Vec<i32>, 
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>, 
    env1: &mut Vec<f64>,
    env2: &mut Vec<f64>,
) {
    let (natm, nbas) = nmol(atm, bas);
    let mut env: Vec<f64> = combine(&env1, &env2);
    cint1e_ovlp_cart(out, shls, atm, natm as i32, bas, nbas as i32, &mut env);
}


// #[no_mangle]
// #[autodiff(denergyf, Reverse, Const, Const, Const, Duplicated, Const, Active)]
// pub fn energyf(
//     atm: &mut Vec<i32>,
//     bas: &mut Vec<i32>,
//     env1: &mut Vec<f64>,
//     env2: &mut Vec<f64>,
//     P: &mut Vec<f64>,
// ) -> f64 {
//     let mut env: Vec<f64> = combine(&env1, &env2);
//     return energyfast(atm, bas, &mut env, P);
// }

// #[no_mangle]
// pub fn gradenergyfast(
//     atm: &mut Vec<i32>,
//     bas: &mut Vec<i32>,
//     env: &mut Vec<f64>,
//     P: &mut Vec<f64>,
// ) -> Vec<f64> {
//     let (s1, s2) = split(bas);

//     let mut env1: Vec<f64> = env[0..s1].to_vec();
//     let mut env2: Vec<f64> = env[s1..s2].to_vec();

//     let mut denv: Vec<f64> = vec![0.0; s2-s1];
//     let _ = denergyf(atm, bas, &mut env1, &mut env2, &mut denv, P, 1.0);

//     return denv;
// }

// #[no_mangle]
// pub fn denergyfast(
//     atm: &mut Vec<i32>,
//     bas: &mut Vec<i32>,
//     env: &mut Vec<f64>,
//     P: &mut Vec<f64>,
// ) -> Vec<f64> {
//     let denergy = gradenergyfast(atm, bas, env, P);
//     // let dS = dSg(atm, bas, env, P);

//     let mut dtotal = vec![0.0; denergy.len()];
//     for i in 0..dtotal.len() {
//         dtotal[i] = denergy[i]; // - 0.5 * dS[i];
//     }

//     return dtotal;
// }

fn main() -> io::Result<()>{
    let mut atm: Vec<i32> = Vec::new();
    let mut bas: Vec<i32> = Vec::new();
    let mut env: Vec<f64> = Vec::new();

    let path = "/u/jpmedina/libcint/librint/molecules/h2/sto3g.txt";
    read_basis(path, &mut atm, &mut bas, &mut env);

    const natm: usize = 2;
    const nbas: usize = 2;
    
    let mut shls = vec![0, 0, 0, 0];

    let mut buf;

	println!("ovlp");
    for i in 0..nbas {
        for j in 0..nbas {
            shls[0] = i as i32;
            shls[1] = j as i32;
            
            let di = CINTcgto_cart(i, &bas);
            let dj = CINTcgto_cart(j, &bas);

            buf = vec![0.0; (di * dj) as usize];
            cint1e_ovlp_cart(&mut buf, &mut shls, &mut atm, natm as i32, &mut bas, nbas as i32, &mut env);

            for i in 0..((di*dj) as usize) {
                print!("{} ", buf[i]);
            }
        }
        println!();
    }

    let (s1, s2) = split(&mut bas);

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let mut dbuf;
    let mut denv;

    println!("dovlp");
    for i in 0..nbas {
        for j in 0..nbas {
            shls[0] = i as i32;
            shls[1] = j as i32;
            
            let di = CINTcgto_cart(i, &bas);
            let dj = CINTcgto_cart(j, &bas);

            buf = vec![0.0; (di * dj) as usize];
            dbuf = vec![0.0; (di * dj) as usize];
            denv = vec![0.0; env2.len()];
            dovlp(&mut buf, &mut dbuf, &mut shls, &mut atm, &mut bas, &mut env1, &mut env2, &mut denv);

            for k in 0..denv.len() {
                print!("{} ", denv[k]);
            }
        }
        println!();
    }

    println!("repulsion");
    for i in 0..nbas {
        for j in 0..nbas {
            for k in 0..nbas {
                for l in 0..nbas {
                    shls[0] = i as i32;
                    shls[1] = j as i32;
                    shls[2] = k as i32;
                    shls[3] = l as i32;
                    
                    let di = CINTcgto_cart(i, &bas);
                    let dj = CINTcgto_cart(j, &bas);
                    let dk = CINTcgto_cart(k, &bas);
                    let dl = CINTcgto_cart(l, &bas);

                    buf = vec![0.0; (di * dj * dk * dl) as usize];
                    cint2e_cart(&mut buf, &mut shls, &mut atm, natm as i32, &mut bas, nbas as i32, &mut env);

                    for i in 0..((di * dj * dk * dl) as usize) {
                        print!("{} ", buf[i]);
                    }
                }
                println!();
            }
        }
        println!();
    }

    // let nelec = 2;

    // let mut P = density(&mut atm, &mut bas, &mut env, nelec, 20, 1e-6);

    // let E = energyfast(&mut atm, &mut bas, &mut env, &mut P);

    // println!("{:?}", P);
    // println!("{}", E);

    // let denv = denergyfast(&mut atm, &mut bas, &mut env, &mut P);

    // println!("{:?}", denv);
    
    Ok(())
}