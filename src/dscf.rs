#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#![feature(autodiff)]

use std::autodiff::autodiff;
use crate::cint_bas::CINTcgto_cart;
use crate::cint1e::{cint1e_ovlp_cart, cint1e_nuc_cart};
use crate::intor1::cint1e_kin_cart;
use crate::cint2e::cint2e_cart;

use crate::scf::{nmol, angl, integral1e, integral2e, calc_F, energy, energyfast};
use crate::utils::{split, combine};
use crate::linalg::matmult;

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


#[no_mangle]
#[autodiff(dkin, Reverse, Duplicated, Const, Const, Const, Const, Duplicated)]
fn kin(
    out: &mut Vec<f64>, 
    shls: &mut Vec<i32>, 
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>, 
    env1: &mut Vec<f64>,
    env2: &mut Vec<f64>,
) {
    let (natm, nbas) = nmol(atm, bas);
    let mut env: Vec<f64> = combine(&env1, &env2);
    cint1e_kin_cart(out, shls, atm, natm as i32, bas, nbas as i32, &mut env);
}

#[no_mangle]
#[autodiff(dnuc, Reverse, Duplicated, Const, Const, Const, Const, Duplicated)]
fn nuc(
    out: &mut Vec<f64>, 
    shls: &mut Vec<i32>, 
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>, 
    env1: &mut Vec<f64>,
    env2: &mut Vec<f64>,
) {
    let (natm, nbas) = nmol(atm, bas);
    let mut env: Vec<f64> = combine(&env1, &env2);
    cint1e_nuc_cart(out, shls, atm, natm as i32, bas, nbas as i32, &mut env);
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
) {
    let (natm, nbas) = nmol(atm, bas);
    let mut env: Vec<f64> = combine(&env1, &env2);
    cint2e_cart(out, shls, atm, natm as i32, bas, nbas as i32, &mut env);
}

#[no_mangle]
fn dSf(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env1: &mut Vec<f64>,
    env2: &mut Vec<f64>,
    Q: &Vec<f64>,
) -> Vec<f64> {
    let (_, nbas) = nmol(&atm, &bas);
    let nshells = angl(&bas, 0);

    let mut dS = vec![0.0; env2.len()];

    let mut buf;
    let mut dbuf;
    let mut denv;
    let mut shls = vec![0; 4];

    let mut mu;
    let mut nu;

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
                    dovlp(&mut buf, &mut dbuf, &mut shls, atm, bas, env1, env2, &mut denv);
                    for l in 0..env2.len() {
                        dS[l] += Q[nuj * nshells + mui] * denv[l];
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
fn dTf(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env1: &mut Vec<f64>,
    env2: &mut Vec<f64>,
    P: &Vec<f64>,
) -> Vec<f64> {
    let (_, nbas) = nmol(&atm, &bas);
    let nshells = angl(&bas, 0);

    let mut dT = vec![0.0; env2.len()];

    let mut buf;
    let mut dbuf;
    let mut denv;
    let mut shls = vec![0; 4];

    let mut mu;
    let mut nu;

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
                    dkin(&mut buf, &mut dbuf, &mut shls, atm, bas, env1, env2, &mut denv);
                    for l in 0..env2.len() {
                        dT[l] += P[nuj * nshells + mui] * denv[l];
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
fn dVf(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env1: &mut Vec<f64>,
    env2: &mut Vec<f64>,
    P: &Vec<f64>,
) -> Vec<f64> {
    let (_, nbas) = nmol(&atm, &bas);
    let nshells = angl(&bas, 0);

    let mut dV = vec![0.0; env2.len()];

    let mut buf;
    let mut dbuf;
    let mut denv;
    let mut shls = vec![0; 4];

    let mut mu;
    let mut nu;

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
                    dnuc(&mut buf, &mut dbuf, &mut shls, atm, bas, env1, env2, &mut denv);
                    for l in 0..env2.len() {
                        dV[l] += P[nuj * nshells + mui] * denv[l];
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
pub fn dHcoreg(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    P: &Vec<f64>,
) -> Vec<f64> {
    let (s1, s2) = split(bas);

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let mut dHcore = vec![0.0; env2.len()];

    let dT = dTf(atm, bas, &mut env1, &mut env2, &P);
    let dV = dVf(atm, bas, &mut env1, &mut env2, &P);

    for i in 0..env2.len() {
        dHcore[i] = dT[i] + dV[i];
    }

    return dHcore;
}

#[no_mangle]
pub fn getF(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    P: &Vec<f64>,
) -> Vec<f64> {
    let nshells = angl(&bas, 0);

    let T = integral1e(atm, bas, env, 0, 1);
    let V = integral1e(atm, bas, env, 0, 2);
    let two = integral2e(atm, bas, env, 0);

    let mut H = vec![0.0; T.len()];
    for i in 0..H.len() {
        H[i] = T[i] + V[i];
    }

    let F = calc_F(nshells, P, &two, &H);
    return F;
}

#[no_mangle]
pub fn dSg(
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

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let dS = dSf(atm, bas, &mut env1, &mut env2, &Q);

    return dS;
}

#[no_mangle]
pub fn dRf(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env1: &mut Vec<f64>,
    env2: &mut Vec<f64>,
    P: &Vec<f64>,
) -> Vec<f64> {
    let (_, nbas) = nmol(&atm, &bas);
    let nshells = angl(&bas, 0);

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
                                    dtwo(&mut buf, &mut dbuf, &mut shls, atm, bas, env1, env2, &mut denv);
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

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let dR = dRf(atm, bas, &mut env1, &mut env2, P);
    return dR;
}

#[no_mangle]
pub fn danalyticalg(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    P: &Vec<f64>,
) -> Vec<f64> {
    let dH = dHcoreg(atm, bas, env, P);
    let dR = dRg(atm, bas, env, P); 
    let dS = dSg(atm, bas, env, P);

    let mut dtotal = vec![0.0; dH.len()];
    for i in 0..dtotal.len() {
        dtotal[i] = dH[i] + dR[i] - 0.5 * dS[i];
    }

    return dtotal;
}

#[no_mangle]
#[autodiff(denergy, Reverse, Const, Const, Const, Duplicated, Const, Active)]
pub fn energywrap(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env1: &mut Vec<f64>,
    env2: &mut Vec<f64>,
    P: &mut Vec<f64>,
) -> f64 {
    let mut env: Vec<f64> = combine(&env1, &env2);
    return energy(atm, bas, &mut env, P);
}

#[no_mangle]
pub fn gradenergy(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    P: &mut Vec<f64>,
) -> Vec<f64> {
    let (s1, s2) = split(bas);

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let mut denv: Vec<f64> = vec![0.0; s2-s1];
    let _ = denergy(atm, bas, &mut env1, &mut env2, &mut denv, P, 1.0);

    return denv;
}

#[no_mangle]
pub fn denergyg(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    P: &mut Vec<f64>,
) -> Vec<f64> {
    let denergy = gradenergy(atm, bas, env, P);
    let dS = dSg(atm, bas, env, P);

    let mut dtotal = vec![0.0; denergy.len()];
    for i in 0..dtotal.len() {
        dtotal[i] = denergy[i] - 0.5 * dS[i];
    }

    return dtotal;
}

#[no_mangle]
#[autodiff(denergyf, Reverse, Const, Const, Const, Duplicated, Const, Active)]
pub fn energyf(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env1: &mut Vec<f64>,
    env2: &mut Vec<f64>,
    P: &mut Vec<f64>,
) -> f64 {
    let mut env: Vec<f64> = combine(&env1, &env2);
    return energyfast(atm, bas, &mut env, P);
}

#[no_mangle]
pub fn gradenergyfast(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    P: &mut Vec<f64>,
) -> Vec<f64> {
    let (s1, s2) = split(bas);

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let mut denv: Vec<f64> = vec![0.0; s2-s1];
    let _ = denergyf(atm, bas, &mut env1, &mut env2, &mut denv, P, 1.0);

    return denv;
}

#[no_mangle]
pub fn denergyfast(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    P: &mut Vec<f64>,
) -> Vec<f64> {
    let denergy = gradenergyfast(atm, bas, env, P);
    let dS = dSg(atm, bas, env, P);

    let mut dtotal = vec![0.0; denergy.len()];
    for i in 0..dtotal.len() {
        dtotal[i] = denergy[i] - 0.5 * dS[i];
    }

    return dtotal;
}
