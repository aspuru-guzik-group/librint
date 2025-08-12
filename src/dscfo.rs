#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#![feature(autodiff)]

use std::autodiff::autodiff;
use crate::cint_bas::CINTcgto_cart;
use crate::cint1e::{cint1e_ovlp_cart, cint1e_nuc_cart};
use crate::intor1::cint1e_kin_cart;
use crate::cint2e::cint2e_cart;

use crate::scf::{nmol, angl};

use crate::dscf::getF;

use crate::p2c::c2r_arr;

use crate::utils::split;
use crate::linalg::matmult;

#[no_mangle]
pub fn combine_env(
    env1: &Vec<f64>,
    env2: &Vec<f64>,
    env3: &Vec<f64>,
    env4: &Vec<f64>,
    env5: &Vec<f64>,
) -> Vec<f64> {
    let mut env: Vec<f64> = vec![0.0; env1.len() + env2.len() + env3.len() + env4.len() + env5.len()];

    let mut c = 0;
    for i in 0..env1.len() {
        env[c] = env1[i];
        c += 1;
    }
    for j in 0..env2.len() {
        env[c] = env2[j];
        c += 1;
    }
    for j in 0..env3.len() {
        env[c] = env3[j];
        c += 1;
    }
    for j in 0..env4.len() {
        env[c] = env4[j];
        c += 1;
    }
    for j in 0..env5.len() {
        env[c] = env5[j];
        c += 1;
    }

    return env;
}

fn env_range_for_basis(
    bas: &[i32], 
    basis: usize,
) -> (usize, usize) {
    let offset = basis * 8;
    let nprim = bas[offset + 2] as usize;
    let p_exp = bas[offset + 5] as usize;
    let p_coeff = bas[offset + 6] as usize;

    let start = p_exp.min(p_coeff);
    let end = p_exp.max(p_coeff) + nprim;
    return (start, end)
}

fn split_env(
    bas: &[i32],
    env: &[f64],
    mu: usize,
    nu: usize,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>){
    let (mu_start, mu_end) = env_range_for_basis(bas, mu);
    let (nu_start, nu_end) = env_range_for_basis(bas, nu);

    // println!("{} {}", mu, nu);

    // println!("{} {} {} {}", mu_start, mu_end, nu_start, nu_end);

    let (s1, e1, s2, e2) = if mu_start <= nu_start {
        (mu_start, mu_end, nu_start, nu_end)
    } else {
        (nu_start, nu_end, mu_start, mu_end)
    };

    // println!("{} {} {} {}", s1, e1, s2, e2);

    let env1;
    let env2;
    let env3;
    let env4;
    let env5;

    // If mu == nu, merge env2 and env4, and clear env3/env4/env5
    if mu == nu {
        env2 = env[mu_start..mu_end].to_vec();
        env1 = env[..mu_start].to_vec();
        env5 = env[mu_end..].to_vec();

        // println!("{}-{} {}-{} {}-{} {}-{} {}-{}", 
        //     0, env1.len() - 1,
        //     env1.len(), env1.len() + env2.len() - 1,
        //     env1.len() + env2.len(), env1.len() + env2.len(),
        //     env1.len() + env2.len(), env1.len() + env2.len(),
        //     env1.len() + env2.len(), env1.len() + env2.len() + env5.len() - 1
        // );

        return (env1, env2, vec![], vec![], env5);
    }

    env1 = env[..s1].to_vec();
    env2 = env[s1..e1].to_vec();
    env3 = env[e1..s2].to_vec();
    env4 = env[s2..e2].to_vec();
    env5 = env[e2..].to_vec();

    // println!("{}-{} {}-{} {}-{} {}-{} {}-{}", 
    //     0, env1.len() - 1,
    //     env1.len(), env1.len() + env2.len() - 1,
    //     env1.len() + env2.len(), env1.len() + env2.len() + env3.len() - 1,
    //     env1.len() + env2.len() + env3.len(), env1.len() + env2.len() + env3.len() + env4.len() - 1,
    //     env1.len() + env2.len() + env3.len() + env4.len(), env1.len() + env2.len() + env3.len() + env4.len() + env5.len() - 1,
    // );

    (env1, env2, env3, env4, env5)
}

#[no_mangle]
#[autodiff(dovlpo, Reverse, Duplicated, Const, Const, Const, Const, Duplicated, Const, Duplicated, Const)]
pub fn ovlpo(
    out: &mut Vec<f64>, 
    shls: &mut Vec<i32>, 
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env1: &mut Vec<f64>,
    env2: &mut Vec<f64>,
    env3: &mut Vec<f64>,
    env4: &mut Vec<f64>,
    env5: &mut Vec<f64>,
) {
    let (natm, nbas) = nmol(atm, bas);

    let mut env = combine_env(&env1, &env2, &env3, &env4, &env5);
    cint1e_ovlp_cart(out, shls, atm, natm as i32, bas, nbas as i32, &mut env, std::ptr::null_mut());
}

#[no_mangle]
pub fn dSo(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    Q: &Vec<f64>,
) -> Vec<f64> {
    let (_, nbas) = nmol(&atm, &bas);
    let nshells = angl(&bas, 0);

    let mut dS = vec![0.0; env.len()];

    let mut buf;
    let mut dbuf;
    
    let mut denv2;
    let mut denv4;

    let mut shls = vec![0; 4];

    let mut mu;
    let mut nu;

    let mut env1;
    let mut env2;
    let mut env3;
    let mut env4;
    let mut env5;

    println!("dS size");
    println!("{} {}", dS.len(), env.len());

    mu = 0;
    for i in 0..nbas {
        shls[0] = i as i32; let di = CINTcgto_cart(i, &bas) as usize;
        nu = 0;
        for j in 0..nbas {
            shls[1] = j as i32; let dj = CINTcgto_cart(j, &bas) as usize;

            buf = vec![0.0; di * dj];
            dbuf = vec![0.0; di * dj];

            (env1, env2, env3, env4, env5) = split_env(bas, env, i, j);
            
            println!("env sizes");
            println!("{} {} {} {} {}", env1.len(), env2.len(), env3.len(), env4.len(), env5.len());

            let mut c: usize = 0;
            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    dbuf[c] = 1.0;

                    denv2 = vec![0.0; env2.len()];
                    denv4 = vec![0.0; env4.len()];
                    dovlpo(&mut buf, &mut dbuf, &mut shls, atm, bas, &mut env1, &mut env2, &mut denv2, &mut env3, &mut env4, &mut denv4, &mut env5);
                    
                    let start_i;
                    let start_j;
                    
                    let end_i;
                    let end_j;
                    
                    if j < i {
                        (start_i, end_i) = env_range_for_basis(bas, j);
                        (start_j, end_j) = env_range_for_basis(bas, i);
                    } else {
                        (start_i, end_i) = env_range_for_basis(bas, i);
                        (start_j, end_j) = env_range_for_basis(bas, j);
                    }

                    println!("roi i j of env");
                    println!("{} {}", start_i, end_i);
                    println!("{} {}", start_j, end_j);

                    for k in 0..denv2.len() {
                        dS[start_i + k] += Q[nuj * nshells + mui] * denv2[k];
                    }

                    if i != j {
                        for k in 0..denv4.len() {
                            dS[start_j + k] += Q[nuj * nshells + mui] * denv4[k];
                        }
                    }
                    
                    dbuf[c] = 0.0;
                    c += 1;
                }
            }
            nu += dj;
        }
        mu += di;
    }
    
    return dS;
}

#[no_mangle]
pub fn dSgo(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    P: &Vec<f64>,
) -> Vec<f64> {
    let nshells = angl(&bas, 0);

    let F = getF(atm, bas, env, P);

    let (s1, s2) = split(bas);

    let pf = matmult(nshells, P, &F);
    let Q = matmult(nshells, &pf, P); // PFP

    // let env1: Vec<f64> = env[0..s1].to_vec();
    // let env2: Vec<f64> = env[s1..s2].to_vec();

    let dS = dSo(atm, bas, env, &Q);

    return dS[s1..s2].to_vec();
}

#[no_mangle]
pub extern "C" fn dSo_c(
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
    P_p: *mut f64,
    P_l: usize,
) -> *mut f64 {
    let (mut atm, mut bas, mut env) = c2r_arr(atm_p, atm_l, bas_p, bas_l, env_p, env_l);

    let P_slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(P_p, P_l) };
    let mut P: Vec<f64> = P_slice.to_vec();

    let mut dS = dSgo(&mut atm, &mut bas, &mut env, &mut P);

    let dS_ptr = dS.as_mut_ptr();
    std::mem::forget(dS);
    return dS_ptr;
}



#[no_mangle]
#[autodiff(dkino, Reverse, Duplicated, Const, Const, Const, Const, Duplicated, Const, Duplicated, Const)]
fn kino(
    out: &mut Vec<f64>, 
    shls: &mut Vec<i32>, 
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env1: &mut Vec<f64>,
    env2: &mut Vec<f64>,
    env3: &mut Vec<f64>,
    env4: &mut Vec<f64>,
    env5: &mut Vec<f64>,
) {
    let (natm, nbas) = nmol(atm, bas);

    let mut env = combine_env(&env1, &env2, &env3, &env4, &env5);
    cint1e_kin_cart(out, shls, atm, natm as i32, bas, nbas as i32, &mut env, std::ptr::null_mut());
}

#[no_mangle]
pub fn dTo(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    P: &Vec<f64>,
) -> Vec<f64> {
    let (_, nbas) = nmol(&atm, &bas);
    let nshells = angl(&bas, 0);

    let mut dT = vec![0.0; env.len()];

    let mut buf;
    let mut dbuf;
    
    let mut denv2;
    let mut denv4;

    let mut shls = vec![0; 4];

    let mut mu;
    let mut nu;

    let mut env1;
    let mut env2;
    let mut env3;
    let mut env4;
    let mut env5;

    mu = 0;
    for i in 0..nbas {
        shls[0] = i as i32; let di = CINTcgto_cart(i, &bas) as usize;
        nu = 0;
        for j in 0..nbas {
            shls[1] = j as i32; let dj = CINTcgto_cart(j, &bas) as usize;

            buf = vec![0.0; di * dj];
            dbuf = vec![0.0; di * dj];

            (env1, env2, env3, env4, env5) = split_env(bas, env, i, j);

            let mut c: usize = 0;
            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    dbuf[c] = 1.0;

                    denv2 = vec![0.0; env2.len()];
                    denv4 = vec![0.0; env4.len()];
                    dkino(&mut buf, &mut dbuf, &mut shls, atm, bas, &mut env1, &mut env2, &mut denv2, &mut env3, &mut env4, &mut denv4, &mut env5);
                    
                    let (start_i, end_i) = env_range_for_basis(bas, i);
                    let (start_j, end_j) = env_range_for_basis(bas, j);

                    for k in 0..denv2.len() {
                        dT[start_i + k] += P[nuj * nshells + mui] * denv2[k];
                    }

                    if i != j {
                        for k in 0..denv4.len() {
                            dT[start_j + k] += P[nuj * nshells + mui] * denv4[k];
                        }
                    }
                    
                    dbuf[c] = 0.0;
                    c += 1;
                }
            }
            nu += dj;
        }
        mu += di;
    }
    
    return dT;
}

#[no_mangle]
#[autodiff(dnuco, Reverse, Duplicated, Const, Const, Const, Const, Duplicated, Const, Duplicated, Const)]
fn nuco(
    out: &mut Vec<f64>, 
    shls: &mut Vec<i32>, 
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env1: &mut Vec<f64>,
    env2: &mut Vec<f64>,
    env3: &mut Vec<f64>,
    env4: &mut Vec<f64>,
    env5: &mut Vec<f64>,
) {
    let (natm, nbas) = nmol(atm, bas);

    let mut env = combine_env(&env1, &env2, &env3, &env4, &env5);
    cint1e_nuc_cart(out, shls, atm, natm as i32, bas, nbas as i32, &mut env, std::ptr::null_mut());
}

#[no_mangle]
pub fn dVo(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    P: &Vec<f64>,
) -> Vec<f64> {
    let (_, nbas) = nmol(&atm, &bas);
    let nshells = angl(&bas, 0);

    let mut dV = vec![0.0; env.len()];

    let mut buf;
    let mut dbuf;
    
    let mut denv2;
    let mut denv4;

    let mut shls = vec![0; 4];

    let mut mu;
    let mut nu;

    let mut env1;
    let mut env2;
    let mut env3;
    let mut env4;
    let mut env5;

    mu = 0;
    for i in 0..nbas {
        shls[0] = i as i32; let di = CINTcgto_cart(i, &bas) as usize;
        nu = 0;
        for j in 0..nbas {
            shls[1] = j as i32; let dj = CINTcgto_cart(j, &bas) as usize;

            buf = vec![0.0; di * dj];
            dbuf = vec![0.0; di * dj];

            (env1, env2, env3, env4, env5) = split_env(bas, env, i, j);

            let mut c: usize = 0;
            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    dbuf[c] = 1.0;

                    denv2 = vec![0.0; env2.len()];
                    denv4 = vec![0.0; env4.len()];
                    dnuco(&mut buf, &mut dbuf, &mut shls, atm, bas, &mut env1, &mut env2, &mut denv2, &mut env3, &mut env4, &mut denv4, &mut env5);
                    
                    let (start_i, end_i) = env_range_for_basis(bas, i);
                    let (start_j, end_j) = env_range_for_basis(bas, j);

                    for k in 0..denv2.len() {
                        dV[start_i + k] += P[nuj * nshells + mui] * denv2[k];
                    }

                    if i != j {
                        for k in 0..denv4.len() {
                            dV[start_j + k] += P[nuj * nshells + mui] * denv4[k];
                        }
                    }
                    
                    dbuf[c] = 0.0;
                    c += 1;
                }
            }
            nu += dj;
        }
        mu += di;
    }
    
    return dV;
}

#[no_mangle]
pub fn dHcorego(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    P: &Vec<f64>,
) -> Vec<f64> {
    let (s1, s2) = split(bas);

    // let mut env1: Vec<f64> = env[0..s1].to_vec();
    // let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let mut dHcore = vec![0.0; env.len()];

    let dT = dTo(atm, bas, env, P);
    let dV = dVo(atm, bas, env, P);

    for i in 0..env.len() {
        dHcore[i] = dT[i] + dV[i];
    }

    return dHcore[s1..s2].to_vec();
}

#[no_mangle]
pub extern "C" fn dHcoreo_c(
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
    P_p: *mut f64,
    P_l: usize,
) -> *mut f64 {
    let (mut atm, mut bas, mut env) = c2r_arr(atm_p, atm_l, bas_p, bas_l, env_p, env_l);

    let P_slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(P_p, P_l) };
    let mut P: Vec<f64> = P_slice.to_vec();

    let mut dH = dHcorego(&mut atm, &mut bas, &mut env, &mut P);

    let dH_ptr = dH.as_mut_ptr();
    std::mem::forget(dH);
    return dH_ptr;
}



#[no_mangle]
pub fn combine_env_four(
    env1: &Vec<f64>,
    env2: &Vec<f64>,
    env3: &Vec<f64>,
    env4: &Vec<f64>,
    env5: &Vec<f64>,
    env6: &Vec<f64>,
    env7: &Vec<f64>,
    env8: &Vec<f64>,
    env9: &Vec<f64>,
) -> Vec<f64> {
    let mut env: Vec<f64> = vec![0.0; env1.len() + env2.len() 
                                    + env3.len() + env4.len() 
                                    + env5.len() + env6.len()
                                    + env7.len() + env8.len()
                                    + env9.len()];

    let mut c = 0;
    for i in 0..env1.len() {
        env[c] = env1[i];
        c += 1;
    }
    for j in 0..env2.len() {
        env[c] = env2[j];
        c += 1;
    }
    for j in 0..env3.len() {
        env[c] = env3[j];
        c += 1;
    }
    for j in 0..env4.len() {
        env[c] = env4[j];
        c += 1;
    }
    for j in 0..env5.len() {
        env[c] = env5[j];
        c += 1;
    }
    for j in 0..env6.len() {
        env[c] = env6[j];
        c += 1;
    }
    for j in 0..env7.len() {
        env[c] = env7[j];
        c += 1;
    }
    for j in 0..env8.len() {
        env[c] = env8[j];
        c += 1;
    }
    for j in 0..env9.len() {
        env[c] = env9[j];
        c += 1;
    }

    return env;
}

fn split_env_four(
    bas: &[i32],
    env: &[f64],
    mu: usize,
    nu: usize,
    sg: usize,
    lm: usize,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let ranges = vec![
        (mu, env_range_for_basis(bas, mu)),
        (nu, env_range_for_basis(bas, nu)),
        (sg, env_range_for_basis(bas, sg)),
        (lm, env_range_for_basis(bas, lm)),
    ];

    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by_key(|&(_, (start, _))| start);

    let env1;
    let env2;
    let env3;
    let env4;
    let env5;
    let env6;
    let env7;
    let env8;
    let env9;

    // If mu == nu, merge env2 and env4, and clear env3/env4/env5
    // if mu == nu {
    //     env2 = env[mu_start..mu_end].to_vec();
    //     env1 = env[..mu_start].to_vec();
    //     env5 = env[mu_end..].to_vec();

    //     return (env1, env2, vec![], vec![], env5);
    // }

    env1 = env[..s1].to_vec();
    env2 = env[s1..e1].to_vec();
    env3 = env[e1..s2].to_vec();
    env4 = env[s2..e2].to_vec();
    env5 = env[e2..].to_vec();
    env6 = env[].to_vec();
    env7 = env[].to_vec();
    env8 = env[].to_vec();
    env9 = env[].to_vec();

    (env1, env2, env3, env4, env5, env6, env7, env8, env9)
}

#[no_mangle]
#[autodiff(dtwo, Reverse, Duplicated, Const, Const, Const, Const, Duplicated)]
fn two(
    out: &mut Vec<f64>, 
    shls: &mut Vec<i32>, 
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>, 
    env1: &mut Vec<f64>,
    env2: &mut Vec<f64>,
    env3: &mut Vec<f64>,
    env4: &mut Vec<f64>,
    env5: &mut Vec<f64>,
    env6: &mut Vec<f64>,
    env7: &mut Vec<f64>,
    env8: &mut Vec<f64>,
    env9: &mut Vec<f64>,
) {
    let (natm, nbas) = nmol(atm, bas);
    let mut env: Vec<f64> = combine_env_four(&env1, &env2, &env3, &env4, &env5, &env6, &env7, &env8, &env9);
    cint2e_cart(out, shls, atm, natm as i32, bas, nbas as i32, &mut env, std::ptr::null_mut());
}

#[no_mangle]
pub fn dRo(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    P: &Vec<f64>,
) -> Vec<f64> {
    let (_, nbas) = nmol(&atm, &bas);
    let nshells = angl(&bas, 0);

    let mut dR = vec![0.0; env.len()];

    let mut buf;
    let mut dbuf;

    let env1;
    let env2;
    let env3;
    let env4;
    let env5;
    let env6;
    let env7;
    let env8;
    let env9;
    
    let mut denv2;
    let mut denv4;
    let mut denv6;
    let mut denv8;

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

                    
                    (env1, env2, env3, env4, env5, env6, env7, env8, env9) = split_env_four(bas, env, i, j, k, l);
                    
                    let mut c: usize = 0;
                    for laml in lam..(lam + dl) {
                        for sigk in sig..(sig + dk) {
                            for nuj in nu..(nu + dj) {
                                for mui in mu..(mu + di) {
                                    dbuf[c] = 1.0;

                                    denv2 = vec![0.0; env2.len()];
                                    denv4 = vec![0.0; env4.len()];
                                    denv6 = vec![0.0; env6.len()];
                                    denv8 = vec![0.0; env8.len()];
                                    
                                    dtwo(&mut buf, &mut dbuf, &mut shls, atm, bas, env1, env2, &mut denv2, env3, env4, &mut denv4, env5, env6, &mut denv6, env7, env8, &mut denv8, env9);

                                    for l in 0..env2.len() {
                                        dR[l] += 0.5 * (P[mui*nshells + nuj] * P[sigk*nshells + laml] - 0.5 * P[mui*nshells + sigk] * P[nuj*nshells + laml]) * denv[l];
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
pub fn dRg(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    P: &Vec<f64>,
) -> Vec<f64> {
    let (s1, s2) = split(bas);

    // let mut env1: Vec<f64> = env[0..s1].to_vec();
    // let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let dR = dRo(atm, bas, env, P);
    
    return dR[s1..s2].to_vec();
}