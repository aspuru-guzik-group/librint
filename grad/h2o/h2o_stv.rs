#![allow(non_snake_case, non_upper_case_globals)]

use std::io;

use librint::utils::read_basis;
use librint::utils::save_arr;
use librint::utils::print_arr;

use librint::cint_bas::CINTcgto_cart;
use librint::cint1e::cint1e_ovlp_cart;
use librint::intor1::cint1e_kin_cart;
use librint::cint1e::cint1e_nuc_cart;
use librint::cint2e::cint2e_cart;

use librint::cint_bas::CINTcgto_spheric;
use librint::cint1e::cint1e_ovlp_sph;
use librint::intor1::cint1e_kin_sph;
use librint::cint1e::cint1e_nuc_sph;
use librint::cint2e::cint2e_sph;

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

fn one(
    natm: usize,
    nbas: usize,
    nshells: usize,
    atm: &mut [i32],
    bas: &mut [i32],
    env: &mut [f64],
    shls: &mut [i32],
    mu: usize,
    nu: usize,
    di: usize,
    dj: usize,
    S: &mut [f64],
    T: &mut [f64],
    V: &mut [f64],
) {
    let mut buf = vec![0.0; di * dj];

    cint1e_ovlp_cart(&mut buf, shls, atm, natm as i32, bas, nbas as i32, env);
    let mut c: usize = 0;
    for nuj in nu..(nu + dj) {
        for mui in mu..(mu + di) {
            S[mui * nshells + nuj] = buf[c];
            c += 1;
        }
    }

    cint1e_kin_cart(&mut buf, shls, atm, natm as i32, bas, nbas as i32, env);
    let mut c: usize = 0;
    for nuj in nu..(nu + dj) {
        for mui in mu..(mu + di) {
            T[mui * nshells + nuj] = buf[c];
            c += 1;
        }
    }

    cint1e_nuc_cart(&mut buf, shls, atm, natm as i32, bas, nbas as i32, env);
    let mut c: usize = 0;
    for nuj in nu..(nu + dj) {
        for mui in mu..(mu + di) {
            V[mui * nshells + nuj] = buf[c];
            c += 1;
        }
    }
}

fn two(
    natm: usize,
    nbas: usize,
    nshells: usize,
    atm: &mut [i32],
    bas: &mut [i32],
    env: &mut [f64],
    shls: &mut [i32],
    mu: usize,
    nu: usize,
    sig: usize,
    lam: usize,
    di: usize,
    dj: usize,
    dk: usize,
    dl: usize,
    rep: &mut [f64],
) {
    let mut buf = vec![0.0; di * dj * dk * dl];
        
    cint2e_cart(&mut buf, shls, atm, natm as i32, bas, nbas as i32, env);
    let mut c: usize = 0;
    for sigi in sig..(sig + dl) {
        for lami in lam..(lam + dk) {
            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    rep[mui*nshells.pow(3) + nuj*nshells.pow(2) + lami*nshells + sigi] = buf[c];
                    c += 1;
                }
            }
        }
    }
}

fn int(
    natm: usize,
    nbas: usize,
    nshells: usize,
    atm: &mut [i32],
    bas: &mut [i32],
    env: &mut [f64],
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    // let mut buf: Vec<f64>;
    let mut shls: [i32; 4] = [0, 0, 0, 0];

    let mut S = vec![0.0; nshells * nshells];
    let mut T = vec![0.0; nshells * nshells];
    let mut V = vec![0.0; nshells * nshells];
    let mut rep = vec![0.0; nshells.pow(4)];

    let mut mu;
    let mut nu;
    let mut sig;
    let mut lam;

    let mut di = 0;
    let mut dj = 0;
    let mut dk = 0;
    let mut dl = 0;

    mu = 0;
    for i in 0..nbas {
        nu = 0;
        for j in 0..nbas {
            sig = 0;

            shls[0] = i as i32;
            shls[1] = j as i32;

            di = CINTcgto_cart(i, &bas) as usize;
            dj = CINTcgto_cart(j, &bas) as usize;

            one(natm, nbas, nshells, atm, bas, env, &mut shls, mu, nu, di, dj, &mut S, &mut T, &mut V);

            for k in 0..nbas {
                lam = 0;
                for l in 0..nbas {
                    shls[2] = k as i32;
                    shls[3] = l as i32;

                    dl = CINTcgto_cart(l, &bas) as usize;
                    dk = CINTcgto_cart(k, &bas) as usize;
                    
                    two(natm, nbas, nshells, atm, bas, env, &mut shls, mu, nu, sig, lam, di, dj, dk, dl, &mut rep);

                    sig += dl;
                }
                lam += dk;
            }
            nu += dj;
        }
        mu += di;
    }

    return (S, T, V, rep);
}

fn main() -> io::Result<()> {
    let mut atm: Vec<i32> = Vec::new();
    let mut bas: Vec<i32> = Vec::new();
    let mut env: Vec<f64> = Vec::new();

    let path = librint::get_path();
    read_basis(&path, &mut atm, &mut bas, &mut env)?;

	const natm: usize = 3;
	const nbas: usize = 5;
    const nshells: usize = 7;

    let mut stv = int(natm, nbas, nshells, &mut atm, &mut bas, &mut env);
    // let mut rep = rep(natm, nbas, n, &mut atm, &mut bas, &mut env);

    println!("ovlp");
    print_arr(nshells, 2, &mut stv.0);

    println!("kin");
    print_arr(nshells, 2, &mut stv.1);

    println!("nuc");
    print_arr(nshells, 2, &mut stv.2);

    // println!("rep");
    // print_arr(nshells, 4, &mut stv.3);

    Ok(())
}
