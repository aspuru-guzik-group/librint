#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::cint_bas::CINTlen_spinor;
use crate::cint_bas::CINTcgto_cart;
use crate::cint_bas::CINTcgto_spheric;
use crate::cint_bas::CINTcgto_spinor;
use crate::cint_bas::CINTtot_pgto_spheric;
use crate::cint_bas::CINTtot_pgto_spinor;
use crate::cint_bas::CINTtot_cgto_cart;
use crate::cint_bas::CINTtot_cgto_spheric;
use crate::cint_bas::CINTtot_cgto_spinor;
use crate::cint_bas::CINTshells_cart_offset;
use crate::cint_bas::CINTshells_spheric_offset;
use crate::cint_bas::CINTshells_spinor_offset;
use crate::misc::CINTgto_norm;
use crate::optimizer::CINTinit_2e_optimizer;
use crate::optimizer::CINTdel_2e_optimizer;

use crate::cint::CINTOpt;

#[no_mangle]
pub unsafe extern "C" fn cintlen_spinor_(
    mut bas_id: *const i32,
    mut bas: *const i32,
) -> i32 {
    return CINTlen_spinor(*bas_id, bas);
}
#[no_mangle]
pub unsafe extern "C" fn cintcgtos_cart_(
    mut bas_id: &usize,
    mut bas: &[i32],
) -> i32 {
    return CINTcgto_cart(*bas_id, bas);
}
#[no_mangle]
pub unsafe extern "C" fn cintcgto_cart_(
    mut bas_id: &usize,
    mut bas: &[i32],
) -> i32 {
    return CINTcgto_cart(*bas_id, bas);
}
#[no_mangle]
pub unsafe extern "C" fn cintcgtos_spheric_(
    mut bas_id: *const i32,
    mut bas: *const i32,
) -> i32 {
    return CINTcgto_spheric(*bas_id, bas);
}
#[no_mangle]
pub unsafe extern "C" fn cintcgto_spheric_(
    mut bas_id: *const i32,
    mut bas: *const i32,
) -> i32 {
    return CINTcgto_spheric(*bas_id, bas);
}
#[no_mangle]
pub unsafe extern "C" fn cintcgtos_spinor_(
    mut bas_id: *const i32,
    mut bas: *const i32,
) -> i32 {
    return CINTcgto_spinor(*bas_id, bas);
}
#[no_mangle]
pub unsafe extern "C" fn cintcgto_spinor_(
    mut bas_id: *const i32,
    mut bas: *const i32,
) -> i32 {
    return CINTcgto_spinor(*bas_id, bas);
}
#[no_mangle]
pub unsafe extern "C" fn cinttot_pgto_spheric_(
    mut bas: *const i32,
    mut nbas: *const i32,
) -> i32 {
    return CINTtot_pgto_spheric(bas, *nbas);
}
#[no_mangle]
pub unsafe extern "C" fn cinttot_pgto_spinor_(
    mut bas: *const i32,
    mut nbas: *const i32,
) -> i32 {
    return CINTtot_pgto_spinor(bas, *nbas);
}
#[no_mangle]
pub unsafe extern "C" fn cinttot_cgto_cart_(
    mut bas: *const i32,
    mut nbas: *const i32,
) -> i32 {
    return CINTtot_cgto_cart(bas, *nbas);
}
#[no_mangle]
pub unsafe extern "C" fn cinttot_cgto_spheric_(
    mut bas: *const i32,
    mut nbas: *const i32,
) -> i32 {
    return CINTtot_cgto_spheric(bas, *nbas);
}
#[no_mangle]
pub unsafe extern "C" fn cinttot_cgto_spinor_(
    mut bas: *const i32,
    mut nbas: *const i32,
) -> i32 {
    return CINTtot_cgto_spinor(bas, *nbas);
}
#[no_mangle]
pub unsafe extern "C" fn cintshells_cart_offset_(
    mut ao_loc: *mut i32,
    mut bas: *const i32,
    mut nbas: *const i32,
) {
    CINTshells_cart_offset(ao_loc, bas, *nbas);
}
#[no_mangle]
pub unsafe extern "C" fn cintshells_spheric_offset_(
    mut ao_loc: *mut i32,
    mut bas: *const i32,
    mut nbas: *const i32,
) {
    CINTshells_spheric_offset(ao_loc, bas, *nbas);
}
#[no_mangle]
pub unsafe extern "C" fn cintshells_spinor_offset_(
    mut ao_loc: *mut i32,
    mut bas: *const i32,
    mut nbas: *const i32,
) {
    CINTshells_spinor_offset(ao_loc, bas, *nbas);
}
#[no_mangle]
pub unsafe extern "C" fn cintgto_norm_(
    mut n: *mut i32,
    mut a: *mut f64,
) -> f64 {
    return CINTgto_norm(*n, *a);
}
#[no_mangle]
pub unsafe extern "C" fn cintinit_2e_optimizer_(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut i32,
    mut natm: *mut i32,
    mut bas: *mut i32,
    mut nbas: *mut i32,
    mut env: *mut f64,
) {
    CINTinit_2e_optimizer(opt, atm, *natm, bas, *nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cintinit_optimizer_(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut i32,
    mut natm: *mut i32,
    mut bas: *mut i32,
    mut nbas: *mut i32,
    mut env: *mut f64,
) {
    cintinit_2e_optimizer_(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cintdel_2e_optimizer_(mut opt: *mut *mut CINTOpt) {
    CINTdel_2e_optimizer(opt);
}
#[no_mangle]
pub unsafe extern "C" fn cintdel_optimizer_(mut opt: *mut *mut CINTOpt) {
    cintdel_2e_optimizer_(opt);
}
