#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]

use crate::cint::PairData;
use crate::cint::CINTOpt;
use crate::cint::CINTEnvVars;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn CINTcart_comp(
        nx: *mut libc::c_int,
        ny: *mut libc::c_int,
        nz: *mut libc::c_int,
        lmax: libc::c_int,
    );
    fn CINTrys_roots(
        nroots: libc::c_int,
        x: libc::c_double,
        u: *mut libc::c_double,
        w: *mut libc::c_double,
    );
    fn CINTsr_rys_roots(
        nroots: libc::c_int,
        x: libc::c_double,
        lower: libc::c_double,
        u: *mut libc::c_double,
        w: *mut libc::c_double,
    );
    fn CINTcommon_fac_sp(l: libc::c_int) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct PairData {
//     pub rij: [libc::c_double; 3],
//     pub eij: libc::c_double,
//     pub cceij: libc::c_double,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct CINTOpt {
//     pub index_xyz_array: *mut *mut libc::c_int,
//     pub non0ctr: *mut *mut libc::c_int,
//     pub sortedidx: *mut *mut libc::c_int,
//     pub nbas: libc::c_int,
//     pub log_max_coeff: *mut *mut libc::c_double,
//     pub pairdata: *mut *mut PairData,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct CINTEnvVars {
//     pub atm: *mut libc::c_int,
//     pub bas: *mut libc::c_int,
//     pub env: *mut libc::c_double,
//     pub shls: *mut libc::c_int,
//     pub natm: libc::c_int,
//     pub nbas: libc::c_int,
//     pub i_l: libc::c_int,
//     pub j_l: libc::c_int,
//     pub k_l: libc::c_int,
//     pub l_l: libc::c_int,
//     pub nfi: libc::c_int,
//     pub nfj: libc::c_int,
//     pub c2rust_unnamed: C2RustUnnamed_1,
//     pub c2rust_unnamed_0: C2RustUnnamed_0,
//     pub nf: libc::c_int,
//     pub rys_order: libc::c_int,
//     pub x_ctr: [libc::c_int; 4],
//     pub gbits: libc::c_int,
//     pub ncomp_e1: libc::c_int,
//     pub ncomp_e2: libc::c_int,
//     pub ncomp_tensor: libc::c_int,
//     pub li_ceil: libc::c_int,
//     pub lj_ceil: libc::c_int,
//     pub lk_ceil: libc::c_int,
//     pub ll_ceil: libc::c_int,
//     pub g_stride_i: libc::c_int,
//     pub g_stride_k: libc::c_int,
//     pub g_stride_l: libc::c_int,
//     pub g_stride_j: libc::c_int,
//     pub nrys_roots: libc::c_int,
//     pub g_size: libc::c_int,
//     pub g2d_ijmax: libc::c_int,
//     pub g2d_klmax: libc::c_int,
//     pub common_factor: libc::c_double,
//     pub expcutoff: libc::c_double,
//     pub rirj: [libc::c_double; 3],
//     pub rkrl: [libc::c_double; 3],
//     pub rx_in_rijrx: *mut libc::c_double,
//     pub rx_in_rklrx: *mut libc::c_double,
//     pub ri: *mut libc::c_double,
//     pub rj: *mut libc::c_double,
//     pub rk: *mut libc::c_double,
//     pub c2rust_unnamed_1: C2RustUnnamed,
//     pub f_g0_2e: Option::<unsafe extern "C" fn() -> libc::c_int>,
//     pub f_g0_2d4d: Option::<unsafe extern "C" fn() -> ()>,
//     pub f_gout: Option::<unsafe extern "C" fn() -> ()>,
//     pub opt: *mut CINTOpt,
//     pub idx: *mut libc::c_int,
//     pub ai: [libc::c_double; 1],
//     pub aj: [libc::c_double; 1],
//     pub ak: [libc::c_double; 1],
//     pub al: [libc::c_double; 1],
//     pub fac: [libc::c_double; 1],
//     pub rij: [libc::c_double; 3],
//     pub rkl: [libc::c_double; 3],
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub union C2RustUnnamed {
//     pub rl: *mut libc::c_double,
//     pub grids: *mut libc::c_double,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub union C2RustUnnamed_0 {
//     pub nfl: libc::c_int,
//     pub ngrids: libc::c_int,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub union C2RustUnnamed_1 {
//     pub nfk: libc::c_int,
//     pub grids_offset: libc::c_int,
// }
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Rys2eT {
    pub c00x: [libc::c_double; 32],
    pub c00y: [libc::c_double; 32],
    pub c00z: [libc::c_double; 32],
    pub c0px: [libc::c_double; 32],
    pub c0py: [libc::c_double; 32],
    pub c0pz: [libc::c_double; 32],
    pub b01: [libc::c_double; 32],
    pub b00: [libc::c_double; 32],
    pub b10: [libc::c_double; 32],
}

#[no_mangle]
pub unsafe extern "C" fn CINTinit_int2e_EnvVars(
    mut envs: *mut CINTEnvVars,
    mut ng: *mut libc::c_int,
    mut shls: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
) {
    (*envs).natm = natm;
    (*envs).nbas = nbas;
    (*envs).atm = atm;
    (*envs).bas = bas;
    (*envs).env = env;
    (*envs).shls = shls;
    let i_sh: libc::c_int = *shls.offset(0 as libc::c_int as isize);
    let j_sh: libc::c_int = *shls.offset(1 as libc::c_int as isize);
    let k_sh: libc::c_int = *shls.offset(2 as libc::c_int as isize);
    let l_sh: libc::c_int = *shls.offset(3 as libc::c_int as isize);
    (*envs).i_l = *bas.offset((8 as libc::c_int * i_sh + 1 as libc::c_int) as isize);
    (*envs).j_l = *bas.offset((8 as libc::c_int * j_sh + 1 as libc::c_int) as isize);
    (*envs).k_l = *bas.offset((8 as libc::c_int * k_sh + 1 as libc::c_int) as isize);
    (*envs).l_l = *bas.offset((8 as libc::c_int * l_sh + 1 as libc::c_int) as isize);
    (*envs)
        .x_ctr[0 as libc::c_int
        as usize] = *bas.offset((8 as libc::c_int * i_sh + 3 as libc::c_int) as isize);
    (*envs)
        .x_ctr[1 as libc::c_int
        as usize] = *bas.offset((8 as libc::c_int * j_sh + 3 as libc::c_int) as isize);
    (*envs)
        .x_ctr[2 as libc::c_int
        as usize] = *bas.offset((8 as libc::c_int * k_sh + 3 as libc::c_int) as isize);
    (*envs)
        .x_ctr[3 as libc::c_int
        as usize] = *bas.offset((8 as libc::c_int * l_sh + 3 as libc::c_int) as isize);
    (*envs)
        .nfi = ((*envs).i_l + 1 as libc::c_int) * ((*envs).i_l + 2 as libc::c_int)
        / 2 as libc::c_int;
    (*envs)
        .nfj = ((*envs).j_l + 1 as libc::c_int) * ((*envs).j_l + 2 as libc::c_int)
        / 2 as libc::c_int;
    (*envs)
        .c2rust_unnamed
        .nfk = ((*envs).k_l + 1 as libc::c_int) * ((*envs).k_l + 2 as libc::c_int)
        / 2 as libc::c_int;
    (*envs)
        .c2rust_unnamed_0
        .nfl = ((*envs).l_l + 1 as libc::c_int) * ((*envs).l_l + 2 as libc::c_int)
        / 2 as libc::c_int;
    (*envs)
        .nf = (*envs).nfi * (*envs).c2rust_unnamed.nfk * (*envs).c2rust_unnamed_0.nfl
        * (*envs).nfj;
    (*envs)
        .ri = env
        .offset(
            *atm
                .offset(
                    (6 as libc::c_int
                        * *bas
                            .offset(
                                (8 as libc::c_int * i_sh + 0 as libc::c_int) as isize,
                            ) + 1 as libc::c_int) as isize,
                ) as isize,
        );
    (*envs)
        .rj = env
        .offset(
            *atm
                .offset(
                    (6 as libc::c_int
                        * *bas
                            .offset(
                                (8 as libc::c_int * j_sh + 0 as libc::c_int) as isize,
                            ) + 1 as libc::c_int) as isize,
                ) as isize,
        );
    (*envs)
        .rk = env
        .offset(
            *atm
                .offset(
                    (6 as libc::c_int
                        * *bas
                            .offset(
                                (8 as libc::c_int * k_sh + 0 as libc::c_int) as isize,
                            ) + 1 as libc::c_int) as isize,
                ) as isize,
        );
    (*envs)
        .c2rust_unnamed_1
        .rl = env
        .offset(
            *atm
                .offset(
                    (6 as libc::c_int
                        * *bas
                            .offset(
                                (8 as libc::c_int * l_sh + 0 as libc::c_int) as isize,
                            ) + 1 as libc::c_int) as isize,
                ) as isize,
        );
    (*envs)
        .common_factor = 3.14159265358979323846f64 * 3.14159265358979323846f64
        * 3.14159265358979323846f64 * 2 as libc::c_int as libc::c_double
        / 1.7724538509055160272981674833411451f64 * CINTcommon_fac_sp((*envs).i_l)
        * CINTcommon_fac_sp((*envs).j_l) * CINTcommon_fac_sp((*envs).k_l)
        * CINTcommon_fac_sp((*envs).l_l);
    if *env.offset(0 as libc::c_int as isize) == 0 as libc::c_int as libc::c_double {
        (*envs).expcutoff = 60 as libc::c_int as libc::c_double;
    } else {
        (*envs)
            .expcutoff = (if 40 as libc::c_int as libc::c_double
            > *env.offset(0 as libc::c_int as isize)
        {
            40 as libc::c_int as libc::c_double
        } else {
            *env.offset(0 as libc::c_int as isize)
        }) + 1 as libc::c_int as libc::c_double;
    }
    (*envs).gbits = *ng.offset(4 as libc::c_int as isize);
    (*envs).ncomp_e1 = *ng.offset(5 as libc::c_int as isize);
    (*envs).ncomp_e2 = *ng.offset(6 as libc::c_int as isize);
    (*envs).ncomp_tensor = *ng.offset(7 as libc::c_int as isize);
    (*envs).li_ceil = (*envs).i_l + *ng.offset(0 as libc::c_int as isize);
    (*envs).lj_ceil = (*envs).j_l + *ng.offset(1 as libc::c_int as isize);
    (*envs).lk_ceil = (*envs).k_l + *ng.offset(2 as libc::c_int as isize);
    (*envs).ll_ceil = (*envs).l_l + *ng.offset(3 as libc::c_int as isize);
    let mut rys_order: libc::c_int = ((*envs).li_ceil + (*envs).lj_ceil + (*envs).lk_ceil
        + (*envs).ll_ceil) / 2 as libc::c_int + 1 as libc::c_int;
    let mut nrys_roots: libc::c_int = rys_order;
    let mut omega: libc::c_double = *env.offset(8 as libc::c_int as isize);
    if omega < 0 as libc::c_int as libc::c_double && rys_order <= 3 as libc::c_int {
        nrys_roots *= 2 as libc::c_int;
    }
    (*envs).rys_order = rys_order;
    (*envs).nrys_roots = nrys_roots;
    let mut dli: libc::c_int = 0;
    let mut dlj: libc::c_int = 0;
    let mut dlk: libc::c_int = 0;
    let mut dll: libc::c_int = 0;
    let mut ibase: libc::c_int = ((*envs).li_ceil > (*envs).lj_ceil) as libc::c_int;
    let mut kbase: libc::c_int = ((*envs).lk_ceil > (*envs).ll_ceil) as libc::c_int;
    if kbase != 0 {
        dlk = (*envs).lk_ceil + (*envs).ll_ceil + 1 as libc::c_int;
        dll = (*envs).ll_ceil + 1 as libc::c_int;
    } else {
        dlk = (*envs).lk_ceil + 1 as libc::c_int;
        dll = (*envs).lk_ceil + (*envs).ll_ceil + 1 as libc::c_int;
    }
    if ibase != 0 {
        dli = (*envs).li_ceil + (*envs).lj_ceil + 1 as libc::c_int;
        dlj = (*envs).lj_ceil + 1 as libc::c_int;
    } else {
        dli = (*envs).li_ceil + 1 as libc::c_int;
        dlj = (*envs).li_ceil + (*envs).lj_ceil + 1 as libc::c_int;
    }
    (*envs).g_stride_i = nrys_roots;
    (*envs).g_stride_k = nrys_roots * dli;
    (*envs).g_stride_l = nrys_roots * dli * dlk;
    (*envs).g_stride_j = nrys_roots * dli * dlk * dll;
    (*envs).g_size = nrys_roots * dli * dlk * dll * dlj;
    if kbase != 0 {
        (*envs).g2d_klmax = (*envs).g_stride_k;
        (*envs).rx_in_rklrx = (*envs).rk;
        (*envs)
            .rkrl[0 as libc::c_int
            as usize] = *((*envs).rk).offset(0 as libc::c_int as isize)
            - *((*envs).c2rust_unnamed_1.rl).offset(0 as libc::c_int as isize);
        (*envs)
            .rkrl[1 as libc::c_int
            as usize] = *((*envs).rk).offset(1 as libc::c_int as isize)
            - *((*envs).c2rust_unnamed_1.rl).offset(1 as libc::c_int as isize);
        (*envs)
            .rkrl[2 as libc::c_int
            as usize] = *((*envs).rk).offset(2 as libc::c_int as isize)
            - *((*envs).c2rust_unnamed_1.rl).offset(2 as libc::c_int as isize);
    } else {
        (*envs).g2d_klmax = (*envs).g_stride_l;
        (*envs).rx_in_rklrx = (*envs).c2rust_unnamed_1.rl;
        (*envs)
            .rkrl[0 as libc::c_int
            as usize] = *((*envs).c2rust_unnamed_1.rl).offset(0 as libc::c_int as isize)
            - *((*envs).rk).offset(0 as libc::c_int as isize);
        (*envs)
            .rkrl[1 as libc::c_int
            as usize] = *((*envs).c2rust_unnamed_1.rl).offset(1 as libc::c_int as isize)
            - *((*envs).rk).offset(1 as libc::c_int as isize);
        (*envs)
            .rkrl[2 as libc::c_int
            as usize] = *((*envs).c2rust_unnamed_1.rl).offset(2 as libc::c_int as isize)
            - *((*envs).rk).offset(2 as libc::c_int as isize);
    }
    if ibase != 0 {
        (*envs).g2d_ijmax = (*envs).g_stride_i;
        (*envs).rx_in_rijrx = (*envs).ri;
        (*envs)
            .rirj[0 as libc::c_int
            as usize] = *((*envs).ri).offset(0 as libc::c_int as isize)
            - *((*envs).rj).offset(0 as libc::c_int as isize);
        (*envs)
            .rirj[1 as libc::c_int
            as usize] = *((*envs).ri).offset(1 as libc::c_int as isize)
            - *((*envs).rj).offset(1 as libc::c_int as isize);
        (*envs)
            .rirj[2 as libc::c_int
            as usize] = *((*envs).ri).offset(2 as libc::c_int as isize)
            - *((*envs).rj).offset(2 as libc::c_int as isize);
    } else {
        (*envs).g2d_ijmax = (*envs).g_stride_j;
        (*envs).rx_in_rijrx = (*envs).rj;
        (*envs)
            .rirj[0 as libc::c_int
            as usize] = *((*envs).rj).offset(0 as libc::c_int as isize)
            - *((*envs).ri).offset(0 as libc::c_int as isize);
        (*envs)
            .rirj[1 as libc::c_int
            as usize] = *((*envs).rj).offset(1 as libc::c_int as isize)
            - *((*envs).ri).offset(1 as libc::c_int as isize);
        (*envs)
            .rirj[2 as libc::c_int
            as usize] = *((*envs).rj).offset(2 as libc::c_int as isize)
            - *((*envs).ri).offset(2 as libc::c_int as isize);
    }
    if rys_order <= 2 as libc::c_int {
        (*envs)
            .f_g0_2d4d = ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut Rys2eT,
                    *mut CINTEnvVars,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg0_2e_2d4d_unrolled
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut Rys2eT,
                        *mut CINTEnvVars,
                    ) -> (),
            ),
        );
        if rys_order != nrys_roots {
            (*envs)
                .f_g0_2d4d = ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut Rys2eT,
                        *mut CINTEnvVars,
                    ) -> (),
                >,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    CINTsrg0_2e_2d4d_unrolled
                        as unsafe extern "C" fn(
                            *mut libc::c_double,
                            *mut Rys2eT,
                            *mut CINTEnvVars,
                        ) -> (),
                ),
            );
        }
    } else if kbase != 0 {
        if ibase != 0 {
            (*envs)
                .f_g0_2d4d = ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut Rys2eT,
                        *mut CINTEnvVars,
                    ) -> (),
                >,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    CINTg0_2e_ik2d4d
                        as unsafe extern "C" fn(
                            *mut libc::c_double,
                            *mut Rys2eT,
                            *mut CINTEnvVars,
                        ) -> (),
                ),
            );
        } else {
            (*envs)
                .f_g0_2d4d = ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut Rys2eT,
                        *mut CINTEnvVars,
                    ) -> (),
                >,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    CINTg0_2e_kj2d4d
                        as unsafe extern "C" fn(
                            *mut libc::c_double,
                            *mut Rys2eT,
                            *mut CINTEnvVars,
                        ) -> (),
                ),
            );
        }
    } else if ibase != 0 {
        (*envs)
            .f_g0_2d4d = ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut Rys2eT,
                    *mut CINTEnvVars,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg0_2e_il2d4d
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut Rys2eT,
                        *mut CINTEnvVars,
                    ) -> (),
            ),
        );
    } else {
        (*envs)
            .f_g0_2d4d = ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut Rys2eT,
                    *mut CINTEnvVars,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg0_2e_lj2d4d
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut Rys2eT,
                        *mut CINTEnvVars,
                    ) -> (),
            ),
        );
    }
    (*envs)
        .f_g0_2e = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut libc::c_double,
                *mut libc::c_double,
                *mut libc::c_double,
                libc::c_double,
                *mut CINTEnvVars,
            ) -> libc::c_int,
        >,
        Option::<unsafe extern "C" fn() -> libc::c_int>,
    >(
        Some(
            CINTg0_2e
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_double,
                    libc::c_double,
                    *mut CINTEnvVars,
                ) -> libc::c_int,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTg2e_index_xyz(
    mut idx: *mut libc::c_int,
    mut envs: *const CINTEnvVars,
) {
    let i_l: libc::c_int = (*envs).i_l;
    let j_l: libc::c_int = (*envs).j_l;
    let k_l: libc::c_int = (*envs).k_l;
    let l_l: libc::c_int = (*envs).l_l;
    let nfi: libc::c_int = (*envs).nfi;
    let nfj: libc::c_int = (*envs).nfj;
    let nfk: libc::c_int = (*envs).c2rust_unnamed.nfk;
    let nfl: libc::c_int = (*envs).c2rust_unnamed_0.nfl;
    let di: libc::c_int = (*envs).g_stride_i;
    let dk: libc::c_int = (*envs).g_stride_k;
    let dl: libc::c_int = (*envs).g_stride_l;
    let dj: libc::c_int = (*envs).g_stride_j;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut ofx: libc::c_int = 0;
    let mut ofkx: libc::c_int = 0;
    let mut oflx: libc::c_int = 0;
    let mut ofy: libc::c_int = 0;
    let mut ofky: libc::c_int = 0;
    let mut ofly: libc::c_int = 0;
    let mut ofz: libc::c_int = 0;
    let mut ofkz: libc::c_int = 0;
    let mut oflz: libc::c_int = 0;
    let mut i_nx: [libc::c_int; 136] = [0; 136];
    let mut i_ny: [libc::c_int; 136] = [0; 136];
    let mut i_nz: [libc::c_int; 136] = [0; 136];
    let mut j_nx: [libc::c_int; 136] = [0; 136];
    let mut j_ny: [libc::c_int; 136] = [0; 136];
    let mut j_nz: [libc::c_int; 136] = [0; 136];
    let mut k_nx: [libc::c_int; 136] = [0; 136];
    let mut k_ny: [libc::c_int; 136] = [0; 136];
    let mut k_nz: [libc::c_int; 136] = [0; 136];
    let mut l_nx: [libc::c_int; 136] = [0; 136];
    let mut l_ny: [libc::c_int; 136] = [0; 136];
    let mut l_nz: [libc::c_int; 136] = [0; 136];
    CINTcart_comp(i_nx.as_mut_ptr(), i_ny.as_mut_ptr(), i_nz.as_mut_ptr(), i_l);
    CINTcart_comp(j_nx.as_mut_ptr(), j_ny.as_mut_ptr(), j_nz.as_mut_ptr(), j_l);
    CINTcart_comp(k_nx.as_mut_ptr(), k_ny.as_mut_ptr(), k_nz.as_mut_ptr(), k_l);
    CINTcart_comp(l_nx.as_mut_ptr(), l_ny.as_mut_ptr(), l_nz.as_mut_ptr(), l_l);
    ofx = 0 as libc::c_int;
    ofy = (*envs).g_size;
    ofz = (*envs).g_size * 2 as libc::c_int;
    n = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while j < nfj {
        l = 0 as libc::c_int;
        while l < nfl {
            oflx = ofx + dj * j_nx[j as usize] + dl * l_nx[l as usize];
            ofly = ofy + dj * j_ny[j as usize] + dl * l_ny[l as usize];
            oflz = ofz + dj * j_nz[j as usize] + dl * l_nz[l as usize];
            k = 0 as libc::c_int;
            while k < nfk {
                ofkx = oflx + dk * k_nx[k as usize];
                ofky = ofly + dk * k_ny[k as usize];
                ofkz = oflz + dk * k_nz[k as usize];
                match i_l {
                    0 => {
                        *idx.offset((n + 0 as libc::c_int) as isize) = ofkx;
                        *idx.offset((n + 1 as libc::c_int) as isize) = ofky;
                        *idx.offset((n + 2 as libc::c_int) as isize) = ofkz;
                        n += 3 as libc::c_int;
                    }
                    1 => {
                        *idx.offset((n + 0 as libc::c_int) as isize) = ofkx + di;
                        *idx.offset((n + 1 as libc::c_int) as isize) = ofky;
                        *idx.offset((n + 2 as libc::c_int) as isize) = ofkz;
                        *idx.offset((n + 3 as libc::c_int) as isize) = ofkx;
                        *idx.offset((n + 4 as libc::c_int) as isize) = ofky + di;
                        *idx.offset((n + 5 as libc::c_int) as isize) = ofkz;
                        *idx.offset((n + 6 as libc::c_int) as isize) = ofkx;
                        *idx.offset((n + 7 as libc::c_int) as isize) = ofky;
                        *idx.offset((n + 8 as libc::c_int) as isize) = ofkz + di;
                        n += 9 as libc::c_int;
                    }
                    2 => {
                        *idx
                            .offset(
                                (n + 0 as libc::c_int) as isize,
                            ) = ofkx + di * 2 as libc::c_int;
                        *idx.offset((n + 1 as libc::c_int) as isize) = ofky;
                        *idx.offset((n + 2 as libc::c_int) as isize) = ofkz;
                        *idx.offset((n + 3 as libc::c_int) as isize) = ofkx + di;
                        *idx.offset((n + 4 as libc::c_int) as isize) = ofky + di;
                        *idx.offset((n + 5 as libc::c_int) as isize) = ofkz;
                        *idx.offset((n + 6 as libc::c_int) as isize) = ofkx + di;
                        *idx.offset((n + 7 as libc::c_int) as isize) = ofky;
                        *idx.offset((n + 8 as libc::c_int) as isize) = ofkz + di;
                        *idx.offset((n + 9 as libc::c_int) as isize) = ofkx;
                        *idx
                            .offset(
                                (n + 10 as libc::c_int) as isize,
                            ) = ofky + di * 2 as libc::c_int;
                        *idx.offset((n + 11 as libc::c_int) as isize) = ofkz;
                        *idx.offset((n + 12 as libc::c_int) as isize) = ofkx;
                        *idx.offset((n + 13 as libc::c_int) as isize) = ofky + di;
                        *idx.offset((n + 14 as libc::c_int) as isize) = ofkz + di;
                        *idx.offset((n + 15 as libc::c_int) as isize) = ofkx;
                        *idx.offset((n + 16 as libc::c_int) as isize) = ofky;
                        *idx
                            .offset(
                                (n + 17 as libc::c_int) as isize,
                            ) = ofkz + di * 2 as libc::c_int;
                        n += 18 as libc::c_int;
                    }
                    _ => {
                        i = 0 as libc::c_int;
                        while i < nfi {
                            *idx
                                .offset(
                                    (n + 0 as libc::c_int) as isize,
                                ) = ofkx + di * i_nx[i as usize];
                            *idx
                                .offset(
                                    (n + 1 as libc::c_int) as isize,
                                ) = ofky + di * i_ny[i as usize];
                            *idx
                                .offset(
                                    (n + 2 as libc::c_int) as isize,
                                ) = ofkz + di * i_nz[i as usize];
                            n += 3 as libc::c_int;
                            i += 1;
                            i;
                        }
                    }
                }
                k += 1;
                k;
            }
            l += 1;
            l;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e_2d(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let nroots: libc::c_int = (*envs).nrys_roots;
    let nmax: libc::c_int = (*envs).li_ceil + (*envs).lj_ceil;
    let mmax: libc::c_int = (*envs).lk_ceil + (*envs).ll_ceil;
    let dm: libc::c_int = (*envs).g2d_klmax;
    let dn: libc::c_int = (*envs).g2d_ijmax;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut off: libc::c_int = 0;
    let mut gx: *mut libc::c_double = g;
    let mut gy: *mut libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *mut libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut p0x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p0y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p0z: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p1x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p1y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p1z: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut nb1: libc::c_double = 0.;
    let mut mb0: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < nroots {
        *gx.offset(i as isize) = 1 as libc::c_int as libc::c_double;
        *gy.offset(i as isize) = 1 as libc::c_int as libc::c_double;
        i += 1;
        i;
    }
    let mut s0x: libc::c_double = 0.;
    let mut s1x: libc::c_double = 0.;
    let mut s2x: libc::c_double = 0.;
    let mut t0x: libc::c_double = 0.;
    let mut t1x: libc::c_double = 0.;
    let mut s0y: libc::c_double = 0.;
    let mut s1y: libc::c_double = 0.;
    let mut s2y: libc::c_double = 0.;
    let mut t0y: libc::c_double = 0.;
    let mut t1y: libc::c_double = 0.;
    let mut s0z: libc::c_double = 0.;
    let mut s1z: libc::c_double = 0.;
    let mut s2z: libc::c_double = 0.;
    let mut t0z: libc::c_double = 0.;
    let mut t1z: libc::c_double = 0.;
    let mut c00x: libc::c_double = 0.;
    let mut c00y: libc::c_double = 0.;
    let mut c00z: libc::c_double = 0.;
    let mut c0px: libc::c_double = 0.;
    let mut c0py: libc::c_double = 0.;
    let mut c0pz: libc::c_double = 0.;
    let mut b10: libc::c_double = 0.;
    let mut b01: libc::c_double = 0.;
    let mut b00: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < nroots {
        c00x = (*bc).c00x[i as usize];
        c00y = (*bc).c00y[i as usize];
        c00z = (*bc).c00z[i as usize];
        c0px = (*bc).c0px[i as usize];
        c0py = (*bc).c0py[i as usize];
        c0pz = (*bc).c0pz[i as usize];
        b10 = (*bc).b10[i as usize];
        b01 = (*bc).b01[i as usize];
        b00 = (*bc).b00[i as usize];
        if nmax > 0 as libc::c_int {
            s0x = *gx.offset(i as isize);
            s0y = *gy.offset(i as isize);
            s0z = *gz.offset(i as isize);
            s1x = c00x * s0x;
            s1y = c00y * s0y;
            s1z = c00z * s0z;
            *gx.offset((i + dn) as isize) = s1x;
            *gy.offset((i + dn) as isize) = s1y;
            *gz.offset((i + dn) as isize) = s1z;
            n = 1 as libc::c_int;
            while n < nmax {
                s2x = c00x * s1x + n as libc::c_double * b10 * s0x;
                s2y = c00y * s1y + n as libc::c_double * b10 * s0y;
                s2z = c00z * s1z + n as libc::c_double * b10 * s0z;
                *gx.offset((i + (n + 1 as libc::c_int) * dn) as isize) = s2x;
                *gy.offset((i + (n + 1 as libc::c_int) * dn) as isize) = s2y;
                *gz.offset((i + (n + 1 as libc::c_int) * dn) as isize) = s2z;
                s0x = s1x;
                s0y = s1y;
                s0z = s1z;
                s1x = s2x;
                s1y = s2y;
                s1z = s2z;
                n += 1;
                n;
            }
        }
        if mmax > 0 as libc::c_int {
            s0x = *gx.offset(i as isize);
            s0y = *gy.offset(i as isize);
            s0z = *gz.offset(i as isize);
            s1x = c0px * s0x;
            s1y = c0py * s0y;
            s1z = c0pz * s0z;
            *gx.offset((i + dm) as isize) = s1x;
            *gy.offset((i + dm) as isize) = s1y;
            *gz.offset((i + dm) as isize) = s1z;
            m = 1 as libc::c_int;
            while m < mmax {
                s2x = c0px * s1x + m as libc::c_double * b01 * s0x;
                s2y = c0py * s1y + m as libc::c_double * b01 * s0y;
                s2z = c0pz * s1z + m as libc::c_double * b01 * s0z;
                *gx.offset((i + (m + 1 as libc::c_int) * dm) as isize) = s2x;
                *gy.offset((i + (m + 1 as libc::c_int) * dm) as isize) = s2y;
                *gz.offset((i + (m + 1 as libc::c_int) * dm) as isize) = s2z;
                s0x = s1x;
                s0y = s1y;
                s0z = s1z;
                s1x = s2x;
                s1y = s2y;
                s1z = s2z;
                m += 1;
                m;
            }
            if nmax > 0 as libc::c_int {
                s0x = *gx.offset((i + dn) as isize);
                s0y = *gy.offset((i + dn) as isize);
                s0z = *gz.offset((i + dn) as isize);
                s1x = c0px * s0x + b00 * *gx.offset(i as isize);
                s1y = c0py * s0y + b00 * *gy.offset(i as isize);
                s1z = c0pz * s0z + b00 * *gz.offset(i as isize);
                *gx.offset((i + dn + dm) as isize) = s1x;
                *gy.offset((i + dn + dm) as isize) = s1y;
                *gz.offset((i + dn + dm) as isize) = s1z;
                m = 1 as libc::c_int;
                while m < mmax {
                    s2x = c0px * s1x + m as libc::c_double * b01 * s0x
                        + b00 * *gx.offset((i + m * dm) as isize);
                    s2y = c0py * s1y + m as libc::c_double * b01 * s0y
                        + b00 * *gy.offset((i + m * dm) as isize);
                    s2z = c0pz * s1z + m as libc::c_double * b01 * s0z
                        + b00 * *gz.offset((i + m * dm) as isize);
                    *gx.offset((i + dn + (m + 1 as libc::c_int) * dm) as isize) = s2x;
                    *gy.offset((i + dn + (m + 1 as libc::c_int) * dm) as isize) = s2y;
                    *gz.offset((i + dn + (m + 1 as libc::c_int) * dm) as isize) = s2z;
                    s0x = s1x;
                    s0y = s1y;
                    s0z = s1z;
                    s1x = s2x;
                    s1y = s2y;
                    s1z = s2z;
                    m += 1;
                    m;
                }
            }
        }
        m = 1 as libc::c_int;
        while m <= mmax {
            off = m * dm;
            j = off + i;
            s0x = *gx.offset(j as isize);
            s0y = *gy.offset(j as isize);
            s0z = *gz.offset(j as isize);
            s1x = *gx.offset((j + dn) as isize);
            s1y = *gy.offset((j + dn) as isize);
            s1z = *gz.offset((j + dn) as isize);
            n = 1 as libc::c_int;
            while n < nmax {
                s2x = c00x * s1x + n as libc::c_double * b10 * s0x
                    + m as libc::c_double * b00 * *gx.offset((j + n * dn - dm) as isize);
                s2y = c00y * s1y + n as libc::c_double * b10 * s0y
                    + m as libc::c_double * b00 * *gy.offset((j + n * dn - dm) as isize);
                s2z = c00z * s1z + n as libc::c_double * b10 * s0z
                    + m as libc::c_double * b00 * *gz.offset((j + n * dn - dm) as isize);
                *gx.offset((j + (n + 1 as libc::c_int) * dn) as isize) = s2x;
                *gy.offset((j + (n + 1 as libc::c_int) * dn) as isize) = s2y;
                *gz.offset((j + (n + 1 as libc::c_int) * dn) as isize) = s2z;
                s0x = s1x;
                s0y = s1y;
                s0z = s1z;
                s1x = s2x;
                s1y = s2y;
                s1z = s2z;
                n += 1;
                n;
            }
            m += 1;
            m;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_lj2d_4d(
    mut g: *mut libc::c_double,
    mut envs: *mut CINTEnvVars,
) {
    let mut li: libc::c_int = (*envs).li_ceil;
    let mut lk: libc::c_int = (*envs).lk_ceil;
    if li == 0 as libc::c_int && lk == 0 as libc::c_int {
        return;
    }
    let mut nmax: libc::c_int = (*envs).li_ceil + (*envs).lj_ceil;
    let mut mmax: libc::c_int = (*envs).lk_ceil + (*envs).ll_ceil;
    let mut lj: libc::c_int = (*envs).lj_ceil;
    let mut nroots: libc::c_int = (*envs).nrys_roots;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut di: libc::c_int = (*envs).g_stride_i;
    let mut dk: libc::c_int = (*envs).g_stride_k;
    let mut dl: libc::c_int = (*envs).g_stride_l;
    let mut dj: libc::c_int = (*envs).g_stride_j;
    let mut rirj: *mut libc::c_double = ((*envs).rirj).as_mut_ptr();
    let mut rkrl: *mut libc::c_double = ((*envs).rkrl).as_mut_ptr();
    let mut gx: *mut libc::c_double = g;
    let mut gy: *mut libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *mut libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut p1x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p1y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p1z: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p2x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p2y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p2z: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rx: libc::c_double = 0.;
    let mut ry: libc::c_double = 0.;
    let mut rz: libc::c_double = 0.;
    rx = *rirj.offset(0 as libc::c_int as isize);
    ry = *rirj.offset(1 as libc::c_int as isize);
    rz = *rirj.offset(2 as libc::c_int as isize);
    p1x = gx.offset(-(di as isize));
    p1y = gy.offset(-(di as isize));
    p1z = gz.offset(-(di as isize));
    p2x = gx.offset(-(di as isize)).offset(dj as isize);
    p2y = gy.offset(-(di as isize)).offset(dj as isize);
    p2z = gz.offset(-(di as isize)).offset(dj as isize);
    i = 1 as libc::c_int;
    while i <= li {
        j = 0 as libc::c_int;
        while j <= nmax - i {
            l = 0 as libc::c_int;
            while l <= mmax {
                ptr = j * dj + l * dl + i * di;
                n = ptr;
                while n < ptr + nroots {
                    *gx
                        .offset(
                            n as isize,
                        ) = rx * *p1x.offset(n as isize) + *p2x.offset(n as isize);
                    *gy
                        .offset(
                            n as isize,
                        ) = ry * *p1y.offset(n as isize) + *p2y.offset(n as isize);
                    *gz
                        .offset(
                            n as isize,
                        ) = rz * *p1z.offset(n as isize) + *p2z.offset(n as isize);
                    n += 1;
                    n;
                }
                l += 1;
                l;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    rx = *rkrl.offset(0 as libc::c_int as isize);
    ry = *rkrl.offset(1 as libc::c_int as isize);
    rz = *rkrl.offset(2 as libc::c_int as isize);
    p1x = gx.offset(-(dk as isize));
    p1y = gy.offset(-(dk as isize));
    p1z = gz.offset(-(dk as isize));
    p2x = gx.offset(-(dk as isize)).offset(dl as isize);
    p2y = gy.offset(-(dk as isize)).offset(dl as isize);
    p2z = gz.offset(-(dk as isize)).offset(dl as isize);
    j = 0 as libc::c_int;
    while j <= lj {
        k = 1 as libc::c_int;
        while k <= lk {
            l = 0 as libc::c_int;
            while l <= mmax - k {
                ptr = j * dj + l * dl + k * dk;
                n = ptr;
                while n < ptr + dk {
                    *gx
                        .offset(
                            n as isize,
                        ) = rx * *p1x.offset(n as isize) + *p2x.offset(n as isize);
                    *gy
                        .offset(
                            n as isize,
                        ) = ry * *p1y.offset(n as isize) + *p2y.offset(n as isize);
                    *gz
                        .offset(
                            n as isize,
                        ) = rz * *p1z.offset(n as isize) + *p2z.offset(n as isize);
                    n += 1;
                    n;
                }
                l += 1;
                l;
            }
            k += 1;
            k;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_kj2d_4d(
    mut g: *mut libc::c_double,
    mut envs: *mut CINTEnvVars,
) {
    let mut li: libc::c_int = (*envs).li_ceil;
    let mut ll: libc::c_int = (*envs).ll_ceil;
    if li == 0 as libc::c_int && ll == 0 as libc::c_int {
        return;
    }
    let mut nmax: libc::c_int = (*envs).li_ceil + (*envs).lj_ceil;
    let mut mmax: libc::c_int = (*envs).lk_ceil + (*envs).ll_ceil;
    let mut lj: libc::c_int = (*envs).lj_ceil;
    let mut nroots: libc::c_int = (*envs).nrys_roots;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut di: libc::c_int = (*envs).g_stride_i;
    let mut dk: libc::c_int = (*envs).g_stride_k;
    let mut dl: libc::c_int = (*envs).g_stride_l;
    let mut dj: libc::c_int = (*envs).g_stride_j;
    let mut rirj: *mut libc::c_double = ((*envs).rirj).as_mut_ptr();
    let mut rkrl: *mut libc::c_double = ((*envs).rkrl).as_mut_ptr();
    let mut gx: *mut libc::c_double = g;
    let mut gy: *mut libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *mut libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut p1x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p1y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p1z: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p2x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p2y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p2z: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rx: libc::c_double = 0.;
    let mut ry: libc::c_double = 0.;
    let mut rz: libc::c_double = 0.;
    rx = *rirj.offset(0 as libc::c_int as isize);
    ry = *rirj.offset(1 as libc::c_int as isize);
    rz = *rirj.offset(2 as libc::c_int as isize);
    p1x = gx.offset(-(di as isize));
    p1y = gy.offset(-(di as isize));
    p1z = gz.offset(-(di as isize));
    p2x = gx.offset(-(di as isize)).offset(dj as isize);
    p2y = gy.offset(-(di as isize)).offset(dj as isize);
    p2z = gz.offset(-(di as isize)).offset(dj as isize);
    i = 1 as libc::c_int;
    while i <= li {
        j = 0 as libc::c_int;
        while j <= nmax - i {
            k = 0 as libc::c_int;
            while k <= mmax {
                ptr = j * dj + k * dk + i * di;
                n = ptr;
                while n < ptr + nroots {
                    *gx
                        .offset(
                            n as isize,
                        ) = rx * *p1x.offset(n as isize) + *p2x.offset(n as isize);
                    *gy
                        .offset(
                            n as isize,
                        ) = ry * *p1y.offset(n as isize) + *p2y.offset(n as isize);
                    *gz
                        .offset(
                            n as isize,
                        ) = rz * *p1z.offset(n as isize) + *p2z.offset(n as isize);
                    n += 1;
                    n;
                }
                k += 1;
                k;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    rx = *rkrl.offset(0 as libc::c_int as isize);
    ry = *rkrl.offset(1 as libc::c_int as isize);
    rz = *rkrl.offset(2 as libc::c_int as isize);
    p1x = gx.offset(-(dl as isize));
    p1y = gy.offset(-(dl as isize));
    p1z = gz.offset(-(dl as isize));
    p2x = gx.offset(-(dl as isize)).offset(dk as isize);
    p2y = gy.offset(-(dl as isize)).offset(dk as isize);
    p2z = gz.offset(-(dl as isize)).offset(dk as isize);
    j = 0 as libc::c_int;
    while j <= lj {
        l = 1 as libc::c_int;
        while l <= ll {
            k = 0 as libc::c_int;
            while k <= mmax - l {
                ptr = j * dj + l * dl + k * dk;
                n = ptr;
                while n < ptr + dk {
                    *gx
                        .offset(
                            n as isize,
                        ) = rx * *p1x.offset(n as isize) + *p2x.offset(n as isize);
                    *gy
                        .offset(
                            n as isize,
                        ) = ry * *p1y.offset(n as isize) + *p2y.offset(n as isize);
                    *gz
                        .offset(
                            n as isize,
                        ) = rz * *p1z.offset(n as isize) + *p2z.offset(n as isize);
                    n += 1;
                    n;
                }
                k += 1;
                k;
            }
            l += 1;
            l;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_il2d_4d(
    mut g: *mut libc::c_double,
    mut envs: *mut CINTEnvVars,
) {
    let mut lk: libc::c_int = (*envs).lk_ceil;
    let mut lj: libc::c_int = (*envs).lj_ceil;
    if lj == 0 as libc::c_int && lk == 0 as libc::c_int {
        return;
    }
    let mut nmax: libc::c_int = (*envs).li_ceil + (*envs).lj_ceil;
    let mut mmax: libc::c_int = (*envs).lk_ceil + (*envs).ll_ceil;
    let mut ll: libc::c_int = (*envs).ll_ceil;
    let mut nroots: libc::c_int = (*envs).nrys_roots;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut di: libc::c_int = (*envs).g_stride_i;
    let mut dk: libc::c_int = (*envs).g_stride_k;
    let mut dl: libc::c_int = (*envs).g_stride_l;
    let mut dj: libc::c_int = (*envs).g_stride_j;
    let mut rirj: *mut libc::c_double = ((*envs).rirj).as_mut_ptr();
    let mut rkrl: *mut libc::c_double = ((*envs).rkrl).as_mut_ptr();
    let mut gx: *mut libc::c_double = g;
    let mut gy: *mut libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *mut libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut p1x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p1y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p1z: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p2x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p2y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p2z: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rx: libc::c_double = 0.;
    let mut ry: libc::c_double = 0.;
    let mut rz: libc::c_double = 0.;
    rx = *rkrl.offset(0 as libc::c_int as isize);
    ry = *rkrl.offset(1 as libc::c_int as isize);
    rz = *rkrl.offset(2 as libc::c_int as isize);
    p1x = gx.offset(-(dk as isize));
    p1y = gy.offset(-(dk as isize));
    p1z = gz.offset(-(dk as isize));
    p2x = gx.offset(-(dk as isize)).offset(dl as isize);
    p2y = gy.offset(-(dk as isize)).offset(dl as isize);
    p2z = gz.offset(-(dk as isize)).offset(dl as isize);
    k = 1 as libc::c_int;
    while k <= lk {
        l = 0 as libc::c_int;
        while l <= mmax - k {
            i = 0 as libc::c_int;
            while i <= nmax {
                ptr = l * dl + k * dk + i * di;
                n = ptr;
                while n < ptr + nroots {
                    *gx
                        .offset(
                            n as isize,
                        ) = rx * *p1x.offset(n as isize) + *p2x.offset(n as isize);
                    *gy
                        .offset(
                            n as isize,
                        ) = ry * *p1y.offset(n as isize) + *p2y.offset(n as isize);
                    *gz
                        .offset(
                            n as isize,
                        ) = rz * *p1z.offset(n as isize) + *p2z.offset(n as isize);
                    n += 1;
                    n;
                }
                i += 1;
                i;
            }
            l += 1;
            l;
        }
        k += 1;
        k;
    }
    rx = *rirj.offset(0 as libc::c_int as isize);
    ry = *rirj.offset(1 as libc::c_int as isize);
    rz = *rirj.offset(2 as libc::c_int as isize);
    p1x = gx.offset(-(dj as isize));
    p1y = gy.offset(-(dj as isize));
    p1z = gz.offset(-(dj as isize));
    p2x = gx.offset(-(dj as isize)).offset(di as isize);
    p2y = gy.offset(-(dj as isize)).offset(di as isize);
    p2z = gz.offset(-(dj as isize)).offset(di as isize);
    j = 1 as libc::c_int;
    while j <= lj {
        l = 0 as libc::c_int;
        while l <= ll {
            k = 0 as libc::c_int;
            while k <= lk {
                ptr = j * dj + l * dl + k * dk;
                n = ptr;
                while n < ptr + dk - di * j {
                    *gx
                        .offset(
                            n as isize,
                        ) = rx * *p1x.offset(n as isize) + *p2x.offset(n as isize);
                    *gy
                        .offset(
                            n as isize,
                        ) = ry * *p1y.offset(n as isize) + *p2y.offset(n as isize);
                    *gz
                        .offset(
                            n as isize,
                        ) = rz * *p1z.offset(n as isize) + *p2z.offset(n as isize);
                    n += 1;
                    n;
                }
                k += 1;
                k;
            }
            l += 1;
            l;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_ik2d_4d(
    mut g: *mut libc::c_double,
    mut envs: *mut CINTEnvVars,
) {
    let mut lj: libc::c_int = (*envs).lj_ceil;
    let mut ll: libc::c_int = (*envs).ll_ceil;
    if lj == 0 as libc::c_int && ll == 0 as libc::c_int {
        return;
    }
    let mut nmax: libc::c_int = (*envs).li_ceil + (*envs).lj_ceil;
    let mut mmax: libc::c_int = (*envs).lk_ceil + (*envs).ll_ceil;
    let mut lk: libc::c_int = (*envs).lk_ceil;
    let mut nroots: libc::c_int = (*envs).nrys_roots;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut di: libc::c_int = (*envs).g_stride_i;
    let mut dk: libc::c_int = (*envs).g_stride_k;
    let mut dl: libc::c_int = (*envs).g_stride_l;
    let mut dj: libc::c_int = (*envs).g_stride_j;
    let mut rirj: *mut libc::c_double = ((*envs).rirj).as_mut_ptr();
    let mut rkrl: *mut libc::c_double = ((*envs).rkrl).as_mut_ptr();
    let mut gx: *mut libc::c_double = g;
    let mut gy: *mut libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *mut libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut p1x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p1y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p1z: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p2x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p2y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p2z: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rx: libc::c_double = 0.;
    let mut ry: libc::c_double = 0.;
    let mut rz: libc::c_double = 0.;
    rx = *rkrl.offset(0 as libc::c_int as isize);
    ry = *rkrl.offset(1 as libc::c_int as isize);
    rz = *rkrl.offset(2 as libc::c_int as isize);
    p1x = gx.offset(-(dl as isize));
    p1y = gy.offset(-(dl as isize));
    p1z = gz.offset(-(dl as isize));
    p2x = gx.offset(-(dl as isize)).offset(dk as isize);
    p2y = gy.offset(-(dl as isize)).offset(dk as isize);
    p2z = gz.offset(-(dl as isize)).offset(dk as isize);
    l = 1 as libc::c_int;
    while l <= ll {
        k = 0 as libc::c_int;
        while k <= mmax - l {
            i = 0 as libc::c_int;
            while i <= nmax {
                ptr = l * dl + k * dk + i * di;
                n = ptr;
                while n < ptr + nroots {
                    *gx
                        .offset(
                            n as isize,
                        ) = rx * *p1x.offset(n as isize) + *p2x.offset(n as isize);
                    *gy
                        .offset(
                            n as isize,
                        ) = ry * *p1y.offset(n as isize) + *p2y.offset(n as isize);
                    *gz
                        .offset(
                            n as isize,
                        ) = rz * *p1z.offset(n as isize) + *p2z.offset(n as isize);
                    n += 1;
                    n;
                }
                i += 1;
                i;
            }
            k += 1;
            k;
        }
        l += 1;
        l;
    }
    rx = *rirj.offset(0 as libc::c_int as isize);
    ry = *rirj.offset(1 as libc::c_int as isize);
    rz = *rirj.offset(2 as libc::c_int as isize);
    p1x = gx.offset(-(dj as isize));
    p1y = gy.offset(-(dj as isize));
    p1z = gz.offset(-(dj as isize));
    p2x = gx.offset(-(dj as isize)).offset(di as isize);
    p2y = gy.offset(-(dj as isize)).offset(di as isize);
    p2z = gz.offset(-(dj as isize)).offset(di as isize);
    j = 1 as libc::c_int;
    while j <= lj {
        l = 0 as libc::c_int;
        while l <= ll {
            k = 0 as libc::c_int;
            while k <= lk {
                ptr = j * dj + l * dl + k * dk;
                n = ptr;
                while n < ptr + dk - di * j {
                    *gx
                        .offset(
                            n as isize,
                        ) = rx * *p1x.offset(n as isize) + *p2x.offset(n as isize);
                    *gy
                        .offset(
                            n as isize,
                        ) = ry * *p1y.offset(n as isize) + *p2y.offset(n as isize);
                    *gz
                        .offset(
                            n as isize,
                        ) = rz * *p1z.offset(n as isize) + *p2z.offset(n as isize);
                    n += 1;
                    n;
                }
                k += 1;
                k;
            }
            l += 1;
            l;
        }
        j += 1;
        j;
    }
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0000(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0001(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g
        .offset(
            5 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(4 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0002(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            4 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *cpx.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            5 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *cpx.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(7 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(8 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(9 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *cpy.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *cpy.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(12 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(13 as libc::c_int as isize);
    *g
        .offset(
            16 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(14 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(12 as libc::c_int as isize);
    *g
        .offset(
            17 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(15 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(13 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0003(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            4 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *cpx.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            5 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *cpx.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            6 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (*g.offset(4 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b01.offset(0 as libc::c_int as isize));
    *g
        .offset(
            7 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (*g.offset(5 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b01.offset(1 as libc::c_int as isize));
    *g.offset(8 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(9 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(10 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(11 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *cpy.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *cpy.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (*g.offset(12 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b01.offset(0 as libc::c_int as isize));
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (*g.offset(13 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b01.offset(1 as libc::c_int as isize));
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(16 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(17 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(18 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(16 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(19 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(17 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(20 as libc::c_int as isize)
        + 2 as libc::c_int as libc::c_double * *b01.offset(0 as libc::c_int as isize)
            * *g.offset(18 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(21 as libc::c_int as isize)
        + 2 as libc::c_int as libc::c_double * *b01.offset(1 as libc::c_int as isize)
            * *g.offset(19 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0010(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g
        .offset(
            5 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(4 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0011(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    let mut xkxl: libc::c_double = (*envs).rkrl[0 as libc::c_int as usize];
    let mut ykyl: libc::c_double = (*envs).rkrl[1 as libc::c_int as usize];
    let mut zkzl: libc::c_double = (*envs).rkrl[2 as libc::c_int as usize];
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            6 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (xkxl + *cpx.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            7 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (xkxl + *cpx.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize);
    *g.offset(2 as libc::c_int as isize) = xkxl + *cpx.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = xkxl + *cpx.offset(1 as libc::c_int as isize);
    *g.offset(12 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(13 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(16 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(17 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (ykyl + *cpy.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (ykyl + *cpy.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(0 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *g.offset(28 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *g.offset(29 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = *g.offset(24 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize));
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = *g.offset(25 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize));
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0012(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    let mut xkxl: libc::c_double = (*envs).rkrl[0 as libc::c_int as usize];
    let mut ykyl: libc::c_double = (*envs).rkrl[1 as libc::c_int as usize];
    let mut zkzl: libc::c_double = (*envs).rkrl[2 as libc::c_int as usize];
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            8 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *cpx.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            9 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *cpx.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *g.offset(8 as libc::c_int as isize)
        * (xkxl + *cpx.offset(0 as libc::c_int as isize))
        + *cpx.offset(0 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *g.offset(9 as libc::c_int as isize)
        * (xkxl + *cpx.offset(1 as libc::c_int as isize))
        + *cpx.offset(1 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            6 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (xkxl + *cpx.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            7 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (xkxl + *cpx.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize);
    *g.offset(2 as libc::c_int as isize) = xkxl + *cpx.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = xkxl + *cpx.offset(1 as libc::c_int as isize);
    *g.offset(16 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(17 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(20 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(21 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            24 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *cpy.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            25 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *cpy.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = *g.offset(24 as libc::c_int as isize)
        * (ykyl + *cpy.offset(0 as libc::c_int as isize))
        + *cpy.offset(0 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = *g.offset(25 as libc::c_int as isize)
        * (ykyl + *cpy.offset(1 as libc::c_int as isize))
        + *cpy.offset(1 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (ykyl + *cpy.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (ykyl + *cpy.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(0 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            36 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            37 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            40 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(36 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            41 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(37 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            42 as libc::c_int as isize,
        ) = *g.offset(40 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize))
        + 2 as libc::c_int as libc::c_double * *b01.offset(0 as libc::c_int as isize)
            * *g.offset(36 as libc::c_int as isize);
    *g
        .offset(
            43 as libc::c_int as isize,
        ) = *g.offset(41 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize))
        + 2 as libc::c_int as libc::c_double * *b01.offset(1 as libc::c_int as isize)
            * *g.offset(37 as libc::c_int as isize);
    *g
        .offset(
            38 as libc::c_int as isize,
        ) = *g.offset(36 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            39 as libc::c_int as isize,
        ) = *g.offset(37 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            34 as libc::c_int as isize,
        ) = *g.offset(32 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize));
    *g
        .offset(
            35 as libc::c_int as isize,
        ) = *g.offset(33 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize));
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0020(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            4 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *cpx.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            5 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *cpx.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(7 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(8 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(9 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *cpy.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *cpy.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(12 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(13 as libc::c_int as isize);
    *g
        .offset(
            16 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(14 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(12 as libc::c_int as isize);
    *g
        .offset(
            17 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(15 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(13 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0021(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    let mut xkxl: libc::c_double = (*envs).rkrl[0 as libc::c_int as usize];
    let mut ykyl: libc::c_double = (*envs).rkrl[1 as libc::c_int as usize];
    let mut zkzl: libc::c_double = (*envs).rkrl[2 as libc::c_int as usize];
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            4 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *cpx.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            5 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *cpx.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g.offset(8 as libc::c_int as isize) = xkxl + *cpx.offset(0 as libc::c_int as isize);
    *g.offset(9 as libc::c_int as isize) = xkxl + *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (xkxl + *cpx.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (xkxl + *cpx.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *g.offset(4 as libc::c_int as isize)
        * (xkxl + *cpx.offset(0 as libc::c_int as isize))
        + *cpx.offset(0 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *g.offset(5 as libc::c_int as isize)
        * (xkxl + *cpx.offset(1 as libc::c_int as isize))
        + *cpx.offset(1 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b01.offset(1 as libc::c_int as isize);
    *g.offset(16 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(17 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(18 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(19 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *cpy.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *cpy.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            24 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(0 as libc::c_int as isize);
    *g
        .offset(
            25 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (ykyl + *cpy.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (ykyl + *cpy.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *g.offset(20 as libc::c_int as isize)
        * (ykyl + *cpy.offset(0 as libc::c_int as isize))
        + *cpy.offset(0 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *g.offset(21 as libc::c_int as isize)
        * (ykyl + *cpy.offset(1 as libc::c_int as isize))
        + *cpy.offset(1 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            34 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            35 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            36 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(34 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            37 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(35 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            40 as libc::c_int as isize,
        ) = *g.offset(32 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize));
    *g
        .offset(
            41 as libc::c_int as isize,
        ) = *g.offset(33 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize));
    *g
        .offset(
            42 as libc::c_int as isize,
        ) = *g.offset(34 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            43 as libc::c_int as isize,
        ) = *g.offset(35 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            44 as libc::c_int as isize,
        ) = *g.offset(36 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize))
        + 2 as libc::c_int as libc::c_double * *b01.offset(0 as libc::c_int as isize)
            * *g.offset(34 as libc::c_int as isize);
    *g
        .offset(
            45 as libc::c_int as isize,
        ) = *g.offset(37 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize))
        + 2 as libc::c_int as libc::c_double * *b01.offset(1 as libc::c_int as isize)
            * *g.offset(35 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0030(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            4 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *cpx.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            5 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *cpx.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            6 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (*g.offset(4 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b01.offset(0 as libc::c_int as isize));
    *g
        .offset(
            7 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (*g.offset(5 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b01.offset(1 as libc::c_int as isize));
    *g.offset(8 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(9 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(10 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(11 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *cpy.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *cpy.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (*g.offset(12 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b01.offset(0 as libc::c_int as isize));
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (*g.offset(13 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b01.offset(1 as libc::c_int as isize));
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(16 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(17 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(18 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(16 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(19 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(17 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(20 as libc::c_int as isize)
        + 2 as libc::c_int as libc::c_double * *b01.offset(0 as libc::c_int as isize)
            * *g.offset(18 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(21 as libc::c_int as isize)
        + 2 as libc::c_int as libc::c_double * *b01.offset(1 as libc::c_int as isize)
            * *g.offset(19 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0100(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g
        .offset(
            5 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(4 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0101(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(4 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g
        .offset(
            6 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            7 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g.offset(8 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(9 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(10 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(11 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(12 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(13 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(16 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(17 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(16 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(17 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(20 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(16 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(21 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(17 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0102(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g
        .offset(
            4 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *cpx.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            5 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *cpx.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            8 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            9 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (*g.offset(8 as libc::c_int as isize) + *b00.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize)
            * *c0x.offset(0 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (*g.offset(9 as libc::c_int as isize) + *b00.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize)
            * *c0x.offset(1 as libc::c_int as isize);
    *g.offset(12 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(13 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(14 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(15 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(18 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(19 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            16 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *cpy.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            17 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *cpy.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (*g.offset(20 as libc::c_int as isize)
            + *b00.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize)
            * *c0y.offset(0 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (*g.offset(21 as libc::c_int as isize)
            + *b00.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize)
            * *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(26 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(27 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            32 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(30 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            33 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(31 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            34 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(32 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(30 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(26 as libc::c_int as isize);
    *g
        .offset(
            35 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(33 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(31 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(27 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0110(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(4 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g
        .offset(
            6 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            7 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g.offset(8 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(9 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(10 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(11 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(12 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(13 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(16 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(17 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(16 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(17 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(20 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(16 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(21 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(17 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0111(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    let mut xkxl: libc::c_double = (*envs).rkrl[0 as libc::c_int as usize];
    let mut ykyl: libc::c_double = (*envs).rkrl[1 as libc::c_int as usize];
    let mut zkzl: libc::c_double = (*envs).rkrl[2 as libc::c_int as usize];
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(12 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(13 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(4 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            16 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            17 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            6 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (xkxl + *cpx.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            7 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (xkxl + *cpx.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = *g.offset(16 as libc::c_int as isize)
        * (xkxl + *cpx.offset(0 as libc::c_int as isize))
        + *cpx.offset(0 as libc::c_int as isize) * *b00.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize)
            * *c0x.offset(0 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = *g.offset(17 as libc::c_int as isize)
        * (xkxl + *cpx.offset(1 as libc::c_int as isize))
        + *cpx.offset(1 as libc::c_int as isize) * *b00.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize)
            * *c0x.offset(1 as libc::c_int as isize);
    *g.offset(2 as libc::c_int as isize) = xkxl + *cpx.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = xkxl + *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (xkxl + *cpx.offset(0 as libc::c_int as isize))
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (xkxl + *cpx.offset(1 as libc::c_int as isize))
        + *b00.offset(1 as libc::c_int as isize);
    *g.offset(24 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(25 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(36 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(37 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(28 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(29 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            40 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            41 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (ykyl + *cpy.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (ykyl + *cpy.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            42 as libc::c_int as isize,
        ) = *g.offset(40 as libc::c_int as isize)
        * (ykyl + *cpy.offset(0 as libc::c_int as isize))
        + *cpy.offset(0 as libc::c_int as isize) * *b00.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize)
            * *c0y.offset(0 as libc::c_int as isize);
    *g
        .offset(
            43 as libc::c_int as isize,
        ) = *g.offset(41 as libc::c_int as isize)
        * (ykyl + *cpy.offset(1 as libc::c_int as isize))
        + *cpy.offset(1 as libc::c_int as isize) * *b00.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize)
            * *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(0 as libc::c_int as isize);
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            38 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (ykyl + *cpy.offset(0 as libc::c_int as isize))
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            39 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (ykyl + *cpy.offset(1 as libc::c_int as isize))
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            60 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            61 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            52 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            53 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            64 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(60 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            65 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(61 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            54 as libc::c_int as isize,
        ) = *g.offset(52 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            55 as libc::c_int as isize,
        ) = *g.offset(53 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            66 as libc::c_int as isize,
        ) = *g.offset(64 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(60 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(52 as libc::c_int as isize);
    *g
        .offset(
            67 as libc::c_int as isize,
        ) = *g.offset(65 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(61 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(53 as libc::c_int as isize);
    *g
        .offset(
            50 as libc::c_int as isize,
        ) = *g.offset(48 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize));
    *g
        .offset(
            51 as libc::c_int as isize,
        ) = *g.offset(49 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize));
    *g
        .offset(
            62 as libc::c_int as isize,
        ) = *g.offset(60 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize))
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            63 as libc::c_int as isize,
        ) = *g.offset(61 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize))
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0120(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g
        .offset(
            4 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *cpx.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            5 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *cpx.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            8 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            9 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (*g.offset(8 as libc::c_int as isize) + *b00.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize)
            * *c0x.offset(0 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (*g.offset(9 as libc::c_int as isize) + *b00.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize)
            * *c0x.offset(1 as libc::c_int as isize);
    *g.offset(12 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(13 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(14 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(15 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(18 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(19 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            16 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *cpy.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            17 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *cpy.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (*g.offset(20 as libc::c_int as isize)
            + *b00.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize)
            * *c0y.offset(0 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (*g.offset(21 as libc::c_int as isize)
            + *b00.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize)
            * *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(26 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(27 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            32 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(30 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            33 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(31 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            34 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(32 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(30 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(26 as libc::c_int as isize);
    *g
        .offset(
            35 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(33 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(31 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(27 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0200(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g
        .offset(
            4 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            5 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(7 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(8 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(9 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(12 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(13 as libc::c_int as isize);
    *g
        .offset(
            16 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(14 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(12 as libc::c_int as isize);
    *g
        .offset(
            17 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(15 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(13 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0201(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g
        .offset(
            8 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            9 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g.offset(2 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            6 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            7 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (*g.offset(6 as libc::c_int as isize) + *b00.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize)
            * *cpx.offset(0 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (*g.offset(7 as libc::c_int as isize) + *b00.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize)
            * *cpx.offset(1 as libc::c_int as isize);
    *g.offset(12 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(13 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(16 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(17 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g.offset(14 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(15 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (*g.offset(18 as libc::c_int as isize)
            + *b00.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize)
            * *cpy.offset(0 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (*g.offset(19 as libc::c_int as isize)
            + *b00.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize)
            * *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            32 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(28 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            33 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(29 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(28 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(29 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            34 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(30 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(26 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(28 as libc::c_int as isize);
    *g
        .offset(
            35 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(31 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(27 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(29 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0210(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(4 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g
        .offset(
            6 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            7 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            8 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            9 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (*g.offset(6 as libc::c_int as isize) + *b00.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize)
            * *cpx.offset(0 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (*g.offset(7 as libc::c_int as isize) + *b00.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize)
            * *cpx.offset(1 as libc::c_int as isize);
    *g.offset(12 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(13 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(14 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(15 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(16 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(17 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (*g.offset(18 as libc::c_int as isize)
            + *b00.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize)
            * *cpy.offset(0 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (*g.offset(19 as libc::c_int as isize)
            + *b00.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize)
            * *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(28 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(29 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            32 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(28 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            33 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(29 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            34 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(30 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(26 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(28 as libc::c_int as isize);
    *g
        .offset(
            35 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(31 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(27 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(29 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0300(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g
        .offset(
            4 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            5 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            6 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (*g.offset(4 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b10.offset(0 as libc::c_int as isize));
    *g
        .offset(
            7 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (*g.offset(5 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b10.offset(1 as libc::c_int as isize));
    *g.offset(8 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(9 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(10 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(11 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (*g.offset(12 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b10.offset(0 as libc::c_int as isize));
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (*g.offset(13 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b10.offset(1 as libc::c_int as isize));
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(16 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(17 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(18 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(16 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(19 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(17 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(20 as libc::c_int as isize)
        + 2 as libc::c_int as libc::c_double * *b10.offset(0 as libc::c_int as isize)
            * *g.offset(18 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(21 as libc::c_int as isize)
        + 2 as libc::c_int as libc::c_double * *b10.offset(1 as libc::c_int as isize)
            * *g.offset(19 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1000(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g
        .offset(
            5 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(4 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1001(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(4 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            6 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            7 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g.offset(8 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(9 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(10 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(11 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(12 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(13 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(16 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(17 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(16 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(17 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(18 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(16 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(19 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(17 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1002(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(4 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            6 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            7 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            8 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *cpx.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            9 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *cpx.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (*g.offset(6 as libc::c_int as isize) + *b00.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize)
            * *c0x.offset(0 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (*g.offset(7 as libc::c_int as isize) + *b00.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize)
            * *c0x.offset(1 as libc::c_int as isize);
    *g.offset(12 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(13 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(14 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(15 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(16 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(17 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *cpy.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *cpy.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (*g.offset(18 as libc::c_int as isize)
            + *b00.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize)
            * *c0y.offset(0 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (*g.offset(19 as libc::c_int as isize)
            + *b00.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize)
            * *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(26 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(27 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            32 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(28 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            33 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(29 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            34 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(30 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(26 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(28 as libc::c_int as isize);
    *g
        .offset(
            35 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(31 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(27 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(29 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1010(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(4 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            6 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            7 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g.offset(8 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(9 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(10 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(11 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(12 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(13 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(16 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(17 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(16 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(17 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(18 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(16 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(19 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(17 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1011(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    let mut xkxl: libc::c_double = (*envs).rkrl[0 as libc::c_int as usize];
    let mut ykyl: libc::c_double = (*envs).rkrl[1 as libc::c_int as usize];
    let mut zkzl: libc::c_double = (*envs).rkrl[2 as libc::c_int as usize];
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(8 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(9 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (xkxl + *cpx.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (xkxl + *cpx.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *g.offset(10 as libc::c_int as isize)
        * (xkxl + *cpx.offset(0 as libc::c_int as isize))
        + *cpx.offset(0 as libc::c_int as isize) * *b00.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize)
            * *c0x.offset(0 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *g.offset(11 as libc::c_int as isize)
        * (xkxl + *cpx.offset(1 as libc::c_int as isize))
        + *cpx.offset(1 as libc::c_int as isize) * *b00.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize)
            * *c0x.offset(1 as libc::c_int as isize);
    *g.offset(4 as libc::c_int as isize) = xkxl + *cpx.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = xkxl + *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            6 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (xkxl + *cpx.offset(0 as libc::c_int as isize))
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            7 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (xkxl + *cpx.offset(1 as libc::c_int as isize))
        + *b00.offset(1 as libc::c_int as isize);
    *g.offset(24 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(25 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(26 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(27 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(32 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(33 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            34 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            35 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            36 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (ykyl + *cpy.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            37 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (ykyl + *cpy.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            38 as libc::c_int as isize,
        ) = *g.offset(34 as libc::c_int as isize)
        * (ykyl + *cpy.offset(0 as libc::c_int as isize))
        + *cpy.offset(0 as libc::c_int as isize) * *b00.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize)
            * *c0y.offset(0 as libc::c_int as isize);
    *g
        .offset(
            39 as libc::c_int as isize,
        ) = *g.offset(35 as libc::c_int as isize)
        * (ykyl + *cpy.offset(1 as libc::c_int as isize))
        + *cpy.offset(1 as libc::c_int as isize) * *b00.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize)
            * *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(0 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (ykyl + *cpy.offset(0 as libc::c_int as isize))
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (ykyl + *cpy.offset(1 as libc::c_int as isize))
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            50 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            51 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            56 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            57 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            58 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(50 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            59 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(51 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            60 as libc::c_int as isize,
        ) = *g.offset(56 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            61 as libc::c_int as isize,
        ) = *g.offset(57 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            62 as libc::c_int as isize,
        ) = *g.offset(58 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(50 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(56 as libc::c_int as isize);
    *g
        .offset(
            63 as libc::c_int as isize,
        ) = *g.offset(59 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(51 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(57 as libc::c_int as isize);
    *g
        .offset(
            52 as libc::c_int as isize,
        ) = *g.offset(48 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize));
    *g
        .offset(
            53 as libc::c_int as isize,
        ) = *g.offset(49 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize));
    *g
        .offset(
            54 as libc::c_int as isize,
        ) = *g.offset(50 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize))
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            55 as libc::c_int as isize,
        ) = *g.offset(51 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize))
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1020(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(4 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            6 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            7 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            8 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *cpx.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            9 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *cpx.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (*g.offset(6 as libc::c_int as isize) + *b00.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize)
            * *c0x.offset(0 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (*g.offset(7 as libc::c_int as isize) + *b00.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize)
            * *c0x.offset(1 as libc::c_int as isize);
    *g.offset(12 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(13 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(14 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(15 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(16 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(17 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *cpy.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *cpy.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (*g.offset(18 as libc::c_int as isize)
            + *b00.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize)
            * *c0y.offset(0 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (*g.offset(19 as libc::c_int as isize)
            + *b00.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize)
            * *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(26 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(27 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            32 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(28 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            33 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(29 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            34 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(30 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(26 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(28 as libc::c_int as isize);
    *g
        .offset(
            35 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(31 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(27 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(29 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1100(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    let mut xixj: libc::c_double = (*envs).rirj[0 as libc::c_int as usize];
    let mut yiyj: libc::c_double = (*envs).rirj[1 as libc::c_int as usize];
    let mut zizj: libc::c_double = (*envs).rirj[2 as libc::c_int as usize];
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g
        .offset(
            6 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (xixj + *c0x.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            7 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (xixj + *c0x.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize);
    *g.offset(2 as libc::c_int as isize) = xixj + *c0x.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = xixj + *c0x.offset(1 as libc::c_int as isize);
    *g.offset(12 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(13 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(16 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(17 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (yiyj + *c0y.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (yiyj + *c0y.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(0 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *g.offset(28 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *g.offset(29 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = *g.offset(24 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize));
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = *g.offset(25 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize));
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1101(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    let mut xixj: libc::c_double = (*envs).rirj[0 as libc::c_int as usize];
    let mut yiyj: libc::c_double = (*envs).rirj[1 as libc::c_int as usize];
    let mut zizj: libc::c_double = (*envs).rirj[2 as libc::c_int as usize];
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(8 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(9 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(4 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (xixj + *c0x.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (xixj + *c0x.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize);
    *g.offset(2 as libc::c_int as isize) = xixj + *c0x.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = xixj + *c0x.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *g.offset(12 as libc::c_int as isize)
        * (xixj + *c0x.offset(0 as libc::c_int as isize))
        + *c0x.offset(0 as libc::c_int as isize) * *b00.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize)
            * *cpx.offset(0 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *g.offset(13 as libc::c_int as isize)
        * (xixj + *c0x.offset(1 as libc::c_int as isize))
        + *c0x.offset(1 as libc::c_int as isize) * *b00.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize)
            * *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            6 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (xixj + *c0x.offset(0 as libc::c_int as isize))
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            7 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (xixj + *c0x.offset(1 as libc::c_int as isize))
        + *b00.offset(1 as libc::c_int as isize);
    *g.offset(24 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(25 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(32 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(33 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(28 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(29 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            36 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            37 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            34 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (yiyj + *c0y.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            35 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (yiyj + *c0y.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(0 as libc::c_int as isize);
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            38 as libc::c_int as isize,
        ) = *g.offset(36 as libc::c_int as isize)
        * (yiyj + *c0y.offset(0 as libc::c_int as isize))
        + *c0y.offset(0 as libc::c_int as isize) * *b00.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize)
            * *cpy.offset(0 as libc::c_int as isize);
    *g
        .offset(
            39 as libc::c_int as isize,
        ) = *g.offset(37 as libc::c_int as isize)
        * (yiyj + *c0y.offset(1 as libc::c_int as isize))
        + *c0y.offset(1 as libc::c_int as isize) * *b00.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize)
            * *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (yiyj + *c0y.offset(0 as libc::c_int as isize))
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (yiyj + *c0y.offset(1 as libc::c_int as isize))
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            56 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            57 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            52 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            53 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            60 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(56 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            61 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(57 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            58 as libc::c_int as isize,
        ) = *g.offset(56 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            59 as libc::c_int as isize,
        ) = *g.offset(57 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            50 as libc::c_int as isize,
        ) = *g.offset(48 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize));
    *g
        .offset(
            51 as libc::c_int as isize,
        ) = *g.offset(49 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize));
    *g
        .offset(
            62 as libc::c_int as isize,
        ) = *g.offset(60 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(52 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(56 as libc::c_int as isize);
    *g
        .offset(
            63 as libc::c_int as isize,
        ) = *g.offset(61 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(53 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(57 as libc::c_int as isize);
    *g
        .offset(
            54 as libc::c_int as isize,
        ) = zizj * *g.offset(52 as libc::c_int as isize)
        + *cpz.offset(0 as libc::c_int as isize) * *g.offset(56 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            55 as libc::c_int as isize,
        ) = zizj * *g.offset(53 as libc::c_int as isize)
        + *cpz.offset(1 as libc::c_int as isize) * *g.offset(57 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1110(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    let mut xixj: libc::c_double = (*envs).rirj[0 as libc::c_int as usize];
    let mut yiyj: libc::c_double = (*envs).rirj[1 as libc::c_int as usize];
    let mut zizj: libc::c_double = (*envs).rirj[2 as libc::c_int as usize];
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(8 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(9 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(4 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (xixj + *c0x.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (xixj + *c0x.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize);
    *g.offset(2 as libc::c_int as isize) = xixj + *c0x.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = xixj + *c0x.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *g.offset(12 as libc::c_int as isize)
        * (xixj + *c0x.offset(0 as libc::c_int as isize))
        + *c0x.offset(0 as libc::c_int as isize) * *b00.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize)
            * *cpx.offset(0 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *g.offset(13 as libc::c_int as isize)
        * (xixj + *c0x.offset(1 as libc::c_int as isize))
        + *c0x.offset(1 as libc::c_int as isize) * *b00.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize)
            * *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            6 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (xixj + *c0x.offset(0 as libc::c_int as isize))
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            7 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (xixj + *c0x.offset(1 as libc::c_int as isize))
        + *b00.offset(1 as libc::c_int as isize);
    *g.offset(24 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(25 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(32 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(33 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(28 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(29 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            36 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            37 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            34 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (yiyj + *c0y.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            35 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (yiyj + *c0y.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(0 as libc::c_int as isize);
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            38 as libc::c_int as isize,
        ) = *g.offset(36 as libc::c_int as isize)
        * (yiyj + *c0y.offset(0 as libc::c_int as isize))
        + *c0y.offset(0 as libc::c_int as isize) * *b00.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize)
            * *cpy.offset(0 as libc::c_int as isize);
    *g
        .offset(
            39 as libc::c_int as isize,
        ) = *g.offset(37 as libc::c_int as isize)
        * (yiyj + *c0y.offset(1 as libc::c_int as isize))
        + *c0y.offset(1 as libc::c_int as isize) * *b00.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize)
            * *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (yiyj + *c0y.offset(0 as libc::c_int as isize))
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (yiyj + *c0y.offset(1 as libc::c_int as isize))
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            56 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            57 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            52 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            53 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            60 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(56 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            61 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(57 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            58 as libc::c_int as isize,
        ) = *g.offset(56 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            59 as libc::c_int as isize,
        ) = *g.offset(57 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            50 as libc::c_int as isize,
        ) = *g.offset(48 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize));
    *g
        .offset(
            51 as libc::c_int as isize,
        ) = *g.offset(49 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize));
    *g
        .offset(
            62 as libc::c_int as isize,
        ) = *g.offset(60 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(52 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(56 as libc::c_int as isize);
    *g
        .offset(
            63 as libc::c_int as isize,
        ) = *g.offset(61 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(53 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(57 as libc::c_int as isize);
    *g
        .offset(
            54 as libc::c_int as isize,
        ) = zizj * *g.offset(52 as libc::c_int as isize)
        + *cpz.offset(0 as libc::c_int as isize) * *g.offset(56 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            55 as libc::c_int as isize,
        ) = zizj * *g.offset(53 as libc::c_int as isize)
        + *cpz.offset(1 as libc::c_int as isize) * *g.offset(57 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1200(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    let mut xixj: libc::c_double = (*envs).rirj[0 as libc::c_int as usize];
    let mut yiyj: libc::c_double = (*envs).rirj[1 as libc::c_int as usize];
    let mut zizj: libc::c_double = (*envs).rirj[2 as libc::c_int as usize];
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g
        .offset(
            8 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            9 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *g.offset(8 as libc::c_int as isize)
        * (xixj + *c0x.offset(0 as libc::c_int as isize))
        + *c0x.offset(0 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *g.offset(9 as libc::c_int as isize)
        * (xixj + *c0x.offset(1 as libc::c_int as isize))
        + *c0x.offset(1 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            6 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (xixj + *c0x.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            7 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (xixj + *c0x.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize);
    *g.offset(2 as libc::c_int as isize) = xixj + *c0x.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = xixj + *c0x.offset(1 as libc::c_int as isize);
    *g.offset(16 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(17 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(20 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(21 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            24 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            25 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = *g.offset(24 as libc::c_int as isize)
        * (yiyj + *c0y.offset(0 as libc::c_int as isize))
        + *c0y.offset(0 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = *g.offset(25 as libc::c_int as isize)
        * (yiyj + *c0y.offset(1 as libc::c_int as isize))
        + *c0y.offset(1 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (yiyj + *c0y.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (yiyj + *c0y.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(0 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            36 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            37 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            40 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(36 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            41 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(37 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            42 as libc::c_int as isize,
        ) = *g.offset(40 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize))
        + 2 as libc::c_int as libc::c_double * *b10.offset(0 as libc::c_int as isize)
            * *g.offset(36 as libc::c_int as isize);
    *g
        .offset(
            43 as libc::c_int as isize,
        ) = *g.offset(41 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize))
        + 2 as libc::c_int as libc::c_double * *b10.offset(1 as libc::c_int as isize)
            * *g.offset(37 as libc::c_int as isize);
    *g
        .offset(
            38 as libc::c_int as isize,
        ) = *g.offset(36 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            39 as libc::c_int as isize,
        ) = *g.offset(37 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            34 as libc::c_int as isize,
        ) = *g.offset(32 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize));
    *g
        .offset(
            35 as libc::c_int as isize,
        ) = *g.offset(33 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize));
}
#[inline]
unsafe extern "C" fn _g0_2d4d_2000(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g
        .offset(
            4 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            5 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(7 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(8 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(9 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(12 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(13 as libc::c_int as isize);
    *g
        .offset(
            16 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(14 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(12 as libc::c_int as isize);
    *g
        .offset(
            17 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(15 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(13 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_2001(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g
        .offset(
            4 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            5 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            8 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            9 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (*g.offset(8 as libc::c_int as isize) + *b00.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize)
            * *cpx.offset(0 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (*g.offset(9 as libc::c_int as isize) + *b00.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize)
            * *cpx.offset(1 as libc::c_int as isize);
    *g.offset(12 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(13 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(14 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(15 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            16 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            17 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g.offset(18 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(19 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (*g.offset(20 as libc::c_int as isize)
            + *b00.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize)
            * *cpy.offset(0 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (*g.offset(21 as libc::c_int as isize)
            + *b00.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize)
            * *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(26 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(27 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            32 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(26 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            33 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(27 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            34 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(32 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(30 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(26 as libc::c_int as isize);
    *g
        .offset(
            35 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(33 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(31 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(27 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_2010(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g
        .offset(
            4 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            5 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            8 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            9 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (*g.offset(8 as libc::c_int as isize) + *b00.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize)
            * *cpx.offset(0 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (*g.offset(9 as libc::c_int as isize) + *b00.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize)
            * *cpx.offset(1 as libc::c_int as isize);
    *g.offset(12 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(13 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(14 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(15 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            16 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            17 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g.offset(18 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(19 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (*g.offset(20 as libc::c_int as isize)
            + *b00.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize)
            * *cpy.offset(0 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (*g.offset(21 as libc::c_int as isize)
            + *b00.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize)
            * *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(26 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(27 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            32 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(26 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            33 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(27 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            34 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(32 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(30 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(26 as libc::c_int as isize);
    *g
        .offset(
            35 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(33 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(31 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(27 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_2100(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    let mut xixj: libc::c_double = (*envs).rirj[0 as libc::c_int as usize];
    let mut yiyj: libc::c_double = (*envs).rirj[1 as libc::c_int as usize];
    let mut zizj: libc::c_double = (*envs).rirj[2 as libc::c_int as usize];
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g
        .offset(
            4 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            5 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *g.offset(4 as libc::c_int as isize)
        * (xixj + *c0x.offset(0 as libc::c_int as isize))
        + *c0x.offset(0 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *g.offset(5 as libc::c_int as isize)
        * (xixj + *c0x.offset(1 as libc::c_int as isize))
        + *c0x.offset(1 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (xixj + *c0x.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (xixj + *c0x.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize);
    *g.offset(8 as libc::c_int as isize) = xixj + *c0x.offset(0 as libc::c_int as isize);
    *g.offset(9 as libc::c_int as isize) = xixj + *c0x.offset(1 as libc::c_int as isize);
    *g.offset(16 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(17 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(18 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(19 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *g.offset(20 as libc::c_int as isize)
        * (yiyj + *c0y.offset(0 as libc::c_int as isize))
        + *c0y.offset(0 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *g.offset(21 as libc::c_int as isize)
        * (yiyj + *c0y.offset(1 as libc::c_int as isize))
        + *c0y.offset(1 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (yiyj + *c0y.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (yiyj + *c0y.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            24 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(0 as libc::c_int as isize);
    *g
        .offset(
            25 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            34 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            35 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            36 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(34 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            37 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(35 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            44 as libc::c_int as isize,
        ) = *g.offset(36 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize))
        + 2 as libc::c_int as libc::c_double * *b10.offset(0 as libc::c_int as isize)
            * *g.offset(34 as libc::c_int as isize);
    *g
        .offset(
            45 as libc::c_int as isize,
        ) = *g.offset(37 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize))
        + 2 as libc::c_int as libc::c_double * *b10.offset(1 as libc::c_int as isize)
            * *g.offset(35 as libc::c_int as isize);
    *g
        .offset(
            42 as libc::c_int as isize,
        ) = *g.offset(34 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            43 as libc::c_int as isize,
        ) = *g.offset(35 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            40 as libc::c_int as isize,
        ) = *g.offset(32 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize));
    *g
        .offset(
            41 as libc::c_int as isize,
        ) = *g.offset(33 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize));
}
#[inline]
unsafe extern "C" fn _g0_2d4d_3000(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g
        .offset(
            4 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            5 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            6 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (*g.offset(4 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b10.offset(0 as libc::c_int as isize));
    *g
        .offset(
            7 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (*g.offset(5 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b10.offset(1 as libc::c_int as isize));
    *g.offset(8 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(9 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(10 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(11 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (*g.offset(12 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b10.offset(0 as libc::c_int as isize));
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (*g.offset(13 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b10.offset(1 as libc::c_int as isize));
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(16 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(17 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(18 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(16 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(19 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(17 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(20 as libc::c_int as isize)
        + 2 as libc::c_int as libc::c_double * *b10.offset(0 as libc::c_int as isize)
            * *g.offset(18 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(21 as libc::c_int as isize)
        + 2 as libc::c_int as libc::c_double * *b10.offset(1 as libc::c_int as isize)
            * *g.offset(19 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e_2d4d_unrolled(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut type_ijkl: libc::c_int = (*envs).li_ceil << 6 as libc::c_int
        | (*envs).lj_ceil << 4 as libc::c_int | (*envs).lk_ceil << 2 as libc::c_int
        | (*envs).ll_ceil;
    match type_ijkl {
        0 => {
            _g0_2d4d_0000(g, bc, envs);
            return;
        }
        1 => {
            _g0_2d4d_0001(g, bc, envs);
            return;
        }
        2 => {
            _g0_2d4d_0002(g, bc, envs);
            return;
        }
        3 => {
            _g0_2d4d_0003(g, bc, envs);
            return;
        }
        4 => {
            _g0_2d4d_0010(g, bc, envs);
            return;
        }
        5 => {
            _g0_2d4d_0011(g, bc, envs);
            return;
        }
        6 => {
            _g0_2d4d_0012(g, bc, envs);
            return;
        }
        8 => {
            _g0_2d4d_0020(g, bc, envs);
            return;
        }
        9 => {
            _g0_2d4d_0021(g, bc, envs);
            return;
        }
        12 => {
            _g0_2d4d_0030(g, bc, envs);
            return;
        }
        16 => {
            _g0_2d4d_0100(g, bc, envs);
            return;
        }
        17 => {
            _g0_2d4d_0101(g, bc, envs);
            return;
        }
        18 => {
            _g0_2d4d_0102(g, bc, envs);
            return;
        }
        20 => {
            _g0_2d4d_0110(g, bc, envs);
            return;
        }
        21 => {
            _g0_2d4d_0111(g, bc, envs);
            return;
        }
        24 => {
            _g0_2d4d_0120(g, bc, envs);
            return;
        }
        32 => {
            _g0_2d4d_0200(g, bc, envs);
            return;
        }
        33 => {
            _g0_2d4d_0201(g, bc, envs);
            return;
        }
        36 => {
            _g0_2d4d_0210(g, bc, envs);
            return;
        }
        48 => {
            _g0_2d4d_0300(g, bc, envs);
            return;
        }
        64 => {
            _g0_2d4d_1000(g, bc, envs);
            return;
        }
        65 => {
            _g0_2d4d_1001(g, bc, envs);
            return;
        }
        66 => {
            _g0_2d4d_1002(g, bc, envs);
            return;
        }
        68 => {
            _g0_2d4d_1010(g, bc, envs);
            return;
        }
        69 => {
            _g0_2d4d_1011(g, bc, envs);
            return;
        }
        72 => {
            _g0_2d4d_1020(g, bc, envs);
            return;
        }
        80 => {
            _g0_2d4d_1100(g, bc, envs);
            return;
        }
        81 => {
            _g0_2d4d_1101(g, bc, envs);
            return;
        }
        84 => {
            _g0_2d4d_1110(g, bc, envs);
            return;
        }
        96 => {
            _g0_2d4d_1200(g, bc, envs);
            return;
        }
        128 => {
            _g0_2d4d_2000(g, bc, envs);
            return;
        }
        129 => {
            _g0_2d4d_2001(g, bc, envs);
            return;
        }
        132 => {
            _g0_2d4d_2010(g, bc, envs);
            return;
        }
        144 => {
            _g0_2d4d_2100(g, bc, envs);
            return;
        }
        192 => {
            _g0_2d4d_3000(g, bc, envs);
            return;
        }
        _ => {}
    }
    fprintf(
        stderr,
        b"Dimension error for CINTg0_2e_lj2d4d: iklj = %d %d %d %d\0" as *const u8
            as *const libc::c_char,
        (*envs).li_ceil,
        (*envs).lk_ceil,
        (*envs).ll_ceil,
        (*envs).lj_ceil,
    );
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0000(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0001(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(4 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(5 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(6 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(8 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(9 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0002(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *cpx.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *cpx.offset(3 as libc::c_int as isize);
    *g
        .offset(
            8 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *cpx.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            9 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *cpx.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *cpx.offset(2 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *cpx.offset(3 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize);
    *g.offset(12 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(13 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(14 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(15 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(16 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(17 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(18 as libc::c_int as isize) = *cpy.offset(2 as libc::c_int as isize);
    *g.offset(19 as libc::c_int as isize) = *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *cpy.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *cpy.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *cpy.offset(2 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *cpy.offset(3 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(26 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(27 as libc::c_int as isize);
    *g
        .offset(
            32 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(28 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            33 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(29 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            34 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(30 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize) * *g.offset(26 as libc::c_int as isize);
    *g
        .offset(
            35 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(31 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize) * *g.offset(27 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0003(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *cpx.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *cpx.offset(3 as libc::c_int as isize);
    *g
        .offset(
            8 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *cpx.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            9 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *cpx.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *cpx.offset(2 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *cpx.offset(3 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (*g.offset(8 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b01.offset(0 as libc::c_int as isize));
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (*g.offset(9 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b01.offset(1 as libc::c_int as isize));
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * (*g.offset(10 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b01.offset(2 as libc::c_int as isize));
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * (*g.offset(11 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b01.offset(3 as libc::c_int as isize));
    *g.offset(16 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(17 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(18 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(19 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(20 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(21 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(22 as libc::c_int as isize) = *cpy.offset(2 as libc::c_int as isize);
    *g.offset(23 as libc::c_int as isize) = *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            24 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *cpy.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            25 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *cpy.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *cpy.offset(2 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *cpy.offset(3 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (*g.offset(24 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b01.offset(0 as libc::c_int as isize));
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (*g.offset(25 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b01.offset(1 as libc::c_int as isize));
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * (*g.offset(26 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b01.offset(2 as libc::c_int as isize));
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * (*g.offset(27 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b01.offset(3 as libc::c_int as isize));
    *g
        .offset(
            36 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            37 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            38 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(34 as libc::c_int as isize);
    *g
        .offset(
            39 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(35 as libc::c_int as isize);
    *g
        .offset(
            40 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(36 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            41 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(37 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            42 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(38 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize) * *g.offset(34 as libc::c_int as isize);
    *g
        .offset(
            43 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(39 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize) * *g.offset(35 as libc::c_int as isize);
    *g
        .offset(
            44 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(40 as libc::c_int as isize)
        + 2 as libc::c_int as libc::c_double * *b01.offset(0 as libc::c_int as isize)
            * *g.offset(36 as libc::c_int as isize);
    *g
        .offset(
            45 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(41 as libc::c_int as isize)
        + 2 as libc::c_int as libc::c_double * *b01.offset(1 as libc::c_int as isize)
            * *g.offset(37 as libc::c_int as isize);
    *g
        .offset(
            46 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(42 as libc::c_int as isize)
        + 2 as libc::c_int as libc::c_double * *b01.offset(2 as libc::c_int as isize)
            * *g.offset(38 as libc::c_int as isize);
    *g
        .offset(
            47 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(43 as libc::c_int as isize)
        + 2 as libc::c_int as libc::c_double * *b01.offset(3 as libc::c_int as isize)
            * *g.offset(39 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0010(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(4 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(5 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(6 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(8 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(9 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0011(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    let mut xkxl: libc::c_double = (*envs).rkrl[0 as libc::c_int as usize];
    let mut ykyl: libc::c_double = (*envs).rkrl[1 as libc::c_int as usize];
    let mut zkzl: libc::c_double = (*envs).rkrl[2 as libc::c_int as usize];
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(8 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(9 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(10 as libc::c_int as isize) = *cpx.offset(2 as libc::c_int as isize);
    *g.offset(11 as libc::c_int as isize) = *cpx.offset(3 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (xkxl + *cpx.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (xkxl + *cpx.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * (xkxl + *cpx.offset(2 as libc::c_int as isize))
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * (xkxl + *cpx.offset(3 as libc::c_int as isize))
        + *b01.offset(3 as libc::c_int as isize);
    *g.offset(4 as libc::c_int as isize) = xkxl + *cpx.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = xkxl + *cpx.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = xkxl + *cpx.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = xkxl + *cpx.offset(3 as libc::c_int as isize);
    *g.offset(24 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(25 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(26 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(27 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(32 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(33 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(34 as libc::c_int as isize) = *cpy.offset(2 as libc::c_int as isize);
    *g.offset(35 as libc::c_int as isize) = *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            36 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (ykyl + *cpy.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            37 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (ykyl + *cpy.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            38 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * (ykyl + *cpy.offset(2 as libc::c_int as isize))
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            39 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * (ykyl + *cpy.offset(3 as libc::c_int as isize))
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(0 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(2 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            56 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            57 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            58 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            59 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            60 as libc::c_int as isize,
        ) = *g.offset(56 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            61 as libc::c_int as isize,
        ) = *g.offset(57 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            62 as libc::c_int as isize,
        ) = *g.offset(58 as libc::c_int as isize)
        * (zkzl + *cpz.offset(2 as libc::c_int as isize))
        + *b01.offset(2 as libc::c_int as isize) * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            63 as libc::c_int as isize,
        ) = *g.offset(59 as libc::c_int as isize)
        * (zkzl + *cpz.offset(3 as libc::c_int as isize))
        + *b01.offset(3 as libc::c_int as isize) * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            52 as libc::c_int as isize,
        ) = *g.offset(48 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize));
    *g
        .offset(
            53 as libc::c_int as isize,
        ) = *g.offset(49 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize));
    *g
        .offset(
            54 as libc::c_int as isize,
        ) = *g.offset(50 as libc::c_int as isize)
        * (zkzl + *cpz.offset(2 as libc::c_int as isize));
    *g
        .offset(
            55 as libc::c_int as isize,
        ) = *g.offset(51 as libc::c_int as isize)
        * (zkzl + *cpz.offset(3 as libc::c_int as isize));
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0012(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    let mut xkxl: libc::c_double = (*envs).rkrl[0 as libc::c_int as usize];
    let mut ykyl: libc::c_double = (*envs).rkrl[1 as libc::c_int as usize];
    let mut zkzl: libc::c_double = (*envs).rkrl[2 as libc::c_int as usize];
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(8 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(9 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(10 as libc::c_int as isize) = *cpx.offset(2 as libc::c_int as isize);
    *g.offset(11 as libc::c_int as isize) = *cpx.offset(3 as libc::c_int as isize);
    *g
        .offset(
            16 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *cpx.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            17 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *cpx.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *cpx.offset(2 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *cpx.offset(3 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *g.offset(16 as libc::c_int as isize)
        * (xkxl + *cpx.offset(0 as libc::c_int as isize))
        + *cpx.offset(0 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *g.offset(17 as libc::c_int as isize)
        * (xkxl + *cpx.offset(1 as libc::c_int as isize))
        + *cpx.offset(1 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *g.offset(18 as libc::c_int as isize)
        * (xkxl + *cpx.offset(2 as libc::c_int as isize))
        + *cpx.offset(2 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *g.offset(19 as libc::c_int as isize)
        * (xkxl + *cpx.offset(3 as libc::c_int as isize))
        + *cpx.offset(3 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (xkxl + *cpx.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (xkxl + *cpx.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * (xkxl + *cpx.offset(2 as libc::c_int as isize))
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * (xkxl + *cpx.offset(3 as libc::c_int as isize))
        + *b01.offset(3 as libc::c_int as isize);
    *g.offset(4 as libc::c_int as isize) = xkxl + *cpx.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = xkxl + *cpx.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = xkxl + *cpx.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = xkxl + *cpx.offset(3 as libc::c_int as isize);
    *g.offset(32 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(33 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(34 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(35 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(40 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(41 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(42 as libc::c_int as isize) = *cpy.offset(2 as libc::c_int as isize);
    *g.offset(43 as libc::c_int as isize) = *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            48 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *cpy.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            49 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *cpy.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            50 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *cpy.offset(2 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            51 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *cpy.offset(3 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            52 as libc::c_int as isize,
        ) = *g.offset(48 as libc::c_int as isize)
        * (ykyl + *cpy.offset(0 as libc::c_int as isize))
        + *cpy.offset(0 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            53 as libc::c_int as isize,
        ) = *g.offset(49 as libc::c_int as isize)
        * (ykyl + *cpy.offset(1 as libc::c_int as isize))
        + *cpy.offset(1 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            54 as libc::c_int as isize,
        ) = *g.offset(50 as libc::c_int as isize)
        * (ykyl + *cpy.offset(2 as libc::c_int as isize))
        + *cpy.offset(2 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            55 as libc::c_int as isize,
        ) = *g.offset(51 as libc::c_int as isize)
        * (ykyl + *cpy.offset(3 as libc::c_int as isize))
        + *cpy.offset(3 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            44 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (ykyl + *cpy.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            45 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (ykyl + *cpy.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            46 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * (ykyl + *cpy.offset(2 as libc::c_int as isize))
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            47 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * (ykyl + *cpy.offset(3 as libc::c_int as isize))
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            36 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(0 as libc::c_int as isize);
    *g
        .offset(
            37 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            38 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(2 as libc::c_int as isize);
    *g
        .offset(
            39 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            72 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(64 as libc::c_int as isize);
    *g
        .offset(
            73 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(65 as libc::c_int as isize);
    *g
        .offset(
            74 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(66 as libc::c_int as isize);
    *g
        .offset(
            75 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(67 as libc::c_int as isize);
    *g
        .offset(
            80 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(72 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(64 as libc::c_int as isize);
    *g
        .offset(
            81 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(73 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(65 as libc::c_int as isize);
    *g
        .offset(
            82 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(74 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize) * *g.offset(66 as libc::c_int as isize);
    *g
        .offset(
            83 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(75 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize) * *g.offset(67 as libc::c_int as isize);
    *g
        .offset(
            84 as libc::c_int as isize,
        ) = *g.offset(80 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize))
        + 2 as libc::c_int as libc::c_double * *b01.offset(0 as libc::c_int as isize)
            * *g.offset(72 as libc::c_int as isize);
    *g
        .offset(
            85 as libc::c_int as isize,
        ) = *g.offset(81 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize))
        + 2 as libc::c_int as libc::c_double * *b01.offset(1 as libc::c_int as isize)
            * *g.offset(73 as libc::c_int as isize);
    *g
        .offset(
            86 as libc::c_int as isize,
        ) = *g.offset(82 as libc::c_int as isize)
        * (zkzl + *cpz.offset(2 as libc::c_int as isize))
        + 2 as libc::c_int as libc::c_double * *b01.offset(2 as libc::c_int as isize)
            * *g.offset(74 as libc::c_int as isize);
    *g
        .offset(
            87 as libc::c_int as isize,
        ) = *g.offset(83 as libc::c_int as isize)
        * (zkzl + *cpz.offset(3 as libc::c_int as isize))
        + 2 as libc::c_int as libc::c_double * *b01.offset(3 as libc::c_int as isize)
            * *g.offset(75 as libc::c_int as isize);
    *g
        .offset(
            76 as libc::c_int as isize,
        ) = *g.offset(72 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(64 as libc::c_int as isize);
    *g
        .offset(
            77 as libc::c_int as isize,
        ) = *g.offset(73 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(65 as libc::c_int as isize);
    *g
        .offset(
            78 as libc::c_int as isize,
        ) = *g.offset(74 as libc::c_int as isize)
        * (zkzl + *cpz.offset(2 as libc::c_int as isize))
        + *b01.offset(2 as libc::c_int as isize) * *g.offset(66 as libc::c_int as isize);
    *g
        .offset(
            79 as libc::c_int as isize,
        ) = *g.offset(75 as libc::c_int as isize)
        * (zkzl + *cpz.offset(3 as libc::c_int as isize))
        + *b01.offset(3 as libc::c_int as isize) * *g.offset(67 as libc::c_int as isize);
    *g
        .offset(
            68 as libc::c_int as isize,
        ) = *g.offset(64 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize));
    *g
        .offset(
            69 as libc::c_int as isize,
        ) = *g.offset(65 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize));
    *g
        .offset(
            70 as libc::c_int as isize,
        ) = *g.offset(66 as libc::c_int as isize)
        * (zkzl + *cpz.offset(2 as libc::c_int as isize));
    *g
        .offset(
            71 as libc::c_int as isize,
        ) = *g.offset(67 as libc::c_int as isize)
        * (zkzl + *cpz.offset(3 as libc::c_int as isize));
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0020(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *cpx.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *cpx.offset(3 as libc::c_int as isize);
    *g
        .offset(
            8 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *cpx.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            9 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *cpx.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *cpx.offset(2 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *cpx.offset(3 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize);
    *g.offset(12 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(13 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(14 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(15 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(16 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(17 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(18 as libc::c_int as isize) = *cpy.offset(2 as libc::c_int as isize);
    *g.offset(19 as libc::c_int as isize) = *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *cpy.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *cpy.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *cpy.offset(2 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *cpy.offset(3 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(26 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(27 as libc::c_int as isize);
    *g
        .offset(
            32 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(28 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            33 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(29 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            34 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(30 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize) * *g.offset(26 as libc::c_int as isize);
    *g
        .offset(
            35 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(31 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize) * *g.offset(27 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0021(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    let mut xkxl: libc::c_double = (*envs).rkrl[0 as libc::c_int as usize];
    let mut ykyl: libc::c_double = (*envs).rkrl[1 as libc::c_int as usize];
    let mut zkzl: libc::c_double = (*envs).rkrl[2 as libc::c_int as usize];
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *cpx.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *cpx.offset(3 as libc::c_int as isize);
    *g
        .offset(
            8 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *cpx.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            9 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *cpx.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *cpx.offset(2 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *cpx.offset(3 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            16 as libc::c_int as isize,
        ) = xkxl + *cpx.offset(0 as libc::c_int as isize);
    *g
        .offset(
            17 as libc::c_int as isize,
        ) = xkxl + *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = xkxl + *cpx.offset(2 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = xkxl + *cpx.offset(3 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (xkxl + *cpx.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (xkxl + *cpx.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * (xkxl + *cpx.offset(2 as libc::c_int as isize))
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * (xkxl + *cpx.offset(3 as libc::c_int as isize))
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            24 as libc::c_int as isize,
        ) = *g.offset(8 as libc::c_int as isize)
        * (xkxl + *cpx.offset(0 as libc::c_int as isize))
        + *cpx.offset(0 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            25 as libc::c_int as isize,
        ) = *g.offset(9 as libc::c_int as isize)
        * (xkxl + *cpx.offset(1 as libc::c_int as isize))
        + *cpx.offset(1 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = *g.offset(10 as libc::c_int as isize)
        * (xkxl + *cpx.offset(2 as libc::c_int as isize))
        + *cpx.offset(2 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = *g.offset(11 as libc::c_int as isize)
        * (xkxl + *cpx.offset(3 as libc::c_int as isize))
        + *cpx.offset(3 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b01.offset(3 as libc::c_int as isize);
    *g.offset(32 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(33 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(34 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(35 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(36 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(37 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(38 as libc::c_int as isize) = *cpy.offset(2 as libc::c_int as isize);
    *g.offset(39 as libc::c_int as isize) = *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            40 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *cpy.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            41 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *cpy.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            42 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *cpy.offset(2 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            43 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *cpy.offset(3 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            48 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(0 as libc::c_int as isize);
    *g
        .offset(
            49 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            50 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(2 as libc::c_int as isize);
    *g
        .offset(
            51 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            52 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (ykyl + *cpy.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            53 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (ykyl + *cpy.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            54 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * (ykyl + *cpy.offset(2 as libc::c_int as isize))
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            55 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * (ykyl + *cpy.offset(3 as libc::c_int as isize))
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            56 as libc::c_int as isize,
        ) = *g.offset(40 as libc::c_int as isize)
        * (ykyl + *cpy.offset(0 as libc::c_int as isize))
        + *cpy.offset(0 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            57 as libc::c_int as isize,
        ) = *g.offset(41 as libc::c_int as isize)
        * (ykyl + *cpy.offset(1 as libc::c_int as isize))
        + *cpy.offset(1 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            58 as libc::c_int as isize,
        ) = *g.offset(42 as libc::c_int as isize)
        * (ykyl + *cpy.offset(2 as libc::c_int as isize))
        + *cpy.offset(2 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            59 as libc::c_int as isize,
        ) = *g.offset(43 as libc::c_int as isize)
        * (ykyl + *cpy.offset(3 as libc::c_int as isize))
        + *cpy.offset(3 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            68 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(64 as libc::c_int as isize);
    *g
        .offset(
            69 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(65 as libc::c_int as isize);
    *g
        .offset(
            70 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(66 as libc::c_int as isize);
    *g
        .offset(
            71 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(67 as libc::c_int as isize);
    *g
        .offset(
            72 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(68 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(64 as libc::c_int as isize);
    *g
        .offset(
            73 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(69 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(65 as libc::c_int as isize);
    *g
        .offset(
            74 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(70 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize) * *g.offset(66 as libc::c_int as isize);
    *g
        .offset(
            75 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(71 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize) * *g.offset(67 as libc::c_int as isize);
    *g
        .offset(
            80 as libc::c_int as isize,
        ) = *g.offset(64 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize));
    *g
        .offset(
            81 as libc::c_int as isize,
        ) = *g.offset(65 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize));
    *g
        .offset(
            82 as libc::c_int as isize,
        ) = *g.offset(66 as libc::c_int as isize)
        * (zkzl + *cpz.offset(2 as libc::c_int as isize));
    *g
        .offset(
            83 as libc::c_int as isize,
        ) = *g.offset(67 as libc::c_int as isize)
        * (zkzl + *cpz.offset(3 as libc::c_int as isize));
    *g
        .offset(
            84 as libc::c_int as isize,
        ) = *g.offset(68 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(64 as libc::c_int as isize);
    *g
        .offset(
            85 as libc::c_int as isize,
        ) = *g.offset(69 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(65 as libc::c_int as isize);
    *g
        .offset(
            86 as libc::c_int as isize,
        ) = *g.offset(70 as libc::c_int as isize)
        * (zkzl + *cpz.offset(2 as libc::c_int as isize))
        + *b01.offset(2 as libc::c_int as isize) * *g.offset(66 as libc::c_int as isize);
    *g
        .offset(
            87 as libc::c_int as isize,
        ) = *g.offset(71 as libc::c_int as isize)
        * (zkzl + *cpz.offset(3 as libc::c_int as isize))
        + *b01.offset(3 as libc::c_int as isize) * *g.offset(67 as libc::c_int as isize);
    *g
        .offset(
            88 as libc::c_int as isize,
        ) = *g.offset(72 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize))
        + 2 as libc::c_int as libc::c_double * *b01.offset(0 as libc::c_int as isize)
            * *g.offset(68 as libc::c_int as isize);
    *g
        .offset(
            89 as libc::c_int as isize,
        ) = *g.offset(73 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize))
        + 2 as libc::c_int as libc::c_double * *b01.offset(1 as libc::c_int as isize)
            * *g.offset(69 as libc::c_int as isize);
    *g
        .offset(
            90 as libc::c_int as isize,
        ) = *g.offset(74 as libc::c_int as isize)
        * (zkzl + *cpz.offset(2 as libc::c_int as isize))
        + 2 as libc::c_int as libc::c_double * *b01.offset(2 as libc::c_int as isize)
            * *g.offset(70 as libc::c_int as isize);
    *g
        .offset(
            91 as libc::c_int as isize,
        ) = *g.offset(75 as libc::c_int as isize)
        * (zkzl + *cpz.offset(3 as libc::c_int as isize))
        + 2 as libc::c_int as libc::c_double * *b01.offset(3 as libc::c_int as isize)
            * *g.offset(71 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0030(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *cpx.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *cpx.offset(3 as libc::c_int as isize);
    *g
        .offset(
            8 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *cpx.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            9 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *cpx.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *cpx.offset(2 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *cpx.offset(3 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (*g.offset(8 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b01.offset(0 as libc::c_int as isize));
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (*g.offset(9 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b01.offset(1 as libc::c_int as isize));
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * (*g.offset(10 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b01.offset(2 as libc::c_int as isize));
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * (*g.offset(11 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b01.offset(3 as libc::c_int as isize));
    *g.offset(16 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(17 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(18 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(19 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(20 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(21 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(22 as libc::c_int as isize) = *cpy.offset(2 as libc::c_int as isize);
    *g.offset(23 as libc::c_int as isize) = *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            24 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *cpy.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            25 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *cpy.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *cpy.offset(2 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *cpy.offset(3 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (*g.offset(24 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b01.offset(0 as libc::c_int as isize));
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (*g.offset(25 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b01.offset(1 as libc::c_int as isize));
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * (*g.offset(26 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b01.offset(2 as libc::c_int as isize));
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * (*g.offset(27 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b01.offset(3 as libc::c_int as isize));
    *g
        .offset(
            36 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            37 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            38 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(34 as libc::c_int as isize);
    *g
        .offset(
            39 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(35 as libc::c_int as isize);
    *g
        .offset(
            40 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(36 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            41 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(37 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            42 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(38 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize) * *g.offset(34 as libc::c_int as isize);
    *g
        .offset(
            43 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(39 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize) * *g.offset(35 as libc::c_int as isize);
    *g
        .offset(
            44 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(40 as libc::c_int as isize)
        + 2 as libc::c_int as libc::c_double * *b01.offset(0 as libc::c_int as isize)
            * *g.offset(36 as libc::c_int as isize);
    *g
        .offset(
            45 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(41 as libc::c_int as isize)
        + 2 as libc::c_int as libc::c_double * *b01.offset(1 as libc::c_int as isize)
            * *g.offset(37 as libc::c_int as isize);
    *g
        .offset(
            46 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(42 as libc::c_int as isize)
        + 2 as libc::c_int as libc::c_double * *b01.offset(2 as libc::c_int as isize)
            * *g.offset(38 as libc::c_int as isize);
    *g
        .offset(
            47 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(43 as libc::c_int as isize)
        + 2 as libc::c_int as libc::c_double * *b01.offset(3 as libc::c_int as isize)
            * *g.offset(39 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0100(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(4 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(5 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(6 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(8 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(9 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0101(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *cpx.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *cpx.offset(3 as libc::c_int as isize);
    *g.offset(8 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(9 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(10 as libc::c_int as isize) = *c0x.offset(2 as libc::c_int as isize);
    *g.offset(11 as libc::c_int as isize) = *c0x.offset(3 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g.offset(16 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(17 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(18 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(19 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(20 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(21 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(22 as libc::c_int as isize) = *cpy.offset(2 as libc::c_int as isize);
    *g.offset(23 as libc::c_int as isize) = *cpy.offset(3 as libc::c_int as isize);
    *g.offset(24 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(25 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(26 as libc::c_int as isize) = *c0y.offset(2 as libc::c_int as isize);
    *g.offset(27 as libc::c_int as isize) = *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            36 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            37 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            38 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(34 as libc::c_int as isize);
    *g
        .offset(
            39 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(35 as libc::c_int as isize);
    *g
        .offset(
            40 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            41 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            42 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(34 as libc::c_int as isize);
    *g
        .offset(
            43 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(35 as libc::c_int as isize);
    *g
        .offset(
            44 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(40 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            45 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(41 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            46 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(42 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(34 as libc::c_int as isize);
    *g
        .offset(
            47 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(43 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(35 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0102(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *cpx.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *cpx.offset(3 as libc::c_int as isize);
    *g.offset(12 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(13 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(14 as libc::c_int as isize) = *c0x.offset(2 as libc::c_int as isize);
    *g.offset(15 as libc::c_int as isize) = *c0x.offset(3 as libc::c_int as isize);
    *g
        .offset(
            8 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *cpx.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            9 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *cpx.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *cpx.offset(2 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *cpx.offset(3 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            16 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            17 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (*g.offset(16 as libc::c_int as isize)
            + *b00.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize)
            * *c0x.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (*g.offset(17 as libc::c_int as isize)
            + *b00.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize)
            * *c0x.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * (*g.offset(18 as libc::c_int as isize)
            + *b00.offset(2 as libc::c_int as isize))
        + *b01.offset(2 as libc::c_int as isize)
            * *c0x.offset(2 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * (*g.offset(19 as libc::c_int as isize)
            + *b00.offset(3 as libc::c_int as isize))
        + *b01.offset(3 as libc::c_int as isize)
            * *c0x.offset(3 as libc::c_int as isize);
    *g.offset(24 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(25 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(26 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(27 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(28 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(29 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(30 as libc::c_int as isize) = *cpy.offset(2 as libc::c_int as isize);
    *g.offset(31 as libc::c_int as isize) = *cpy.offset(3 as libc::c_int as isize);
    *g.offset(36 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(37 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(38 as libc::c_int as isize) = *c0y.offset(2 as libc::c_int as isize);
    *g.offset(39 as libc::c_int as isize) = *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            32 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *cpy.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            33 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *cpy.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            34 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *cpy.offset(2 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            35 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *cpy.offset(3 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            40 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            41 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            42 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            43 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            44 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (*g.offset(40 as libc::c_int as isize)
            + *b00.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize)
            * *c0y.offset(0 as libc::c_int as isize);
    *g
        .offset(
            45 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (*g.offset(41 as libc::c_int as isize)
            + *b00.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize)
            * *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            46 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * (*g.offset(42 as libc::c_int as isize)
            + *b00.offset(2 as libc::c_int as isize))
        + *b01.offset(2 as libc::c_int as isize)
            * *c0y.offset(2 as libc::c_int as isize);
    *g
        .offset(
            47 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * (*g.offset(43 as libc::c_int as isize)
            + *b00.offset(3 as libc::c_int as isize))
        + *b01.offset(3 as libc::c_int as isize)
            * *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            52 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            53 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            54 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            55 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            60 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            61 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            62 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            63 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            56 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(52 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            57 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(53 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            58 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(54 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize) * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            59 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(55 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize) * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            64 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(60 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            65 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(61 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            66 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(62 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            67 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(63 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            68 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(64 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(60 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(52 as libc::c_int as isize);
    *g
        .offset(
            69 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(65 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(61 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(53 as libc::c_int as isize);
    *g
        .offset(
            70 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(66 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize) * *g.offset(62 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(54 as libc::c_int as isize);
    *g
        .offset(
            71 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(67 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize) * *g.offset(63 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(55 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0110(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *cpx.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *cpx.offset(3 as libc::c_int as isize);
    *g.offset(8 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(9 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(10 as libc::c_int as isize) = *c0x.offset(2 as libc::c_int as isize);
    *g.offset(11 as libc::c_int as isize) = *c0x.offset(3 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g.offset(16 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(17 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(18 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(19 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(20 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(21 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(22 as libc::c_int as isize) = *cpy.offset(2 as libc::c_int as isize);
    *g.offset(23 as libc::c_int as isize) = *cpy.offset(3 as libc::c_int as isize);
    *g.offset(24 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(25 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(26 as libc::c_int as isize) = *c0y.offset(2 as libc::c_int as isize);
    *g.offset(27 as libc::c_int as isize) = *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            36 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            37 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            38 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(34 as libc::c_int as isize);
    *g
        .offset(
            39 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(35 as libc::c_int as isize);
    *g
        .offset(
            40 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            41 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            42 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(34 as libc::c_int as isize);
    *g
        .offset(
            43 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(35 as libc::c_int as isize);
    *g
        .offset(
            44 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(40 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            45 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(41 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            46 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(42 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(34 as libc::c_int as isize);
    *g
        .offset(
            47 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(43 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(35 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0111(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    let mut xkxl: libc::c_double = (*envs).rkrl[0 as libc::c_int as usize];
    let mut ykyl: libc::c_double = (*envs).rkrl[1 as libc::c_int as usize];
    let mut zkzl: libc::c_double = (*envs).rkrl[2 as libc::c_int as usize];
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(24 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(25 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(26 as libc::c_int as isize) = *c0x.offset(2 as libc::c_int as isize);
    *g.offset(27 as libc::c_int as isize) = *c0x.offset(3 as libc::c_int as isize);
    *g.offset(8 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(9 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(10 as libc::c_int as isize) = *cpx.offset(2 as libc::c_int as isize);
    *g.offset(11 as libc::c_int as isize) = *cpx.offset(3 as libc::c_int as isize);
    *g
        .offset(
            32 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            33 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            34 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            35 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (xkxl + *cpx.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (xkxl + *cpx.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * (xkxl + *cpx.offset(2 as libc::c_int as isize))
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * (xkxl + *cpx.offset(3 as libc::c_int as isize))
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            36 as libc::c_int as isize,
        ) = *g.offset(32 as libc::c_int as isize)
        * (xkxl + *cpx.offset(0 as libc::c_int as isize))
        + *cpx.offset(0 as libc::c_int as isize) * *b00.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize)
            * *c0x.offset(0 as libc::c_int as isize);
    *g
        .offset(
            37 as libc::c_int as isize,
        ) = *g.offset(33 as libc::c_int as isize)
        * (xkxl + *cpx.offset(1 as libc::c_int as isize))
        + *cpx.offset(1 as libc::c_int as isize) * *b00.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize)
            * *c0x.offset(1 as libc::c_int as isize);
    *g
        .offset(
            38 as libc::c_int as isize,
        ) = *g.offset(34 as libc::c_int as isize)
        * (xkxl + *cpx.offset(2 as libc::c_int as isize))
        + *cpx.offset(2 as libc::c_int as isize) * *b00.offset(2 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize)
            * *c0x.offset(2 as libc::c_int as isize);
    *g
        .offset(
            39 as libc::c_int as isize,
        ) = *g.offset(35 as libc::c_int as isize)
        * (xkxl + *cpx.offset(3 as libc::c_int as isize))
        + *cpx.offset(3 as libc::c_int as isize) * *b00.offset(3 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize)
            * *c0x.offset(3 as libc::c_int as isize);
    *g.offset(4 as libc::c_int as isize) = xkxl + *cpx.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = xkxl + *cpx.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = xkxl + *cpx.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = xkxl + *cpx.offset(3 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (xkxl + *cpx.offset(0 as libc::c_int as isize))
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (xkxl + *cpx.offset(1 as libc::c_int as isize))
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *c0x.offset(2 as libc::c_int as isize)
        * (xkxl + *cpx.offset(2 as libc::c_int as isize))
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *c0x.offset(3 as libc::c_int as isize)
        * (xkxl + *cpx.offset(3 as libc::c_int as isize))
        + *b00.offset(3 as libc::c_int as isize);
    *g.offset(48 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(49 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(50 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(51 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(72 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(73 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(74 as libc::c_int as isize) = *c0y.offset(2 as libc::c_int as isize);
    *g.offset(75 as libc::c_int as isize) = *c0y.offset(3 as libc::c_int as isize);
    *g.offset(56 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(57 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(58 as libc::c_int as isize) = *cpy.offset(2 as libc::c_int as isize);
    *g.offset(59 as libc::c_int as isize) = *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            80 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            81 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            82 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            83 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            60 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (ykyl + *cpy.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            61 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (ykyl + *cpy.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            62 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * (ykyl + *cpy.offset(2 as libc::c_int as isize))
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            63 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * (ykyl + *cpy.offset(3 as libc::c_int as isize))
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            84 as libc::c_int as isize,
        ) = *g.offset(80 as libc::c_int as isize)
        * (ykyl + *cpy.offset(0 as libc::c_int as isize))
        + *cpy.offset(0 as libc::c_int as isize) * *b00.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize)
            * *c0y.offset(0 as libc::c_int as isize);
    *g
        .offset(
            85 as libc::c_int as isize,
        ) = *g.offset(81 as libc::c_int as isize)
        * (ykyl + *cpy.offset(1 as libc::c_int as isize))
        + *cpy.offset(1 as libc::c_int as isize) * *b00.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize)
            * *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            86 as libc::c_int as isize,
        ) = *g.offset(82 as libc::c_int as isize)
        * (ykyl + *cpy.offset(2 as libc::c_int as isize))
        + *cpy.offset(2 as libc::c_int as isize) * *b00.offset(2 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize)
            * *c0y.offset(2 as libc::c_int as isize);
    *g
        .offset(
            87 as libc::c_int as isize,
        ) = *g.offset(83 as libc::c_int as isize)
        * (ykyl + *cpy.offset(3 as libc::c_int as isize))
        + *cpy.offset(3 as libc::c_int as isize) * *b00.offset(3 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize)
            * *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            52 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(0 as libc::c_int as isize);
    *g
        .offset(
            53 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            54 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(2 as libc::c_int as isize);
    *g
        .offset(
            55 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            76 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (ykyl + *cpy.offset(0 as libc::c_int as isize))
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            77 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (ykyl + *cpy.offset(1 as libc::c_int as isize))
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            78 as libc::c_int as isize,
        ) = *c0y.offset(2 as libc::c_int as isize)
        * (ykyl + *cpy.offset(2 as libc::c_int as isize))
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            79 as libc::c_int as isize,
        ) = *c0y.offset(3 as libc::c_int as isize)
        * (ykyl + *cpy.offset(3 as libc::c_int as isize))
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            120 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(96 as libc::c_int as isize);
    *g
        .offset(
            121 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(97 as libc::c_int as isize);
    *g
        .offset(
            122 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(98 as libc::c_int as isize);
    *g
        .offset(
            123 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(99 as libc::c_int as isize);
    *g
        .offset(
            104 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(96 as libc::c_int as isize);
    *g
        .offset(
            105 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(97 as libc::c_int as isize);
    *g
        .offset(
            106 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(98 as libc::c_int as isize);
    *g
        .offset(
            107 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(99 as libc::c_int as isize);
    *g
        .offset(
            128 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(120 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(96 as libc::c_int as isize);
    *g
        .offset(
            129 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(121 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(97 as libc::c_int as isize);
    *g
        .offset(
            130 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(122 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(98 as libc::c_int as isize);
    *g
        .offset(
            131 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(123 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(99 as libc::c_int as isize);
    *g
        .offset(
            108 as libc::c_int as isize,
        ) = *g.offset(104 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(96 as libc::c_int as isize);
    *g
        .offset(
            109 as libc::c_int as isize,
        ) = *g.offset(105 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(97 as libc::c_int as isize);
    *g
        .offset(
            110 as libc::c_int as isize,
        ) = *g.offset(106 as libc::c_int as isize)
        * (zkzl + *cpz.offset(2 as libc::c_int as isize))
        + *b01.offset(2 as libc::c_int as isize) * *g.offset(98 as libc::c_int as isize);
    *g
        .offset(
            111 as libc::c_int as isize,
        ) = *g.offset(107 as libc::c_int as isize)
        * (zkzl + *cpz.offset(3 as libc::c_int as isize))
        + *b01.offset(3 as libc::c_int as isize) * *g.offset(99 as libc::c_int as isize);
    *g
        .offset(
            132 as libc::c_int as isize,
        ) = *g.offset(128 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(120 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize)
            * *g.offset(104 as libc::c_int as isize);
    *g
        .offset(
            133 as libc::c_int as isize,
        ) = *g.offset(129 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(121 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize)
            * *g.offset(105 as libc::c_int as isize);
    *g
        .offset(
            134 as libc::c_int as isize,
        ) = *g.offset(130 as libc::c_int as isize)
        * (zkzl + *cpz.offset(2 as libc::c_int as isize))
        + *b01.offset(2 as libc::c_int as isize) * *g.offset(122 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize)
            * *g.offset(106 as libc::c_int as isize);
    *g
        .offset(
            135 as libc::c_int as isize,
        ) = *g.offset(131 as libc::c_int as isize)
        * (zkzl + *cpz.offset(3 as libc::c_int as isize))
        + *b01.offset(3 as libc::c_int as isize) * *g.offset(123 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize)
            * *g.offset(107 as libc::c_int as isize);
    *g
        .offset(
            100 as libc::c_int as isize,
        ) = *g.offset(96 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize));
    *g
        .offset(
            101 as libc::c_int as isize,
        ) = *g.offset(97 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize));
    *g
        .offset(
            102 as libc::c_int as isize,
        ) = *g.offset(98 as libc::c_int as isize)
        * (zkzl + *cpz.offset(2 as libc::c_int as isize));
    *g
        .offset(
            103 as libc::c_int as isize,
        ) = *g.offset(99 as libc::c_int as isize)
        * (zkzl + *cpz.offset(3 as libc::c_int as isize));
    *g
        .offset(
            124 as libc::c_int as isize,
        ) = *g.offset(120 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize))
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(96 as libc::c_int as isize);
    *g
        .offset(
            125 as libc::c_int as isize,
        ) = *g.offset(121 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize))
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(97 as libc::c_int as isize);
    *g
        .offset(
            126 as libc::c_int as isize,
        ) = *g.offset(122 as libc::c_int as isize)
        * (zkzl + *cpz.offset(2 as libc::c_int as isize))
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(98 as libc::c_int as isize);
    *g
        .offset(
            127 as libc::c_int as isize,
        ) = *g.offset(123 as libc::c_int as isize)
        * (zkzl + *cpz.offset(3 as libc::c_int as isize))
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(99 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0120(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *cpx.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *cpx.offset(3 as libc::c_int as isize);
    *g.offset(12 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(13 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(14 as libc::c_int as isize) = *c0x.offset(2 as libc::c_int as isize);
    *g.offset(15 as libc::c_int as isize) = *c0x.offset(3 as libc::c_int as isize);
    *g
        .offset(
            8 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *cpx.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            9 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *cpx.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *cpx.offset(2 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *cpx.offset(3 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            16 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            17 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (*g.offset(16 as libc::c_int as isize)
            + *b00.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize)
            * *c0x.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (*g.offset(17 as libc::c_int as isize)
            + *b00.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize)
            * *c0x.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * (*g.offset(18 as libc::c_int as isize)
            + *b00.offset(2 as libc::c_int as isize))
        + *b01.offset(2 as libc::c_int as isize)
            * *c0x.offset(2 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * (*g.offset(19 as libc::c_int as isize)
            + *b00.offset(3 as libc::c_int as isize))
        + *b01.offset(3 as libc::c_int as isize)
            * *c0x.offset(3 as libc::c_int as isize);
    *g.offset(24 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(25 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(26 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(27 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(28 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(29 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(30 as libc::c_int as isize) = *cpy.offset(2 as libc::c_int as isize);
    *g.offset(31 as libc::c_int as isize) = *cpy.offset(3 as libc::c_int as isize);
    *g.offset(36 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(37 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(38 as libc::c_int as isize) = *c0y.offset(2 as libc::c_int as isize);
    *g.offset(39 as libc::c_int as isize) = *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            32 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *cpy.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            33 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *cpy.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            34 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *cpy.offset(2 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            35 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *cpy.offset(3 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            40 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            41 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            42 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            43 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            44 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (*g.offset(40 as libc::c_int as isize)
            + *b00.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize)
            * *c0y.offset(0 as libc::c_int as isize);
    *g
        .offset(
            45 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (*g.offset(41 as libc::c_int as isize)
            + *b00.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize)
            * *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            46 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * (*g.offset(42 as libc::c_int as isize)
            + *b00.offset(2 as libc::c_int as isize))
        + *b01.offset(2 as libc::c_int as isize)
            * *c0y.offset(2 as libc::c_int as isize);
    *g
        .offset(
            47 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * (*g.offset(43 as libc::c_int as isize)
            + *b00.offset(3 as libc::c_int as isize))
        + *b01.offset(3 as libc::c_int as isize)
            * *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            52 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            53 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            54 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            55 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            60 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            61 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            62 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            63 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            56 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(52 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            57 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(53 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            58 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(54 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize) * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            59 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(55 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize) * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            64 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(60 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            65 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(61 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            66 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(62 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            67 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(63 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            68 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(64 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(60 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(52 as libc::c_int as isize);
    *g
        .offset(
            69 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(65 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(61 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(53 as libc::c_int as isize);
    *g
        .offset(
            70 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(66 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize) * *g.offset(62 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(54 as libc::c_int as isize);
    *g
        .offset(
            71 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(67 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize) * *g.offset(63 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(55 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0200(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *c0x.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *c0x.offset(3 as libc::c_int as isize);
    *g
        .offset(
            8 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            9 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *c0x.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *c0x.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize);
    *g.offset(12 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(13 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(14 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(15 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(16 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(17 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(18 as libc::c_int as isize) = *c0y.offset(2 as libc::c_int as isize);
    *g.offset(19 as libc::c_int as isize) = *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *c0y.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *c0y.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(26 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(27 as libc::c_int as isize);
    *g
        .offset(
            32 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(28 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            33 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(29 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            34 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(30 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize) * *g.offset(26 as libc::c_int as isize);
    *g
        .offset(
            35 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(31 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize) * *g.offset(27 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0201(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(8 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(9 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(10 as libc::c_int as isize) = *c0x.offset(2 as libc::c_int as isize);
    *g.offset(11 as libc::c_int as isize) = *c0x.offset(3 as libc::c_int as isize);
    *g
        .offset(
            16 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            17 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = *c0x.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = *c0x.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize);
    *g.offset(4 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *cpx.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *cpx.offset(3 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (*g.offset(12 as libc::c_int as isize)
            + *b00.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize)
            * *cpx.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (*g.offset(13 as libc::c_int as isize)
            + *b00.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize)
            * *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *c0x.offset(2 as libc::c_int as isize)
        * (*g.offset(14 as libc::c_int as isize)
            + *b00.offset(2 as libc::c_int as isize))
        + *b10.offset(2 as libc::c_int as isize)
            * *cpx.offset(2 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *c0x.offset(3 as libc::c_int as isize)
        * (*g.offset(15 as libc::c_int as isize)
            + *b00.offset(3 as libc::c_int as isize))
        + *b10.offset(3 as libc::c_int as isize)
            * *cpx.offset(3 as libc::c_int as isize);
    *g.offset(24 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(25 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(26 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(27 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(32 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(33 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(34 as libc::c_int as isize) = *c0y.offset(2 as libc::c_int as isize);
    *g.offset(35 as libc::c_int as isize) = *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            40 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            41 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            42 as libc::c_int as isize,
        ) = *c0y.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            43 as libc::c_int as isize,
        ) = *c0y.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize);
    *g.offset(28 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(29 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(30 as libc::c_int as isize) = *cpy.offset(2 as libc::c_int as isize);
    *g.offset(31 as libc::c_int as isize) = *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            36 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            37 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            38 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            39 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            44 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (*g.offset(36 as libc::c_int as isize)
            + *b00.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize)
            * *cpy.offset(0 as libc::c_int as isize);
    *g
        .offset(
            45 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (*g.offset(37 as libc::c_int as isize)
            + *b00.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize)
            * *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            46 as libc::c_int as isize,
        ) = *c0y.offset(2 as libc::c_int as isize)
        * (*g.offset(38 as libc::c_int as isize)
            + *b00.offset(2 as libc::c_int as isize))
        + *b10.offset(2 as libc::c_int as isize)
            * *cpy.offset(2 as libc::c_int as isize);
    *g
        .offset(
            47 as libc::c_int as isize,
        ) = *c0y.offset(3 as libc::c_int as isize)
        * (*g.offset(39 as libc::c_int as isize)
            + *b00.offset(3 as libc::c_int as isize))
        + *b10.offset(3 as libc::c_int as isize)
            * *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            56 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            57 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            58 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            59 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            64 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(56 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            65 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(57 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            66 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(58 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize) * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            67 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(59 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize) * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            52 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            53 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            54 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            55 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            60 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(56 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            61 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(57 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            62 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(58 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            63 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(59 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            68 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(60 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(52 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(56 as libc::c_int as isize);
    *g
        .offset(
            69 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(61 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(53 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(57 as libc::c_int as isize);
    *g
        .offset(
            70 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(62 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize) * *g.offset(54 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(58 as libc::c_int as isize);
    *g
        .offset(
            71 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(63 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize) * *g.offset(55 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(59 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0210(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *cpx.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *cpx.offset(3 as libc::c_int as isize);
    *g.offset(8 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(9 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(10 as libc::c_int as isize) = *c0x.offset(2 as libc::c_int as isize);
    *g.offset(11 as libc::c_int as isize) = *c0x.offset(3 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            16 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            17 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = *c0x.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = *c0x.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (*g.offset(12 as libc::c_int as isize)
            + *b00.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize)
            * *cpx.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (*g.offset(13 as libc::c_int as isize)
            + *b00.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize)
            * *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *c0x.offset(2 as libc::c_int as isize)
        * (*g.offset(14 as libc::c_int as isize)
            + *b00.offset(2 as libc::c_int as isize))
        + *b10.offset(2 as libc::c_int as isize)
            * *cpx.offset(2 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *c0x.offset(3 as libc::c_int as isize)
        * (*g.offset(15 as libc::c_int as isize)
            + *b00.offset(3 as libc::c_int as isize))
        + *b10.offset(3 as libc::c_int as isize)
            * *cpx.offset(3 as libc::c_int as isize);
    *g.offset(24 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(25 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(26 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(27 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(28 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(29 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(30 as libc::c_int as isize) = *cpy.offset(2 as libc::c_int as isize);
    *g.offset(31 as libc::c_int as isize) = *cpy.offset(3 as libc::c_int as isize);
    *g.offset(32 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(33 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(34 as libc::c_int as isize) = *c0y.offset(2 as libc::c_int as isize);
    *g.offset(35 as libc::c_int as isize) = *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            36 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            37 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            38 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            39 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            40 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            41 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            42 as libc::c_int as isize,
        ) = *c0y.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            43 as libc::c_int as isize,
        ) = *c0y.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize);
    *g
        .offset(
            44 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (*g.offset(36 as libc::c_int as isize)
            + *b00.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize)
            * *cpy.offset(0 as libc::c_int as isize);
    *g
        .offset(
            45 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (*g.offset(37 as libc::c_int as isize)
            + *b00.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize)
            * *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            46 as libc::c_int as isize,
        ) = *c0y.offset(2 as libc::c_int as isize)
        * (*g.offset(38 as libc::c_int as isize)
            + *b00.offset(2 as libc::c_int as isize))
        + *b10.offset(2 as libc::c_int as isize)
            * *cpy.offset(2 as libc::c_int as isize);
    *g
        .offset(
            47 as libc::c_int as isize,
        ) = *c0y.offset(3 as libc::c_int as isize)
        * (*g.offset(39 as libc::c_int as isize)
            + *b00.offset(3 as libc::c_int as isize))
        + *b10.offset(3 as libc::c_int as isize)
            * *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            52 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            53 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            54 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            55 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            56 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            57 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            58 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            59 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            60 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(56 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            61 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(57 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            62 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(58 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            63 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(59 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            64 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(56 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            65 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(57 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            66 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(58 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize) * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            67 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(59 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize) * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            68 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(60 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(52 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(56 as libc::c_int as isize);
    *g
        .offset(
            69 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(61 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(53 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(57 as libc::c_int as isize);
    *g
        .offset(
            70 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(62 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize) * *g.offset(54 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(58 as libc::c_int as isize);
    *g
        .offset(
            71 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(63 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize) * *g.offset(55 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(59 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0300(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *c0x.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *c0x.offset(3 as libc::c_int as isize);
    *g
        .offset(
            8 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            9 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *c0x.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *c0x.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (*g.offset(8 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b10.offset(0 as libc::c_int as isize));
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (*g.offset(9 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b10.offset(1 as libc::c_int as isize));
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *c0x.offset(2 as libc::c_int as isize)
        * (*g.offset(10 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b10.offset(2 as libc::c_int as isize));
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *c0x.offset(3 as libc::c_int as isize)
        * (*g.offset(11 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b10.offset(3 as libc::c_int as isize));
    *g.offset(16 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(17 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(18 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(19 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(20 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(21 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(22 as libc::c_int as isize) = *c0y.offset(2 as libc::c_int as isize);
    *g.offset(23 as libc::c_int as isize) = *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            24 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            25 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = *c0y.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = *c0y.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (*g.offset(24 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b10.offset(0 as libc::c_int as isize));
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (*g.offset(25 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b10.offset(1 as libc::c_int as isize));
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *c0y.offset(2 as libc::c_int as isize)
        * (*g.offset(26 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b10.offset(2 as libc::c_int as isize));
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *c0y.offset(3 as libc::c_int as isize)
        * (*g.offset(27 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b10.offset(3 as libc::c_int as isize));
    *g
        .offset(
            36 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            37 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            38 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(34 as libc::c_int as isize);
    *g
        .offset(
            39 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(35 as libc::c_int as isize);
    *g
        .offset(
            40 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(36 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            41 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(37 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            42 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(38 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize) * *g.offset(34 as libc::c_int as isize);
    *g
        .offset(
            43 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(39 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize) * *g.offset(35 as libc::c_int as isize);
    *g
        .offset(
            44 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(40 as libc::c_int as isize)
        + 2 as libc::c_int as libc::c_double * *b10.offset(0 as libc::c_int as isize)
            * *g.offset(36 as libc::c_int as isize);
    *g
        .offset(
            45 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(41 as libc::c_int as isize)
        + 2 as libc::c_int as libc::c_double * *b10.offset(1 as libc::c_int as isize)
            * *g.offset(37 as libc::c_int as isize);
    *g
        .offset(
            46 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(42 as libc::c_int as isize)
        + 2 as libc::c_int as libc::c_double * *b10.offset(2 as libc::c_int as isize)
            * *g.offset(38 as libc::c_int as isize);
    *g
        .offset(
            47 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(43 as libc::c_int as isize)
        + 2 as libc::c_int as libc::c_double * *b10.offset(3 as libc::c_int as isize)
            * *g.offset(39 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1000(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(3 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(4 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(5 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(6 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(8 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(9 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1001(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *c0x.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *c0x.offset(3 as libc::c_int as isize);
    *g.offset(8 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(9 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(10 as libc::c_int as isize) = *cpx.offset(2 as libc::c_int as isize);
    *g.offset(11 as libc::c_int as isize) = *cpx.offset(3 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g.offset(16 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(17 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(18 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(19 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(20 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(21 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(22 as libc::c_int as isize) = *c0y.offset(2 as libc::c_int as isize);
    *g.offset(23 as libc::c_int as isize) = *c0y.offset(3 as libc::c_int as isize);
    *g.offset(24 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(25 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(26 as libc::c_int as isize) = *cpy.offset(2 as libc::c_int as isize);
    *g.offset(27 as libc::c_int as isize) = *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            36 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            37 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            38 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(34 as libc::c_int as isize);
    *g
        .offset(
            39 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(35 as libc::c_int as isize);
    *g
        .offset(
            40 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            41 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            42 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(34 as libc::c_int as isize);
    *g
        .offset(
            43 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(35 as libc::c_int as isize);
    *g
        .offset(
            44 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(36 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            45 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(37 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            46 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(38 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(34 as libc::c_int as isize);
    *g
        .offset(
            47 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(39 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(35 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1002(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *c0x.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *c0x.offset(3 as libc::c_int as isize);
    *g.offset(8 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(9 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(10 as libc::c_int as isize) = *cpx.offset(2 as libc::c_int as isize);
    *g.offset(11 as libc::c_int as isize) = *cpx.offset(3 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            16 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *cpx.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            17 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *cpx.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *cpx.offset(2 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *cpx.offset(3 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (*g.offset(12 as libc::c_int as isize)
            + *b00.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize)
            * *c0x.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (*g.offset(13 as libc::c_int as isize)
            + *b00.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize)
            * *c0x.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * (*g.offset(14 as libc::c_int as isize)
            + *b00.offset(2 as libc::c_int as isize))
        + *b01.offset(2 as libc::c_int as isize)
            * *c0x.offset(2 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * (*g.offset(15 as libc::c_int as isize)
            + *b00.offset(3 as libc::c_int as isize))
        + *b01.offset(3 as libc::c_int as isize)
            * *c0x.offset(3 as libc::c_int as isize);
    *g.offset(24 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(25 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(26 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(27 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(28 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(29 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(30 as libc::c_int as isize) = *c0y.offset(2 as libc::c_int as isize);
    *g.offset(31 as libc::c_int as isize) = *c0y.offset(3 as libc::c_int as isize);
    *g.offset(32 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(33 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(34 as libc::c_int as isize) = *cpy.offset(2 as libc::c_int as isize);
    *g.offset(35 as libc::c_int as isize) = *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            36 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            37 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            38 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            39 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            40 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *cpy.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            41 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *cpy.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            42 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *cpy.offset(2 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            43 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *cpy.offset(3 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            44 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (*g.offset(36 as libc::c_int as isize)
            + *b00.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize)
            * *c0y.offset(0 as libc::c_int as isize);
    *g
        .offset(
            45 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (*g.offset(37 as libc::c_int as isize)
            + *b00.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize)
            * *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            46 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * (*g.offset(38 as libc::c_int as isize)
            + *b00.offset(2 as libc::c_int as isize))
        + *b01.offset(2 as libc::c_int as isize)
            * *c0y.offset(2 as libc::c_int as isize);
    *g
        .offset(
            47 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * (*g.offset(39 as libc::c_int as isize)
            + *b00.offset(3 as libc::c_int as isize))
        + *b01.offset(3 as libc::c_int as isize)
            * *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            52 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            53 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            54 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            55 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            56 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            57 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            58 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            59 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            60 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(52 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            61 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(53 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            62 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(54 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            63 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(55 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            64 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(56 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            65 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(57 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            66 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(58 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize) * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            67 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(59 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize) * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            68 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(60 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(52 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(56 as libc::c_int as isize);
    *g
        .offset(
            69 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(61 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(53 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(57 as libc::c_int as isize);
    *g
        .offset(
            70 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(62 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize) * *g.offset(54 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(58 as libc::c_int as isize);
    *g
        .offset(
            71 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(63 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize) * *g.offset(55 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(59 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1010(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *c0x.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *c0x.offset(3 as libc::c_int as isize);
    *g.offset(8 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(9 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(10 as libc::c_int as isize) = *cpx.offset(2 as libc::c_int as isize);
    *g.offset(11 as libc::c_int as isize) = *cpx.offset(3 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g.offset(16 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(17 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(18 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(19 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(20 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(21 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(22 as libc::c_int as isize) = *c0y.offset(2 as libc::c_int as isize);
    *g.offset(23 as libc::c_int as isize) = *c0y.offset(3 as libc::c_int as isize);
    *g.offset(24 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(25 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(26 as libc::c_int as isize) = *cpy.offset(2 as libc::c_int as isize);
    *g.offset(27 as libc::c_int as isize) = *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            36 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            37 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            38 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(34 as libc::c_int as isize);
    *g
        .offset(
            39 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(35 as libc::c_int as isize);
    *g
        .offset(
            40 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            41 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            42 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(34 as libc::c_int as isize);
    *g
        .offset(
            43 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(35 as libc::c_int as isize);
    *g
        .offset(
            44 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(36 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            45 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(37 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            46 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(38 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(34 as libc::c_int as isize);
    *g
        .offset(
            47 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(39 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(35 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1011(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    let mut xkxl: libc::c_double = (*envs).rkrl[0 as libc::c_int as usize];
    let mut ykyl: libc::c_double = (*envs).rkrl[1 as libc::c_int as usize];
    let mut zkzl: libc::c_double = (*envs).rkrl[2 as libc::c_int as usize];
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *c0x.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *c0x.offset(3 as libc::c_int as isize);
    *g.offset(16 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(17 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(18 as libc::c_int as isize) = *cpx.offset(2 as libc::c_int as isize);
    *g.offset(19 as libc::c_int as isize) = *cpx.offset(3 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            24 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (xkxl + *cpx.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            25 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (xkxl + *cpx.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * (xkxl + *cpx.offset(2 as libc::c_int as isize))
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * (xkxl + *cpx.offset(3 as libc::c_int as isize))
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *g.offset(20 as libc::c_int as isize)
        * (xkxl + *cpx.offset(0 as libc::c_int as isize))
        + *cpx.offset(0 as libc::c_int as isize) * *b00.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize)
            * *c0x.offset(0 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *g.offset(21 as libc::c_int as isize)
        * (xkxl + *cpx.offset(1 as libc::c_int as isize))
        + *cpx.offset(1 as libc::c_int as isize) * *b00.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize)
            * *c0x.offset(1 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *g.offset(22 as libc::c_int as isize)
        * (xkxl + *cpx.offset(2 as libc::c_int as isize))
        + *cpx.offset(2 as libc::c_int as isize) * *b00.offset(2 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize)
            * *c0x.offset(2 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *g.offset(23 as libc::c_int as isize)
        * (xkxl + *cpx.offset(3 as libc::c_int as isize))
        + *cpx.offset(3 as libc::c_int as isize) * *b00.offset(3 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize)
            * *c0x.offset(3 as libc::c_int as isize);
    *g.offset(8 as libc::c_int as isize) = xkxl + *cpx.offset(0 as libc::c_int as isize);
    *g.offset(9 as libc::c_int as isize) = xkxl + *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = xkxl + *cpx.offset(2 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = xkxl + *cpx.offset(3 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (xkxl + *cpx.offset(0 as libc::c_int as isize))
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (xkxl + *cpx.offset(1 as libc::c_int as isize))
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *c0x.offset(2 as libc::c_int as isize)
        * (xkxl + *cpx.offset(2 as libc::c_int as isize))
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *c0x.offset(3 as libc::c_int as isize)
        * (xkxl + *cpx.offset(3 as libc::c_int as isize))
        + *b00.offset(3 as libc::c_int as isize);
    *g.offset(48 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(49 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(50 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(51 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(52 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(53 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(54 as libc::c_int as isize) = *c0y.offset(2 as libc::c_int as isize);
    *g.offset(55 as libc::c_int as isize) = *c0y.offset(3 as libc::c_int as isize);
    *g.offset(64 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(65 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(66 as libc::c_int as isize) = *cpy.offset(2 as libc::c_int as isize);
    *g.offset(67 as libc::c_int as isize) = *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            68 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            69 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            70 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            71 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            72 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (ykyl + *cpy.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            73 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (ykyl + *cpy.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            74 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * (ykyl + *cpy.offset(2 as libc::c_int as isize))
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            75 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * (ykyl + *cpy.offset(3 as libc::c_int as isize))
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            76 as libc::c_int as isize,
        ) = *g.offset(68 as libc::c_int as isize)
        * (ykyl + *cpy.offset(0 as libc::c_int as isize))
        + *cpy.offset(0 as libc::c_int as isize) * *b00.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize)
            * *c0y.offset(0 as libc::c_int as isize);
    *g
        .offset(
            77 as libc::c_int as isize,
        ) = *g.offset(69 as libc::c_int as isize)
        * (ykyl + *cpy.offset(1 as libc::c_int as isize))
        + *cpy.offset(1 as libc::c_int as isize) * *b00.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize)
            * *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            78 as libc::c_int as isize,
        ) = *g.offset(70 as libc::c_int as isize)
        * (ykyl + *cpy.offset(2 as libc::c_int as isize))
        + *cpy.offset(2 as libc::c_int as isize) * *b00.offset(2 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize)
            * *c0y.offset(2 as libc::c_int as isize);
    *g
        .offset(
            79 as libc::c_int as isize,
        ) = *g.offset(71 as libc::c_int as isize)
        * (ykyl + *cpy.offset(3 as libc::c_int as isize))
        + *cpy.offset(3 as libc::c_int as isize) * *b00.offset(3 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize)
            * *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            56 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(0 as libc::c_int as isize);
    *g
        .offset(
            57 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            58 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(2 as libc::c_int as isize);
    *g
        .offset(
            59 as libc::c_int as isize,
        ) = ykyl + *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            60 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (ykyl + *cpy.offset(0 as libc::c_int as isize))
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            61 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (ykyl + *cpy.offset(1 as libc::c_int as isize))
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            62 as libc::c_int as isize,
        ) = *c0y.offset(2 as libc::c_int as isize)
        * (ykyl + *cpy.offset(2 as libc::c_int as isize))
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            63 as libc::c_int as isize,
        ) = *c0y.offset(3 as libc::c_int as isize)
        * (ykyl + *cpy.offset(3 as libc::c_int as isize))
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            100 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(96 as libc::c_int as isize);
    *g
        .offset(
            101 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(97 as libc::c_int as isize);
    *g
        .offset(
            102 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(98 as libc::c_int as isize);
    *g
        .offset(
            103 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(99 as libc::c_int as isize);
    *g
        .offset(
            112 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(96 as libc::c_int as isize);
    *g
        .offset(
            113 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(97 as libc::c_int as isize);
    *g
        .offset(
            114 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(98 as libc::c_int as isize);
    *g
        .offset(
            115 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(99 as libc::c_int as isize);
    *g
        .offset(
            116 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(100 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(96 as libc::c_int as isize);
    *g
        .offset(
            117 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(101 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(97 as libc::c_int as isize);
    *g
        .offset(
            118 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(102 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(98 as libc::c_int as isize);
    *g
        .offset(
            119 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(103 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(99 as libc::c_int as isize);
    *g
        .offset(
            120 as libc::c_int as isize,
        ) = *g.offset(112 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(96 as libc::c_int as isize);
    *g
        .offset(
            121 as libc::c_int as isize,
        ) = *g.offset(113 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(97 as libc::c_int as isize);
    *g
        .offset(
            122 as libc::c_int as isize,
        ) = *g.offset(114 as libc::c_int as isize)
        * (zkzl + *cpz.offset(2 as libc::c_int as isize))
        + *b01.offset(2 as libc::c_int as isize) * *g.offset(98 as libc::c_int as isize);
    *g
        .offset(
            123 as libc::c_int as isize,
        ) = *g.offset(115 as libc::c_int as isize)
        * (zkzl + *cpz.offset(3 as libc::c_int as isize))
        + *b01.offset(3 as libc::c_int as isize) * *g.offset(99 as libc::c_int as isize);
    *g
        .offset(
            124 as libc::c_int as isize,
        ) = *g.offset(116 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(100 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize)
            * *g.offset(112 as libc::c_int as isize);
    *g
        .offset(
            125 as libc::c_int as isize,
        ) = *g.offset(117 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(101 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize)
            * *g.offset(113 as libc::c_int as isize);
    *g
        .offset(
            126 as libc::c_int as isize,
        ) = *g.offset(118 as libc::c_int as isize)
        * (zkzl + *cpz.offset(2 as libc::c_int as isize))
        + *b01.offset(2 as libc::c_int as isize) * *g.offset(102 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize)
            * *g.offset(114 as libc::c_int as isize);
    *g
        .offset(
            127 as libc::c_int as isize,
        ) = *g.offset(119 as libc::c_int as isize)
        * (zkzl + *cpz.offset(3 as libc::c_int as isize))
        + *b01.offset(3 as libc::c_int as isize) * *g.offset(103 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize)
            * *g.offset(115 as libc::c_int as isize);
    *g
        .offset(
            104 as libc::c_int as isize,
        ) = *g.offset(96 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize));
    *g
        .offset(
            105 as libc::c_int as isize,
        ) = *g.offset(97 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize));
    *g
        .offset(
            106 as libc::c_int as isize,
        ) = *g.offset(98 as libc::c_int as isize)
        * (zkzl + *cpz.offset(2 as libc::c_int as isize));
    *g
        .offset(
            107 as libc::c_int as isize,
        ) = *g.offset(99 as libc::c_int as isize)
        * (zkzl + *cpz.offset(3 as libc::c_int as isize));
    *g
        .offset(
            108 as libc::c_int as isize,
        ) = *g.offset(100 as libc::c_int as isize)
        * (zkzl + *cpz.offset(0 as libc::c_int as isize))
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(96 as libc::c_int as isize);
    *g
        .offset(
            109 as libc::c_int as isize,
        ) = *g.offset(101 as libc::c_int as isize)
        * (zkzl + *cpz.offset(1 as libc::c_int as isize))
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(97 as libc::c_int as isize);
    *g
        .offset(
            110 as libc::c_int as isize,
        ) = *g.offset(102 as libc::c_int as isize)
        * (zkzl + *cpz.offset(2 as libc::c_int as isize))
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(98 as libc::c_int as isize);
    *g
        .offset(
            111 as libc::c_int as isize,
        ) = *g.offset(103 as libc::c_int as isize)
        * (zkzl + *cpz.offset(3 as libc::c_int as isize))
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(99 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1020(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    let mut b01: *mut libc::c_double = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *c0x.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *c0x.offset(3 as libc::c_int as isize);
    *g.offset(8 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(9 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(10 as libc::c_int as isize) = *cpx.offset(2 as libc::c_int as isize);
    *g.offset(11 as libc::c_int as isize) = *cpx.offset(3 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            16 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *cpx.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            17 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *cpx.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *cpx.offset(2 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *cpx.offset(3 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (*g.offset(12 as libc::c_int as isize)
            + *b00.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize)
            * *c0x.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (*g.offset(13 as libc::c_int as isize)
            + *b00.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize)
            * *c0x.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * (*g.offset(14 as libc::c_int as isize)
            + *b00.offset(2 as libc::c_int as isize))
        + *b01.offset(2 as libc::c_int as isize)
            * *c0x.offset(2 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * (*g.offset(15 as libc::c_int as isize)
            + *b00.offset(3 as libc::c_int as isize))
        + *b01.offset(3 as libc::c_int as isize)
            * *c0x.offset(3 as libc::c_int as isize);
    *g.offset(24 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(25 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(26 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(27 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(28 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(29 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(30 as libc::c_int as isize) = *c0y.offset(2 as libc::c_int as isize);
    *g.offset(31 as libc::c_int as isize) = *c0y.offset(3 as libc::c_int as isize);
    *g.offset(32 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(33 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(34 as libc::c_int as isize) = *cpy.offset(2 as libc::c_int as isize);
    *g.offset(35 as libc::c_int as isize) = *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            36 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            37 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            38 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            39 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            40 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *cpy.offset(0 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize);
    *g
        .offset(
            41 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *cpy.offset(1 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize);
    *g
        .offset(
            42 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *cpy.offset(2 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize);
    *g
        .offset(
            43 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *cpy.offset(3 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize);
    *g
        .offset(
            44 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (*g.offset(36 as libc::c_int as isize)
            + *b00.offset(0 as libc::c_int as isize))
        + *b01.offset(0 as libc::c_int as isize)
            * *c0y.offset(0 as libc::c_int as isize);
    *g
        .offset(
            45 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (*g.offset(37 as libc::c_int as isize)
            + *b00.offset(1 as libc::c_int as isize))
        + *b01.offset(1 as libc::c_int as isize)
            * *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            46 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * (*g.offset(38 as libc::c_int as isize)
            + *b00.offset(2 as libc::c_int as isize))
        + *b01.offset(2 as libc::c_int as isize)
            * *c0y.offset(2 as libc::c_int as isize);
    *g
        .offset(
            47 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * (*g.offset(39 as libc::c_int as isize)
            + *b00.offset(3 as libc::c_int as isize))
        + *b01.offset(3 as libc::c_int as isize)
            * *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            52 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            53 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            54 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            55 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            56 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            57 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            58 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            59 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            60 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(52 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            61 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(53 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            62 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(54 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            63 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(55 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            64 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(56 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            65 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(57 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            66 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(58 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize) * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            67 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(59 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize) * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            68 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(60 as libc::c_int as isize)
        + *b01.offset(0 as libc::c_int as isize) * *g.offset(52 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(56 as libc::c_int as isize);
    *g
        .offset(
            69 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(61 as libc::c_int as isize)
        + *b01.offset(1 as libc::c_int as isize) * *g.offset(53 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(57 as libc::c_int as isize);
    *g
        .offset(
            70 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(62 as libc::c_int as isize)
        + *b01.offset(2 as libc::c_int as isize) * *g.offset(54 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(58 as libc::c_int as isize);
    *g
        .offset(
            71 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(63 as libc::c_int as isize)
        + *b01.offset(3 as libc::c_int as isize) * *g.offset(55 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(59 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1100(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    let mut xixj: libc::c_double = (*envs).rirj[0 as libc::c_int as usize];
    let mut yiyj: libc::c_double = (*envs).rirj[1 as libc::c_int as usize];
    let mut zizj: libc::c_double = (*envs).rirj[2 as libc::c_int as usize];
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(8 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(9 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(10 as libc::c_int as isize) = *c0x.offset(2 as libc::c_int as isize);
    *g.offset(11 as libc::c_int as isize) = *c0x.offset(3 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (xixj + *c0x.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (xixj + *c0x.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *c0x.offset(2 as libc::c_int as isize)
        * (xixj + *c0x.offset(2 as libc::c_int as isize))
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *c0x.offset(3 as libc::c_int as isize)
        * (xixj + *c0x.offset(3 as libc::c_int as isize))
        + *b10.offset(3 as libc::c_int as isize);
    *g.offset(4 as libc::c_int as isize) = xixj + *c0x.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = xixj + *c0x.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = xixj + *c0x.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = xixj + *c0x.offset(3 as libc::c_int as isize);
    *g.offset(24 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(25 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(26 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(27 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(32 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(33 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(34 as libc::c_int as isize) = *c0y.offset(2 as libc::c_int as isize);
    *g.offset(35 as libc::c_int as isize) = *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            36 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (yiyj + *c0y.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            37 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (yiyj + *c0y.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            38 as libc::c_int as isize,
        ) = *c0y.offset(2 as libc::c_int as isize)
        * (yiyj + *c0y.offset(2 as libc::c_int as isize))
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            39 as libc::c_int as isize,
        ) = *c0y.offset(3 as libc::c_int as isize)
        * (yiyj + *c0y.offset(3 as libc::c_int as isize))
        + *b10.offset(3 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(0 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(2 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            56 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            57 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            58 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            59 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            60 as libc::c_int as isize,
        ) = *g.offset(56 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            61 as libc::c_int as isize,
        ) = *g.offset(57 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            62 as libc::c_int as isize,
        ) = *g.offset(58 as libc::c_int as isize)
        * (zizj + *c0z.offset(2 as libc::c_int as isize))
        + *b10.offset(2 as libc::c_int as isize) * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            63 as libc::c_int as isize,
        ) = *g.offset(59 as libc::c_int as isize)
        * (zizj + *c0z.offset(3 as libc::c_int as isize))
        + *b10.offset(3 as libc::c_int as isize) * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            52 as libc::c_int as isize,
        ) = *g.offset(48 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize));
    *g
        .offset(
            53 as libc::c_int as isize,
        ) = *g.offset(49 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize));
    *g
        .offset(
            54 as libc::c_int as isize,
        ) = *g.offset(50 as libc::c_int as isize)
        * (zizj + *c0z.offset(2 as libc::c_int as isize));
    *g
        .offset(
            55 as libc::c_int as isize,
        ) = *g.offset(51 as libc::c_int as isize)
        * (zizj + *c0z.offset(3 as libc::c_int as isize));
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1101(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    let mut xixj: libc::c_double = (*envs).rirj[0 as libc::c_int as usize];
    let mut yiyj: libc::c_double = (*envs).rirj[1 as libc::c_int as usize];
    let mut zizj: libc::c_double = (*envs).rirj[2 as libc::c_int as usize];
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(16 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(17 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(18 as libc::c_int as isize) = *c0x.offset(2 as libc::c_int as isize);
    *g.offset(19 as libc::c_int as isize) = *c0x.offset(3 as libc::c_int as isize);
    *g.offset(8 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(9 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(10 as libc::c_int as isize) = *cpx.offset(2 as libc::c_int as isize);
    *g.offset(11 as libc::c_int as isize) = *cpx.offset(3 as libc::c_int as isize);
    *g
        .offset(
            24 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            25 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (xixj + *c0x.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (xixj + *c0x.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *c0x.offset(2 as libc::c_int as isize)
        * (xixj + *c0x.offset(2 as libc::c_int as isize))
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *c0x.offset(3 as libc::c_int as isize)
        * (xixj + *c0x.offset(3 as libc::c_int as isize))
        + *b10.offset(3 as libc::c_int as isize);
    *g.offset(4 as libc::c_int as isize) = xixj + *c0x.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = xixj + *c0x.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = xixj + *c0x.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = xixj + *c0x.offset(3 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *g.offset(24 as libc::c_int as isize)
        * (xixj + *c0x.offset(0 as libc::c_int as isize))
        + *c0x.offset(0 as libc::c_int as isize) * *b00.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize)
            * *cpx.offset(0 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *g.offset(25 as libc::c_int as isize)
        * (xixj + *c0x.offset(1 as libc::c_int as isize))
        + *c0x.offset(1 as libc::c_int as isize) * *b00.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize)
            * *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *g.offset(26 as libc::c_int as isize)
        * (xixj + *c0x.offset(2 as libc::c_int as isize))
        + *c0x.offset(2 as libc::c_int as isize) * *b00.offset(2 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize)
            * *cpx.offset(2 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *g.offset(27 as libc::c_int as isize)
        * (xixj + *c0x.offset(3 as libc::c_int as isize))
        + *c0x.offset(3 as libc::c_int as isize) * *b00.offset(3 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize)
            * *cpx.offset(3 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (xixj + *c0x.offset(0 as libc::c_int as isize))
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (xixj + *c0x.offset(1 as libc::c_int as isize))
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * (xixj + *c0x.offset(2 as libc::c_int as isize))
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * (xixj + *c0x.offset(3 as libc::c_int as isize))
        + *b00.offset(3 as libc::c_int as isize);
    *g.offset(48 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(49 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(50 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(51 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(64 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(65 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(66 as libc::c_int as isize) = *c0y.offset(2 as libc::c_int as isize);
    *g.offset(67 as libc::c_int as isize) = *c0y.offset(3 as libc::c_int as isize);
    *g.offset(56 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(57 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(58 as libc::c_int as isize) = *cpy.offset(2 as libc::c_int as isize);
    *g.offset(59 as libc::c_int as isize) = *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            72 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            73 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            74 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            75 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            68 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (yiyj + *c0y.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            69 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (yiyj + *c0y.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            70 as libc::c_int as isize,
        ) = *c0y.offset(2 as libc::c_int as isize)
        * (yiyj + *c0y.offset(2 as libc::c_int as isize))
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            71 as libc::c_int as isize,
        ) = *c0y.offset(3 as libc::c_int as isize)
        * (yiyj + *c0y.offset(3 as libc::c_int as isize))
        + *b10.offset(3 as libc::c_int as isize);
    *g
        .offset(
            52 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(0 as libc::c_int as isize);
    *g
        .offset(
            53 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            54 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(2 as libc::c_int as isize);
    *g
        .offset(
            55 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            76 as libc::c_int as isize,
        ) = *g.offset(72 as libc::c_int as isize)
        * (yiyj + *c0y.offset(0 as libc::c_int as isize))
        + *c0y.offset(0 as libc::c_int as isize) * *b00.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize)
            * *cpy.offset(0 as libc::c_int as isize);
    *g
        .offset(
            77 as libc::c_int as isize,
        ) = *g.offset(73 as libc::c_int as isize)
        * (yiyj + *c0y.offset(1 as libc::c_int as isize))
        + *c0y.offset(1 as libc::c_int as isize) * *b00.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize)
            * *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            78 as libc::c_int as isize,
        ) = *g.offset(74 as libc::c_int as isize)
        * (yiyj + *c0y.offset(2 as libc::c_int as isize))
        + *c0y.offset(2 as libc::c_int as isize) * *b00.offset(2 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize)
            * *cpy.offset(2 as libc::c_int as isize);
    *g
        .offset(
            79 as libc::c_int as isize,
        ) = *g.offset(75 as libc::c_int as isize)
        * (yiyj + *c0y.offset(3 as libc::c_int as isize))
        + *c0y.offset(3 as libc::c_int as isize) * *b00.offset(3 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize)
            * *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            60 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (yiyj + *c0y.offset(0 as libc::c_int as isize))
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            61 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (yiyj + *c0y.offset(1 as libc::c_int as isize))
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            62 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * (yiyj + *c0y.offset(2 as libc::c_int as isize))
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            63 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * (yiyj + *c0y.offset(3 as libc::c_int as isize))
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            112 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(96 as libc::c_int as isize);
    *g
        .offset(
            113 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(97 as libc::c_int as isize);
    *g
        .offset(
            114 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(98 as libc::c_int as isize);
    *g
        .offset(
            115 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(99 as libc::c_int as isize);
    *g
        .offset(
            104 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(96 as libc::c_int as isize);
    *g
        .offset(
            105 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(97 as libc::c_int as isize);
    *g
        .offset(
            106 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(98 as libc::c_int as isize);
    *g
        .offset(
            107 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(99 as libc::c_int as isize);
    *g
        .offset(
            120 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(112 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(96 as libc::c_int as isize);
    *g
        .offset(
            121 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(113 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(97 as libc::c_int as isize);
    *g
        .offset(
            122 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(114 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(98 as libc::c_int as isize);
    *g
        .offset(
            123 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(115 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(99 as libc::c_int as isize);
    *g
        .offset(
            116 as libc::c_int as isize,
        ) = *g.offset(112 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(96 as libc::c_int as isize);
    *g
        .offset(
            117 as libc::c_int as isize,
        ) = *g.offset(113 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(97 as libc::c_int as isize);
    *g
        .offset(
            118 as libc::c_int as isize,
        ) = *g.offset(114 as libc::c_int as isize)
        * (zizj + *c0z.offset(2 as libc::c_int as isize))
        + *b10.offset(2 as libc::c_int as isize) * *g.offset(98 as libc::c_int as isize);
    *g
        .offset(
            119 as libc::c_int as isize,
        ) = *g.offset(115 as libc::c_int as isize)
        * (zizj + *c0z.offset(3 as libc::c_int as isize))
        + *b10.offset(3 as libc::c_int as isize) * *g.offset(99 as libc::c_int as isize);
    *g
        .offset(
            100 as libc::c_int as isize,
        ) = *g.offset(96 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize));
    *g
        .offset(
            101 as libc::c_int as isize,
        ) = *g.offset(97 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize));
    *g
        .offset(
            102 as libc::c_int as isize,
        ) = *g.offset(98 as libc::c_int as isize)
        * (zizj + *c0z.offset(2 as libc::c_int as isize));
    *g
        .offset(
            103 as libc::c_int as isize,
        ) = *g.offset(99 as libc::c_int as isize)
        * (zizj + *c0z.offset(3 as libc::c_int as isize));
    *g
        .offset(
            124 as libc::c_int as isize,
        ) = *g.offset(120 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(104 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize)
            * *g.offset(112 as libc::c_int as isize);
    *g
        .offset(
            125 as libc::c_int as isize,
        ) = *g.offset(121 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(105 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize)
            * *g.offset(113 as libc::c_int as isize);
    *g
        .offset(
            126 as libc::c_int as isize,
        ) = *g.offset(122 as libc::c_int as isize)
        * (zizj + *c0z.offset(2 as libc::c_int as isize))
        + *b10.offset(2 as libc::c_int as isize) * *g.offset(106 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize)
            * *g.offset(114 as libc::c_int as isize);
    *g
        .offset(
            127 as libc::c_int as isize,
        ) = *g.offset(123 as libc::c_int as isize)
        * (zizj + *c0z.offset(3 as libc::c_int as isize))
        + *b10.offset(3 as libc::c_int as isize) * *g.offset(107 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize)
            * *g.offset(115 as libc::c_int as isize);
    *g
        .offset(
            108 as libc::c_int as isize,
        ) = zizj * *g.offset(104 as libc::c_int as isize)
        + *cpz.offset(0 as libc::c_int as isize) * *g.offset(112 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(96 as libc::c_int as isize);
    *g
        .offset(
            109 as libc::c_int as isize,
        ) = zizj * *g.offset(105 as libc::c_int as isize)
        + *cpz.offset(1 as libc::c_int as isize) * *g.offset(113 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(97 as libc::c_int as isize);
    *g
        .offset(
            110 as libc::c_int as isize,
        ) = zizj * *g.offset(106 as libc::c_int as isize)
        + *cpz.offset(2 as libc::c_int as isize) * *g.offset(114 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(98 as libc::c_int as isize);
    *g
        .offset(
            111 as libc::c_int as isize,
        ) = zizj * *g.offset(107 as libc::c_int as isize)
        + *cpz.offset(3 as libc::c_int as isize) * *g.offset(115 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(99 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1110(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    let mut xixj: libc::c_double = (*envs).rirj[0 as libc::c_int as usize];
    let mut yiyj: libc::c_double = (*envs).rirj[1 as libc::c_int as usize];
    let mut zizj: libc::c_double = (*envs).rirj[2 as libc::c_int as usize];
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(16 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(17 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(18 as libc::c_int as isize) = *c0x.offset(2 as libc::c_int as isize);
    *g.offset(19 as libc::c_int as isize) = *c0x.offset(3 as libc::c_int as isize);
    *g.offset(8 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(9 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(10 as libc::c_int as isize) = *cpx.offset(2 as libc::c_int as isize);
    *g.offset(11 as libc::c_int as isize) = *cpx.offset(3 as libc::c_int as isize);
    *g
        .offset(
            24 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            25 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (xixj + *c0x.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (xixj + *c0x.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *c0x.offset(2 as libc::c_int as isize)
        * (xixj + *c0x.offset(2 as libc::c_int as isize))
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *c0x.offset(3 as libc::c_int as isize)
        * (xixj + *c0x.offset(3 as libc::c_int as isize))
        + *b10.offset(3 as libc::c_int as isize);
    *g.offset(4 as libc::c_int as isize) = xixj + *c0x.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = xixj + *c0x.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = xixj + *c0x.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = xixj + *c0x.offset(3 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *g.offset(24 as libc::c_int as isize)
        * (xixj + *c0x.offset(0 as libc::c_int as isize))
        + *c0x.offset(0 as libc::c_int as isize) * *b00.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize)
            * *cpx.offset(0 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *g.offset(25 as libc::c_int as isize)
        * (xixj + *c0x.offset(1 as libc::c_int as isize))
        + *c0x.offset(1 as libc::c_int as isize) * *b00.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize)
            * *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *g.offset(26 as libc::c_int as isize)
        * (xixj + *c0x.offset(2 as libc::c_int as isize))
        + *c0x.offset(2 as libc::c_int as isize) * *b00.offset(2 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize)
            * *cpx.offset(2 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *g.offset(27 as libc::c_int as isize)
        * (xixj + *c0x.offset(3 as libc::c_int as isize))
        + *c0x.offset(3 as libc::c_int as isize) * *b00.offset(3 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize)
            * *cpx.offset(3 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * (xixj + *c0x.offset(0 as libc::c_int as isize))
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * (xixj + *c0x.offset(1 as libc::c_int as isize))
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * (xixj + *c0x.offset(2 as libc::c_int as isize))
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * (xixj + *c0x.offset(3 as libc::c_int as isize))
        + *b00.offset(3 as libc::c_int as isize);
    *g.offset(48 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(49 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(50 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(51 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(64 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(65 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(66 as libc::c_int as isize) = *c0y.offset(2 as libc::c_int as isize);
    *g.offset(67 as libc::c_int as isize) = *c0y.offset(3 as libc::c_int as isize);
    *g.offset(56 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(57 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(58 as libc::c_int as isize) = *cpy.offset(2 as libc::c_int as isize);
    *g.offset(59 as libc::c_int as isize) = *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            72 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            73 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            74 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            75 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            68 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (yiyj + *c0y.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            69 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (yiyj + *c0y.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            70 as libc::c_int as isize,
        ) = *c0y.offset(2 as libc::c_int as isize)
        * (yiyj + *c0y.offset(2 as libc::c_int as isize))
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            71 as libc::c_int as isize,
        ) = *c0y.offset(3 as libc::c_int as isize)
        * (yiyj + *c0y.offset(3 as libc::c_int as isize))
        + *b10.offset(3 as libc::c_int as isize);
    *g
        .offset(
            52 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(0 as libc::c_int as isize);
    *g
        .offset(
            53 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            54 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(2 as libc::c_int as isize);
    *g
        .offset(
            55 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            76 as libc::c_int as isize,
        ) = *g.offset(72 as libc::c_int as isize)
        * (yiyj + *c0y.offset(0 as libc::c_int as isize))
        + *c0y.offset(0 as libc::c_int as isize) * *b00.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize)
            * *cpy.offset(0 as libc::c_int as isize);
    *g
        .offset(
            77 as libc::c_int as isize,
        ) = *g.offset(73 as libc::c_int as isize)
        * (yiyj + *c0y.offset(1 as libc::c_int as isize))
        + *c0y.offset(1 as libc::c_int as isize) * *b00.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize)
            * *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            78 as libc::c_int as isize,
        ) = *g.offset(74 as libc::c_int as isize)
        * (yiyj + *c0y.offset(2 as libc::c_int as isize))
        + *c0y.offset(2 as libc::c_int as isize) * *b00.offset(2 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize)
            * *cpy.offset(2 as libc::c_int as isize);
    *g
        .offset(
            79 as libc::c_int as isize,
        ) = *g.offset(75 as libc::c_int as isize)
        * (yiyj + *c0y.offset(3 as libc::c_int as isize))
        + *c0y.offset(3 as libc::c_int as isize) * *b00.offset(3 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize)
            * *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            60 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * (yiyj + *c0y.offset(0 as libc::c_int as isize))
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            61 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * (yiyj + *c0y.offset(1 as libc::c_int as isize))
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            62 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * (yiyj + *c0y.offset(2 as libc::c_int as isize))
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            63 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * (yiyj + *c0y.offset(3 as libc::c_int as isize))
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            112 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(96 as libc::c_int as isize);
    *g
        .offset(
            113 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(97 as libc::c_int as isize);
    *g
        .offset(
            114 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(98 as libc::c_int as isize);
    *g
        .offset(
            115 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(99 as libc::c_int as isize);
    *g
        .offset(
            104 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(96 as libc::c_int as isize);
    *g
        .offset(
            105 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(97 as libc::c_int as isize);
    *g
        .offset(
            106 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(98 as libc::c_int as isize);
    *g
        .offset(
            107 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(99 as libc::c_int as isize);
    *g
        .offset(
            120 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(112 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(96 as libc::c_int as isize);
    *g
        .offset(
            121 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(113 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(97 as libc::c_int as isize);
    *g
        .offset(
            122 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(114 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(98 as libc::c_int as isize);
    *g
        .offset(
            123 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(115 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(99 as libc::c_int as isize);
    *g
        .offset(
            116 as libc::c_int as isize,
        ) = *g.offset(112 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(96 as libc::c_int as isize);
    *g
        .offset(
            117 as libc::c_int as isize,
        ) = *g.offset(113 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(97 as libc::c_int as isize);
    *g
        .offset(
            118 as libc::c_int as isize,
        ) = *g.offset(114 as libc::c_int as isize)
        * (zizj + *c0z.offset(2 as libc::c_int as isize))
        + *b10.offset(2 as libc::c_int as isize) * *g.offset(98 as libc::c_int as isize);
    *g
        .offset(
            119 as libc::c_int as isize,
        ) = *g.offset(115 as libc::c_int as isize)
        * (zizj + *c0z.offset(3 as libc::c_int as isize))
        + *b10.offset(3 as libc::c_int as isize) * *g.offset(99 as libc::c_int as isize);
    *g
        .offset(
            100 as libc::c_int as isize,
        ) = *g.offset(96 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize));
    *g
        .offset(
            101 as libc::c_int as isize,
        ) = *g.offset(97 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize));
    *g
        .offset(
            102 as libc::c_int as isize,
        ) = *g.offset(98 as libc::c_int as isize)
        * (zizj + *c0z.offset(2 as libc::c_int as isize));
    *g
        .offset(
            103 as libc::c_int as isize,
        ) = *g.offset(99 as libc::c_int as isize)
        * (zizj + *c0z.offset(3 as libc::c_int as isize));
    *g
        .offset(
            124 as libc::c_int as isize,
        ) = *g.offset(120 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(104 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize)
            * *g.offset(112 as libc::c_int as isize);
    *g
        .offset(
            125 as libc::c_int as isize,
        ) = *g.offset(121 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(105 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize)
            * *g.offset(113 as libc::c_int as isize);
    *g
        .offset(
            126 as libc::c_int as isize,
        ) = *g.offset(122 as libc::c_int as isize)
        * (zizj + *c0z.offset(2 as libc::c_int as isize))
        + *b10.offset(2 as libc::c_int as isize) * *g.offset(106 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize)
            * *g.offset(114 as libc::c_int as isize);
    *g
        .offset(
            127 as libc::c_int as isize,
        ) = *g.offset(123 as libc::c_int as isize)
        * (zizj + *c0z.offset(3 as libc::c_int as isize))
        + *b10.offset(3 as libc::c_int as isize) * *g.offset(107 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize)
            * *g.offset(115 as libc::c_int as isize);
    *g
        .offset(
            108 as libc::c_int as isize,
        ) = zizj * *g.offset(104 as libc::c_int as isize)
        + *cpz.offset(0 as libc::c_int as isize) * *g.offset(112 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(96 as libc::c_int as isize);
    *g
        .offset(
            109 as libc::c_int as isize,
        ) = zizj * *g.offset(105 as libc::c_int as isize)
        + *cpz.offset(1 as libc::c_int as isize) * *g.offset(113 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(97 as libc::c_int as isize);
    *g
        .offset(
            110 as libc::c_int as isize,
        ) = zizj * *g.offset(106 as libc::c_int as isize)
        + *cpz.offset(2 as libc::c_int as isize) * *g.offset(114 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(98 as libc::c_int as isize);
    *g
        .offset(
            111 as libc::c_int as isize,
        ) = zizj * *g.offset(107 as libc::c_int as isize)
        + *cpz.offset(3 as libc::c_int as isize) * *g.offset(115 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(99 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1200(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    let mut xixj: libc::c_double = (*envs).rirj[0 as libc::c_int as usize];
    let mut yiyj: libc::c_double = (*envs).rirj[1 as libc::c_int as usize];
    let mut zizj: libc::c_double = (*envs).rirj[2 as libc::c_int as usize];
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(8 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(9 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(10 as libc::c_int as isize) = *c0x.offset(2 as libc::c_int as isize);
    *g.offset(11 as libc::c_int as isize) = *c0x.offset(3 as libc::c_int as isize);
    *g
        .offset(
            16 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            17 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = *c0x.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = *c0x.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *g.offset(16 as libc::c_int as isize)
        * (xixj + *c0x.offset(0 as libc::c_int as isize))
        + *c0x.offset(0 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *g.offset(17 as libc::c_int as isize)
        * (xixj + *c0x.offset(1 as libc::c_int as isize))
        + *c0x.offset(1 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *g.offset(18 as libc::c_int as isize)
        * (xixj + *c0x.offset(2 as libc::c_int as isize))
        + *c0x.offset(2 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *g.offset(19 as libc::c_int as isize)
        * (xixj + *c0x.offset(3 as libc::c_int as isize))
        + *c0x.offset(3 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b10.offset(3 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (xixj + *c0x.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (xixj + *c0x.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *c0x.offset(2 as libc::c_int as isize)
        * (xixj + *c0x.offset(2 as libc::c_int as isize))
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *c0x.offset(3 as libc::c_int as isize)
        * (xixj + *c0x.offset(3 as libc::c_int as isize))
        + *b10.offset(3 as libc::c_int as isize);
    *g.offset(4 as libc::c_int as isize) = xixj + *c0x.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = xixj + *c0x.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = xixj + *c0x.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = xixj + *c0x.offset(3 as libc::c_int as isize);
    *g.offset(32 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(33 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(34 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(35 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(40 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(41 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(42 as libc::c_int as isize) = *c0y.offset(2 as libc::c_int as isize);
    *g.offset(43 as libc::c_int as isize) = *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            48 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            49 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            50 as libc::c_int as isize,
        ) = *c0y.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            51 as libc::c_int as isize,
        ) = *c0y.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize);
    *g
        .offset(
            52 as libc::c_int as isize,
        ) = *g.offset(48 as libc::c_int as isize)
        * (yiyj + *c0y.offset(0 as libc::c_int as isize))
        + *c0y.offset(0 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            53 as libc::c_int as isize,
        ) = *g.offset(49 as libc::c_int as isize)
        * (yiyj + *c0y.offset(1 as libc::c_int as isize))
        + *c0y.offset(1 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            54 as libc::c_int as isize,
        ) = *g.offset(50 as libc::c_int as isize)
        * (yiyj + *c0y.offset(2 as libc::c_int as isize))
        + *c0y.offset(2 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            55 as libc::c_int as isize,
        ) = *g.offset(51 as libc::c_int as isize)
        * (yiyj + *c0y.offset(3 as libc::c_int as isize))
        + *c0y.offset(3 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b10.offset(3 as libc::c_int as isize);
    *g
        .offset(
            44 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (yiyj + *c0y.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            45 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (yiyj + *c0y.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            46 as libc::c_int as isize,
        ) = *c0y.offset(2 as libc::c_int as isize)
        * (yiyj + *c0y.offset(2 as libc::c_int as isize))
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            47 as libc::c_int as isize,
        ) = *c0y.offset(3 as libc::c_int as isize)
        * (yiyj + *c0y.offset(3 as libc::c_int as isize))
        + *b10.offset(3 as libc::c_int as isize);
    *g
        .offset(
            36 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(0 as libc::c_int as isize);
    *g
        .offset(
            37 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            38 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(2 as libc::c_int as isize);
    *g
        .offset(
            39 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            72 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(64 as libc::c_int as isize);
    *g
        .offset(
            73 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(65 as libc::c_int as isize);
    *g
        .offset(
            74 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(66 as libc::c_int as isize);
    *g
        .offset(
            75 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(67 as libc::c_int as isize);
    *g
        .offset(
            80 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(72 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(64 as libc::c_int as isize);
    *g
        .offset(
            81 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(73 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(65 as libc::c_int as isize);
    *g
        .offset(
            82 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(74 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize) * *g.offset(66 as libc::c_int as isize);
    *g
        .offset(
            83 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(75 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize) * *g.offset(67 as libc::c_int as isize);
    *g
        .offset(
            84 as libc::c_int as isize,
        ) = *g.offset(80 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize))
        + 2 as libc::c_int as libc::c_double * *b10.offset(0 as libc::c_int as isize)
            * *g.offset(72 as libc::c_int as isize);
    *g
        .offset(
            85 as libc::c_int as isize,
        ) = *g.offset(81 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize))
        + 2 as libc::c_int as libc::c_double * *b10.offset(1 as libc::c_int as isize)
            * *g.offset(73 as libc::c_int as isize);
    *g
        .offset(
            86 as libc::c_int as isize,
        ) = *g.offset(82 as libc::c_int as isize)
        * (zizj + *c0z.offset(2 as libc::c_int as isize))
        + 2 as libc::c_int as libc::c_double * *b10.offset(2 as libc::c_int as isize)
            * *g.offset(74 as libc::c_int as isize);
    *g
        .offset(
            87 as libc::c_int as isize,
        ) = *g.offset(83 as libc::c_int as isize)
        * (zizj + *c0z.offset(3 as libc::c_int as isize))
        + 2 as libc::c_int as libc::c_double * *b10.offset(3 as libc::c_int as isize)
            * *g.offset(75 as libc::c_int as isize);
    *g
        .offset(
            76 as libc::c_int as isize,
        ) = *g.offset(72 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(64 as libc::c_int as isize);
    *g
        .offset(
            77 as libc::c_int as isize,
        ) = *g.offset(73 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(65 as libc::c_int as isize);
    *g
        .offset(
            78 as libc::c_int as isize,
        ) = *g.offset(74 as libc::c_int as isize)
        * (zizj + *c0z.offset(2 as libc::c_int as isize))
        + *b10.offset(2 as libc::c_int as isize) * *g.offset(66 as libc::c_int as isize);
    *g
        .offset(
            79 as libc::c_int as isize,
        ) = *g.offset(75 as libc::c_int as isize)
        * (zizj + *c0z.offset(3 as libc::c_int as isize))
        + *b10.offset(3 as libc::c_int as isize) * *g.offset(67 as libc::c_int as isize);
    *g
        .offset(
            68 as libc::c_int as isize,
        ) = *g.offset(64 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize));
    *g
        .offset(
            69 as libc::c_int as isize,
        ) = *g.offset(65 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize));
    *g
        .offset(
            70 as libc::c_int as isize,
        ) = *g.offset(66 as libc::c_int as isize)
        * (zizj + *c0z.offset(2 as libc::c_int as isize));
    *g
        .offset(
            71 as libc::c_int as isize,
        ) = *g.offset(67 as libc::c_int as isize)
        * (zizj + *c0z.offset(3 as libc::c_int as isize));
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_2000(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *c0x.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *c0x.offset(3 as libc::c_int as isize);
    *g
        .offset(
            8 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            9 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *c0x.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *c0x.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize);
    *g.offset(12 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(13 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(14 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(15 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(16 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(17 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(18 as libc::c_int as isize) = *c0y.offset(2 as libc::c_int as isize);
    *g.offset(19 as libc::c_int as isize) = *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *c0y.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *c0y.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(26 as libc::c_int as isize);
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(27 as libc::c_int as isize);
    *g
        .offset(
            32 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(28 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(24 as libc::c_int as isize);
    *g
        .offset(
            33 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(29 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(25 as libc::c_int as isize);
    *g
        .offset(
            34 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(30 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize) * *g.offset(26 as libc::c_int as isize);
    *g
        .offset(
            35 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(31 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize) * *g.offset(27 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_2001(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *c0x.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *c0x.offset(3 as libc::c_int as isize);
    *g
        .offset(
            8 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            9 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *c0x.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *c0x.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize);
    *g.offset(12 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(13 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(14 as libc::c_int as isize) = *cpx.offset(2 as libc::c_int as isize);
    *g.offset(15 as libc::c_int as isize) = *cpx.offset(3 as libc::c_int as isize);
    *g
        .offset(
            16 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            17 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (*g.offset(16 as libc::c_int as isize)
            + *b00.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize)
            * *cpx.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (*g.offset(17 as libc::c_int as isize)
            + *b00.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize)
            * *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *c0x.offset(2 as libc::c_int as isize)
        * (*g.offset(18 as libc::c_int as isize)
            + *b00.offset(2 as libc::c_int as isize))
        + *b10.offset(2 as libc::c_int as isize)
            * *cpx.offset(2 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *c0x.offset(3 as libc::c_int as isize)
        * (*g.offset(19 as libc::c_int as isize)
            + *b00.offset(3 as libc::c_int as isize))
        + *b10.offset(3 as libc::c_int as isize)
            * *cpx.offset(3 as libc::c_int as isize);
    *g.offset(24 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(25 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(26 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(27 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(28 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(29 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(30 as libc::c_int as isize) = *c0y.offset(2 as libc::c_int as isize);
    *g.offset(31 as libc::c_int as isize) = *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            32 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            33 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            34 as libc::c_int as isize,
        ) = *c0y.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            35 as libc::c_int as isize,
        ) = *c0y.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize);
    *g.offset(36 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(37 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(38 as libc::c_int as isize) = *cpy.offset(2 as libc::c_int as isize);
    *g.offset(39 as libc::c_int as isize) = *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            40 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            41 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            42 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            43 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            44 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (*g.offset(40 as libc::c_int as isize)
            + *b00.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize)
            * *cpy.offset(0 as libc::c_int as isize);
    *g
        .offset(
            45 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (*g.offset(41 as libc::c_int as isize)
            + *b00.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize)
            * *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            46 as libc::c_int as isize,
        ) = *c0y.offset(2 as libc::c_int as isize)
        * (*g.offset(42 as libc::c_int as isize)
            + *b00.offset(2 as libc::c_int as isize))
        + *b10.offset(2 as libc::c_int as isize)
            * *cpy.offset(2 as libc::c_int as isize);
    *g
        .offset(
            47 as libc::c_int as isize,
        ) = *c0y.offset(3 as libc::c_int as isize)
        * (*g.offset(43 as libc::c_int as isize)
            + *b00.offset(3 as libc::c_int as isize))
        + *b10.offset(3 as libc::c_int as isize)
            * *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            52 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            53 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            54 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            55 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            56 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(52 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            57 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(53 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            58 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(54 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize) * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            59 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(55 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize) * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            60 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            61 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            62 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            63 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            64 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(52 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            65 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(53 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            66 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(54 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            67 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(55 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            68 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(64 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(60 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(52 as libc::c_int as isize);
    *g
        .offset(
            69 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(65 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(61 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(53 as libc::c_int as isize);
    *g
        .offset(
            70 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(66 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize) * *g.offset(62 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(54 as libc::c_int as isize);
    *g
        .offset(
            71 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(67 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize) * *g.offset(63 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(55 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_2010(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut libc::c_double = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut libc::c_double = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut libc::c_double = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut libc::c_double = ((*bc).b00).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *c0x.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *c0x.offset(3 as libc::c_int as isize);
    *g
        .offset(
            8 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            9 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *c0x.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *c0x.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize);
    *g.offset(12 as libc::c_int as isize) = *cpx.offset(0 as libc::c_int as isize);
    *g.offset(13 as libc::c_int as isize) = *cpx.offset(1 as libc::c_int as isize);
    *g.offset(14 as libc::c_int as isize) = *cpx.offset(2 as libc::c_int as isize);
    *g.offset(15 as libc::c_int as isize) = *cpx.offset(3 as libc::c_int as isize);
    *g
        .offset(
            16 as libc::c_int as isize,
        ) = *cpx.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            17 as libc::c_int as isize,
        ) = *cpx.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = *cpx.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = *cpx.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (*g.offset(16 as libc::c_int as isize)
            + *b00.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize)
            * *cpx.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (*g.offset(17 as libc::c_int as isize)
            + *b00.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize)
            * *cpx.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *c0x.offset(2 as libc::c_int as isize)
        * (*g.offset(18 as libc::c_int as isize)
            + *b00.offset(2 as libc::c_int as isize))
        + *b10.offset(2 as libc::c_int as isize)
            * *cpx.offset(2 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *c0x.offset(3 as libc::c_int as isize)
        * (*g.offset(19 as libc::c_int as isize)
            + *b00.offset(3 as libc::c_int as isize))
        + *b10.offset(3 as libc::c_int as isize)
            * *cpx.offset(3 as libc::c_int as isize);
    *g.offset(24 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(25 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(26 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(27 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(28 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(29 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(30 as libc::c_int as isize) = *c0y.offset(2 as libc::c_int as isize);
    *g.offset(31 as libc::c_int as isize) = *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            32 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            33 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            34 as libc::c_int as isize,
        ) = *c0y.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            35 as libc::c_int as isize,
        ) = *c0y.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize);
    *g.offset(36 as libc::c_int as isize) = *cpy.offset(0 as libc::c_int as isize);
    *g.offset(37 as libc::c_int as isize) = *cpy.offset(1 as libc::c_int as isize);
    *g.offset(38 as libc::c_int as isize) = *cpy.offset(2 as libc::c_int as isize);
    *g.offset(39 as libc::c_int as isize) = *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            40 as libc::c_int as isize,
        ) = *cpy.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize);
    *g
        .offset(
            41 as libc::c_int as isize,
        ) = *cpy.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize);
    *g
        .offset(
            42 as libc::c_int as isize,
        ) = *cpy.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize);
    *g
        .offset(
            43 as libc::c_int as isize,
        ) = *cpy.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize);
    *g
        .offset(
            44 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (*g.offset(40 as libc::c_int as isize)
            + *b00.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize)
            * *cpy.offset(0 as libc::c_int as isize);
    *g
        .offset(
            45 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (*g.offset(41 as libc::c_int as isize)
            + *b00.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize)
            * *cpy.offset(1 as libc::c_int as isize);
    *g
        .offset(
            46 as libc::c_int as isize,
        ) = *c0y.offset(2 as libc::c_int as isize)
        * (*g.offset(42 as libc::c_int as isize)
            + *b00.offset(2 as libc::c_int as isize))
        + *b10.offset(2 as libc::c_int as isize)
            * *cpy.offset(2 as libc::c_int as isize);
    *g
        .offset(
            47 as libc::c_int as isize,
        ) = *c0y.offset(3 as libc::c_int as isize)
        * (*g.offset(43 as libc::c_int as isize)
            + *b00.offset(3 as libc::c_int as isize))
        + *b10.offset(3 as libc::c_int as isize)
            * *cpy.offset(3 as libc::c_int as isize);
    *g
        .offset(
            52 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            53 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            54 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            55 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            56 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(52 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            57 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(53 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            58 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(54 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize) * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            59 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(55 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize) * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            60 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            61 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            62 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            63 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            64 as libc::c_int as isize,
        ) = *cpz.offset(0 as libc::c_int as isize)
        * *g.offset(52 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(48 as libc::c_int as isize);
    *g
        .offset(
            65 as libc::c_int as isize,
        ) = *cpz.offset(1 as libc::c_int as isize)
        * *g.offset(53 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(49 as libc::c_int as isize);
    *g
        .offset(
            66 as libc::c_int as isize,
        ) = *cpz.offset(2 as libc::c_int as isize)
        * *g.offset(54 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(50 as libc::c_int as isize);
    *g
        .offset(
            67 as libc::c_int as isize,
        ) = *cpz.offset(3 as libc::c_int as isize)
        * *g.offset(55 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(51 as libc::c_int as isize);
    *g
        .offset(
            68 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(64 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(60 as libc::c_int as isize)
        + *b00.offset(0 as libc::c_int as isize) * *g.offset(52 as libc::c_int as isize);
    *g
        .offset(
            69 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(65 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(61 as libc::c_int as isize)
        + *b00.offset(1 as libc::c_int as isize) * *g.offset(53 as libc::c_int as isize);
    *g
        .offset(
            70 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(66 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize) * *g.offset(62 as libc::c_int as isize)
        + *b00.offset(2 as libc::c_int as isize) * *g.offset(54 as libc::c_int as isize);
    *g
        .offset(
            71 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(67 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize) * *g.offset(63 as libc::c_int as isize)
        + *b00.offset(3 as libc::c_int as isize) * *g.offset(55 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_2100(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    let mut xixj: libc::c_double = (*envs).rirj[0 as libc::c_int as usize];
    let mut yiyj: libc::c_double = (*envs).rirj[1 as libc::c_int as usize];
    let mut zizj: libc::c_double = (*envs).rirj[2 as libc::c_int as usize];
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *c0x.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *c0x.offset(3 as libc::c_int as isize);
    *g
        .offset(
            8 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            9 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *c0x.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *c0x.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize);
    *g
        .offset(
            24 as libc::c_int as isize,
        ) = *g.offset(8 as libc::c_int as isize)
        * (xixj + *c0x.offset(0 as libc::c_int as isize))
        + *c0x.offset(0 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            25 as libc::c_int as isize,
        ) = *g.offset(9 as libc::c_int as isize)
        * (xixj + *c0x.offset(1 as libc::c_int as isize))
        + *c0x.offset(1 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = *g.offset(10 as libc::c_int as isize)
        * (xixj + *c0x.offset(2 as libc::c_int as isize))
        + *c0x.offset(2 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = *g.offset(11 as libc::c_int as isize)
        * (xixj + *c0x.offset(3 as libc::c_int as isize))
        + *c0x.offset(3 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b10.offset(3 as libc::c_int as isize);
    *g
        .offset(
            20 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (xixj + *c0x.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            21 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (xixj + *c0x.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            22 as libc::c_int as isize,
        ) = *c0x.offset(2 as libc::c_int as isize)
        * (xixj + *c0x.offset(2 as libc::c_int as isize))
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            23 as libc::c_int as isize,
        ) = *c0x.offset(3 as libc::c_int as isize)
        * (xixj + *c0x.offset(3 as libc::c_int as isize))
        + *b10.offset(3 as libc::c_int as isize);
    *g
        .offset(
            16 as libc::c_int as isize,
        ) = xixj + *c0x.offset(0 as libc::c_int as isize);
    *g
        .offset(
            17 as libc::c_int as isize,
        ) = xixj + *c0x.offset(1 as libc::c_int as isize);
    *g
        .offset(
            18 as libc::c_int as isize,
        ) = xixj + *c0x.offset(2 as libc::c_int as isize);
    *g
        .offset(
            19 as libc::c_int as isize,
        ) = xixj + *c0x.offset(3 as libc::c_int as isize);
    *g.offset(32 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(33 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(34 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(35 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(36 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(37 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(38 as libc::c_int as isize) = *c0y.offset(2 as libc::c_int as isize);
    *g.offset(39 as libc::c_int as isize) = *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            40 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            41 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            42 as libc::c_int as isize,
        ) = *c0y.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            43 as libc::c_int as isize,
        ) = *c0y.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize);
    *g
        .offset(
            56 as libc::c_int as isize,
        ) = *g.offset(40 as libc::c_int as isize)
        * (yiyj + *c0y.offset(0 as libc::c_int as isize))
        + *c0y.offset(0 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            57 as libc::c_int as isize,
        ) = *g.offset(41 as libc::c_int as isize)
        * (yiyj + *c0y.offset(1 as libc::c_int as isize))
        + *c0y.offset(1 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            58 as libc::c_int as isize,
        ) = *g.offset(42 as libc::c_int as isize)
        * (yiyj + *c0y.offset(2 as libc::c_int as isize))
        + *c0y.offset(2 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            59 as libc::c_int as isize,
        ) = *g.offset(43 as libc::c_int as isize)
        * (yiyj + *c0y.offset(3 as libc::c_int as isize))
        + *c0y.offset(3 as libc::c_int as isize) * 2 as libc::c_int as libc::c_double
            * *b10.offset(3 as libc::c_int as isize);
    *g
        .offset(
            52 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (yiyj + *c0y.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            53 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (yiyj + *c0y.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            54 as libc::c_int as isize,
        ) = *c0y.offset(2 as libc::c_int as isize)
        * (yiyj + *c0y.offset(2 as libc::c_int as isize))
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            55 as libc::c_int as isize,
        ) = *c0y.offset(3 as libc::c_int as isize)
        * (yiyj + *c0y.offset(3 as libc::c_int as isize))
        + *b10.offset(3 as libc::c_int as isize);
    *g
        .offset(
            48 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(0 as libc::c_int as isize);
    *g
        .offset(
            49 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(1 as libc::c_int as isize);
    *g
        .offset(
            50 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(2 as libc::c_int as isize);
    *g
        .offset(
            51 as libc::c_int as isize,
        ) = yiyj + *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            68 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(64 as libc::c_int as isize);
    *g
        .offset(
            69 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(65 as libc::c_int as isize);
    *g
        .offset(
            70 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(66 as libc::c_int as isize);
    *g
        .offset(
            71 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(67 as libc::c_int as isize);
    *g
        .offset(
            72 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(68 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(64 as libc::c_int as isize);
    *g
        .offset(
            73 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(69 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(65 as libc::c_int as isize);
    *g
        .offset(
            74 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(70 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize) * *g.offset(66 as libc::c_int as isize);
    *g
        .offset(
            75 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(71 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize) * *g.offset(67 as libc::c_int as isize);
    *g
        .offset(
            88 as libc::c_int as isize,
        ) = *g.offset(72 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize))
        + 2 as libc::c_int as libc::c_double * *b10.offset(0 as libc::c_int as isize)
            * *g.offset(68 as libc::c_int as isize);
    *g
        .offset(
            89 as libc::c_int as isize,
        ) = *g.offset(73 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize))
        + 2 as libc::c_int as libc::c_double * *b10.offset(1 as libc::c_int as isize)
            * *g.offset(69 as libc::c_int as isize);
    *g
        .offset(
            90 as libc::c_int as isize,
        ) = *g.offset(74 as libc::c_int as isize)
        * (zizj + *c0z.offset(2 as libc::c_int as isize))
        + 2 as libc::c_int as libc::c_double * *b10.offset(2 as libc::c_int as isize)
            * *g.offset(70 as libc::c_int as isize);
    *g
        .offset(
            91 as libc::c_int as isize,
        ) = *g.offset(75 as libc::c_int as isize)
        * (zizj + *c0z.offset(3 as libc::c_int as isize))
        + 2 as libc::c_int as libc::c_double * *b10.offset(3 as libc::c_int as isize)
            * *g.offset(71 as libc::c_int as isize);
    *g
        .offset(
            84 as libc::c_int as isize,
        ) = *g.offset(68 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize))
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(64 as libc::c_int as isize);
    *g
        .offset(
            85 as libc::c_int as isize,
        ) = *g.offset(69 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize))
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(65 as libc::c_int as isize);
    *g
        .offset(
            86 as libc::c_int as isize,
        ) = *g.offset(70 as libc::c_int as isize)
        * (zizj + *c0z.offset(2 as libc::c_int as isize))
        + *b10.offset(2 as libc::c_int as isize) * *g.offset(66 as libc::c_int as isize);
    *g
        .offset(
            87 as libc::c_int as isize,
        ) = *g.offset(71 as libc::c_int as isize)
        * (zizj + *c0z.offset(3 as libc::c_int as isize))
        + *b10.offset(3 as libc::c_int as isize) * *g.offset(67 as libc::c_int as isize);
    *g
        .offset(
            80 as libc::c_int as isize,
        ) = *g.offset(64 as libc::c_int as isize)
        * (zizj + *c0z.offset(0 as libc::c_int as isize));
    *g
        .offset(
            81 as libc::c_int as isize,
        ) = *g.offset(65 as libc::c_int as isize)
        * (zizj + *c0z.offset(1 as libc::c_int as isize));
    *g
        .offset(
            82 as libc::c_int as isize,
        ) = *g.offset(66 as libc::c_int as isize)
        * (zizj + *c0z.offset(2 as libc::c_int as isize));
    *g
        .offset(
            83 as libc::c_int as isize,
        ) = *g.offset(67 as libc::c_int as isize)
        * (zizj + *c0z.offset(3 as libc::c_int as isize));
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_3000(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut libc::c_double = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut libc::c_double = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut libc::c_double = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut libc::c_double = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(4 as libc::c_int as isize) = *c0x.offset(0 as libc::c_int as isize);
    *g.offset(5 as libc::c_int as isize) = *c0x.offset(1 as libc::c_int as isize);
    *g.offset(6 as libc::c_int as isize) = *c0x.offset(2 as libc::c_int as isize);
    *g.offset(7 as libc::c_int as isize) = *c0x.offset(3 as libc::c_int as isize);
    *g
        .offset(
            8 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * *c0x.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            9 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * *c0x.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            10 as libc::c_int as isize,
        ) = *c0x.offset(2 as libc::c_int as isize)
        * *c0x.offset(2 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            11 as libc::c_int as isize,
        ) = *c0x.offset(3 as libc::c_int as isize)
        * *c0x.offset(3 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize);
    *g
        .offset(
            12 as libc::c_int as isize,
        ) = *c0x.offset(0 as libc::c_int as isize)
        * (*g.offset(8 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b10.offset(0 as libc::c_int as isize));
    *g
        .offset(
            13 as libc::c_int as isize,
        ) = *c0x.offset(1 as libc::c_int as isize)
        * (*g.offset(9 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b10.offset(1 as libc::c_int as isize));
    *g
        .offset(
            14 as libc::c_int as isize,
        ) = *c0x.offset(2 as libc::c_int as isize)
        * (*g.offset(10 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b10.offset(2 as libc::c_int as isize));
    *g
        .offset(
            15 as libc::c_int as isize,
        ) = *c0x.offset(3 as libc::c_int as isize)
        * (*g.offset(11 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b10.offset(3 as libc::c_int as isize));
    *g.offset(16 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(17 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(18 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(19 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *g.offset(20 as libc::c_int as isize) = *c0y.offset(0 as libc::c_int as isize);
    *g.offset(21 as libc::c_int as isize) = *c0y.offset(1 as libc::c_int as isize);
    *g.offset(22 as libc::c_int as isize) = *c0y.offset(2 as libc::c_int as isize);
    *g.offset(23 as libc::c_int as isize) = *c0y.offset(3 as libc::c_int as isize);
    *g
        .offset(
            24 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * *c0y.offset(0 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize);
    *g
        .offset(
            25 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * *c0y.offset(1 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize);
    *g
        .offset(
            26 as libc::c_int as isize,
        ) = *c0y.offset(2 as libc::c_int as isize)
        * *c0y.offset(2 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize);
    *g
        .offset(
            27 as libc::c_int as isize,
        ) = *c0y.offset(3 as libc::c_int as isize)
        * *c0y.offset(3 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize);
    *g
        .offset(
            28 as libc::c_int as isize,
        ) = *c0y.offset(0 as libc::c_int as isize)
        * (*g.offset(24 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b10.offset(0 as libc::c_int as isize));
    *g
        .offset(
            29 as libc::c_int as isize,
        ) = *c0y.offset(1 as libc::c_int as isize)
        * (*g.offset(25 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b10.offset(1 as libc::c_int as isize));
    *g
        .offset(
            30 as libc::c_int as isize,
        ) = *c0y.offset(2 as libc::c_int as isize)
        * (*g.offset(26 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b10.offset(2 as libc::c_int as isize));
    *g
        .offset(
            31 as libc::c_int as isize,
        ) = *c0y.offset(3 as libc::c_int as isize)
        * (*g.offset(27 as libc::c_int as isize)
            + 2 as libc::c_int as libc::c_double
                * *b10.offset(3 as libc::c_int as isize));
    *g
        .offset(
            36 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            37 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            38 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(34 as libc::c_int as isize);
    *g
        .offset(
            39 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(35 as libc::c_int as isize);
    *g
        .offset(
            40 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(36 as libc::c_int as isize)
        + *b10.offset(0 as libc::c_int as isize) * *g.offset(32 as libc::c_int as isize);
    *g
        .offset(
            41 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(37 as libc::c_int as isize)
        + *b10.offset(1 as libc::c_int as isize) * *g.offset(33 as libc::c_int as isize);
    *g
        .offset(
            42 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(38 as libc::c_int as isize)
        + *b10.offset(2 as libc::c_int as isize) * *g.offset(34 as libc::c_int as isize);
    *g
        .offset(
            43 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(39 as libc::c_int as isize)
        + *b10.offset(3 as libc::c_int as isize) * *g.offset(35 as libc::c_int as isize);
    *g
        .offset(
            44 as libc::c_int as isize,
        ) = *c0z.offset(0 as libc::c_int as isize)
        * *g.offset(40 as libc::c_int as isize)
        + 2 as libc::c_int as libc::c_double * *b10.offset(0 as libc::c_int as isize)
            * *g.offset(36 as libc::c_int as isize);
    *g
        .offset(
            45 as libc::c_int as isize,
        ) = *c0z.offset(1 as libc::c_int as isize)
        * *g.offset(41 as libc::c_int as isize)
        + 2 as libc::c_int as libc::c_double * *b10.offset(1 as libc::c_int as isize)
            * *g.offset(37 as libc::c_int as isize);
    *g
        .offset(
            46 as libc::c_int as isize,
        ) = *c0z.offset(2 as libc::c_int as isize)
        * *g.offset(42 as libc::c_int as isize)
        + 2 as libc::c_int as libc::c_double * *b10.offset(2 as libc::c_int as isize)
            * *g.offset(38 as libc::c_int as isize);
    *g
        .offset(
            47 as libc::c_int as isize,
        ) = *c0z.offset(3 as libc::c_int as isize)
        * *g.offset(43 as libc::c_int as isize)
        + 2 as libc::c_int as libc::c_double * *b10.offset(3 as libc::c_int as isize)
            * *g.offset(39 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CINTsrg0_2e_2d4d_unrolled(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut type_ijkl: libc::c_int = (*envs).li_ceil << 6 as libc::c_int
        | (*envs).lj_ceil << 4 as libc::c_int | (*envs).lk_ceil << 2 as libc::c_int
        | (*envs).ll_ceil;
    match type_ijkl {
        0 => {
            _srg0_2d4d_0000(g, bc, envs);
            return;
        }
        1 => {
            _srg0_2d4d_0001(g, bc, envs);
            return;
        }
        2 => {
            _srg0_2d4d_0002(g, bc, envs);
            return;
        }
        3 => {
            _srg0_2d4d_0003(g, bc, envs);
            return;
        }
        4 => {
            _srg0_2d4d_0010(g, bc, envs);
            return;
        }
        5 => {
            _srg0_2d4d_0011(g, bc, envs);
            return;
        }
        6 => {
            _srg0_2d4d_0012(g, bc, envs);
            return;
        }
        8 => {
            _srg0_2d4d_0020(g, bc, envs);
            return;
        }
        9 => {
            _srg0_2d4d_0021(g, bc, envs);
            return;
        }
        12 => {
            _srg0_2d4d_0030(g, bc, envs);
            return;
        }
        16 => {
            _srg0_2d4d_0100(g, bc, envs);
            return;
        }
        17 => {
            _srg0_2d4d_0101(g, bc, envs);
            return;
        }
        18 => {
            _srg0_2d4d_0102(g, bc, envs);
            return;
        }
        20 => {
            _srg0_2d4d_0110(g, bc, envs);
            return;
        }
        21 => {
            _srg0_2d4d_0111(g, bc, envs);
            return;
        }
        24 => {
            _srg0_2d4d_0120(g, bc, envs);
            return;
        }
        32 => {
            _srg0_2d4d_0200(g, bc, envs);
            return;
        }
        33 => {
            _srg0_2d4d_0201(g, bc, envs);
            return;
        }
        36 => {
            _srg0_2d4d_0210(g, bc, envs);
            return;
        }
        48 => {
            _srg0_2d4d_0300(g, bc, envs);
            return;
        }
        64 => {
            _srg0_2d4d_1000(g, bc, envs);
            return;
        }
        65 => {
            _srg0_2d4d_1001(g, bc, envs);
            return;
        }
        66 => {
            _srg0_2d4d_1002(g, bc, envs);
            return;
        }
        68 => {
            _srg0_2d4d_1010(g, bc, envs);
            return;
        }
        69 => {
            _srg0_2d4d_1011(g, bc, envs);
            return;
        }
        72 => {
            _srg0_2d4d_1020(g, bc, envs);
            return;
        }
        80 => {
            _srg0_2d4d_1100(g, bc, envs);
            return;
        }
        81 => {
            _srg0_2d4d_1101(g, bc, envs);
            return;
        }
        84 => {
            _srg0_2d4d_1110(g, bc, envs);
            return;
        }
        96 => {
            _srg0_2d4d_1200(g, bc, envs);
            return;
        }
        128 => {
            _srg0_2d4d_2000(g, bc, envs);
            return;
        }
        129 => {
            _srg0_2d4d_2001(g, bc, envs);
            return;
        }
        132 => {
            _srg0_2d4d_2010(g, bc, envs);
            return;
        }
        144 => {
            _srg0_2d4d_2100(g, bc, envs);
            return;
        }
        192 => {
            _srg0_2d4d_3000(g, bc, envs);
            return;
        }
        _ => {}
    }
    fprintf(
        stderr,
        b"Dimension error for CINTg0_2e_lj2d4d: iklj = %d %d %d %d\0" as *const u8
            as *const libc::c_char,
        (*envs).li_ceil,
        (*envs).lk_ceil,
        (*envs).ll_ceil,
        (*envs).lj_ceil,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e_lj2d4d(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    CINTg0_2e_2d(g, bc, envs);
    CINTg0_lj2d_4d(g, envs);
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e_kj2d4d(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    CINTg0_2e_2d(g, bc, envs);
    CINTg0_kj2d_4d(g, envs);
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e_ik2d4d(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    CINTg0_2e_2d(g, bc, envs);
    CINTg0_ik2d_4d(g, envs);
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e_il2d4d(
    mut g: *mut libc::c_double,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    CINTg0_2e_2d(g, bc, envs);
    CINTg0_il2d_4d(g, envs);
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e(
    mut g: *mut libc::c_double,
    mut rij: *mut libc::c_double,
    mut rkl: *mut libc::c_double,
    mut cutoff: libc::c_double,
    mut envs: *mut CINTEnvVars,
) -> libc::c_int {
    let mut irys: libc::c_int = 0;
    let mut nroots: libc::c_int = (*envs).nrys_roots;
    let mut aij: libc::c_double = (*envs).ai[0 as libc::c_int as usize]
        + (*envs).aj[0 as libc::c_int as usize];
    let mut akl: libc::c_double = (*envs).ak[0 as libc::c_int as usize]
        + (*envs).al[0 as libc::c_int as usize];
    let mut a0: libc::c_double = 0.;
    let mut a1: libc::c_double = 0.;
    let mut fac1: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut u: [libc::c_double; 32] = [0.; 32];
    let mut w: *mut libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut xij_kl: libc::c_double = *rij.offset(0 as libc::c_int as isize)
        - *rkl.offset(0 as libc::c_int as isize);
    let mut yij_kl: libc::c_double = *rij.offset(1 as libc::c_int as isize)
        - *rkl.offset(1 as libc::c_int as isize);
    let mut zij_kl: libc::c_double = *rij.offset(2 as libc::c_int as isize)
        - *rkl.offset(2 as libc::c_int as isize);
    let mut rr: libc::c_double = xij_kl * xij_kl + yij_kl * yij_kl + zij_kl * zij_kl;
    a1 = aij * akl;
    a0 = a1 / (aij + akl);
    fac1 = sqrt(a0 / (a1 * a1 * a1)) * (*envs).fac[0 as libc::c_int as usize];
    x = a0 * rr;
    let omega: libc::c_double = *((*envs).env).offset(8 as libc::c_int as isize);
    let mut theta: libc::c_double = 0 as libc::c_int as libc::c_double;
    if omega == 0.0f64 {
        CINTrys_roots(nroots, x, u.as_mut_ptr(), w);
    } else if omega < 0.0f64 {
        theta = omega * omega / (omega * omega + a0);
        if theta * x > cutoff || theta * x > 40 as libc::c_int as libc::c_double {
            return 0 as libc::c_int;
        }
        let mut rorder: libc::c_int = (*envs).rys_order;
        if rorder == nroots {
            CINTsr_rys_roots(nroots, x, sqrt(theta), u.as_mut_ptr(), w);
        } else {
            let mut sqrt_theta: libc::c_double = -sqrt(theta);
            CINTrys_roots(rorder, x, u.as_mut_ptr(), w);
            CINTrys_roots(
                rorder,
                theta * x,
                u.as_mut_ptr().offset(rorder as isize),
                w.offset(rorder as isize),
            );
            if (*envs).g_size == 2 as libc::c_int {
                *g
                    .offset(
                        0 as libc::c_int as isize,
                    ) = 1 as libc::c_int as libc::c_double;
                *g
                    .offset(
                        1 as libc::c_int as isize,
                    ) = 1 as libc::c_int as libc::c_double;
                *g
                    .offset(
                        2 as libc::c_int as isize,
                    ) = 1 as libc::c_int as libc::c_double;
                *g
                    .offset(
                        3 as libc::c_int as isize,
                    ) = 1 as libc::c_int as libc::c_double;
                *g.offset(4 as libc::c_int as isize) *= fac1;
                *g.offset(5 as libc::c_int as isize) *= fac1 * sqrt_theta;
                return 1 as libc::c_int;
            }
            irys = rorder;
            while irys < nroots {
                let mut ut: libc::c_double = u[irys as usize] * theta;
                u[irys as usize] = ut / (u[irys as usize] + 1.0f64 - ut);
                *w.offset(irys as isize) *= sqrt_theta;
                irys += 1;
                irys;
            }
        }
    } else {
        theta = omega * omega / (omega * omega + a0);
        x *= theta;
        fac1 *= sqrt(theta);
        CINTrys_roots(nroots, x, u.as_mut_ptr(), w);
        irys = 0 as libc::c_int;
        while irys < nroots {
            let mut ut_0: libc::c_double = u[irys as usize] * theta;
            u[irys as usize] = ut_0 / (u[irys as usize] + 1.0f64 - ut_0);
            irys += 1;
            irys;
        }
    }
    if (*envs).g_size == 1 as libc::c_int {
        *g.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
        *g.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
        *g.offset(2 as libc::c_int as isize) *= fac1;
        return 1 as libc::c_int;
    }
    let mut u2: libc::c_double = 0.;
    let mut tmp1: libc::c_double = 0.;
    let mut tmp2: libc::c_double = 0.;
    let mut tmp3: libc::c_double = 0.;
    let mut tmp4: libc::c_double = 0.;
    let mut tmp5: libc::c_double = 0.;
    let mut rijrx: libc::c_double = *rij.offset(0 as libc::c_int as isize)
        - *((*envs).rx_in_rijrx).offset(0 as libc::c_int as isize);
    let mut rijry: libc::c_double = *rij.offset(1 as libc::c_int as isize)
        - *((*envs).rx_in_rijrx).offset(1 as libc::c_int as isize);
    let mut rijrz: libc::c_double = *rij.offset(2 as libc::c_int as isize)
        - *((*envs).rx_in_rijrx).offset(2 as libc::c_int as isize);
    let mut rklrx: libc::c_double = *rkl.offset(0 as libc::c_int as isize)
        - *((*envs).rx_in_rklrx).offset(0 as libc::c_int as isize);
    let mut rklry: libc::c_double = *rkl.offset(1 as libc::c_int as isize)
        - *((*envs).rx_in_rklrx).offset(1 as libc::c_int as isize);
    let mut rklrz: libc::c_double = *rkl.offset(2 as libc::c_int as isize)
        - *((*envs).rx_in_rklrx).offset(2 as libc::c_int as isize);
    let mut bc: Rys2eT = Rys2eT {
        c00x: [0.; 32],
        c00y: [0.; 32],
        c00z: [0.; 32],
        c0px: [0.; 32],
        c0py: [0.; 32],
        c0pz: [0.; 32],
        b01: [0.; 32],
        b00: [0.; 32],
        b10: [0.; 32],
    };
    let mut b00: *mut libc::c_double = (bc.b00).as_mut_ptr();
    let mut b10: *mut libc::c_double = (bc.b10).as_mut_ptr();
    let mut b01: *mut libc::c_double = (bc.b01).as_mut_ptr();
    let mut c00x: *mut libc::c_double = (bc.c00x).as_mut_ptr();
    let mut c00y: *mut libc::c_double = (bc.c00y).as_mut_ptr();
    let mut c00z: *mut libc::c_double = (bc.c00z).as_mut_ptr();
    let mut c0px: *mut libc::c_double = (bc.c0px).as_mut_ptr();
    let mut c0py: *mut libc::c_double = (bc.c0py).as_mut_ptr();
    let mut c0pz: *mut libc::c_double = (bc.c0pz).as_mut_ptr();
    irys = 0 as libc::c_int;
    while irys < nroots {
        u2 = a0 * u[irys as usize];
        tmp4 = 0.5f64 / (u2 * (aij + akl) + a1);
        tmp5 = u2 * tmp4;
        tmp1 = 2.0f64 * tmp5;
        tmp2 = tmp1 * akl;
        tmp3 = tmp1 * aij;
        *b00.offset(irys as isize) = tmp5;
        *b10.offset(irys as isize) = tmp5 + tmp4 * akl;
        *b01.offset(irys as isize) = tmp5 + tmp4 * aij;
        *c00x.offset(irys as isize) = rijrx - tmp2 * xij_kl;
        *c00y.offset(irys as isize) = rijry - tmp2 * yij_kl;
        *c00z.offset(irys as isize) = rijrz - tmp2 * zij_kl;
        *c0px.offset(irys as isize) = rklrx + tmp3 * xij_kl;
        *c0py.offset(irys as isize) = rklry + tmp3 * yij_kl;
        *c0pz.offset(irys as isize) = rklrz + tmp3 * zij_kl;
        *w.offset(irys as isize) *= fac1;
        irys += 1;
        irys;
    }
    ::core::mem::transmute::<
        _,
        fn(_, _, _),
    >(
        (Some(((*envs).f_g0_2d4d).expect("non-null function pointer")))
            .expect("non-null function pointer"),
    )(g, &mut bc, envs);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1i_2e(
    mut f: *mut libc::c_double,
    mut g: *const libc::c_double,
    li: libc::c_int,
    lj: libc::c_int,
    lk: libc::c_int,
    ll: libc::c_int,
    mut envs: *const CINTEnvVars,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let di: libc::c_int = (*envs).g_stride_i;
    let dk: libc::c_int = (*envs).g_stride_k;
    let dl: libc::c_int = (*envs).g_stride_l;
    let dj: libc::c_int = (*envs).g_stride_j;
    let nroots: libc::c_int = (*envs).nrys_roots;
    let ai2: libc::c_double = -(2 as libc::c_int) as libc::c_double
        * (*envs).ai[0 as libc::c_int as usize];
    let mut gx: *const libc::c_double = g;
    let mut gy: *const libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut fx: *mut libc::c_double = f;
    let mut fy: *mut libc::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut libc::c_double = f
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut p1x: *const libc::c_double = gx.offset(-(di as isize));
    let mut p1y: *const libc::c_double = gy.offset(-(di as isize));
    let mut p1z: *const libc::c_double = gz.offset(-(di as isize));
    let mut p2x: *const libc::c_double = gx.offset(di as isize);
    let mut p2y: *const libc::c_double = gy.offset(di as isize);
    let mut p2z: *const libc::c_double = gz.offset(di as isize);
    j = 0 as libc::c_int;
    while j <= lj {
        l = 0 as libc::c_int;
        while l <= ll {
            k = 0 as libc::c_int;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                n = ptr;
                while n < ptr + nroots {
                    *fx.offset(n as isize) = ai2 * *p2x.offset(n as isize);
                    *fy.offset(n as isize) = ai2 * *p2y.offset(n as isize);
                    *fz.offset(n as isize) = ai2 * *p2z.offset(n as isize);
                    n += 1;
                    n;
                }
                ptr += di;
                i = 1 as libc::c_int;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx
                            .offset(
                                n as isize,
                            ) = i as libc::c_double * *p1x.offset(n as isize)
                            + ai2 * *p2x.offset(n as isize);
                        *fy
                            .offset(
                                n as isize,
                            ) = i as libc::c_double * *p1y.offset(n as isize)
                            + ai2 * *p2y.offset(n as isize);
                        *fz
                            .offset(
                                n as isize,
                            ) = i as libc::c_double * *p1z.offset(n as isize)
                            + ai2 * *p2z.offset(n as isize);
                        n += 1;
                        n;
                    }
                    ptr += di;
                    i += 1;
                    i;
                }
                k += 1;
                k;
            }
            l += 1;
            l;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1j_2e(
    mut f: *mut libc::c_double,
    mut g: *const libc::c_double,
    li: libc::c_int,
    lj: libc::c_int,
    lk: libc::c_int,
    ll: libc::c_int,
    mut envs: *const CINTEnvVars,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let di: libc::c_int = (*envs).g_stride_i;
    let dk: libc::c_int = (*envs).g_stride_k;
    let dl: libc::c_int = (*envs).g_stride_l;
    let dj: libc::c_int = (*envs).g_stride_j;
    let nroots: libc::c_int = (*envs).nrys_roots;
    let aj2: libc::c_double = -(2 as libc::c_int) as libc::c_double
        * (*envs).aj[0 as libc::c_int as usize];
    let mut gx: *const libc::c_double = g;
    let mut gy: *const libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut fx: *mut libc::c_double = f;
    let mut fy: *mut libc::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut libc::c_double = f
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut p1x: *const libc::c_double = gx.offset(-(dj as isize));
    let mut p1y: *const libc::c_double = gy.offset(-(dj as isize));
    let mut p1z: *const libc::c_double = gz.offset(-(dj as isize));
    let mut p2x: *const libc::c_double = gx.offset(dj as isize);
    let mut p2y: *const libc::c_double = gy.offset(dj as isize);
    let mut p2z: *const libc::c_double = gz.offset(dj as isize);
    l = 0 as libc::c_int;
    while l <= ll {
        k = 0 as libc::c_int;
        while k <= lk {
            ptr = dl * l + dk * k;
            i = 0 as libc::c_int;
            while i <= li {
                n = ptr;
                while n < ptr + nroots {
                    *fx.offset(n as isize) = aj2 * *p2x.offset(n as isize);
                    *fy.offset(n as isize) = aj2 * *p2y.offset(n as isize);
                    *fz.offset(n as isize) = aj2 * *p2z.offset(n as isize);
                    n += 1;
                    n;
                }
                ptr += di;
                i += 1;
                i;
            }
            k += 1;
            k;
        }
        l += 1;
        l;
    }
    j = 1 as libc::c_int;
    while j <= lj {
        l = 0 as libc::c_int;
        while l <= ll {
            k = 0 as libc::c_int;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0 as libc::c_int;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx
                            .offset(
                                n as isize,
                            ) = j as libc::c_double * *p1x.offset(n as isize)
                            + aj2 * *p2x.offset(n as isize);
                        *fy
                            .offset(
                                n as isize,
                            ) = j as libc::c_double * *p1y.offset(n as isize)
                            + aj2 * *p2y.offset(n as isize);
                        *fz
                            .offset(
                                n as isize,
                            ) = j as libc::c_double * *p1z.offset(n as isize)
                            + aj2 * *p2z.offset(n as isize);
                        n += 1;
                        n;
                    }
                    ptr += di;
                    i += 1;
                    i;
                }
                k += 1;
                k;
            }
            l += 1;
            l;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1k_2e(
    mut f: *mut libc::c_double,
    mut g: *const libc::c_double,
    li: libc::c_int,
    lj: libc::c_int,
    lk: libc::c_int,
    ll: libc::c_int,
    mut envs: *const CINTEnvVars,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let di: libc::c_int = (*envs).g_stride_i;
    let dk: libc::c_int = (*envs).g_stride_k;
    let dl: libc::c_int = (*envs).g_stride_l;
    let dj: libc::c_int = (*envs).g_stride_j;
    let nroots: libc::c_int = (*envs).nrys_roots;
    let ak2: libc::c_double = -(2 as libc::c_int) as libc::c_double
        * (*envs).ak[0 as libc::c_int as usize];
    let mut gx: *const libc::c_double = g;
    let mut gy: *const libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut fx: *mut libc::c_double = f;
    let mut fy: *mut libc::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut libc::c_double = f
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut p1x: *const libc::c_double = gx.offset(-(dk as isize));
    let mut p1y: *const libc::c_double = gy.offset(-(dk as isize));
    let mut p1z: *const libc::c_double = gz.offset(-(dk as isize));
    let mut p2x: *const libc::c_double = gx.offset(dk as isize);
    let mut p2y: *const libc::c_double = gy.offset(dk as isize);
    let mut p2z: *const libc::c_double = gz.offset(dk as isize);
    j = 0 as libc::c_int;
    while j <= lj {
        l = 0 as libc::c_int;
        while l <= ll {
            ptr = dj * j + dl * l;
            i = 0 as libc::c_int;
            while i <= li {
                n = ptr;
                while n < ptr + nroots {
                    *fx.offset(n as isize) = ak2 * *p2x.offset(n as isize);
                    *fy.offset(n as isize) = ak2 * *p2y.offset(n as isize);
                    *fz.offset(n as isize) = ak2 * *p2z.offset(n as isize);
                    n += 1;
                    n;
                }
                ptr += di;
                i += 1;
                i;
            }
            k = 1 as libc::c_int;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0 as libc::c_int;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx
                            .offset(
                                n as isize,
                            ) = k as libc::c_double * *p1x.offset(n as isize)
                            + ak2 * *p2x.offset(n as isize);
                        *fy
                            .offset(
                                n as isize,
                            ) = k as libc::c_double * *p1y.offset(n as isize)
                            + ak2 * *p2y.offset(n as isize);
                        *fz
                            .offset(
                                n as isize,
                            ) = k as libc::c_double * *p1z.offset(n as isize)
                            + ak2 * *p2z.offset(n as isize);
                        n += 1;
                        n;
                    }
                    ptr += di;
                    i += 1;
                    i;
                }
                k += 1;
                k;
            }
            l += 1;
            l;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1l_2e(
    mut f: *mut libc::c_double,
    mut g: *const libc::c_double,
    li: libc::c_int,
    lj: libc::c_int,
    lk: libc::c_int,
    ll: libc::c_int,
    mut envs: *const CINTEnvVars,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let di: libc::c_int = (*envs).g_stride_i;
    let dk: libc::c_int = (*envs).g_stride_k;
    let dl: libc::c_int = (*envs).g_stride_l;
    let dj: libc::c_int = (*envs).g_stride_j;
    let nroots: libc::c_int = (*envs).nrys_roots;
    let al2: libc::c_double = -(2 as libc::c_int) as libc::c_double
        * (*envs).al[0 as libc::c_int as usize];
    let mut gx: *const libc::c_double = g;
    let mut gy: *const libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut fx: *mut libc::c_double = f;
    let mut fy: *mut libc::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut libc::c_double = f
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut p1x: *const libc::c_double = gx.offset(-(dl as isize));
    let mut p1y: *const libc::c_double = gy.offset(-(dl as isize));
    let mut p1z: *const libc::c_double = gz.offset(-(dl as isize));
    let mut p2x: *const libc::c_double = gx.offset(dl as isize);
    let mut p2y: *const libc::c_double = gy.offset(dl as isize);
    let mut p2z: *const libc::c_double = gz.offset(dl as isize);
    j = 0 as libc::c_int;
    while j <= lj {
        k = 0 as libc::c_int;
        while k <= lk {
            ptr = dj * j + dk * k;
            i = 0 as libc::c_int;
            while i <= li {
                n = ptr;
                while n < ptr + nroots {
                    *fx.offset(n as isize) = al2 * *p2x.offset(n as isize);
                    *fy.offset(n as isize) = al2 * *p2y.offset(n as isize);
                    *fz.offset(n as isize) = al2 * *p2z.offset(n as isize);
                    n += 1;
                    n;
                }
                ptr += di;
                i += 1;
                i;
            }
            k += 1;
            k;
        }
        l = 1 as libc::c_int;
        while l <= ll {
            k = 0 as libc::c_int;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0 as libc::c_int;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx
                            .offset(
                                n as isize,
                            ) = l as libc::c_double * *p1x.offset(n as isize)
                            + al2 * *p2x.offset(n as isize);
                        *fy
                            .offset(
                                n as isize,
                            ) = l as libc::c_double * *p1y.offset(n as isize)
                            + al2 * *p2y.offset(n as isize);
                        *fz
                            .offset(
                                n as isize,
                            ) = l as libc::c_double * *p1z.offset(n as isize)
                            + al2 * *p2z.offset(n as isize);
                        n += 1;
                        n;
                    }
                    i += 1;
                    i;
                    ptr += di;
                }
                k += 1;
                k;
            }
            l += 1;
            l;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1i_2e(
    mut f: *mut libc::c_double,
    mut g: *const libc::c_double,
    mut ri: *const libc::c_double,
    li: libc::c_int,
    lj: libc::c_int,
    lk: libc::c_int,
    ll: libc::c_int,
    mut envs: *const CINTEnvVars,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let di: libc::c_int = (*envs).g_stride_i;
    let dk: libc::c_int = (*envs).g_stride_k;
    let dl: libc::c_int = (*envs).g_stride_l;
    let dj: libc::c_int = (*envs).g_stride_j;
    let nroots: libc::c_int = (*envs).nrys_roots;
    let mut gx: *const libc::c_double = g;
    let mut gy: *const libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut fx: *mut libc::c_double = f;
    let mut fy: *mut libc::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut libc::c_double = f
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut p1x: *const libc::c_double = gx.offset(di as isize);
    let mut p1y: *const libc::c_double = gy.offset(di as isize);
    let mut p1z: *const libc::c_double = gz.offset(di as isize);
    j = 0 as libc::c_int;
    while j <= lj {
        l = 0 as libc::c_int;
        while l <= ll {
            k = 0 as libc::c_int;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0 as libc::c_int;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx
                            .offset(
                                n as isize,
                            ) = *p1x.offset(n as isize)
                            + *ri.offset(0 as libc::c_int as isize)
                                * *gx.offset(n as isize);
                        *fy
                            .offset(
                                n as isize,
                            ) = *p1y.offset(n as isize)
                            + *ri.offset(1 as libc::c_int as isize)
                                * *gy.offset(n as isize);
                        *fz
                            .offset(
                                n as isize,
                            ) = *p1z.offset(n as isize)
                            + *ri.offset(2 as libc::c_int as isize)
                                * *gz.offset(n as isize);
                        n += 1;
                        n;
                    }
                    ptr += di;
                    i += 1;
                    i;
                }
                k += 1;
                k;
            }
            l += 1;
            l;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1j_2e(
    mut f: *mut libc::c_double,
    mut g: *const libc::c_double,
    mut rj: *const libc::c_double,
    li: libc::c_int,
    lj: libc::c_int,
    lk: libc::c_int,
    ll: libc::c_int,
    mut envs: *const CINTEnvVars,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let di: libc::c_int = (*envs).g_stride_i;
    let dk: libc::c_int = (*envs).g_stride_k;
    let dl: libc::c_int = (*envs).g_stride_l;
    let dj: libc::c_int = (*envs).g_stride_j;
    let nroots: libc::c_int = (*envs).nrys_roots;
    let mut gx: *const libc::c_double = g;
    let mut gy: *const libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut fx: *mut libc::c_double = f;
    let mut fy: *mut libc::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut libc::c_double = f
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut p1x: *const libc::c_double = gx.offset(dj as isize);
    let mut p1y: *const libc::c_double = gy.offset(dj as isize);
    let mut p1z: *const libc::c_double = gz.offset(dj as isize);
    j = 0 as libc::c_int;
    while j <= lj {
        l = 0 as libc::c_int;
        while l <= ll {
            k = 0 as libc::c_int;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0 as libc::c_int;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx
                            .offset(
                                n as isize,
                            ) = *p1x.offset(n as isize)
                            + *rj.offset(0 as libc::c_int as isize)
                                * *gx.offset(n as isize);
                        *fy
                            .offset(
                                n as isize,
                            ) = *p1y.offset(n as isize)
                            + *rj.offset(1 as libc::c_int as isize)
                                * *gy.offset(n as isize);
                        *fz
                            .offset(
                                n as isize,
                            ) = *p1z.offset(n as isize)
                            + *rj.offset(2 as libc::c_int as isize)
                                * *gz.offset(n as isize);
                        n += 1;
                        n;
                    }
                    ptr += di;
                    i += 1;
                    i;
                }
                k += 1;
                k;
            }
            l += 1;
            l;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1k_2e(
    mut f: *mut libc::c_double,
    mut g: *const libc::c_double,
    mut rk: *const libc::c_double,
    li: libc::c_int,
    lj: libc::c_int,
    lk: libc::c_int,
    ll: libc::c_int,
    mut envs: *const CINTEnvVars,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let di: libc::c_int = (*envs).g_stride_i;
    let dk: libc::c_int = (*envs).g_stride_k;
    let dl: libc::c_int = (*envs).g_stride_l;
    let dj: libc::c_int = (*envs).g_stride_j;
    let nroots: libc::c_int = (*envs).nrys_roots;
    let mut gx: *const libc::c_double = g;
    let mut gy: *const libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut fx: *mut libc::c_double = f;
    let mut fy: *mut libc::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut libc::c_double = f
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut p1x: *const libc::c_double = gx.offset(dk as isize);
    let mut p1y: *const libc::c_double = gy.offset(dk as isize);
    let mut p1z: *const libc::c_double = gz.offset(dk as isize);
    j = 0 as libc::c_int;
    while j <= lj {
        l = 0 as libc::c_int;
        while l <= ll {
            k = 0 as libc::c_int;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0 as libc::c_int;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx
                            .offset(
                                n as isize,
                            ) = *p1x.offset(n as isize)
                            + *rk.offset(0 as libc::c_int as isize)
                                * *gx.offset(n as isize);
                        *fy
                            .offset(
                                n as isize,
                            ) = *p1y.offset(n as isize)
                            + *rk.offset(1 as libc::c_int as isize)
                                * *gy.offset(n as isize);
                        *fz
                            .offset(
                                n as isize,
                            ) = *p1z.offset(n as isize)
                            + *rk.offset(2 as libc::c_int as isize)
                                * *gz.offset(n as isize);
                        n += 1;
                        n;
                    }
                    ptr += di;
                    i += 1;
                    i;
                }
                k += 1;
                k;
            }
            l += 1;
            l;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1l_2e(
    mut f: *mut libc::c_double,
    mut g: *const libc::c_double,
    mut rl: *const libc::c_double,
    li: libc::c_int,
    lj: libc::c_int,
    lk: libc::c_int,
    ll: libc::c_int,
    mut envs: *const CINTEnvVars,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let di: libc::c_int = (*envs).g_stride_i;
    let dk: libc::c_int = (*envs).g_stride_k;
    let dl: libc::c_int = (*envs).g_stride_l;
    let dj: libc::c_int = (*envs).g_stride_j;
    let nroots: libc::c_int = (*envs).nrys_roots;
    let mut gx: *const libc::c_double = g;
    let mut gy: *const libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut fx: *mut libc::c_double = f;
    let mut fy: *mut libc::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut libc::c_double = f
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut p1x: *const libc::c_double = gx.offset(dl as isize);
    let mut p1y: *const libc::c_double = gy.offset(dl as isize);
    let mut p1z: *const libc::c_double = gz.offset(dl as isize);
    j = 0 as libc::c_int;
    while j <= lj {
        l = 0 as libc::c_int;
        while l <= ll {
            k = 0 as libc::c_int;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0 as libc::c_int;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx
                            .offset(
                                n as isize,
                            ) = *p1x.offset(n as isize)
                            + *rl.offset(0 as libc::c_int as isize)
                                * *gx.offset(n as isize);
                        *fy
                            .offset(
                                n as isize,
                            ) = *p1y.offset(n as isize)
                            + *rl.offset(1 as libc::c_int as isize)
                                * *gy.offset(n as isize);
                        *fz
                            .offset(
                                n as isize,
                            ) = *p1z.offset(n as isize)
                            + *rl.offset(2 as libc::c_int as isize)
                                * *gz.offset(n as isize);
                        n += 1;
                        n;
                    }
                    ptr += di;
                    i += 1;
                    i;
                }
                k += 1;
                k;
            }
            l += 1;
            l;
        }
        j += 1;
        j;
    }
}
