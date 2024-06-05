#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::g1e::CINTcommon_fac_sp;
use crate::g2e::CINTg0_2e;
use crate::g2e::CINTg0_2e_2d;
use crate::g2e::CINTg0_2e_2d4d_unrolled;
use crate::g2e::CINTsrg0_2e_2d4d_unrolled;

use crate::cint::CINTEnvVars;
use crate::g2e::Rys2eT;

#[no_mangle]
pub unsafe extern "C" fn CINTinit_int2c2e_EnvVars(
    mut envs: *mut CINTEnvVars,
    mut ng: *mut i32,
    mut shls: *mut i32,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
) {
    (*envs).natm = natm;
    (*envs).nbas = nbas;
    (*envs).atm = atm;
    (*envs).bas = bas;
    (*envs).env = env;
    (*envs).shls = shls;
    let i_sh: i32 = *shls.offset(0 as i32 as isize);
    let k_sh: i32 = *shls.offset(1 as i32 as isize);
    (*envs).i_l = *bas.offset((8 as i32 * i_sh + 1 as i32) as isize);
    (*envs).j_l = 0 as i32;
    (*envs).k_l = *bas.offset((8 as i32 * k_sh + 1 as i32) as isize);
    (*envs).l_l = 0 as i32;
    (*envs)
        .x_ctr[0 as i32
        as usize] = *bas.offset((8 as i32 * i_sh + 3 as i32) as isize);
    (*envs)
        .x_ctr[1 as i32
        as usize] = *bas.offset((8 as i32 * k_sh + 3 as i32) as isize);
    (*envs).x_ctr[2 as i32 as usize] = 1 as i32;
    (*envs).x_ctr[3 as i32 as usize] = 1 as i32;
    (*envs)
        .nfi = ((*envs).i_l + 1 as i32) * ((*envs).i_l + 2 as i32)
        / 2 as i32;
    (*envs).nfj = 1 as i32;
    (*envs)
        .c2rust_unnamed
        .nfk = ((*envs).k_l + 1 as i32) * ((*envs).k_l + 2 as i32)
        / 2 as i32;
    (*envs).c2rust_unnamed_0.nfl = 1 as i32;
    (*envs).nf = (*envs).nfi * (*envs).c2rust_unnamed.nfk;
    (*envs)
        .ri = env
        .offset(
            *atm
                .offset(
                    (6 as i32
                        * *bas
                            .offset(
                                (8 as i32 * i_sh + 0 as i32) as isize,
                            ) + 1 as i32) as isize,
                ) as isize,
        );
    (*envs)
        .rk = env
        .offset(
            *atm
                .offset(
                    (6 as i32
                        * *bas
                            .offset(
                                (8 as i32 * k_sh + 0 as i32) as isize,
                            ) + 1 as i32) as isize,
                ) as isize,
        );
    (*envs)
        .common_factor = 3.14159265358979323846f64 * 3.14159265358979323846f64
        * 3.14159265358979323846f64 * 2 as i32 as f64
        / 1.7724538509055160272981674833411451f64 * CINTcommon_fac_sp((*envs).i_l)
        * CINTcommon_fac_sp((*envs).k_l);
    if *env.offset(0 as i32 as isize) == 0 as i32 as f64 {
        (*envs).expcutoff = 60 as i32 as f64;
    } else {
        (*envs)
            .expcutoff = if 40 as i32 as f64
            > *env.offset(0 as i32 as isize)
        {
            40 as i32 as f64
        } else {
            *env.offset(0 as i32 as isize)
        };
    }
    (*envs).gbits = *ng.offset(4 as i32 as isize);
    (*envs).ncomp_e1 = *ng.offset(5 as i32 as isize);
    (*envs).ncomp_e2 = *ng.offset(6 as i32 as isize);
    (*envs).ncomp_tensor = *ng.offset(7 as i32 as isize);
    (*envs).li_ceil = (*envs).i_l + *ng.offset(0 as i32 as isize);
    (*envs).lj_ceil = 0 as i32;
    (*envs).lk_ceil = (*envs).k_l + *ng.offset(2 as i32 as isize);
    (*envs).ll_ceil = 0 as i32;
    let mut rys_order: i32 = ((*envs).li_ceil + (*envs).lk_ceil)
        / 2 as i32 + 1 as i32;
    let mut nrys_roots: i32 = rys_order;
    let mut omega: f64 = *env.offset(8 as i32 as isize);
    if omega < 0 as i32 as f64 && rys_order <= 3 as i32 {
        nrys_roots *= 2 as i32;
    }
    (*envs).rys_order = rys_order;
    (*envs).nrys_roots = nrys_roots;
    let mut dli: i32 = (*envs).li_ceil + 1 as i32;
    let mut dlk: i32 = (*envs).lk_ceil + 1 as i32;
    (*envs).g_stride_i = nrys_roots;
    (*envs).g_stride_k = nrys_roots * dli;
    (*envs).g_stride_l = (*envs).g_stride_k;
    (*envs).g_size = nrys_roots * dli * dlk;
    (*envs).aj[0 as i32 as usize] = 0 as i32 as f64;
    (*envs).al[0 as i32 as usize] = 0 as i32 as f64;
    (*envs)
        .rij[0 as i32
        as usize] = *((*envs).ri).offset(0 as i32 as isize);
    (*envs)
        .rij[1 as i32
        as usize] = *((*envs).ri).offset(1 as i32 as isize);
    (*envs)
        .rij[2 as i32
        as usize] = *((*envs).ri).offset(2 as i32 as isize);
    (*envs)
        .rkl[0 as i32
        as usize] = *((*envs).rk).offset(0 as i32 as isize);
    (*envs)
        .rkl[1 as i32
        as usize] = *((*envs).rk).offset(1 as i32 as isize);
    (*envs)
        .rkl[2 as i32
        as usize] = *((*envs).rk).offset(2 as i32 as isize);
    (*envs).g2d_ijmax = (*envs).g_stride_i;
    (*envs).g2d_klmax = (*envs).g_stride_k;
    (*envs)
        .rkrl[0 as i32
        as usize] = *((*envs).rk).offset(0 as i32 as isize);
    (*envs)
        .rkrl[1 as i32
        as usize] = *((*envs).rk).offset(1 as i32 as isize);
    (*envs)
        .rkrl[2 as i32
        as usize] = *((*envs).rk).offset(2 as i32 as isize);
    (*envs)
        .rirj[0 as i32
        as usize] = *((*envs).ri).offset(0 as i32 as isize);
    (*envs)
        .rirj[1 as i32
        as usize] = *((*envs).ri).offset(1 as i32 as isize);
    (*envs)
        .rirj[2 as i32
        as usize] = *((*envs).ri).offset(2 as i32 as isize);
    (*envs).rx_in_rklrx = (*envs).rk;
    (*envs).rx_in_rijrx = (*envs).ri;
    if rys_order <= 2 as i32 {
        (*envs)
            .f_g0_2d4d = ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut f64,
                    *mut Rys2eT,
                    *mut CINTEnvVars,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg0_2e_2d4d_unrolled
                    as unsafe extern "C" fn(
                        *mut f64,
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
                        *mut f64,
                        *mut Rys2eT,
                        *mut CINTEnvVars,
                    ) -> (),
                >,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    CINTsrg0_2e_2d4d_unrolled
                        as unsafe extern "C" fn(
                            *mut f64,
                            *mut Rys2eT,
                            *mut CINTEnvVars,
                        ) -> (),
                ),
            );
        }
    } else {
        (*envs)
            .f_g0_2d4d = ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut f64,
                    *mut Rys2eT,
                    *mut CINTEnvVars,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg0_2e_2d
                    as unsafe extern "C" fn(
                        *mut f64,
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
                *mut f64,
                *mut f64,
                *mut f64,
                f64,
                *mut CINTEnvVars,
            ) -> i32,
        >,
        Option::<unsafe extern "C" fn() -> i32>,
    >(
        Some(
            CINTg0_2e
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut f64,
                    f64,
                    *mut CINTEnvVars,
                ) -> i32,
        ),
    );
    (*envs).j_l = (*envs).k_l;
    (*envs).nfj = (*envs).c2rust_unnamed.nfk;
    (*envs).g_stride_j = (*envs).g_stride_k;
}
