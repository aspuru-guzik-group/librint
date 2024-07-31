use crate::scf::{integral1e, integral2e, RHF, energy, denergy};

#[no_mangle]
pub extern "C" fn int1e_C(
    natm: usize,
    nbas: usize,
    nshells: usize,
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

    let mut R: Vec<f64> = integral1e(natm, nbas, nshells, &mut atm, &mut bas, &mut env, coord, typec);

    let R_ptr = R.as_mut_ptr();
    std::mem::forget(R); // Prevent Rust from freeing the memory
    return R_ptr;
}

#[no_mangle]
pub extern "C" fn int2e_C(
    natm: usize,
    nbas: usize,
    nshells: usize,
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

    let mut R: Vec<f64> = integral2e(natm, nbas, nshells, &mut atm, &mut bas, &mut env, coord);

    let R_ptr = R.as_mut_ptr();
    std::mem::forget(R); // Prevent Rust from freeing the memory
    return R_ptr;
}

#[no_mangle]
pub extern "C" fn RHF_C(
    natm: usize,
    nbas: usize,
    nelec: usize,
    nshells: usize,
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
    imax: i32,
    conv: f64,
) -> *mut f64 {
    // Convert the pointer to a slice, and then to a Vec
    let atm_slice: &mut [i32] = unsafe { std::slice::from_raw_parts_mut(atm_p, atm_l) };
    let mut atm: Vec<i32> = atm_slice.to_vec();

    let bas_slice: &mut [i32] = unsafe { std::slice::from_raw_parts_mut(bas_p, bas_l) };
    let mut bas: Vec<i32> = bas_slice.to_vec();

    let env_slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(env_p, env_l) };
    let mut env: Vec<f64> = env_slice.to_vec();

    let mut P: Vec<f64> = RHF(natm, nbas, nelec, nshells, &mut atm, &mut bas, &mut env, imax, conv);
    
    // // Convert the result Vec back to a pointer
    let P_ptr = P.as_mut_ptr();
    std::mem::forget(P); // Prevent Rust from freeing the memory
    return P_ptr;
}

#[no_mangle]
pub extern "C" fn energy_C(
    natm: usize,
    nbas: usize,
    nshells: usize,
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
    P_p: *mut f64,
    P_l: usize,
) -> f64 {
    // Convert the pointer to a slice, and then to a Vec
    let atm_slice: &mut [i32] = unsafe { std::slice::from_raw_parts_mut(atm_p, atm_l) };
    let mut atm: Vec<i32> = atm_slice.to_vec();

    let bas_slice: &mut [i32] = unsafe { std::slice::from_raw_parts_mut(bas_p, bas_l) };
    let mut bas: Vec<i32> = bas_slice.to_vec();

    let env_slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(env_p, env_l) };
    let mut env: Vec<f64> = env_slice.to_vec();

    let P_slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(P_p, P_l) };
    let mut P: Vec<f64> = P_slice.to_vec();

    let E: f64 = energy(natm, nbas, nshells, &mut atm, &mut bas, &mut env, &mut P);
    return E;
}

#[no_mangle]
pub fn grad_C(
    natm: usize,
    nbas: usize,
    nshells: usize,
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
    P_p: *mut f64,
    P_l: usize,
) -> *mut f64 {
    // Convert the pointer to a slice, and then to a Vec
    let atm_slice: &mut [i32] = unsafe { std::slice::from_raw_parts_mut(atm_p, atm_l) };
    let mut atm: Vec<i32> = atm_slice.to_vec();

    let bas_slice: &mut [i32] = unsafe { std::slice::from_raw_parts_mut(bas_p, bas_l) };
    let mut bas: Vec<i32> = bas_slice.to_vec();

    let env_slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(env_p, env_l) };
    let mut env: Vec<f64> = env_slice.to_vec();

    let P_slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(P_p, P_l) };
    let mut P: Vec<f64> = P_slice.to_vec();

    let mut denv: Vec<f64> = vec![0.0; 1000];

    let _ = denergy(natm, nbas, nshells, &mut atm, &mut bas, &mut env, &mut denv, &mut P, 1.0);

    let denv_ptr = denv.as_mut_ptr();
    std::mem::forget(denv); // Prevent Rust from freeing the memory
    return denv_ptr;
}