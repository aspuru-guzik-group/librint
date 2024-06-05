#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::g3c1e::CINTinit_int3c1e_EnvVars;
use crate::g1e::CINTnabla1i_1e;
use crate::cart2sph::c2s_sph_3c1e;
use crate::cart2sph::c2s_cart_3c1e;
use crate::optimizer::CINTall_3c1e_optimizer;
use crate::cint3c1e::CINT3c1e_drv;

use crate::cint::CINTOpt;
use crate::cint::CINTEnvVars;


unsafe extern "C" fn CINTgout1e_int3c1e_r2_origk(
    mut gout: *mut f64,
    mut g: *mut f64,
    mut idx: *mut i32,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: i32,
) {
    let mut nf: i32 = (*envs).nf;
    let mut ix: i32 = 0;
    let mut iy: i32 = 0;
    let mut iz: i32 = 0;
    let mut n: i32 = 0;
    let mut g0: *mut f64 = g;
    let mut g1: *mut f64 = g0
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g2: *mut f64 = g1
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g3: *mut f64 = g2
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut s: f64 = 0.;
    g1 = g0.offset((*envs).g_stride_k as isize);
    g3 = g1.offset((*envs).g_stride_k as isize);
    n = 0 as i32;
    while n < nf {
        ix = *idx.offset((0 as i32 + n * 3 as i32) as isize);
        iy = *idx.offset((1 as i32 + n * 3 as i32) as isize);
        iz = *idx.offset((2 as i32 + n * 3 as i32) as isize);
        s = *g3.offset((ix + 0 as i32) as isize)
            * *g0.offset((iy + 0 as i32) as isize)
            * *g0.offset((iz + 0 as i32) as isize);
        s
            += *g0.offset((ix + 0 as i32) as isize)
                * *g3.offset((iy + 0 as i32) as isize)
                * *g0.offset((iz + 0 as i32) as isize);
        s
            += *g0.offset((ix + 0 as i32) as isize)
                * *g0.offset((iy + 0 as i32) as isize)
                * *g3.offset((iz + 0 as i32) as isize);
        if gout_empty != 0 {
            *gout.offset(n as isize) = s;
        } else {
            *gout.offset(n as isize) += s;
        }
        n += 1;
        n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_r2_origk_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
) {
    let mut ng: [i32; 8] = [
        0 as i32,
        0 as i32,
        2 as i32,
        0 as i32,
        2 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    CINTall_3c1e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_r2_origk_cart(
    mut out: *mut f64,
    mut dims: *mut i32,
    mut shls: *mut i32,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
    mut opt: *mut CINTOpt,
    mut cache: *mut f64,
) -> i32 {
    let mut ng: [i32; 8] = [
        0 as i32,
        0 as i32,
        2 as i32,
        0 as i32,
        2 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int3c1e_EnvVars(
        &mut envs,
        ng.as_mut_ptr(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs
        .f_gout = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut f64,
                *mut f64,
                *mut i32,
                *mut CINTEnvVars,
                i32,
            ) -> (),
        >,
        Option::<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int3c1e_r2_origk
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
        out,
        dims,
        &mut envs,
        opt,
        cache,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    *mut f64,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_3c1e
                    as unsafe extern "C" fn(
                        *mut f64,
                        *mut f64,
                        *mut i32,
                        *mut CINTEnvVars,
                        *mut f64,
                    ) -> (),
            ),
        ),
        0 as i32,
        0 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_r2_origk_sph(
    mut out: *mut f64,
    mut dims: *mut i32,
    mut shls: *mut i32,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
    mut opt: *mut CINTOpt,
    mut cache: *mut f64,
) -> i32 {
    let mut ng: [i32; 8] = [
        0 as i32,
        0 as i32,
        2 as i32,
        0 as i32,
        2 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int3c1e_EnvVars(
        &mut envs,
        ng.as_mut_ptr(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs
        .f_gout = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut f64,
                *mut f64,
                *mut i32,
                *mut CINTEnvVars,
                i32,
            ) -> (),
        >,
        Option::<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int3c1e_r2_origk
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
        out,
        dims,
        &mut envs,
        opt,
        cache,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    *mut f64,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_3c1e
                    as unsafe extern "C" fn(
                        *mut f64,
                        *mut f64,
                        *mut i32,
                        *mut CINTEnvVars,
                        *mut f64,
                    ) -> (),
            ),
        ),
        0 as i32,
        0 as i32,
    );
}
unsafe extern "C" fn CINTgout1e_int3c1e_r4_origk(
    mut gout: *mut f64,
    mut g: *mut f64,
    mut idx: *mut i32,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: i32,
) {
    let mut nf: i32 = (*envs).nf;
    let mut ix: i32 = 0;
    let mut iy: i32 = 0;
    let mut iz: i32 = 0;
    let mut n: i32 = 0;
    let mut g0: *mut f64 = g;
    let mut g1: *mut f64 = g0
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g2: *mut f64 = g1
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g3: *mut f64 = g2
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g4: *mut f64 = g3
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g5: *mut f64 = g4
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g6: *mut f64 = g5
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g7: *mut f64 = g6
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g8: *mut f64 = g7
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g9: *mut f64 = g8
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g10: *mut f64 = g9
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g11: *mut f64 = g10
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g12: *mut f64 = g11
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g13: *mut f64 = g12
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g14: *mut f64 = g13
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g15: *mut f64 = g14
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut s: f64 = 0.;
    g1 = g0.offset((*envs).g_stride_k as isize);
    g3 = g1.offset((*envs).g_stride_k as isize);
    g4 = g0.offset((*envs).g_stride_k as isize);
    g7 = g3.offset((*envs).g_stride_k as isize);
    g12 = g4.offset((*envs).g_stride_k as isize);
    g15 = g7.offset((*envs).g_stride_k as isize);
    n = 0 as i32;
    while n < nf {
        ix = *idx.offset((0 as i32 + n * 3 as i32) as isize);
        iy = *idx.offset((1 as i32 + n * 3 as i32) as isize);
        iz = *idx.offset((2 as i32 + n * 3 as i32) as isize);
        s = *g15.offset((ix + 0 as i32) as isize)
            * *g0.offset((iy + 0 as i32) as isize)
            * *g0.offset((iz + 0 as i32) as isize);
        s
            += *g12.offset((ix + 0 as i32) as isize)
                * *g3.offset((iy + 0 as i32) as isize)
                * *g0.offset((iz + 0 as i32) as isize)
                * 2 as i32 as f64;
        s
            += *g12.offset((ix + 0 as i32) as isize)
                * *g0.offset((iy + 0 as i32) as isize)
                * *g3.offset((iz + 0 as i32) as isize)
                * 2 as i32 as f64;
        s
            += *g0.offset((ix + 0 as i32) as isize)
                * *g15.offset((iy + 0 as i32) as isize)
                * *g0.offset((iz + 0 as i32) as isize);
        s
            += *g0.offset((ix + 0 as i32) as isize)
                * *g12.offset((iy + 0 as i32) as isize)
                * *g3.offset((iz + 0 as i32) as isize)
                * 2 as i32 as f64;
        s
            += *g0.offset((ix + 0 as i32) as isize)
                * *g0.offset((iy + 0 as i32) as isize)
                * *g15.offset((iz + 0 as i32) as isize);
        if gout_empty != 0 {
            *gout.offset(n as isize) = s;
        } else {
            *gout.offset(n as isize) += s;
        }
        n += 1;
        n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_r4_origk_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
) {
    let mut ng: [i32; 8] = [
        0 as i32,
        0 as i32,
        4 as i32,
        0 as i32,
        4 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    CINTall_3c1e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_r4_origk_cart(
    mut out: *mut f64,
    mut dims: *mut i32,
    mut shls: *mut i32,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
    mut opt: *mut CINTOpt,
    mut cache: *mut f64,
) -> i32 {
    let mut ng: [i32; 8] = [
        0 as i32,
        0 as i32,
        4 as i32,
        0 as i32,
        4 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int3c1e_EnvVars(
        &mut envs,
        ng.as_mut_ptr(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs
        .f_gout = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut f64,
                *mut f64,
                *mut i32,
                *mut CINTEnvVars,
                i32,
            ) -> (),
        >,
        Option::<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int3c1e_r4_origk
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
        out,
        dims,
        &mut envs,
        opt,
        cache,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    *mut f64,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_3c1e
                    as unsafe extern "C" fn(
                        *mut f64,
                        *mut f64,
                        *mut i32,
                        *mut CINTEnvVars,
                        *mut f64,
                    ) -> (),
            ),
        ),
        0 as i32,
        0 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_r4_origk_sph(
    mut out: *mut f64,
    mut dims: *mut i32,
    mut shls: *mut i32,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
    mut opt: *mut CINTOpt,
    mut cache: *mut f64,
) -> i32 {
    let mut ng: [i32; 8] = [
        0 as i32,
        0 as i32,
        4 as i32,
        0 as i32,
        4 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int3c1e_EnvVars(
        &mut envs,
        ng.as_mut_ptr(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs
        .f_gout = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut f64,
                *mut f64,
                *mut i32,
                *mut CINTEnvVars,
                i32,
            ) -> (),
        >,
        Option::<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int3c1e_r4_origk
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
        out,
        dims,
        &mut envs,
        opt,
        cache,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    *mut f64,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_3c1e
                    as unsafe extern "C" fn(
                        *mut f64,
                        *mut f64,
                        *mut i32,
                        *mut CINTEnvVars,
                        *mut f64,
                    ) -> (),
            ),
        ),
        0 as i32,
        0 as i32,
    );
}
unsafe extern "C" fn CINTgout1e_int3c1e_r6_origk(
    mut gout: *mut f64,
    mut g: *mut f64,
    mut idx: *mut i32,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: i32,
) {
    let mut nf: i32 = (*envs).nf;
    let mut ix: i32 = 0;
    let mut iy: i32 = 0;
    let mut iz: i32 = 0;
    let mut n: i32 = 0;
    let mut g0: *mut f64 = g;
    let mut g1: *mut f64 = g0
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g2: *mut f64 = g1
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g3: *mut f64 = g2
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g4: *mut f64 = g3
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g5: *mut f64 = g4
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g6: *mut f64 = g5
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g7: *mut f64 = g6
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g8: *mut f64 = g7
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g9: *mut f64 = g8
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g10: *mut f64 = g9
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g11: *mut f64 = g10
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g12: *mut f64 = g11
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g13: *mut f64 = g12
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g14: *mut f64 = g13
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g15: *mut f64 = g14
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g16: *mut f64 = g15
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g17: *mut f64 = g16
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g18: *mut f64 = g17
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g19: *mut f64 = g18
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g20: *mut f64 = g19
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g21: *mut f64 = g20
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g22: *mut f64 = g21
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g23: *mut f64 = g22
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g24: *mut f64 = g23
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g25: *mut f64 = g24
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g26: *mut f64 = g25
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g27: *mut f64 = g26
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g28: *mut f64 = g27
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g29: *mut f64 = g28
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g30: *mut f64 = g29
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g31: *mut f64 = g30
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g32: *mut f64 = g31
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g33: *mut f64 = g32
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g34: *mut f64 = g33
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g35: *mut f64 = g34
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g36: *mut f64 = g35
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g37: *mut f64 = g36
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g38: *mut f64 = g37
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g39: *mut f64 = g38
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g40: *mut f64 = g39
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g41: *mut f64 = g40
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g42: *mut f64 = g41
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g43: *mut f64 = g42
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g44: *mut f64 = g43
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g45: *mut f64 = g44
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g46: *mut f64 = g45
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g47: *mut f64 = g46
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g48: *mut f64 = g47
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g49: *mut f64 = g48
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g50: *mut f64 = g49
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g51: *mut f64 = g50
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g52: *mut f64 = g51
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g53: *mut f64 = g52
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g54: *mut f64 = g53
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g55: *mut f64 = g54
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g56: *mut f64 = g55
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g57: *mut f64 = g56
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g58: *mut f64 = g57
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g59: *mut f64 = g58
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g60: *mut f64 = g59
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g61: *mut f64 = g60
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g62: *mut f64 = g61
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g63: *mut f64 = g62
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut s: f64 = 0.;
    g1 = g0.offset((*envs).g_stride_k as isize);
    g3 = g1.offset((*envs).g_stride_k as isize);
    g4 = g0.offset((*envs).g_stride_k as isize);
    g7 = g3.offset((*envs).g_stride_k as isize);
    g12 = g4.offset((*envs).g_stride_k as isize);
    g15 = g7.offset((*envs).g_stride_k as isize);
    g16 = g0.offset((*envs).g_stride_k as isize);
    g28 = g12.offset((*envs).g_stride_k as isize);
    g31 = g15.offset((*envs).g_stride_k as isize);
    g48 = g16.offset((*envs).g_stride_k as isize);
    g60 = g28.offset((*envs).g_stride_k as isize);
    g63 = g31.offset((*envs).g_stride_k as isize);
    n = 0 as i32;
    while n < nf {
        ix = *idx.offset((0 as i32 + n * 3 as i32) as isize);
        iy = *idx.offset((1 as i32 + n * 3 as i32) as isize);
        iz = *idx.offset((2 as i32 + n * 3 as i32) as isize);
        s = *g63.offset((ix + 0 as i32) as isize)
            * *g0.offset((iy + 0 as i32) as isize)
            * *g0.offset((iz + 0 as i32) as isize);
        s
            += *g60.offset((ix + 0 as i32) as isize)
                * *g3.offset((iy + 0 as i32) as isize)
                * *g0.offset((iz + 0 as i32) as isize)
                * 3 as i32 as f64;
        s
            += *g60.offset((ix + 0 as i32) as isize)
                * *g0.offset((iy + 0 as i32) as isize)
                * *g3.offset((iz + 0 as i32) as isize)
                * 3 as i32 as f64;
        s
            += *g48.offset((ix + 0 as i32) as isize)
                * *g15.offset((iy + 0 as i32) as isize)
                * *g0.offset((iz + 0 as i32) as isize)
                * 3 as i32 as f64;
        s
            += *g48.offset((ix + 0 as i32) as isize)
                * *g12.offset((iy + 0 as i32) as isize)
                * *g3.offset((iz + 0 as i32) as isize)
                * 6 as i32 as f64;
        s
            += *g48.offset((ix + 0 as i32) as isize)
                * *g0.offset((iy + 0 as i32) as isize)
                * *g15.offset((iz + 0 as i32) as isize)
                * 3 as i32 as f64;
        s
            += *g0.offset((ix + 0 as i32) as isize)
                * *g63.offset((iy + 0 as i32) as isize)
                * *g0.offset((iz + 0 as i32) as isize);
        s
            += *g0.offset((ix + 0 as i32) as isize)
                * *g60.offset((iy + 0 as i32) as isize)
                * *g3.offset((iz + 0 as i32) as isize)
                * 3 as i32 as f64;
        s
            += *g0.offset((ix + 0 as i32) as isize)
                * *g48.offset((iy + 0 as i32) as isize)
                * *g15.offset((iz + 0 as i32) as isize)
                * 3 as i32 as f64;
        s
            += *g0.offset((ix + 0 as i32) as isize)
                * *g0.offset((iy + 0 as i32) as isize)
                * *g63.offset((iz + 0 as i32) as isize);
        if gout_empty != 0 {
            *gout.offset(n as isize) = s;
        } else {
            *gout.offset(n as isize) += s;
        }
        n += 1;
        n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_r6_origk_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
) {
    let mut ng: [i32; 8] = [
        0 as i32,
        0 as i32,
        6 as i32,
        0 as i32,
        6 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    CINTall_3c1e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_r6_origk_cart(
    mut out: *mut f64,
    mut dims: *mut i32,
    mut shls: *mut i32,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
    mut opt: *mut CINTOpt,
    mut cache: *mut f64,
) -> i32 {
    let mut ng: [i32; 8] = [
        0 as i32,
        0 as i32,
        6 as i32,
        0 as i32,
        6 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int3c1e_EnvVars(
        &mut envs,
        ng.as_mut_ptr(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs
        .f_gout = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut f64,
                *mut f64,
                *mut i32,
                *mut CINTEnvVars,
                i32,
            ) -> (),
        >,
        Option::<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int3c1e_r6_origk
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
        out,
        dims,
        &mut envs,
        opt,
        cache,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    *mut f64,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_3c1e
                    as unsafe extern "C" fn(
                        *mut f64,
                        *mut f64,
                        *mut i32,
                        *mut CINTEnvVars,
                        *mut f64,
                    ) -> (),
            ),
        ),
        0 as i32,
        0 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_r6_origk_sph(
    mut out: *mut f64,
    mut dims: *mut i32,
    mut shls: *mut i32,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
    mut opt: *mut CINTOpt,
    mut cache: *mut f64,
) -> i32 {
    let mut ng: [i32; 8] = [
        0 as i32,
        0 as i32,
        6 as i32,
        0 as i32,
        6 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int3c1e_EnvVars(
        &mut envs,
        ng.as_mut_ptr(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs
        .f_gout = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut f64,
                *mut f64,
                *mut i32,
                *mut CINTEnvVars,
                i32,
            ) -> (),
        >,
        Option::<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int3c1e_r6_origk
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
        out,
        dims,
        &mut envs,
        opt,
        cache,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    *mut f64,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_3c1e
                    as unsafe extern "C" fn(
                        *mut f64,
                        *mut f64,
                        *mut i32,
                        *mut CINTEnvVars,
                        *mut f64,
                    ) -> (),
            ),
        ),
        0 as i32,
        0 as i32,
    );
}
unsafe extern "C" fn CINTgout1e_int3c1e_ip1_r2_origk(
    mut gout: *mut f64,
    mut g: *mut f64,
    mut idx: *mut i32,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: i32,
) {
    let mut nf: i32 = (*envs).nf;
    let mut ix: i32 = 0;
    let mut iy: i32 = 0;
    let mut iz: i32 = 0;
    let mut n: i32 = 0;
    let mut g0: *mut f64 = g;
    let mut g1: *mut f64 = g0
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g2: *mut f64 = g1
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g3: *mut f64 = g2
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g4: *mut f64 = g3
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g5: *mut f64 = g4
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g6: *mut f64 = g5
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g7: *mut f64 = g6
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut s: [f64; 3] = [0.; 3];
    g1 = g0.offset((*envs).g_stride_k as isize);
    g3 = g1.offset((*envs).g_stride_k as isize);
    CINTnabla1i_1e(
        g4,
        g0,
        (*envs).i_l + 0 as i32,
        (*envs).j_l,
        (*envs).k_l,
        envs,
    );
    CINTnabla1i_1e(
        g7,
        g3,
        (*envs).i_l + 0 as i32,
        (*envs).j_l,
        (*envs).k_l,
        envs,
    );
    n = 0 as i32;
    while n < nf {
        ix = *idx.offset((0 as i32 + n * 3 as i32) as isize);
        iy = *idx.offset((1 as i32 + n * 3 as i32) as isize);
        iz = *idx.offset((2 as i32 + n * 3 as i32) as isize);
        s[0 as i32
            as usize] = *g7.offset(ix as isize) * *g0.offset(iy as isize)
            * *g0.offset(iz as isize)
            + *g4.offset(ix as isize) * *g3.offset(iy as isize) * *g0.offset(iz as isize)
            + *g4.offset(ix as isize) * *g0.offset(iy as isize)
                * *g3.offset(iz as isize);
        s[1 as i32
            as usize] = *g3.offset(ix as isize) * *g4.offset(iy as isize)
            * *g0.offset(iz as isize)
            + *g0.offset(ix as isize) * *g7.offset(iy as isize) * *g0.offset(iz as isize)
            + *g0.offset(ix as isize) * *g4.offset(iy as isize)
                * *g3.offset(iz as isize);
        s[2 as i32
            as usize] = *g3.offset(ix as isize) * *g0.offset(iy as isize)
            * *g4.offset(iz as isize)
            + *g0.offset(ix as isize) * *g3.offset(iy as isize) * *g4.offset(iz as isize)
            + *g0.offset(ix as isize) * *g0.offset(iy as isize)
                * *g7.offset(iz as isize);
        if gout_empty != 0 {
            *gout.offset((n * 3 as i32) as isize) = s[0 as i32 as usize];
            *gout
                .offset(
                    (n * 3 as i32 + 1 as i32) as isize,
                ) = s[1 as i32 as usize];
            *gout
                .offset(
                    (n * 3 as i32 + 2 as i32) as isize,
                ) = s[2 as i32 as usize];
        } else {
            *gout.offset((n * 3 as i32) as isize)
                += s[0 as i32 as usize];
            *gout.offset((n * 3 as i32 + 1 as i32) as isize)
                += s[1 as i32 as usize];
            *gout.offset((n * 3 as i32 + 2 as i32) as isize)
                += s[2 as i32 as usize];
        }
        n += 1;
        n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_ip1_r2_origk_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
) {
    let mut ng: [i32; 8] = [
        1 as i32,
        0 as i32,
        2 as i32,
        0 as i32,
        3 as i32,
        1 as i32,
        1 as i32,
        3 as i32,
    ];
    CINTall_3c1e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_ip1_r2_origk_cart(
    mut out: *mut f64,
    mut dims: *mut i32,
    mut shls: *mut i32,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
    mut opt: *mut CINTOpt,
    mut cache: *mut f64,
) -> i32 {
    let mut ng: [i32; 8] = [
        1 as i32,
        0 as i32,
        2 as i32,
        0 as i32,
        3 as i32,
        1 as i32,
        1 as i32,
        3 as i32,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int3c1e_EnvVars(
        &mut envs,
        ng.as_mut_ptr(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs
        .f_gout = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut f64,
                *mut f64,
                *mut i32,
                *mut CINTEnvVars,
                i32,
            ) -> (),
        >,
        Option::<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int3c1e_ip1_r2_origk
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
        out,
        dims,
        &mut envs,
        opt,
        cache,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    *mut f64,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_3c1e
                    as unsafe extern "C" fn(
                        *mut f64,
                        *mut f64,
                        *mut i32,
                        *mut CINTEnvVars,
                        *mut f64,
                    ) -> (),
            ),
        ),
        0 as i32,
        0 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_ip1_r2_origk_sph(
    mut out: *mut f64,
    mut dims: *mut i32,
    mut shls: *mut i32,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
    mut opt: *mut CINTOpt,
    mut cache: *mut f64,
) -> i32 {
    let mut ng: [i32; 8] = [
        1 as i32,
        0 as i32,
        2 as i32,
        0 as i32,
        3 as i32,
        1 as i32,
        1 as i32,
        3 as i32,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int3c1e_EnvVars(
        &mut envs,
        ng.as_mut_ptr(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs
        .f_gout = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut f64,
                *mut f64,
                *mut i32,
                *mut CINTEnvVars,
                i32,
            ) -> (),
        >,
        Option::<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int3c1e_ip1_r2_origk
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
        out,
        dims,
        &mut envs,
        opt,
        cache,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    *mut f64,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_3c1e
                    as unsafe extern "C" fn(
                        *mut f64,
                        *mut f64,
                        *mut i32,
                        *mut CINTEnvVars,
                        *mut f64,
                    ) -> (),
            ),
        ),
        0 as i32,
        0 as i32,
    );
}
unsafe extern "C" fn CINTgout1e_int3c1e_ip1_r4_origk(
    mut gout: *mut f64,
    mut g: *mut f64,
    mut idx: *mut i32,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: i32,
) {
    let mut nf: i32 = (*envs).nf;
    let mut ix: i32 = 0;
    let mut iy: i32 = 0;
    let mut iz: i32 = 0;
    let mut n: i32 = 0;
    let mut g0: *mut f64 = g;
    let mut g1: *mut f64 = g0
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g2: *mut f64 = g1
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g3: *mut f64 = g2
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g4: *mut f64 = g3
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g5: *mut f64 = g4
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g6: *mut f64 = g5
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g7: *mut f64 = g6
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g8: *mut f64 = g7
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g9: *mut f64 = g8
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g10: *mut f64 = g9
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g11: *mut f64 = g10
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g12: *mut f64 = g11
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g13: *mut f64 = g12
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g14: *mut f64 = g13
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g15: *mut f64 = g14
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g16: *mut f64 = g15
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g17: *mut f64 = g16
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g18: *mut f64 = g17
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g19: *mut f64 = g18
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g20: *mut f64 = g19
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g21: *mut f64 = g20
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g22: *mut f64 = g21
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g23: *mut f64 = g22
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g24: *mut f64 = g23
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g25: *mut f64 = g24
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g26: *mut f64 = g25
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g27: *mut f64 = g26
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g28: *mut f64 = g27
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g29: *mut f64 = g28
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g30: *mut f64 = g29
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g31: *mut f64 = g30
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut s: [f64; 3] = [0.; 3];
    g1 = g0.offset((*envs).g_stride_k as isize);
    g3 = g1.offset((*envs).g_stride_k as isize);
    g4 = g0.offset((*envs).g_stride_k as isize);
    g7 = g3.offset((*envs).g_stride_k as isize);
    g12 = g4.offset((*envs).g_stride_k as isize);
    g15 = g7.offset((*envs).g_stride_k as isize);
    CINTnabla1i_1e(
        g16,
        g0,
        (*envs).i_l + 0 as i32,
        (*envs).j_l,
        (*envs).k_l,
        envs,
    );
    CINTnabla1i_1e(
        g19,
        g3,
        (*envs).i_l + 0 as i32,
        (*envs).j_l,
        (*envs).k_l,
        envs,
    );
    CINTnabla1i_1e(
        g28,
        g12,
        (*envs).i_l + 0 as i32,
        (*envs).j_l,
        (*envs).k_l,
        envs,
    );
    CINTnabla1i_1e(
        g31,
        g15,
        (*envs).i_l + 0 as i32,
        (*envs).j_l,
        (*envs).k_l,
        envs,
    );
    n = 0 as i32;
    while n < nf {
        ix = *idx.offset((0 as i32 + n * 3 as i32) as isize);
        iy = *idx.offset((1 as i32 + n * 3 as i32) as isize);
        iz = *idx.offset((2 as i32 + n * 3 as i32) as isize);
        s[0 as i32
            as usize] = *g31.offset(ix as isize) * *g0.offset(iy as isize)
            * *g0.offset(iz as isize)
            + 2 as i32 as f64 * *g28.offset(ix as isize)
                * *g3.offset(iy as isize) * *g0.offset(iz as isize)
            + 2 as i32 as f64 * *g28.offset(ix as isize)
                * *g0.offset(iy as isize) * *g3.offset(iz as isize)
            + *g16.offset(ix as isize) * *g15.offset(iy as isize)
                * *g0.offset(iz as isize)
            + 2 as i32 as f64 * *g16.offset(ix as isize)
                * *g12.offset(iy as isize) * *g3.offset(iz as isize)
            + *g16.offset(ix as isize) * *g0.offset(iy as isize)
                * *g15.offset(iz as isize);
        s[1 as i32
            as usize] = *g15.offset(ix as isize) * *g16.offset(iy as isize)
            * *g0.offset(iz as isize)
            + 2 as i32 as f64 * *g12.offset(ix as isize)
                * *g19.offset(iy as isize) * *g0.offset(iz as isize)
            + 2 as i32 as f64 * *g12.offset(ix as isize)
                * *g16.offset(iy as isize) * *g3.offset(iz as isize)
            + *g0.offset(ix as isize) * *g31.offset(iy as isize)
                * *g0.offset(iz as isize)
            + 2 as i32 as f64 * *g0.offset(ix as isize)
                * *g28.offset(iy as isize) * *g3.offset(iz as isize)
            + *g0.offset(ix as isize) * *g16.offset(iy as isize)
                * *g15.offset(iz as isize);
        s[2 as i32
            as usize] = *g15.offset(ix as isize) * *g0.offset(iy as isize)
            * *g16.offset(iz as isize)
            + 2 as i32 as f64 * *g12.offset(ix as isize)
                * *g3.offset(iy as isize) * *g16.offset(iz as isize)
            + 2 as i32 as f64 * *g12.offset(ix as isize)
                * *g0.offset(iy as isize) * *g19.offset(iz as isize)
            + *g0.offset(ix as isize) * *g15.offset(iy as isize)
                * *g16.offset(iz as isize)
            + 2 as i32 as f64 * *g0.offset(ix as isize)
                * *g12.offset(iy as isize) * *g19.offset(iz as isize)
            + *g0.offset(ix as isize) * *g0.offset(iy as isize)
                * *g31.offset(iz as isize);
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 3 as i32 + 0 as i32) as isize,
                ) = s[0 as i32 as usize];
            *gout
                .offset(
                    (n * 3 as i32 + 1 as i32) as isize,
                ) = s[1 as i32 as usize];
            *gout
                .offset(
                    (n * 3 as i32 + 2 as i32) as isize,
                ) = s[2 as i32 as usize];
        } else {
            *gout.offset((n * 3 as i32 + 0 as i32) as isize)
                += s[0 as i32 as usize];
            *gout.offset((n * 3 as i32 + 1 as i32) as isize)
                += s[1 as i32 as usize];
            *gout.offset((n * 3 as i32 + 2 as i32) as isize)
                += s[2 as i32 as usize];
        }
        n += 1;
        n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_ip1_r4_origk_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
) {
    let mut ng: [i32; 8] = [
        1 as i32,
        0 as i32,
        4 as i32,
        0 as i32,
        5 as i32,
        1 as i32,
        1 as i32,
        3 as i32,
    ];
    CINTall_3c1e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_ip1_r4_origk_cart(
    mut out: *mut f64,
    mut dims: *mut i32,
    mut shls: *mut i32,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
    mut opt: *mut CINTOpt,
    mut cache: *mut f64,
) -> i32 {
    let mut ng: [i32; 8] = [
        1 as i32,
        0 as i32,
        4 as i32,
        0 as i32,
        5 as i32,
        1 as i32,
        1 as i32,
        3 as i32,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int3c1e_EnvVars(
        &mut envs,
        ng.as_mut_ptr(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs
        .f_gout = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut f64,
                *mut f64,
                *mut i32,
                *mut CINTEnvVars,
                i32,
            ) -> (),
        >,
        Option::<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int3c1e_ip1_r4_origk
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
        out,
        dims,
        &mut envs,
        opt,
        cache,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    *mut f64,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_3c1e
                    as unsafe extern "C" fn(
                        *mut f64,
                        *mut f64,
                        *mut i32,
                        *mut CINTEnvVars,
                        *mut f64,
                    ) -> (),
            ),
        ),
        0 as i32,
        0 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_ip1_r4_origk_sph(
    mut out: *mut f64,
    mut dims: *mut i32,
    mut shls: *mut i32,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
    mut opt: *mut CINTOpt,
    mut cache: *mut f64,
) -> i32 {
    let mut ng: [i32; 8] = [
        1 as i32,
        0 as i32,
        4 as i32,
        0 as i32,
        5 as i32,
        1 as i32,
        1 as i32,
        3 as i32,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int3c1e_EnvVars(
        &mut envs,
        ng.as_mut_ptr(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs
        .f_gout = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut f64,
                *mut f64,
                *mut i32,
                *mut CINTEnvVars,
                i32,
            ) -> (),
        >,
        Option::<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int3c1e_ip1_r4_origk
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
        out,
        dims,
        &mut envs,
        opt,
        cache,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    *mut f64,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_3c1e
                    as unsafe extern "C" fn(
                        *mut f64,
                        *mut f64,
                        *mut i32,
                        *mut CINTEnvVars,
                        *mut f64,
                    ) -> (),
            ),
        ),
        0 as i32,
        0 as i32,
    );
}
unsafe extern "C" fn CINTgout1e_int3c1e_ip1_r6_origk(
    mut gout: *mut f64,
    mut g: *mut f64,
    mut idx: *mut i32,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: i32,
) {
    let mut nf: i32 = (*envs).nf;
    let mut ix: i32 = 0;
    let mut iy: i32 = 0;
    let mut iz: i32 = 0;
    let mut n: i32 = 0;
    let mut g0: *mut f64 = g;
    let mut g1: *mut f64 = g0
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g2: *mut f64 = g1
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g3: *mut f64 = g2
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g4: *mut f64 = g3
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g5: *mut f64 = g4
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g6: *mut f64 = g5
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g7: *mut f64 = g6
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g8: *mut f64 = g7
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g9: *mut f64 = g8
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g10: *mut f64 = g9
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g11: *mut f64 = g10
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g12: *mut f64 = g11
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g13: *mut f64 = g12
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g14: *mut f64 = g13
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g15: *mut f64 = g14
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g16: *mut f64 = g15
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g17: *mut f64 = g16
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g18: *mut f64 = g17
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g19: *mut f64 = g18
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g20: *mut f64 = g19
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g21: *mut f64 = g20
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g22: *mut f64 = g21
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g23: *mut f64 = g22
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g24: *mut f64 = g23
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g25: *mut f64 = g24
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g26: *mut f64 = g25
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g27: *mut f64 = g26
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g28: *mut f64 = g27
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g29: *mut f64 = g28
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g30: *mut f64 = g29
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g31: *mut f64 = g30
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g32: *mut f64 = g31
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g33: *mut f64 = g32
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g34: *mut f64 = g33
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g35: *mut f64 = g34
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g36: *mut f64 = g35
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g37: *mut f64 = g36
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g38: *mut f64 = g37
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g39: *mut f64 = g38
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g40: *mut f64 = g39
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g41: *mut f64 = g40
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g42: *mut f64 = g41
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g43: *mut f64 = g42
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g44: *mut f64 = g43
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g45: *mut f64 = g44
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g46: *mut f64 = g45
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g47: *mut f64 = g46
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g48: *mut f64 = g47
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g49: *mut f64 = g48
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g50: *mut f64 = g49
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g51: *mut f64 = g50
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g52: *mut f64 = g51
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g53: *mut f64 = g52
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g54: *mut f64 = g53
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g55: *mut f64 = g54
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g56: *mut f64 = g55
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g57: *mut f64 = g56
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g58: *mut f64 = g57
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g59: *mut f64 = g58
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g60: *mut f64 = g59
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g61: *mut f64 = g60
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g62: *mut f64 = g61
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g63: *mut f64 = g62
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g64: *mut f64 = g63
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g65: *mut f64 = g64
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g66: *mut f64 = g65
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g67: *mut f64 = g66
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g68: *mut f64 = g67
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g69: *mut f64 = g68
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g70: *mut f64 = g69
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g71: *mut f64 = g70
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g72: *mut f64 = g71
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g73: *mut f64 = g72
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g74: *mut f64 = g73
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g75: *mut f64 = g74
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g76: *mut f64 = g75
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g77: *mut f64 = g76
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g78: *mut f64 = g77
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g79: *mut f64 = g78
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g80: *mut f64 = g79
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g81: *mut f64 = g80
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g82: *mut f64 = g81
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g83: *mut f64 = g82
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g84: *mut f64 = g83
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g85: *mut f64 = g84
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g86: *mut f64 = g85
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g87: *mut f64 = g86
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g88: *mut f64 = g87
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g89: *mut f64 = g88
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g90: *mut f64 = g89
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g91: *mut f64 = g90
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g92: *mut f64 = g91
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g93: *mut f64 = g92
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g94: *mut f64 = g93
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g95: *mut f64 = g94
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g96: *mut f64 = g95
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g97: *mut f64 = g96
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g98: *mut f64 = g97
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g99: *mut f64 = g98
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g100: *mut f64 = g99
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g101: *mut f64 = g100
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g102: *mut f64 = g101
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g103: *mut f64 = g102
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g104: *mut f64 = g103
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g105: *mut f64 = g104
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g106: *mut f64 = g105
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g107: *mut f64 = g106
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g108: *mut f64 = g107
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g109: *mut f64 = g108
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g110: *mut f64 = g109
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g111: *mut f64 = g110
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g112: *mut f64 = g111
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g113: *mut f64 = g112
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g114: *mut f64 = g113
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g115: *mut f64 = g114
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g116: *mut f64 = g115
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g117: *mut f64 = g116
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g118: *mut f64 = g117
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g119: *mut f64 = g118
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g120: *mut f64 = g119
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g121: *mut f64 = g120
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g122: *mut f64 = g121
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g123: *mut f64 = g122
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g124: *mut f64 = g123
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g125: *mut f64 = g124
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g126: *mut f64 = g125
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g127: *mut f64 = g126
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut s: [f64; 3] = [0.; 3];
    g1 = g0.offset((*envs).g_stride_k as isize);
    g3 = g1.offset((*envs).g_stride_k as isize);
    g4 = g0.offset((*envs).g_stride_k as isize);
    g7 = g3.offset((*envs).g_stride_k as isize);
    g12 = g4.offset((*envs).g_stride_k as isize);
    g15 = g7.offset((*envs).g_stride_k as isize);
    g16 = g0.offset((*envs).g_stride_k as isize);
    g28 = g12.offset((*envs).g_stride_k as isize);
    g31 = g15.offset((*envs).g_stride_k as isize);
    g48 = g16.offset((*envs).g_stride_k as isize);
    g60 = g28.offset((*envs).g_stride_k as isize);
    g63 = g31.offset((*envs).g_stride_k as isize);
    CINTnabla1i_1e(
        g64,
        g0,
        (*envs).i_l + 0 as i32,
        (*envs).j_l,
        (*envs).k_l,
        envs,
    );
    CINTnabla1i_1e(
        g67,
        g3,
        (*envs).i_l + 0 as i32,
        (*envs).j_l,
        (*envs).k_l,
        envs,
    );
    CINTnabla1i_1e(
        g79,
        g15,
        (*envs).i_l + 0 as i32,
        (*envs).j_l,
        (*envs).k_l,
        envs,
    );
    CINTnabla1i_1e(
        g112,
        g48,
        (*envs).i_l + 0 as i32,
        (*envs).j_l,
        (*envs).k_l,
        envs,
    );
    CINTnabla1i_1e(
        g124,
        g60,
        (*envs).i_l + 0 as i32,
        (*envs).j_l,
        (*envs).k_l,
        envs,
    );
    CINTnabla1i_1e(
        g127,
        g63,
        (*envs).i_l + 0 as i32,
        (*envs).j_l,
        (*envs).k_l,
        envs,
    );
    n = 0 as i32;
    while n < nf {
        ix = *idx.offset((0 as i32 + n * 3 as i32) as isize);
        iy = *idx.offset((1 as i32 + n * 3 as i32) as isize);
        iz = *idx.offset((2 as i32 + n * 3 as i32) as isize);
        s[0 as i32
            as usize] = *g127.offset(ix as isize) * *g0.offset(iy as isize)
            * *g0.offset(iz as isize)
            + 3 as i32 as f64 * *g124.offset(ix as isize)
                * *g3.offset(iy as isize) * *g0.offset(iz as isize)
            + 3 as i32 as f64 * *g124.offset(ix as isize)
                * *g0.offset(iy as isize) * *g3.offset(iz as isize)
            + 3 as i32 as f64 * *g112.offset(ix as isize)
                * *g15.offset(iy as isize) * *g0.offset(iz as isize)
            + 6 as i32 as f64 * *g112.offset(ix as isize)
                * *g12.offset(iy as isize) * *g3.offset(iz as isize)
            + 3 as i32 as f64 * *g112.offset(ix as isize)
                * *g0.offset(iy as isize) * *g15.offset(iz as isize)
            + *g64.offset(ix as isize) * *g63.offset(iy as isize)
                * *g0.offset(iz as isize)
            + 3 as i32 as f64 * *g64.offset(ix as isize)
                * *g60.offset(iy as isize) * *g3.offset(iz as isize)
            + 3 as i32 as f64 * *g64.offset(ix as isize)
                * *g48.offset(iy as isize) * *g15.offset(iz as isize)
            + *g64.offset(ix as isize) * *g0.offset(iy as isize)
                * *g63.offset(iz as isize);
        s[1 as i32
            as usize] = *g63.offset(ix as isize) * *g64.offset(iy as isize)
            * *g0.offset(iz as isize)
            + 3 as i32 as f64 * *g60.offset(ix as isize)
                * *g67.offset(iy as isize) * *g0.offset(iz as isize)
            + 3 as i32 as f64 * *g60.offset(ix as isize)
                * *g64.offset(iy as isize) * *g3.offset(iz as isize)
            + 3 as i32 as f64 * *g48.offset(ix as isize)
                * *g79.offset(iy as isize) * *g0.offset(iz as isize)
            + 6 as i32 as f64 * *g48.offset(ix as isize)
                * *g76.offset(iy as isize) * *g3.offset(iz as isize)
            + 3 as i32 as f64 * *g48.offset(ix as isize)
                * *g64.offset(iy as isize) * *g15.offset(iz as isize)
            + *g0.offset(ix as isize) * *g127.offset(iy as isize)
                * *g0.offset(iz as isize)
            + 3 as i32 as f64 * *g0.offset(ix as isize)
                * *g124.offset(iy as isize) * *g3.offset(iz as isize)
            + 3 as i32 as f64 * *g0.offset(ix as isize)
                * *g112.offset(iy as isize) * *g15.offset(iz as isize)
            + *g0.offset(ix as isize) * *g64.offset(iy as isize)
                * *g63.offset(iz as isize);
        s[2 as i32
            as usize] = *g63.offset(ix as isize) * *g0.offset(iy as isize)
            * *g64.offset(iz as isize)
            + 3 as i32 as f64 * *g60.offset(ix as isize)
                * *g3.offset(iy as isize) * *g64.offset(iz as isize)
            + 3 as i32 as f64 * *g60.offset(ix as isize)
                * *g0.offset(iy as isize) * *g67.offset(iz as isize)
            + 3 as i32 as f64 * *g48.offset(ix as isize)
                * *g15.offset(iy as isize) * *g64.offset(iz as isize)
            + 6 as i32 as f64 * *g48.offset(ix as isize)
                * *g12.offset(iy as isize) * *g67.offset(iz as isize)
            + 3 as i32 as f64 * *g48.offset(ix as isize)
                * *g0.offset(iy as isize) * *g79.offset(iz as isize)
            + *g0.offset(ix as isize) * *g63.offset(iy as isize)
                * *g64.offset(iz as isize)
            + 3 as i32 as f64 * *g0.offset(ix as isize)
                * *g60.offset(iy as isize) * *g67.offset(iz as isize)
            + 3 as i32 as f64 * *g0.offset(ix as isize)
                * *g48.offset(iy as isize) * *g79.offset(iz as isize)
            + *g0.offset(ix as isize) * *g0.offset(iy as isize)
                * *g127.offset(iz as isize);
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 3 as i32 + 0 as i32) as isize,
                ) = s[0 as i32 as usize];
            *gout
                .offset(
                    (n * 3 as i32 + 1 as i32) as isize,
                ) = s[1 as i32 as usize];
            *gout
                .offset(
                    (n * 3 as i32 + 2 as i32) as isize,
                ) = s[2 as i32 as usize];
        } else {
            *gout.offset((n * 3 as i32 + 0 as i32) as isize)
                += s[0 as i32 as usize];
            *gout.offset((n * 3 as i32 + 1 as i32) as isize)
                += s[1 as i32 as usize];
            *gout.offset((n * 3 as i32 + 2 as i32) as isize)
                += s[2 as i32 as usize];
        }
        n += 1;
        n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_ip1_r6_origk_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
) {
    let mut ng: [i32; 8] = [
        1 as i32,
        0 as i32,
        6 as i32,
        0 as i32,
        7 as i32,
        1 as i32,
        1 as i32,
        3 as i32,
    ];
    CINTall_3c1e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_ip1_r6_origk_cart(
    mut out: *mut f64,
    mut dims: *mut i32,
    mut shls: *mut i32,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
    mut opt: *mut CINTOpt,
    mut cache: *mut f64,
) -> i32 {
    let mut ng: [i32; 8] = [
        1 as i32,
        0 as i32,
        6 as i32,
        0 as i32,
        7 as i32,
        1 as i32,
        1 as i32,
        3 as i32,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int3c1e_EnvVars(
        &mut envs,
        ng.as_mut_ptr(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs
        .f_gout = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut f64,
                *mut f64,
                *mut i32,
                *mut CINTEnvVars,
                i32,
            ) -> (),
        >,
        Option::<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int3c1e_ip1_r6_origk
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
        out,
        dims,
        &mut envs,
        opt,
        cache,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    *mut f64,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_3c1e
                    as unsafe extern "C" fn(
                        *mut f64,
                        *mut f64,
                        *mut i32,
                        *mut CINTEnvVars,
                        *mut f64,
                    ) -> (),
            ),
        ),
        0 as i32,
        0 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_ip1_r6_origk_sph(
    mut out: *mut f64,
    mut dims: *mut i32,
    mut shls: *mut i32,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
    mut opt: *mut CINTOpt,
    mut cache: *mut f64,
) -> i32 {
    let mut ng: [i32; 8] = [
        1 as i32,
        0 as i32,
        6 as i32,
        0 as i32,
        7 as i32,
        1 as i32,
        1 as i32,
        3 as i32,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int3c1e_EnvVars(
        &mut envs,
        ng.as_mut_ptr(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs
        .f_gout = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut f64,
                *mut f64,
                *mut i32,
                *mut CINTEnvVars,
                i32,
            ) -> (),
        >,
        Option::<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int3c1e_ip1_r6_origk
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
        out,
        dims,
        &mut envs,
        opt,
        cache,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    *mut f64,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_3c1e
                    as unsafe extern "C" fn(
                        *mut f64,
                        *mut f64,
                        *mut i32,
                        *mut CINTEnvVars,
                        *mut f64,
                    ) -> (),
            ),
        ),
        0 as i32,
        0 as i32,
    );
}
