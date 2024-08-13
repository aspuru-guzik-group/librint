#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]

use crate::scf::{integral1e, integral2e, density, energyfast, scf};
use crate::dscf::{dHcoreg, dSg, dRg, gradenergy, danalyticalg, denergyfast};

#[no_mangle]
fn c2r_arr(
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
) -> (Vec<i32>, Vec<i32>, Vec<f64>) {
    let atm_slice: &mut [i32] = unsafe { std::slice::from_raw_parts_mut(atm_p, atm_l) };
    let atm: Vec<i32> = atm_slice.to_vec();

    let bas_slice: &mut [i32] = unsafe { std::slice::from_raw_parts_mut(bas_p, bas_l) };
    let bas: Vec<i32> = bas_slice.to_vec();

    let env_slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(env_p, env_l) };
    let env: Vec<f64> = env_slice.to_vec();

    return (atm, bas, env); 
}

#[no_mangle]
pub extern "C" fn int1e_c(
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
    coord: i32,
    typec: i32,
) -> *mut f64 {
    let (mut atm, mut bas, mut env) = c2r_arr(atm_p, atm_l, bas_p, bas_l, env_p, env_l);
    let mut R: Vec<f64> = integral1e(&mut atm, &mut bas, &mut env, coord, typec);

    let R_ptr = R.as_mut_ptr();
    std::mem::forget(R);
    return R_ptr;
}

#[no_mangle]
pub extern "C" fn int2e_c(
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
    coord: i32,
) -> *mut f64 {
    let (mut atm, mut bas, mut env) = c2r_arr(atm_p, atm_l, bas_p, bas_l, env_p, env_l);

    let mut R: Vec<f64> = integral2e(&mut atm, &mut bas, &mut env, coord);

    let R_ptr = R.as_mut_ptr();
    std::mem::forget(R);
    return R_ptr;
}

#[no_mangle]
pub extern "C" fn density_c(
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
    nelec: usize,
    imax: i32,
    conv: f64,
) -> *mut f64 {
    let (mut atm, mut bas, mut env) = c2r_arr(atm_p, atm_l, bas_p, bas_l, env_p, env_l);

    let mut P: Vec<f64> = density(&mut atm, &mut bas, &mut env, nelec, imax, conv);
    
    let P_ptr = P.as_mut_ptr();
    std::mem::forget(P);
    return P_ptr;
}

#[no_mangle]
pub extern "C" fn energy_c(
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
    P_p: *mut f64,
    P_l: usize,
) -> f64 {
    let (mut atm, mut bas, mut env) = c2r_arr(atm_p, atm_l, bas_p, bas_l, env_p, env_l);

    let P_slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(P_p, P_l) };
    let mut P: Vec<f64> = P_slice.to_vec();

    let E: f64 = energyfast(&mut atm, &mut bas, &mut env, &mut P);
    return E;
}

#[no_mangle]
pub extern "C" fn scf_c(
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
    nelec: usize,
    imax: i32,
    conv: f64,
) -> f64 {
    let (mut atm, mut bas, mut env) = c2r_arr(atm_p, atm_l, bas_p, bas_l, env_p, env_l);
    let E: f64 = scf(&mut atm, &mut bas, &mut env, nelec, imax, conv);
    return E;
}

#[no_mangle]
pub extern "C" fn grad_c(
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

    let mut denv: Vec<f64> = gradenergy(&mut atm, &mut bas, &mut env, &mut P);

    let denv_ptr = denv.as_mut_ptr();
    std::mem::forget(denv);
    return denv_ptr;
}

#[no_mangle]
pub extern "C" fn dS_c(
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

    let mut dS = dSg(&mut atm, &mut bas, &mut env, &mut P);

    let dS_ptr = dS.as_mut_ptr();
    std::mem::forget(dS);
    return dS_ptr;
}

#[no_mangle]
pub extern "C" fn dHcore_c(
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

    let mut dH = dHcoreg(&mut atm, &mut bas, &mut env, &mut P);

    let dH_ptr = dH.as_mut_ptr();
    std::mem::forget(dH);
    return dH_ptr;
}

#[no_mangle]
pub extern "C" fn dR_c(
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

    let mut dR = dRg(&mut atm, &mut bas, &mut env, &mut P);

    let dR_ptr = dR.as_mut_ptr();
    std::mem::forget(dR);
    return dR_ptr;
}

#[no_mangle]
pub extern "C" fn danalytical_c(
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

    let mut dR = danalyticalg(&mut atm, &mut bas, &mut env, &mut P);

    let dR_ptr = dR.as_mut_ptr();
    std::mem::forget(dR);
    return dR_ptr;
}

#[no_mangle]
pub extern "C" fn denergy_c(
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

    let mut dR = denergyfast(&mut atm, &mut bas, &mut env, &mut P);

    let dR_ptr = dR.as_mut_ptr();
    std::mem::forget(dR);
    return dR_ptr;
}