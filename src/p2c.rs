#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]

use crate::scf::{split, integral1e, integral2e, RHF, energy, denergy};

#[no_mangle]
pub extern "C" fn int1e_C(
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
    coord: i32,
    typec: i32,
) -> *mut f64 {
    let atm_slice: &mut [i32] = unsafe { std::slice::from_raw_parts_mut(atm_p, atm_l) };
    let mut atm: Vec<i32> = atm_slice.to_vec();

    let bas_slice: &mut [i32] = unsafe { std::slice::from_raw_parts_mut(bas_p, bas_l) };
    let mut bas: Vec<i32> = bas_slice.to_vec();

    let env_slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(env_p, env_l) };
    let mut env: Vec<f64> = env_slice.to_vec();

    let mut R: Vec<f64> = integral1e(&mut atm, &mut bas, &mut env, coord, typec);

    let R_ptr = R.as_mut_ptr();
    std::mem::forget(R);
    return R_ptr;
}

#[no_mangle]
pub extern "C" fn int2e_C(
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
    coord: i32,
) -> *mut f64 {
    let atm_slice: &mut [i32] = unsafe { std::slice::from_raw_parts_mut(atm_p, atm_l) };
    let mut atm: Vec<i32> = atm_slice.to_vec();

    let bas_slice: &mut [i32] = unsafe { std::slice::from_raw_parts_mut(bas_p, bas_l) };
    let mut bas: Vec<i32> = bas_slice.to_vec();

    let env_slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(env_p, env_l) };
    let mut env: Vec<f64> = env_slice.to_vec();

    let mut R: Vec<f64> = integral2e(&mut atm, &mut bas, &mut env, coord);

    let R_ptr = R.as_mut_ptr();
    std::mem::forget(R);
    return R_ptr;
}

#[no_mangle]
pub extern "C" fn RHF_C(
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
    let atm_slice: &mut [i32] = unsafe { std::slice::from_raw_parts_mut(atm_p, atm_l) };
    let mut atm: Vec<i32> = atm_slice.to_vec();

    let bas_slice: &mut [i32] = unsafe { std::slice::from_raw_parts_mut(bas_p, bas_l) };
    let mut bas: Vec<i32> = bas_slice.to_vec();

    let env_slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(env_p, env_l) };
    let mut env: Vec<f64> = env_slice.to_vec();

    let mut P: Vec<f64> = RHF(&mut atm, &mut bas, &mut env, nelec, imax, conv);
    
    let P_ptr = P.as_mut_ptr();
    std::mem::forget(P);
    return P_ptr;
}

#[no_mangle]
pub extern "C" fn energy_C(
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
    P_p: *mut f64,
    P_l: usize,
) -> f64 {
    let atm_slice: &mut [i32] = unsafe { std::slice::from_raw_parts_mut(atm_p, atm_l) };
    let mut atm: Vec<i32> = atm_slice.to_vec();

    let bas_slice: &mut [i32] = unsafe { std::slice::from_raw_parts_mut(bas_p, bas_l) };
    let mut bas: Vec<i32> = bas_slice.to_vec();

    let env_slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(env_p, env_l) };
    let mut env: Vec<f64> = env_slice.to_vec();

    let P_slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(P_p, P_l) };
    let mut P: Vec<f64> = P_slice.to_vec();

    let E: f64 = energy(&mut atm, &mut bas, &mut env, &mut P);
    return E;
}

#[no_mangle]
pub fn grad_C(
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
    P_p: *mut f64,
    P_l: usize,
) -> *mut f64 {
    let atm_slice: &mut [i32] = unsafe { std::slice::from_raw_parts_mut(atm_p, atm_l) };
    let mut atm: Vec<i32> = atm_slice.to_vec();

    let bas_slice: &mut [i32] = unsafe { std::slice::from_raw_parts_mut(bas_p, bas_l) };
    let mut bas: Vec<i32> = bas_slice.to_vec();

    let env_slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(env_p, env_l) };
    let env: Vec<f64> = env_slice.to_vec();

    let P_slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(P_p, P_l) };
    let mut P: Vec<f64> = P_slice.to_vec();

    let (s1, s2) = split(&mut bas);

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let mut denv: Vec<f64> = vec![0.0; s2-s1];

    let _ = denergy(&mut atm, &mut bas, &mut env1, &mut env2, &mut denv, &mut P, 1.0);

    let denv_ptr = denv.as_mut_ptr();
    std::mem::forget(denv);
    return denv_ptr;
}