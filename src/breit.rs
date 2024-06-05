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
    mut gout: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut idx: *mut libc::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: libc::c_int,
) {
    let mut nf: libc::c_int = (*envs).nf;
    let mut nrys_roots: libc::c_int = (*envs).nrys_roots;
    let mut ix: libc::c_int = 0;
    let mut iy: libc::c_int = 0;
    let mut iz: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut g0: *mut libc::c_double = g;
    let mut g1: *mut libc::c_double = g0
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g2: *mut libc::c_double = g1
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g3: *mut libc::c_double = g2
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g4: *mut libc::c_double = g3
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g5: *mut libc::c_double = g4
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g6: *mut libc::c_double = g5
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g7: *mut libc::c_double = g6
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g8: *mut libc::c_double = g7
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g9: *mut libc::c_double = g8
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g10: *mut libc::c_double = g9
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g11: *mut libc::c_double = g10
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g12: *mut libc::c_double = g11
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g13: *mut libc::c_double = g12
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g14: *mut libc::c_double = g13
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g15: *mut libc::c_double = g14
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    CINTnabla1l_2e(
        g1,
        g0,
        (*envs).i_l + 2 as libc::c_int,
        (*envs).j_l + 2 as libc::c_int,
        (*envs).k_l + 0 as libc::c_int,
        (*envs).l_l + 0 as libc::c_int,
        envs,
    );
    CINTx1j_2e(
        g3,
        g1,
        (*envs).rj,
        (*envs).i_l + 1 as libc::c_int,
        (*envs).j_l + 0 as libc::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g4,
        g0,
        (*envs).i_l + 1 as libc::c_int,
        (*envs).j_l + 1 as libc::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g5,
        g0,
        (*envs).i_l + 1 as libc::c_int,
        (*envs).j_l + 1 as libc::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as libc::c_int;
    while ix < (*envs).g_size * 3 as libc::c_int {
        *g4.offset(ix as isize) += *g5.offset(ix as isize);
        ix += 1;
        ix;
    }
    CINTnabla1j_2e(
        g5,
        g1,
        (*envs).i_l + 1 as libc::c_int,
        (*envs).j_l + 1 as libc::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g6,
        g1,
        (*envs).i_l + 1 as libc::c_int,
        (*envs).j_l + 1 as libc::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as libc::c_int;
    while ix < (*envs).g_size * 3 as libc::c_int {
        *g5.offset(ix as isize) += *g6.offset(ix as isize);
        ix += 1;
        ix;
    }
    CINTx1j_2e(
        g7,
        g5,
        (*envs).rj,
        (*envs).i_l + 1 as libc::c_int,
        (*envs).j_l + 0 as libc::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g12,
        g4,
        (*envs).i_l + 0 as libc::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g15,
        g7,
        (*envs).i_l + 0 as libc::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    let mut s: libc::c_double = 0.;
    n = 0 as libc::c_int;
    while n < nf {
        ix = *idx.offset((0 as libc::c_int + n * 3 as libc::c_int) as isize);
        iy = *idx.offset((1 as libc::c_int + n * 3 as libc::c_int) as isize);
        iz = *idx.offset((2 as libc::c_int + n * 3 as libc::c_int) as isize);
        s = 0 as libc::c_int as libc::c_double;
        i = 0 as libc::c_int;
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
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
) {
    let mut ng: [libc::c_int; 8] = [
        2 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    CINTall_2e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int2e_breit_r1p2_cart(
    mut out: *mut libc::c_double,
    mut dims: *mut libc::c_int,
    mut shls: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut libc::c_double,
) -> libc::c_int {
    let mut ng: [libc::c_int; 8] = [
        2 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int2e_EnvVars(&mut envs, ng.as_mut_ptr(), shls, atm, natm, bas, nbas, env);
    envs
        .f_gout = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut libc::c_double,
                *mut libc::c_double,
                *mut libc::c_int,
                *mut CINTEnvVars,
                libc::c_int,
            ) -> (),
        >,
        Option::<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout2e_int2e_breit_r1p2
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
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
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_2e1
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut libc::c_double,
                        *mut libc::c_int,
                        *mut CINTEnvVars,
                        *mut libc::c_double,
                    ) -> (),
            ),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn int2e_breit_r1p2_sph(
    mut out: *mut libc::c_double,
    mut dims: *mut libc::c_int,
    mut shls: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut libc::c_double,
) -> libc::c_int {
    let mut ng: [libc::c_int; 8] = [
        2 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int2e_EnvVars(&mut envs, ng.as_mut_ptr(), shls, atm, natm, bas, nbas, env);
    envs
        .f_gout = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut libc::c_double,
                *mut libc::c_double,
                *mut libc::c_int,
                *mut CINTEnvVars,
                libc::c_int,
            ) -> (),
        >,
        Option::<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout2e_int2e_breit_r1p2
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
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
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_2e1
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut libc::c_double,
                        *mut libc::c_int,
                        *mut CINTEnvVars,
                        *mut libc::c_double,
                    ) -> (),
            ),
        ),
    );
}
unsafe extern "C" fn CINTgout2e_int2e_breit_r2p2(
    mut gout: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut idx: *mut libc::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: libc::c_int,
) {
    let mut nf: libc::c_int = (*envs).nf;
    let mut nrys_roots: libc::c_int = (*envs).nrys_roots;
    let mut ix: libc::c_int = 0;
    let mut iy: libc::c_int = 0;
    let mut iz: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut g0: *mut libc::c_double = g;
    let mut g1: *mut libc::c_double = g0
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g2: *mut libc::c_double = g1
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g3: *mut libc::c_double = g2
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g4: *mut libc::c_double = g3
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g5: *mut libc::c_double = g4
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g6: *mut libc::c_double = g5
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g7: *mut libc::c_double = g6
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g8: *mut libc::c_double = g7
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g9: *mut libc::c_double = g8
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g10: *mut libc::c_double = g9
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g11: *mut libc::c_double = g10
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g12: *mut libc::c_double = g11
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g13: *mut libc::c_double = g12
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g14: *mut libc::c_double = g13
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g15: *mut libc::c_double = g14
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    CINTx1l_2e(
        g2,
        g0,
        (*envs).c2rust_unnamed_1.rl,
        (*envs).i_l + 2 as libc::c_int,
        (*envs).j_l + 1 as libc::c_int,
        (*envs).k_l + 0 as libc::c_int,
        (*envs).l_l + 1 as libc::c_int,
        envs,
    );
    CINTnabla1l_2e(
        g3,
        g2,
        (*envs).i_l + 2 as libc::c_int,
        (*envs).j_l + 1 as libc::c_int,
        (*envs).k_l + 0 as libc::c_int,
        (*envs).l_l + 0 as libc::c_int,
        envs,
    );
    CINTnabla1j_2e(
        g4,
        g0,
        (*envs).i_l + 1 as libc::c_int,
        (*envs).j_l + 0 as libc::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g5,
        g0,
        (*envs).i_l + 1 as libc::c_int,
        (*envs).j_l + 0 as libc::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as libc::c_int;
    while ix < (*envs).g_size * 3 as libc::c_int {
        *g4.offset(ix as isize) += *g5.offset(ix as isize);
        ix += 1;
        ix;
    }
    CINTnabla1j_2e(
        g7,
        g3,
        (*envs).i_l + 1 as libc::c_int,
        (*envs).j_l + 0 as libc::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g8,
        g3,
        (*envs).i_l + 1 as libc::c_int,
        (*envs).j_l + 0 as libc::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as libc::c_int;
    while ix < (*envs).g_size * 3 as libc::c_int {
        *g7.offset(ix as isize) += *g8.offset(ix as isize);
        ix += 1;
        ix;
    }
    CINTnabla1i_2e(
        g12,
        g4,
        (*envs).i_l + 0 as libc::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g15,
        g7,
        (*envs).i_l + 0 as libc::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    let mut s: libc::c_double = 0.;
    n = 0 as libc::c_int;
    while n < nf {
        ix = *idx.offset((0 as libc::c_int + n * 3 as libc::c_int) as isize);
        iy = *idx.offset((1 as libc::c_int + n * 3 as libc::c_int) as isize);
        iz = *idx.offset((2 as libc::c_int + n * 3 as libc::c_int) as isize);
        s = 0 as libc::c_int as libc::c_double;
        i = 0 as libc::c_int;
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
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
) {
    let mut ng: [libc::c_int; 8] = [
        2 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    CINTall_2e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int2e_breit_r2p2_cart(
    mut out: *mut libc::c_double,
    mut dims: *mut libc::c_int,
    mut shls: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut libc::c_double,
) -> libc::c_int {
    let mut ng: [libc::c_int; 8] = [
        2 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int2e_EnvVars(&mut envs, ng.as_mut_ptr(), shls, atm, natm, bas, nbas, env);
    envs
        .f_gout = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut libc::c_double,
                *mut libc::c_double,
                *mut libc::c_int,
                *mut CINTEnvVars,
                libc::c_int,
            ) -> (),
        >,
        Option::<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout2e_int2e_breit_r2p2
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
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
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_2e1
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut libc::c_double,
                        *mut libc::c_int,
                        *mut CINTEnvVars,
                        *mut libc::c_double,
                    ) -> (),
            ),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn int2e_breit_r2p2_sph(
    mut out: *mut libc::c_double,
    mut dims: *mut libc::c_int,
    mut shls: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut libc::c_double,
) -> libc::c_int {
    let mut ng: [libc::c_int; 8] = [
        2 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int2e_EnvVars(&mut envs, ng.as_mut_ptr(), shls, atm, natm, bas, nbas, env);
    envs
        .f_gout = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut libc::c_double,
                *mut libc::c_double,
                *mut libc::c_int,
                *mut CINTEnvVars,
                libc::c_int,
            ) -> (),
        >,
        Option::<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout2e_int2e_breit_r2p2
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
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
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_2e1
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut libc::c_double,
                        *mut libc::c_int,
                        *mut CINTEnvVars,
                        *mut libc::c_double,
                    ) -> (),
            ),
        ),
    );
}
