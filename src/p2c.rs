//! Expose Rust functions to C (Python), everything on top uses Rust types.
//!
//! SAFETY:
//! Every entry point takes the PySCF `Mole` arrays as (pointer, element count)
//! pairs: `atm`/`bas` are `i32` arrays whose lengths are multiples of
//! `ATM_SLOTS` (6) and `BAS_SLOTS` (8), and `env` is the `f64` parameter pool
//! they index into. The arrays are only read; they are copied into owned `Vec`s
//! on entry, so the caller may free or mutate them as soon as the call returns.
//!
//! Entry points returning `*mut f64` hand back a leaked allocation. **The
//! caller must release it with [`free_c`], passing the same element count that
//! call produced**, or the buffer leaks -- for `int2e_c` that is `nao^4`
//! doubles per call. `python/librint/utils.py:take` copies then calls `free_c`.
//!
//! Failures are returned as NULL or NaN, so Python can handle it.
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#![warn(unsafe_op_in_unsafe_fn)]

use crate::dscf::{dHcoreg, dRg, dS_uncontracted, dSg, danalyticalg, denergyfast, gradenergy};
use crate::scf::{density, energyfast, integral1e, integral2e, scf};

/// Copy the caller's `atm`/`bas`/`env` arrays into owned `Vec`s.
///
/// # Safety
///
/// Each pointer must be valid for reads of the matching element count and
/// properly aligned, as described in the module contract.
unsafe fn c2r_arr(
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
pub(crate) fn leak_vec(v: Vec<f64>) -> *mut f64 {
    let mut b = v.into_boxed_slice();
    let ptr = b.as_mut_ptr();
    std::mem::forget(b);
    ptr
}

/// Release a buffer returned by any of the `*_c` entry points. `len` must be
/// the element count that entry point produced.
///
/// # Safety
///
/// `ptr` must be null, or a pointer returned by one of the entry points in this
/// module and not yet freed, with `len` exactly the count that call produced.
/// Passing any other pointer, or freeing twice, is undefined behaviour.
#[no_mangle]
pub unsafe extern "C" fn free_c(ptr: *mut f64, len: usize) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(Vec::from_raw_parts(ptr, len, len));
    }
}

/// One-electron integral matrix, `nao * nao` doubles.
///
/// `coord` selects cartesian (0) or spherical (1); `typec` selects overlap (0),
/// kinetic (1) or nuclear attraction (2).
///
/// # Safety
///
/// See the module contract. The result must be released with [`free_c`].
#[no_mangle]
pub unsafe extern "C" fn int1e_c(
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
    coord: i32,
    typec: i32,
) -> *mut f64 {
    let (mut atm, mut bas, mut env) = unsafe { c2r_arr(atm_p, atm_l, bas_p, bas_l, env_p, env_l) };
    let R: Vec<f64> = integral1e(&mut atm, &mut bas, &mut env, coord, typec);

    leak_vec(R)
}

/// Full two-electron integral tensor, `nao^4` doubles.
///
/// # Safety
///
/// See the module contract. The result must be released with [`free_c`]; at
/// `nao^4` doubles this is the most expensive buffer to leak.
#[no_mangle]
pub unsafe extern "C" fn int2e_c(
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
    coord: i32,
) -> *mut f64 {
    let (mut atm, mut bas, mut env) = unsafe { c2r_arr(atm_p, atm_l, bas_p, bas_l, env_p, env_l) };

    let R: Vec<f64> = integral2e(&mut atm, &mut bas, &mut env, coord);

    leak_vec(R)
}

/// Uncontracted overlap derivative, `nao * nao * len(env2)` doubles.
///
/// # Safety
///
/// See the module contract. The result must be released with [`free_c`].
#[no_mangle]
pub unsafe extern "C" fn dS_u(
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
) -> *mut f64 {
    let (mut atm, mut bas, mut env) = unsafe { c2r_arr(atm_p, atm_l, bas_p, bas_l, env_p, env_l) };

    let dS = dS_uncontracted(&mut atm, &mut bas, &mut env);

    leak_vec(dS)
}

/// Converged RHF density matrix, `nao * nao` doubles, or NULL if the SCF did
/// not converge within `imax` cycles to `conv`.
///
/// # Safety
///
/// See the module contract. A non-null result must be released with [`free_c`].
#[no_mangle]
pub unsafe extern "C" fn density_c(
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
    let (mut atm, mut bas, mut env) = unsafe { c2r_arr(atm_p, atm_l, bas_p, bas_l, env_p, env_l) };

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

/// Total RHF energy for a given density matrix.
///
/// # Safety
///
/// See the module contract; `P_p` must additionally be valid for reads of
/// `P_l` doubles.
#[no_mangle]
pub unsafe extern "C" fn energy_c(
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
    P_p: *mut f64,
    P_l: usize,
) -> f64 {
    let (mut atm, mut bas, mut env) = unsafe { c2r_arr(atm_p, atm_l, bas_p, bas_l, env_p, env_l) };

    let P_slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(P_p, P_l) };
    let mut P: Vec<f64> = P_slice.to_vec();

    energyfast(&mut atm, &mut bas, &mut env, &mut P)
}

/// Run the SCF and return the converged energy, or NaN on failure.
///
/// # Safety
///
/// See the module contract.
#[no_mangle]
pub unsafe extern "C" fn scf_c(
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
    let (mut atm, mut bas, mut env) = unsafe { c2r_arr(atm_p, atm_l, bas_p, bas_l, env_p, env_l) };
    // NaN on failure (see density_c); python turns it into an exception.
    match scf(&mut atm, &mut bas, &mut env, nelec, imax, conv) {
        Ok(E) => E,
        Err(msg) => {
            eprintln!("librint: {}", msg);
            f64::NAN
        }
    }
}

/// Energy gradient with respect to the differentiable `env` parameters,
/// `len(env2)` doubles. See the `env` split convention in `utils::split`.
///
/// # Safety
///
/// See the module contract; `P_p` must additionally be valid for reads of
/// `P_l` doubles. The result must be released with [`free_c`].
#[no_mangle]
pub unsafe extern "C" fn grad_c(
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
    P_p: *mut f64,
    P_l: usize,
) -> *mut f64 {
    let (mut atm, mut bas, mut env) = unsafe { c2r_arr(atm_p, atm_l, bas_p, bas_l, env_p, env_l) };

    let P_slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(P_p, P_l) };
    let mut P: Vec<f64> = P_slice.to_vec();

    let denv: Vec<f64> = gradenergy(&mut atm, &mut bas, &mut env, &mut P);

    leak_vec(denv)
}

/// Overlap-matrix contribution to the basis-parameter gradient.
///
/// # Safety
///
/// See the module contract; `P_p` must additionally be valid for reads of
/// `P_l` doubles. The result must be released with [`free_c`].
#[no_mangle]
pub unsafe extern "C" fn dS_c(
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
    P_p: *mut f64,
    P_l: usize,
) -> *mut f64 {
    let (mut atm, mut bas, mut env) = unsafe { c2r_arr(atm_p, atm_l, bas_p, bas_l, env_p, env_l) };

    let P_slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(P_p, P_l) };
    let mut P: Vec<f64> = P_slice.to_vec();

    let dS = dSg(&mut atm, &mut bas, &mut env, &mut P);

    leak_vec(dS)
}

/// Core-Hamiltonian contribution to the basis-parameter gradient.
///
/// # Safety
///
/// See the module contract; `P_p` must additionally be valid for reads of
/// `P_l` doubles. The result must be released with [`free_c`].
#[no_mangle]
pub unsafe extern "C" fn dHcore_c(
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
    P_p: *mut f64,
    P_l: usize,
) -> *mut f64 {
    let (mut atm, mut bas, mut env) = unsafe { c2r_arr(atm_p, atm_l, bas_p, bas_l, env_p, env_l) };

    let P_slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(P_p, P_l) };
    let mut P: Vec<f64> = P_slice.to_vec();

    let dH = dHcoreg(&mut atm, &mut bas, &mut env, &mut P);

    leak_vec(dH)
}

/// Two-electron (Fock) contribution to the basis-parameter gradient.
///
/// # Safety
///
/// See the module contract; `P_p` must additionally be valid for reads of
/// `P_l` doubles. The result must be released with [`free_c`].
#[no_mangle]
pub unsafe extern "C" fn dR_c(
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
    P_p: *mut f64,
    P_l: usize,
) -> *mut f64 {
    let (mut atm, mut bas, mut env) = unsafe { c2r_arr(atm_p, atm_l, bas_p, bas_l, env_p, env_l) };

    let P_slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(P_p, P_l) };
    let mut P: Vec<f64> = P_slice.to_vec();

    let dR = dRg(&mut atm, &mut bas, &mut env, &mut P);

    leak_vec(dR)
}

/// Assembled analytic basis-parameter gradient, `dHcore + dR - 0.5 dS`.
///
/// # Safety
///
/// See the module contract; `P_p` must additionally be valid for reads of
/// `P_l` doubles. The result must be released with [`free_c`].
#[no_mangle]
pub unsafe extern "C" fn danalytical_c(
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
    P_p: *mut f64,
    P_l: usize,
) -> *mut f64 {
    let (mut atm, mut bas, mut env) = unsafe { c2r_arr(atm_p, atm_l, bas_p, bas_l, env_p, env_l) };

    let P_slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(P_p, P_l) };
    let mut P: Vec<f64> = P_slice.to_vec();

    let dR = danalyticalg(&mut atm, &mut bas, &mut env, &mut P);

    leak_vec(dR)
}

/// Kept for compatibility; delegates to the same assembled gradient as
/// [`danalytical_c`].
///
/// # Safety
///
/// See the module contract; `P_p` must additionally be valid for reads of
/// `P_l` doubles. The result must be released with [`free_c`].
#[no_mangle]
pub unsafe extern "C" fn denergy_c(
    atm_p: *mut i32,
    atm_l: usize,
    bas_p: *mut i32,
    bas_l: usize,
    env_p: *mut f64,
    env_l: usize,
    P_p: *mut f64,
    P_l: usize,
) -> *mut f64 {
    let (mut atm, mut bas, mut env) = unsafe { c2r_arr(atm_p, atm_l, bas_p, bas_l, env_p, env_l) };

    let P_slice: &mut [f64] = unsafe { std::slice::from_raw_parts_mut(P_p, P_l) };
    let mut P: Vec<f64> = P_slice.to_vec();

    let dR = denergyfast(&mut atm, &mut bas, &mut env, &mut P);

    leak_vec(dR)
}
