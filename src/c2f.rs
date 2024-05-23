#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn CINTlen_spinor(bas_id: libc::c_int, bas: *const libc::c_int) -> libc::c_int;
    fn CINTcgto_cart(bas_id: libc::c_int, bas: *const libc::c_int) -> libc::c_int;
    fn CINTcgto_spheric(bas_id: libc::c_int, bas: *const libc::c_int) -> libc::c_int;
    fn CINTcgto_spinor(bas_id: libc::c_int, bas: *const libc::c_int) -> libc::c_int;
    fn CINTtot_pgto_spheric(bas: *const libc::c_int, nbas: libc::c_int) -> libc::c_int;
    fn CINTtot_pgto_spinor(bas: *const libc::c_int, nbas: libc::c_int) -> libc::c_int;
    fn CINTtot_cgto_cart(bas: *const libc::c_int, nbas: libc::c_int) -> libc::c_int;
    fn CINTtot_cgto_spheric(bas: *const libc::c_int, nbas: libc::c_int) -> libc::c_int;
    fn CINTtot_cgto_spinor(bas: *const libc::c_int, nbas: libc::c_int) -> libc::c_int;
    fn CINTshells_cart_offset(
        ao_loc: *mut libc::c_int,
        bas: *const libc::c_int,
        nbas: libc::c_int,
    );
    fn CINTshells_spheric_offset(
        ao_loc: *mut libc::c_int,
        bas: *const libc::c_int,
        nbas: libc::c_int,
    );
    fn CINTshells_spinor_offset(
        ao_loc: *mut libc::c_int,
        bas: *const libc::c_int,
        nbas: libc::c_int,
    );
    fn CINTgto_norm(n: libc::c_int, a: libc::c_double) -> libc::c_double;
    fn CINTinit_2e_optimizer(
        opt: *mut *mut CINTOpt,
        atm: *mut libc::c_int,
        natm: libc::c_int,
        bas: *mut libc::c_int,
        nbas: libc::c_int,
        env: *mut libc::c_double,
    );
    fn CINTdel_2e_optimizer(opt: *mut *mut CINTOpt);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PairData {
    pub rij: [libc::c_double; 3],
    pub eij: libc::c_double,
    pub cceij: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CINTOpt {
    pub index_xyz_array: *mut *mut libc::c_int,
    pub non0ctr: *mut *mut libc::c_int,
    pub sortedidx: *mut *mut libc::c_int,
    pub nbas: libc::c_int,
    pub log_max_coeff: *mut *mut libc::c_double,
    pub pairdata: *mut *mut PairData,
}
#[no_mangle]
pub unsafe extern "C" fn cintlen_spinor_(
    mut bas_id: *const libc::c_int,
    mut bas: *const libc::c_int,
) -> libc::c_int {
    return CINTlen_spinor(*bas_id, bas);
}
#[no_mangle]
pub unsafe extern "C" fn cintcgtos_cart_(
    mut bas_id: *const libc::c_int,
    mut bas: *const libc::c_int,
) -> libc::c_int {
    return CINTcgto_cart(*bas_id, bas);
}
#[no_mangle]
pub unsafe extern "C" fn cintcgto_cart_(
    mut bas_id: *const libc::c_int,
    mut bas: *const libc::c_int,
) -> libc::c_int {
    return CINTcgto_cart(*bas_id, bas);
}
#[no_mangle]
pub unsafe extern "C" fn cintcgtos_spheric_(
    mut bas_id: *const libc::c_int,
    mut bas: *const libc::c_int,
) -> libc::c_int {
    return CINTcgto_spheric(*bas_id, bas);
}
#[no_mangle]
pub unsafe extern "C" fn cintcgto_spheric_(
    mut bas_id: *const libc::c_int,
    mut bas: *const libc::c_int,
) -> libc::c_int {
    return CINTcgto_spheric(*bas_id, bas);
}
#[no_mangle]
pub unsafe extern "C" fn cintcgtos_spinor_(
    mut bas_id: *const libc::c_int,
    mut bas: *const libc::c_int,
) -> libc::c_int {
    return CINTcgto_spinor(*bas_id, bas);
}
#[no_mangle]
pub unsafe extern "C" fn cintcgto_spinor_(
    mut bas_id: *const libc::c_int,
    mut bas: *const libc::c_int,
) -> libc::c_int {
    return CINTcgto_spinor(*bas_id, bas);
}
#[no_mangle]
pub unsafe extern "C" fn cinttot_pgto_spheric_(
    mut bas: *const libc::c_int,
    mut nbas: *const libc::c_int,
) -> libc::c_int {
    return CINTtot_pgto_spheric(bas, *nbas);
}
#[no_mangle]
pub unsafe extern "C" fn cinttot_pgto_spinor_(
    mut bas: *const libc::c_int,
    mut nbas: *const libc::c_int,
) -> libc::c_int {
    return CINTtot_pgto_spinor(bas, *nbas);
}
#[no_mangle]
pub unsafe extern "C" fn cinttot_cgto_cart_(
    mut bas: *const libc::c_int,
    mut nbas: *const libc::c_int,
) -> libc::c_int {
    return CINTtot_cgto_cart(bas, *nbas);
}
#[no_mangle]
pub unsafe extern "C" fn cinttot_cgto_spheric_(
    mut bas: *const libc::c_int,
    mut nbas: *const libc::c_int,
) -> libc::c_int {
    return CINTtot_cgto_spheric(bas, *nbas);
}
#[no_mangle]
pub unsafe extern "C" fn cinttot_cgto_spinor_(
    mut bas: *const libc::c_int,
    mut nbas: *const libc::c_int,
) -> libc::c_int {
    return CINTtot_cgto_spinor(bas, *nbas);
}
#[no_mangle]
pub unsafe extern "C" fn cintshells_cart_offset_(
    mut ao_loc: *mut libc::c_int,
    mut bas: *const libc::c_int,
    mut nbas: *const libc::c_int,
) {
    CINTshells_cart_offset(ao_loc, bas, *nbas);
}
#[no_mangle]
pub unsafe extern "C" fn cintshells_spheric_offset_(
    mut ao_loc: *mut libc::c_int,
    mut bas: *const libc::c_int,
    mut nbas: *const libc::c_int,
) {
    CINTshells_spheric_offset(ao_loc, bas, *nbas);
}
#[no_mangle]
pub unsafe extern "C" fn cintshells_spinor_offset_(
    mut ao_loc: *mut libc::c_int,
    mut bas: *const libc::c_int,
    mut nbas: *const libc::c_int,
) {
    CINTshells_spinor_offset(ao_loc, bas, *nbas);
}
#[no_mangle]
pub unsafe extern "C" fn cintgto_norm_(
    mut n: *mut libc::c_int,
    mut a: *mut libc::c_double,
) -> libc::c_double {
    return CINTgto_norm(*n, *a);
}
#[no_mangle]
pub unsafe extern "C" fn cintinit_2e_optimizer_(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: *mut libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: *mut libc::c_int,
    mut env: *mut libc::c_double,
) {
    CINTinit_2e_optimizer(opt, atm, *natm, bas, *nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cintinit_optimizer_(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: *mut libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: *mut libc::c_int,
    mut env: *mut libc::c_double,
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
