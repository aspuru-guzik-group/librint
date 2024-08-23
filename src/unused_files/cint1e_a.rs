#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::cart2sph::c2s_sph_1e;
use crate::cart2sph::c2s_cart_1e;
use crate::g1e::CINTnabla1j_1e;
use crate::g1e::CINTinit_int1e_EnvVars;
use crate::optimizer::CINTall_1e_optimizer;
use crate::cint1e::CINT1e_drv;

use crate::cint::CINTOpt;
use crate::cint::CINTEnvVars;

unsafe extern "C" fn CINTgout1e_int1e_r2_origi(
    mut gout: *mut f64,
    mut g: *mut f64,
    mut idx: *mut i32,
    mut envs: *mut CINTEnvVars,
    mut empty: i32,
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
    g1 = g0.offset((*envs).g_stride_i as isize);
    g3 = g1.offset((*envs).g_stride_i as isize);
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
        if empty != 0 {
            *gout.offset(n as isize) = s;
        } else {
            *gout.offset(n as isize) += s;
        }
        n += 1;
        n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_r2_origi_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
) {
    let mut ng: [i32; 8] = [
        2 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        2 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    CINTall_1e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int1e_r2_origi_cart(
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
        2 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        2 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int1e_EnvVars(&mut envs, ng.as_mut_ptr(), shls, atm, natm, bas, nbas, env);
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
            CINTgout1e_int1e_r2_origi
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &mut envs,
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
                c2s_cart_1e
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
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_r2_origi_sph(
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
        2 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        2 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int1e_EnvVars(&mut envs, ng.as_mut_ptr(), shls, atm, natm, bas, nbas, env);
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
            CINTgout1e_int1e_r2_origi
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &mut envs,
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
                c2s_sph_1e
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
    );
}
unsafe extern "C" fn CINTgout1e_int1e_r4_origi(
    mut gout: *mut f64,
    mut g: *mut f64,
    mut idx: *mut i32,
    mut envs: *mut CINTEnvVars,
    mut empty: i32,
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
    g1 = g0.offset((*envs).g_stride_i as isize);
    g3 = g1.offset((*envs).g_stride_i as isize);
    g4 = g0.offset((*envs).g_stride_i as isize);
    g7 = g3.offset((*envs).g_stride_i as isize);
    g12 = g4.offset((*envs).g_stride_i as isize);
    g15 = g7.offset((*envs).g_stride_i as isize);
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
        if empty != 0 {
            *gout.offset(n as isize) = s;
        } else {
            *gout.offset(n as isize) += s;
        }
        n += 1;
        n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_r4_origi_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
) {
    let mut ng: [i32; 8] = [
        4 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        4 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    CINTall_1e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int1e_r4_origi_cart(
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
        4 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        4 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int1e_EnvVars(&mut envs, ng.as_mut_ptr(), shls, atm, natm, bas, nbas, env);
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
            CINTgout1e_int1e_r4_origi
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &mut envs,
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
                c2s_cart_1e
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
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_r4_origi_sph(
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
        4 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        4 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int1e_EnvVars(&mut envs, ng.as_mut_ptr(), shls, atm, natm, bas, nbas, env);
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
            CINTgout1e_int1e_r4_origi
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &mut envs,
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
                c2s_sph_1e
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
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_r2_origi_ip2(
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
    CINTnabla1j_1e(
        g1,
        g0,
        (*envs).i_l + 2 as i32,
        (*envs).j_l + 0 as i32,
        0 as i32,
        envs,
    );
    g2 = g0.offset((*envs).g_stride_i as isize);
    g3 = g1.offset((*envs).g_stride_i as isize);
    g6 = g2.offset((*envs).g_stride_i as isize);
    g7 = g3.offset((*envs).g_stride_i as isize);
    n = 0 as i32;
    while n < nf {
        ix = *idx.offset((0 as i32 + n * 3 as i32) as isize);
        iy = *idx.offset((1 as i32 + n * 3 as i32) as isize);
        iz = *idx.offset((2 as i32 + n * 3 as i32) as isize);
        s[0 as i32
            as usize] = *g7.offset((ix + 0 as i32) as isize)
            * *g0.offset((iy + 0 as i32) as isize)
            * *g0.offset((iz + 0 as i32) as isize)
            + *g1.offset((ix + 0 as i32) as isize)
                * *g6.offset((iy + 0 as i32) as isize)
                * *g0.offset((iz + 0 as i32) as isize)
            + *g1.offset((ix + 0 as i32) as isize)
                * *g0.offset((iy + 0 as i32) as isize)
                * *g6.offset((iz + 0 as i32) as isize);
        s[1 as i32
            as usize] = *g6.offset((ix + 0 as i32) as isize)
            * *g1.offset((iy + 0 as i32) as isize)
            * *g0.offset((iz + 0 as i32) as isize)
            + *g0.offset((ix + 0 as i32) as isize)
                * *g7.offset((iy + 0 as i32) as isize)
                * *g0.offset((iz + 0 as i32) as isize)
            + *g0.offset((ix + 0 as i32) as isize)
                * *g1.offset((iy + 0 as i32) as isize)
                * *g6.offset((iz + 0 as i32) as isize);
        s[2 as i32
            as usize] = *g6.offset((ix + 0 as i32) as isize)
            * *g0.offset((iy + 0 as i32) as isize)
            * *g1.offset((iz + 0 as i32) as isize)
            + *g0.offset((ix + 0 as i32) as isize)
                * *g6.offset((iy + 0 as i32) as isize)
                * *g1.offset((iz + 0 as i32) as isize)
            + *g0.offset((ix + 0 as i32) as isize)
                * *g0.offset((iy + 0 as i32) as isize)
                * *g7.offset((iz + 0 as i32) as isize);
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
pub unsafe extern "C" fn int1e_r2_origi_ip2_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
) {
    let mut ng: [i32; 8] = [
        2 as i32,
        1 as i32,
        0 as i32,
        0 as i32,
        3 as i32,
        1 as i32,
        1 as i32,
        3 as i32,
    ];
    CINTall_1e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int1e_r2_origi_ip2_cart(
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
        2 as i32,
        1 as i32,
        0 as i32,
        0 as i32,
        3 as i32,
        1 as i32,
        1 as i32,
        3 as i32,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int1e_EnvVars(&mut envs, ng.as_mut_ptr(), shls, atm, natm, bas, nbas, env);
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
            CINTgout1e_int1e_r2_origi_ip2
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &mut envs,
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
                c2s_cart_1e
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
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_r2_origi_ip2_sph(
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
        2 as i32,
        1 as i32,
        0 as i32,
        0 as i32,
        3 as i32,
        1 as i32,
        1 as i32,
        3 as i32,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int1e_EnvVars(&mut envs, ng.as_mut_ptr(), shls, atm, natm, bas, nbas, env);
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
            CINTgout1e_int1e_r2_origi_ip2
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &mut envs,
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
                c2s_sph_1e
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
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_r4_origi_ip2(
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
    let mut s: [f64; 243] = [0.; 243];
    CINTnabla1j_1e(
        g1,
        g0,
        (*envs).i_l + 4 as i32,
        (*envs).j_l + 0 as i32,
        0 as i32,
        envs,
    );
    g2 = g0.offset((*envs).g_stride_i as isize);
    g3 = g1.offset((*envs).g_stride_i as isize);
    g6 = g2.offset((*envs).g_stride_i as isize);
    g7 = g3.offset((*envs).g_stride_i as isize);
    g8 = g0.offset((*envs).g_stride_i as isize);
    g9 = g1.offset((*envs).g_stride_i as isize);
    g14 = g6.offset((*envs).g_stride_i as isize);
    g15 = g7.offset((*envs).g_stride_i as isize);
    g24 = g8.offset((*envs).g_stride_i as isize);
    g25 = g9.offset((*envs).g_stride_i as isize);
    g30 = g14.offset((*envs).g_stride_i as isize);
    g31 = g15.offset((*envs).g_stride_i as isize);
    n = 0 as i32;
    while n < nf {
        ix = *idx.offset((0 as i32 + n * 3 as i32) as isize);
        iy = *idx.offset((1 as i32 + n * 3 as i32) as isize);
        iz = *idx.offset((2 as i32 + n * 3 as i32) as isize);
        s[0 as i32
            as usize] = *g31.offset((ix + 0 as i32) as isize)
            * *g0.offset((iy + 0 as i32) as isize)
            * *g0.offset((iz + 0 as i32) as isize)
            + 2 as i32 as f64
                * *g25.offset((ix + 0 as i32) as isize)
                * *g6.offset((iy + 0 as i32) as isize)
                * *g0.offset((iz + 0 as i32) as isize)
            + 2 as i32 as f64
                * *g25.offset((ix + 0 as i32) as isize)
                * *g0.offset((iy + 0 as i32) as isize)
                * *g6.offset((iz + 0 as i32) as isize)
            + *g1.offset((ix + 0 as i32) as isize)
                * *g30.offset((iy + 0 as i32) as isize)
                * *g0.offset((iz + 0 as i32) as isize)
            + 2 as i32 as f64
                * *g1.offset((ix + 0 as i32) as isize)
                * *g24.offset((iy + 0 as i32) as isize)
                * *g6.offset((iz + 0 as i32) as isize)
            + *g1.offset((ix + 0 as i32) as isize)
                * *g0.offset((iy + 0 as i32) as isize)
                * *g30.offset((iz + 0 as i32) as isize);
        s[1 as i32
            as usize] = *g30.offset((ix + 0 as i32) as isize)
            * *g1.offset((iy + 0 as i32) as isize)
            * *g0.offset((iz + 0 as i32) as isize)
            + 2 as i32 as f64
                * *g24.offset((ix + 0 as i32) as isize)
                * *g7.offset((iy + 0 as i32) as isize)
                * *g0.offset((iz + 0 as i32) as isize)
            + 2 as i32 as f64
                * *g24.offset((ix + 0 as i32) as isize)
                * *g1.offset((iy + 0 as i32) as isize)
                * *g6.offset((iz + 0 as i32) as isize)
            + *g0.offset((ix + 0 as i32) as isize)
                * *g31.offset((iy + 0 as i32) as isize)
                * *g0.offset((iz + 0 as i32) as isize)
            + 2 as i32 as f64
                * *g0.offset((ix + 0 as i32) as isize)
                * *g25.offset((iy + 0 as i32) as isize)
                * *g6.offset((iz + 0 as i32) as isize)
            + *g0.offset((ix + 0 as i32) as isize)
                * *g1.offset((iy + 0 as i32) as isize)
                * *g30.offset((iz + 0 as i32) as isize);
        s[2 as i32
            as usize] = *g30.offset((ix + 0 as i32) as isize)
            * *g0.offset((iy + 0 as i32) as isize)
            * *g1.offset((iz + 0 as i32) as isize)
            + 2 as i32 as f64
                * *g24.offset((ix + 0 as i32) as isize)
                * *g6.offset((iy + 0 as i32) as isize)
                * *g1.offset((iz + 0 as i32) as isize)
            + 2 as i32 as f64
                * *g24.offset((ix + 0 as i32) as isize)
                * *g0.offset((iy + 0 as i32) as isize)
                * *g7.offset((iz + 0 as i32) as isize)
            + *g0.offset((ix + 0 as i32) as isize)
                * *g30.offset((iy + 0 as i32) as isize)
                * *g1.offset((iz + 0 as i32) as isize)
            + 2 as i32 as f64
                * *g0.offset((ix + 0 as i32) as isize)
                * *g24.offset((iy + 0 as i32) as isize)
                * *g7.offset((iz + 0 as i32) as isize)
            + *g0.offset((ix + 0 as i32) as isize)
                * *g0.offset((iy + 0 as i32) as isize)
                * *g31.offset((iz + 0 as i32) as isize);
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
pub unsafe extern "C" fn int1e_r4_origi_ip2_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
) {
    let mut ng: [i32; 8] = [
        4 as i32,
        1 as i32,
        0 as i32,
        0 as i32,
        5 as i32,
        1 as i32,
        1 as i32,
        3 as i32,
    ];
    CINTall_1e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int1e_r4_origi_ip2_cart(
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
        4 as i32,
        1 as i32,
        0 as i32,
        0 as i32,
        5 as i32,
        1 as i32,
        1 as i32,
        3 as i32,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int1e_EnvVars(&mut envs, ng.as_mut_ptr(), shls, atm, natm, bas, nbas, env);
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
            CINTgout1e_int1e_r4_origi_ip2
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &mut envs,
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
                c2s_cart_1e
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
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_r4_origi_ip2_sph(
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
        4 as i32,
        1 as i32,
        0 as i32,
        0 as i32,
        5 as i32,
        1 as i32,
        1 as i32,
        3 as i32,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int1e_EnvVars(&mut envs, ng.as_mut_ptr(), shls, atm, natm, bas, nbas, env);
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
            CINTgout1e_int1e_r4_origi_ip2
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &mut envs,
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
                c2s_sph_1e
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
    );
}
