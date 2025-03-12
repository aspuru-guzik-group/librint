#![allow(non_snake_case, non_upper_case_globals)]
#![feature(autodiff)]
use std::autodiff::autodiff;
use std::io;
use std::time::Instant;

use librint::utils::read_basis;

use librint::cint_bas::CINTcgto_cart;
use librint::cint2e::cint2e_cart;

use librint::scf::{nmol, angl, density, calc_F};
use librint::dscf::getF;

use librint::linalg::matmult;

use librint::utils::{print_arr, split};

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

#[no_mangle]
#[autodiff(dtwow, Reverse, Duplicated, Const, Const, Const, Const, Duplicated)]
fn twow(
    out: &mut Vec<f64>, 
    shls: &mut Vec<i32>, 
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>, 
    env1: &mut Vec<f64>,
    env2: &mut Vec<f64>,
) {
    let mut env: Vec<f64> = vec![0.0; env1.len() + env2.len()];
    let mut c = 0;
    for i in 0..env1.len() {
        env[c] = env1[i];
        c += 1;
    }
    for j in 0..env2.len() {
        env[c] = env2[j];
        c += 1;
    }

    let (natm, nbas) = nmol(atm, bas);
    cint2e_cart(out, shls, atm, natm as i32, bas, nbas as i32, &mut env);
}

#[no_mangle]
pub fn dRcont(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env1: &mut Vec<f64>,
    env2: &mut Vec<f64>,
    P: &Vec<f64>,
) -> Vec<f64> {
    let (_, nbas) = nmol(atm, bas);
    let nshells = angl(bas, 0);

    let mut dR = vec![0.0; env2.len()];

    let mut buf;
    let mut dbuf;
    let mut denv;
    let mut shls = vec![0; 4];

    let mut mu;
    let mut nu;
    let mut sig;
    let mut lam;

    mu = 0;
    for i in 0..nbas {
        shls[0] = i as i32; let di = CINTcgto_cart(i, &bas) as usize;
        nu = 0;
        for j in 0..nbas {
            shls[1] = j as i32; let dj = CINTcgto_cart(j, &bas) as usize;
            sig = 0;
            for k in 0..nbas {
                shls[2] = k as i32; let dk = CINTcgto_cart(k, &bas) as usize;
                lam = 0;
                for l in 0..nbas {
                    shls[3] = l as i32; let dl = CINTcgto_cart(l, &bas) as usize;

                    buf = vec![0.0; di * dj * dk * dl];
                    dbuf = vec![0.0; di * dj * dk * dl];
                    
                    let mut c: usize = 0;
                    for laml in lam..(lam + dl) {
                        for sigk in sig..(sig + dk) {
                            for nuj in nu..(nu + dj) {
                                for mui in mu..(mu + di) {
                                    dbuf[c] = 1.0;

                                    denv = vec![0.0; env2.len()];
                                    dtwow(&mut buf, &mut dbuf, &mut shls, atm, bas, env1, env2, &mut denv);
                                    for l in 0..env2.len() {
                                        dR[l] += P[nuj*nshells + mui] * denv[l] * P[laml*nshells + sigk];
                                    }
                                    
                                    dbuf[c] = 0.0;
                                    c += 1;
                                }
                            }
                        }
                    }
                    lam += dl;
                }
                sig += dk;
            }
            nu += dj;
        }
        mu += di;
    }
    
    return dR;
}

#[no_mangle]
pub fn dRgw(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    P: &Vec<f64>,
) -> Vec<f64> {
    let (s1, s2) = split(bas);

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let dR = dRcont(atm, bas, &mut env1, &mut env2, P);
    return dR;
}

fn main() -> io::Result<()> {
    let mut atm = Vec::new();
    let mut bas = Vec::new();
    let mut env = Vec::new();

    let path = "/u/jpmedina/libcint/librint/molecules/h2/sto3g.txt";
    read_basis(path, &mut atm, &mut bas, &mut env)?;

    // f vs df time
    let (s1, s2) = split(&mut bas);

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let i = 0;
    let j = 0;
    let k = 0;
    let l = 0;

    let mut shls = vec![0; 4];
    shls[0] = i as i32; let di = CINTcgto_cart(i, &bas) as usize;
    shls[1] = j as i32; let dj = CINTcgto_cart(j, &bas) as usize;
    shls[2] = k as i32; let dk = CINTcgto_cart(k, &bas) as usize;
    shls[3] = l as i32; let dl = CINTcgto_cart(l, &bas) as usize;

    let mut buf = vec![0.0; di * dj * dk * dl];
    let mut dbuf = vec![0.0; di * dj * dk * dl];

    let mut denv = vec![0.0; s2-s1];

    let dnow = Instant::now();
    twow(&mut buf, &mut shls, &mut atm, &mut bas, &mut env1, &mut env2);
    let delapsed_time = dnow.elapsed();

    println!("two time: {}", delapsed_time.as_micros());

    dbuf[0] = 1.0;

    let dnow = Instant::now();
    dtwow(&mut buf, &mut dbuf, &mut shls, &mut atm, &mut bas, &mut env1, &mut env2, &mut denv);
    let delapsed_time = dnow.elapsed();

    println!("dtwo time: {}", delapsed_time.as_micros());

    // full time
    let nelec = 2;

    let P = density(&mut atm, &mut bas, &mut env, nelec, 20, 1e-6);

    let (s1, s2) = split(&mut bas);

    let dnow = Instant::now();
    let mut dR = dRgw(&mut atm, &mut bas, &mut env, &P);
    let delapsed_time = dnow.elapsed();
    println!("tensor time contracted: {}", delapsed_time.as_micros());

    println!("dR size: {} {} {}", dR.len(), s2-s1, P.len() * P.len() * (s2-s1));

    for i in 0..dR.len() {
        print!("{:.5} ", dR[i]);
    }
    println!();

    Ok(())
}
