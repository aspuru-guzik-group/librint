#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#![feature(autodiff)]



use crate::dscf::{dHcoreg, dRg, dS_uncontracted, dSg, danalyticalg, denergyfast, gradenergy};
use crate::scf::{density, energyfast, integral1e, integral2e, scf};

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

    (atm, bas, env)
}

// Hand a Vec to the caller as a bare pointer. The allocation stays alive until
// free_c is called on it -- as a boxed slice, so capacity == len and free_c can
// reconstruct the Vec exactly. Callers that drop the pointer leak the whole
// buffer, which for int2e_c is nao^4 doubles per call.
fn leak_vec(v: Vec<f64>) -> *mut f64 {
    let mut b = v.into_boxed_slice();
    let ptr = b.as_mut_ptr();
    std::mem::forget(b);
    ptr
}

/// Release a buffer returned by any of the `*_c` entry points. `len` must be
/// the element count that entry point produced.
#[no_mangle]
pub extern "C" fn free_c(ptr: *mut f64, len: usize) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(Vec::from_raw_parts(ptr, len, len));
    }
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
    let R: Vec<f64> = integral1e(&mut atm, &mut bas, &mut env, coord, typec);

    leak_vec(R)
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

    let R: Vec<f64> = integral2e(&mut atm, &mut bas, &mut env, coord);

    leak_vec(R)
}

#[no_mangle]
pub extern "C" fn dS_u(
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
) -> *mut f64 {
    let (mut atm, mut bas, mut env) = c2r_arr(atm_p, atm_l, bas_p, bas_l, env_p, env_l);

    let dS = dS_uncontracted(&mut atm, &mut bas, &mut env);

    leak_vec(dS)
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

    // NULL on failure -- python raises. Never hand back a placeholder density:
    // downstream gradients of a bogus P look like ordinary numbers.
    let P: Vec<f64> = match density(&mut atm, &mut bas, &mut env, nelec, imax, conv) {
        Ok(P) => P,
        Err(msg) => {
            eprintln!("librint: {}", msg);
            return std::ptr::null_mut();
        }
    };

    leak_vec(P)
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
    E
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
    // NaN on failure (see density_c); python turns it into an exception.
    match scf(&mut atm, &mut bas, &mut env, nelec, imax, conv) {
        Ok(E) => E,
        Err(msg) => {
            eprintln!("librint: {}", msg);
            f64::NAN
        }
    }
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

    let denv: Vec<f64> = gradenergy(&mut atm, &mut bas, &mut env, &mut P);

    leak_vec(denv)
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

    let dS = dSg(&mut atm, &mut bas, &mut env, &mut P);

    leak_vec(dS)
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

    let dH = dHcoreg(&mut atm, &mut bas, &mut env, &mut P);

    leak_vec(dH)
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

    let dR = dRg(&mut atm, &mut bas, &mut env, &mut P);

    leak_vec(dR)
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

    let dR = danalyticalg(&mut atm, &mut bas, &mut env, &mut P);

    leak_vec(dR)
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

    let dR = denergyfast(&mut atm, &mut bas, &mut env, &mut P);

    leak_vec(dR)
}
