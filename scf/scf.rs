#![allow(non_snake_case, non_upper_case_globals)]
#![feature(autodiff)]
use std::autodiff::autodiff;
use std::io;

use librint::utils::{read_basis, print_arr, split};

use librint::scf::{nmol, angl, density, norm, calc_F};

use librint::cint_bas::{CINTcgto_cart, CINTcgto_spheric};
use librint::cint1e::{cint1e_ovlp_cart, cint1e_nuc_cart, cint1e_ovlp_sph, cint1e_nuc_sph};
use librint::intor1::{cint1e_kin_cart, cint1e_kin_sph};
use librint::cint2e::{cint2e_cart, cint2e_sph};

#[no_mangle]
pub fn kinf(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    coord: i32,
    typec: i32,
) -> Vec<f64> {
    let (natm, nbas) = nmol(atm, bas);
    let nshells = angl(bas, 0);
    
    let mut R = vec![0.0; nshells * nshells];

    let mut buf: Vec<f64>;
    let mut shls: [i32; 4] = [0, 0, 0, 0];

    let mut mu;
    let mut nu;

    let mut di;
    let mut dj;

    mu = 0;
    for i in 0..nbas {
        shls[0] = i as i32;
        di = CINTcgto_spheric(i, &bas) as usize;

        nu = 0;
        for j in 0..nbas {
            shls[1] = j as i32;
            dj = CINTcgto_spheric(j, &bas) as usize;

            buf = vec![0.0; di * dj];

            cint1e_kin_sph(&mut buf, &mut shls, atm, natm as i32, bas, nbas as i32, env);
            let mut c: usize = 0;
            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    R[mui * nshells + nuj] = buf[c];
                    c += 1;
                }
            }

            nu += dj;
        }
        mu += di;
    }
    
    return R;
}

#[no_mangle]
pub fn nucf(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    coord: i32,
    typec: i32,
) -> Vec<f64> {
    let (natm, nbas) = nmol(atm, bas);
    let nshells = angl(bas, 0);
    
    let mut R = vec![0.0; nshells * nshells];

    let mut buf: Vec<f64>;
    let mut shls: [i32; 4] = [0, 0, 0, 0];

    let mut mu;
    let mut nu;

    let mut di;
    let mut dj;

    mu = 0;
    for i in 0..nbas {
        shls[0] = i as i32;
        di = CINTcgto_spheric(i, &bas) as usize;

        nu = 0;
        for j in 0..nbas {
            shls[1] = j as i32;
            dj = CINTcgto_spheric(j, &bas) as usize;

            buf = vec![0.0; di * dj];

            cint1e_nuc_sph(&mut buf, &mut shls, atm, natm as i32, bas, nbas as i32, env);
            let mut c: usize = 0;
            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    R[mui * nshells + nuj] = buf[c];
                    c += 1;
                }
            }

            nu += dj;
        }
        mu += di;
    }
    
    return R;
}

#[no_mangle]
pub fn twof(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    coord: i32,
) -> Vec<f64> {
    let (natm, nbas) = nmol(atm, bas);
    let nshells = angl(bas, 0);

    let mut R = vec![0.0; nshells * nshells * nshells * nshells];

    let mut buf: Vec<f64>;
    let mut shls: [i32; 4] = [0, 0, 0, 0];

    let mut mu;
    let mut nu;
    let mut sig;
    let mut lam;

    let mut di;
    let mut dj;
    let mut dk;
    let mut dl;

    mu = 0;
    for i in 0..nbas {
        shls[0] = i as i32;
        di = CINTcgto_spheric(i, &bas) as usize;

        nu = 0;
        for j in 0..nbas {
            shls[1] = j as i32;
            dj = CINTcgto_spheric(j, &bas) as usize;

            sig = 0;
            for k in 0..nbas {
                shls[2] = k as i32;
                dk = CINTcgto_spheric(k, &bas) as usize;
                
                lam = 0;
                for l in 0..nbas {
                    shls[3] = l as i32;
                    dl = CINTcgto_spheric(l, &bas) as usize;
                    
                    buf = vec![0.0; di * dj * dk * dl];
        
                    cint2e_sph(&mut buf, &mut shls, atm, natm as i32, bas, nbas as i32, env);
                    let mut c: usize = 0;
                    for laml in lam..(lam + dl) {
                        for sigk in sig..(sig + dk) {
                            for nuj in nu..(nu + dj) {
                                for mui in mu..(mu + di) {
                                    R[mui*nshells.pow(3) + nuj*nshells.pow(2) + sigk*nshells + laml] = buf[c];
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

    return R;
}

#[no_mangle]
pub fn energyw(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    P: &mut Vec<f64>,
) -> f64 {
    let (natm, nbas) = nmol(atm, bas);
    let nshells = angl(bas, 0);

    // let (_, H, two) = integrals(natm, nbas, nshells, atm, bas, env);
    let T = kinf(atm, bas, env, 1, 1);
    let V = nucf(atm, bas, env, 1, 2);
    let two = twof(atm, bas, env, 1);

    let mut H = vec![0.0; T.len()];
    for i in 0..H.len() {
        H[i] = T[i] + V[i];
    }

    let F = calc_F(nshells, P, &two, &H);

    let mut E0: f64 = 0.0;
    for mu in 0..nshells {
        for nu in 0..nshells {
            E0 += 0.5 * P[mu*nshells + nu] * (H[mu*nshells + nu]  + F[mu*nshells + nu]);
        }
    }

    let mut Enuc: f64 = 0.0;
    for i in 0..natm {
        for j in 0..natm {
            if i > j {
                Enuc += (atm[i*6 + 0] * atm[j*6 + 0]) as f64 / (norm(atm, env, i, j));
            }
        }
    }

    return E0 + Enuc;
}

#[no_mangle]
#[autodiff(gradscf, Reverse, Const, Const, Const, Duplicated, Const, Active)]
pub fn energyscf(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env1: &mut Vec<f64>,
    env2: &mut Vec<f64>,
    P: &mut Vec<f64>,
) -> f64 {
    let mut env = vec![0.0; env1.len() + env2.len()];

    let mut c = 0;
    for i in 0..env1.len() {
        env[c] = env1[i];
        c += 1;
    }
    for j in 0..env2.len() {
        env[c] = env2[j];
        c += 1;
    }

    return energyw(atm, bas, &mut env, P);
}

fn main() -> io::Result<()> {
    const nelec: usize = 10;

    let mut atm = Vec::new();
    let mut bas = Vec::new();
    let mut env = Vec::new();

    let path = librint::get_path();
    read_basis(&path, &mut atm, &mut bas, &mut env)?;

    let (nbas, nshells) = nmol(&atm, &bas);
    //let (natm, nbas, nshells) = nmol(&atm, &bas);
    let nshells = angl(&bas, 0);
    println!("{} {}", nbas, nshells);
    //println!("{} {} {}", natm, nbas, nshells);

    const imax: i32 = 20;
    const conv: f64 = 0.000001;

    // let mut S = integral1e(&mut atm, &mut bas, &mut env, 1, 0);
    // print_arr(nshells, 2, &mut S);

    let mut P = density(&mut atm, &mut bas, &mut env, nelec, imax, conv);
    print_arr(nshells, 2, &mut P);

    let Etot: f64 = energyw(&mut atm, &mut bas, &mut env, &mut P);
    println!("E: {}", Etot);

    let (s1, s2) = split(&mut bas);
    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let mut denv = vec![0.0; s2-s1];
    let _dEtot = gradscf(&mut atm, &mut bas, &mut env1, &mut env2, &mut denv, &mut P, 1.0);
    println!("denv: {:.6?}", denv);

    Ok(())
}
