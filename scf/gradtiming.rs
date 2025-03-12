#![allow(non_snake_case, non_upper_case_globals)]
#![feature(autodiff)]
use std::autodiff::autodiff;
use std::io;
use std::time::Instant;

use librint::utils::{read_basis, split, print_arr, combine};

use librint::scf::{nmol, angl, calc_F, integral1e, integral2e, density, energy, energyfast};
use librint::dscf::getF;

use librint::linalg::matmult;

use librint::cint_bas::CINTcgto_cart;
use librint::cint1e::cint1e_ovlp_cart;
use librint::cint1e::cint1e_nuc_cart;
use librint::intor1::cint1e_kin_cart;
use librint::cint2e::cint2e_cart;

#[no_mangle]
#[autodiff(dovlpt, Reverse, Duplicated, Const, Const, Const, Const, Duplicated)]
pub fn ovlpt(
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
#[autodiff(dkint, Reverse, Duplicated, Const, Const, Const, Const, Duplicated)]
fn kint(
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
#[autodiff(dnuct, Reverse, Duplicated, Const, Const, Const, Const, Duplicated)]
fn nuct(
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
#[autodiff(dtwot, Reverse, Duplicated, Const, Const, Const, Const, Duplicated)]
fn twot(
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
fn dSft(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env1: &mut Vec<f64>,
    env2: &mut Vec<f64>,
    Q: &Vec<f64>,
) -> Vec<f64> {
    let (_, nbas) = nmol(atm, bas);
    let nshells = angl(bas, 0);

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
                    dovlpt(&mut buf, &mut dbuf, &mut shls, atm, bas, env1, env2, &mut denv);
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
fn dTft(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env1: &mut Vec<f64>,
    env2: &mut Vec<f64>,
    P: &Vec<f64>,
) -> Vec<f64> {
    let (_, nbas) = nmol(atm, bas);
    let nshells = angl(bas, 0);

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
                    dkint(&mut buf, &mut dbuf, &mut shls, atm, bas, env1, env2, &mut denv);
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
fn dVft(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env1: &mut Vec<f64>,
    env2: &mut Vec<f64>,
    P: &Vec<f64>,
) -> Vec<f64> {
    let (_, nbas) = nmol(atm, bas);
    let nshells = angl(bas, 0);

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
                    dnuct(&mut buf, &mut dbuf, &mut shls, atm, bas, env1, env2, &mut denv);
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
pub fn dHcoregt(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    P: &Vec<f64>,
) -> Vec<f64> {
    let (s1, s2) = split(bas);

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let mut dHcore = vec![0.0; env2.len()];

    let dT = dTft(atm, bas, &mut env1, &mut env2, &P);
    let dV = dVft(atm, bas, &mut env1, &mut env2, &P);

    for i in 0..env2.len() {
        dHcore[i] = dT[i] + dV[i];
    }

    return dHcore;
}

#[no_mangle]
pub fn getFt(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    P: &Vec<f64>,
) -> Vec<f64> {
    let nshells = angl(bas, 0);

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
pub fn dSgt(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    P: &Vec<f64>,
) -> Vec<f64> {
    let nshells = angl(bas, 0);

    let F = getF(atm, bas, env, P);

    let (s1, s2) = split(bas);

    let pf = matmult(nshells, P, &F);
    let Q = matmult(nshells, &pf, P); // PFP

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let dS = dSft(atm, bas, &mut env1, &mut env2, &Q);

    return dS;
}

#[no_mangle]
pub fn dRft(
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
                                    dtwot(&mut buf, &mut dbuf, &mut shls, atm, bas, env1, env2, &mut denv);
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
pub fn dRgt(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    P: &Vec<f64>,
) -> Vec<f64> {
    let (s1, s2) = split(bas);

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let dR = dRft(atm, bas, &mut env1, &mut env2, P);
    return dR;
}

#[no_mangle]
pub fn dgradt(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    P: &Vec<f64>,
) -> Vec<f64> {
    let dH = dHcoregt(atm, bas, env, P);
    let dR = dRgt(atm, bas, env, P); 
    let dS = dSgt(atm, bas, env, P);

    let mut dtotal = vec![0.0; dH.len()];
    for i in 0..dtotal.len() {
        dtotal[i] = dH[i] + dR[i] + dS[i];
    }

    return dtotal;
}

#[no_mangle]
#[autodiff(denergyt, Reverse, Const, Const, Const, Duplicated, Const, Active)]
pub fn energyt(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env1: &mut Vec<f64>,
    env2: &mut Vec<f64>,
    P: &mut Vec<f64>,
) -> f64 {
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

    return energy(atm, bas, &mut env, P);
}

#[no_mangle]
pub fn gradenergyt(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    P: &mut Vec<f64>,
) -> Vec<f64> {
    let (s1, s2) = split(bas);

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let mut denv: Vec<f64> = vec![0.0; s2-s1];

    let _ = denergyt(atm, bas, &mut env1, &mut env2, &mut denv, P, 1.0);

    return denv;
}

#[no_mangle]
#[autodiff(denergycontt, Reverse, Const, Const, Const, Duplicated, Const, Active)]
pub fn energyct(
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
pub fn gradcontenergyt(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    P: &mut Vec<f64>,
) -> Vec<f64> {
    let (s1, s2) = split(bas);

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let mut denv: Vec<f64> = vec![0.0; s2-s1];
    let _ = denergycontt(atm, bas, &mut env1, &mut env2, &mut denv, P, 1.0);

    return denv;
}

#[no_mangle]
pub fn denergycontgt(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    P: &mut Vec<f64>,
) -> Vec<f64> {
    let denergy = gradcontenergyt(atm, bas, env, P);
    let dS = dSgt(atm, bas, env, P);

    let mut dtotal = vec![0.0; denergy.len()];
    for i in 0..dtotal.len() {
        dtotal[i] = denergy[i] - 0.5 * dS[i];
    }

    return dtotal;
}

fn main() -> io::Result<()> {
    let mut atm = Vec::new();
    let mut bas = Vec::new();
    let mut env = Vec::new();

    let path = "/u/jpmedina/libcint/librint/molecules/h2/sto3g.txt";
    read_basis(path, &mut atm, &mut bas, &mut env)?;

    let nshells = angl(&bas, 0);

    let (s1, s2) = split(&mut bas);

    const nelec: usize = 2;

    let mut P = density(&mut atm, &mut bas, &mut env, nelec, 20, 1e-6);

    let mut dA = vec![0.0; s2-s1];
    let now = Instant::now();
    let dH = dHcoregt(&mut atm, &mut bas, &mut env, &mut P);
    let dR = dRgt(&mut atm, &mut bas, &mut env, &mut P);
    let dS = dSgt(&mut atm, &mut bas, &mut env, &mut P);
    for i in 0..dH.len() {
        dA[i] = dH[i] + dR[i] + dS[i];
    }
    let delapsed_time = now.elapsed();
    println!("time: {}", delapsed_time.as_micros());

    let now = Instant::now();
    let dG = dgradt(&mut atm, &mut bas, &mut env, &mut P);
    let delapsed_time = now.elapsed();
    println!("time: {}", delapsed_time.as_micros());

    let now = Instant::now();
    let dAD = gradenergyt(&mut atm, &mut bas, &mut env, &mut P);
    let delapsed_time = now.elapsed();
    println!("time: {}", delapsed_time.as_micros());

    let now = Instant::now();
    let dADOPT = denergycontgt(&mut atm, &mut bas, &mut env, &mut P);
    let delapsed_time = now.elapsed();
    println!("time: {}", delapsed_time.as_micros());

    Ok(())
}
