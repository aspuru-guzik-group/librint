#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::cart2sph::c2s_sph_2e1;
use crate::cart2sph::c2s_cart_2e1;
use crate::g2e::CINTinit_int2e_EnvVars;
use crate::g2e::CINTnabla1i_2e;
use crate::g2e::CINTnabla1j_2e;
use crate::g2e::CINTnabla1l_2e;
use crate::g2e::CINTx1j_2e;
use crate::g2e::CINTx1l_2e;
use crate::optimizer::CINTall_2e_optimizer;
use crate::cint2e::CINT2e_drv;

use crate::cint::CINTOpt;
use crate::cint::CINTEnvVars;


unsafe extern "C" fn CINTgout2e_int2e_breit_r1p2(
    mut gout: *mut f64,
    mut g: *mut f64,
    mut idx: *mut i32,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: i32,
) {
    let mut nf: i32 = (*envs).nf;
    let mut nrys_roots: i32 = (*envs).nrys_roots;
    let mut ix: i32 = 0;
    let mut iy: i32 = 0;
    let mut iz: i32 = 0;
    let mut i: i32 = 0;
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
    CINTnabla1l_2e(
        g1,
        g0,
        (*envs).i_l + 2 as i32,
        (*envs).j_l + 2 as i32,
        (*envs).k_l + 0 as i32,
        (*envs).l_l + 0 as i32,
        envs,
    );
    CINTx1j_2e(
        g3,
        g1,
        (*envs).rj,
        (*envs).i_l + 1 as i32,
        (*envs).j_l + 0 as i32,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g4,
        g0,
        (*envs).i_l + 1 as i32,
        (*envs).j_l + 1 as i32,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g5,
        g0,
        (*envs).i_l + 1 as i32,
        (*envs).j_l + 1 as i32,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as i32;
    while ix < (*envs).g_size * 3 as i32 {
        *g4.offset(ix as isize) += *g5.offset(ix as isize);
        ix += 1;
        ix;
    }
    CINTnabla1j_2e(
        g5,
        g1,
        (*envs).i_l + 1 as i32,
        (*envs).j_l + 1 as i32,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g6,
        g1,
        (*envs).i_l + 1 as i32,
        (*envs).j_l + 1 as i32,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as i32;
    while ix < (*envs).g_size * 3 as i32 {
        *g5.offset(ix as isize) += *g6.offset(ix as isize);
        ix += 1;
        ix;
    }
    CINTx1j_2e(
        g7,
        g5,
        (*envs).rj,
        (*envs).i_l + 1 as i32,
        (*envs).j_l + 0 as i32,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g12,
        g4,
        (*envs).i_l + 0 as i32,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g15,
        g7,
        (*envs).i_l + 0 as i32,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    let mut s: f64 = 0.;
    n = 0 as i32;
    while n < nf {
        ix = *idx.offset((0 as i32 + n * 3 as i32) as isize);
        iy = *idx.offset((1 as i32 + n * 3 as i32) as isize);
        iz = *idx.offset((2 as i32 + n * 3 as i32) as isize);
        s = 0 as i32 as f64;
        i = 0 as i32;
        while i < nrys_roots {
            s
                += *g15.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s
                += *g12.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s
                += *g12.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s
                += *g3.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s
                += *g0.offset((ix + i) as isize) * *g15.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s
                += *g0.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s
                += *g3.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s
                += *g0.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s
                += *g0.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g15.offset((iz + i) as isize);
            i += 1;
            i;
        }
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
pub unsafe extern "C" fn int2e_breit_r1p2_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
) {
    let mut ng: [i32; 8] = [
        2 as i32,
        2 as i32,
        0 as i32,
        1 as i32,
        4 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    CINTall_2e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int2e_breit_r1p2_cart(
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
        2 as i32,
        0 as i32,
        1 as i32,
        4 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int2e_EnvVars(&mut envs, ng.as_mut_ptr(), shls, atm, natm, bas, nbas, env);
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
            CINTgout2e_int2e_breit_r1p2
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT2e_drv(
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
                c2s_cart_2e1
                    as unsafe extern "C" fn(
                        *mut f64,
                        *mut f64,
                        *mut i32,
                        *mut CINTEnvVars,
                        *mut f64,
                    ) -> (),
            ),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn int2e_breit_r1p2_sph(
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
        2 as i32,
        0 as i32,
        1 as i32,
        4 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int2e_EnvVars(&mut envs, ng.as_mut_ptr(), shls, atm, natm, bas, nbas, env);
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
            CINTgout2e_int2e_breit_r1p2
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT2e_drv(
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
                c2s_sph_2e1
                    as unsafe extern "C" fn(
                        *mut f64,
                        *mut f64,
                        *mut i32,
                        *mut CINTEnvVars,
                        *mut f64,
                    ) -> (),
            ),
        ),
    );
}
unsafe extern "C" fn CINTgout2e_int2e_breit_r2p2(
    mut gout: *mut f64,
    mut g: *mut f64,
    mut idx: *mut i32,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: i32,
) {
    let mut nf: i32 = (*envs).nf;
    let mut nrys_roots: i32 = (*envs).nrys_roots;
    let mut ix: i32 = 0;
    let mut iy: i32 = 0;
    let mut iz: i32 = 0;
    let mut i: i32 = 0;
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
    CINTx1l_2e(
        g2,
        g0,
        (*envs).c2rust_unnamed_1.rl,
        (*envs).i_l + 2 as i32,
        (*envs).j_l + 1 as i32,
        (*envs).k_l + 0 as i32,
        (*envs).l_l + 1 as i32,
        envs,
    );
    CINTnabla1l_2e(
        g3,
        g2,
        (*envs).i_l + 2 as i32,
        (*envs).j_l + 1 as i32,
        (*envs).k_l + 0 as i32,
        (*envs).l_l + 0 as i32,
        envs,
    );
    CINTnabla1j_2e(
        g4,
        g0,
        (*envs).i_l + 1 as i32,
        (*envs).j_l + 0 as i32,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g5,
        g0,
        (*envs).i_l + 1 as i32,
        (*envs).j_l + 0 as i32,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as i32;
    while ix < (*envs).g_size * 3 as i32 {
        *g4.offset(ix as isize) += *g5.offset(ix as isize);
        ix += 1;
        ix;
    }
    CINTnabla1j_2e(
        g7,
        g3,
        (*envs).i_l + 1 as i32,
        (*envs).j_l + 0 as i32,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g8,
        g3,
        (*envs).i_l + 1 as i32,
        (*envs).j_l + 0 as i32,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as i32;
    while ix < (*envs).g_size * 3 as i32 {
        *g7.offset(ix as isize) += *g8.offset(ix as isize);
        ix += 1;
        ix;
    }
    CINTnabla1i_2e(
        g12,
        g4,
        (*envs).i_l + 0 as i32,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g15,
        g7,
        (*envs).i_l + 0 as i32,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    let mut s: f64 = 0.;
    n = 0 as i32;
    while n < nf {
        ix = *idx.offset((0 as i32 + n * 3 as i32) as isize);
        iy = *idx.offset((1 as i32 + n * 3 as i32) as isize);
        iz = *idx.offset((2 as i32 + n * 3 as i32) as isize);
        s = 0 as i32 as f64;
        i = 0 as i32;
        while i < nrys_roots {
            s
                += *g15.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s
                += *g12.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s
                += *g12.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s
                += *g3.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s
                += *g0.offset((ix + i) as isize) * *g15.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s
                += *g0.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s
                += *g3.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s
                += *g0.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s
                += *g0.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g15.offset((iz + i) as isize);
            i += 1;
            i;
        }
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
pub unsafe extern "C" fn int2e_breit_r2p2_optimizer(
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
        2 as i32,
        4 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    CINTall_2e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int2e_breit_r2p2_cart(
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
        2 as i32,
        4 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int2e_EnvVars(&mut envs, ng.as_mut_ptr(), shls, atm, natm, bas, nbas, env);
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
            CINTgout2e_int2e_breit_r2p2
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT2e_drv(
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
                c2s_cart_2e1
                    as unsafe extern "C" fn(
                        *mut f64,
                        *mut f64,
                        *mut i32,
                        *mut CINTEnvVars,
                        *mut f64,
                    ) -> (),
            ),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn int2e_breit_r2p2_sph(
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
        2 as i32,
        4 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int2e_EnvVars(&mut envs, ng.as_mut_ptr(), shls, atm, natm, bas, nbas, env);
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
            CINTgout2e_int2e_breit_r2p2
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT2e_drv(
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
                c2s_sph_2e1
                    as unsafe extern "C" fn(
                        *mut f64,
                        *mut f64,
                        *mut i32,
                        *mut CINTEnvVars,
                        *mut f64,
                    ) -> (),
            ),
        ),
    );
}
