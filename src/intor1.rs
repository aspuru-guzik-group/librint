#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn CINTinit_int1e_EnvVars(
        envs: *mut CINTEnvVars,
        ng: *mut libc::c_int,
        shls: *mut libc::c_int,
        atm: *mut libc::c_int,
        natm: libc::c_int,
        bas: *mut libc::c_int,
        nbas: libc::c_int,
        env: *mut libc::c_double,
    );
    fn CINTnabla1j_1e(
        f: *mut libc::c_double,
        g: *mut libc::c_double,
        li: libc::c_int,
        lj: libc::c_int,
        lk: libc::c_int,
        envs: *mut CINTEnvVars,
    );
    fn c2s_sph_1e(
        opij: *mut libc::c_double,
        gctr: *mut libc::c_double,
        dims: *mut libc::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut libc::c_double,
    );
    fn c2s_cart_1e(
        opij: *mut libc::c_double,
        gctr: *mut libc::c_double,
        dims: *mut libc::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut libc::c_double,
    );
    fn CINTall_1e_optimizer(
        opt: *mut *mut CINTOpt,
        ng: *mut libc::c_int,
        atm: *mut libc::c_int,
        natm: libc::c_int,
        bas: *mut libc::c_int,
        nbas: libc::c_int,
        env: *mut libc::c_double,
    );
    fn CINT1e_drv(
        out: *mut libc::c_double,
        dims: *mut libc::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut libc::c_double,
        f_c2s: Option::<unsafe extern "C" fn() -> ()>,
        int1e_type: libc::c_int,
    ) -> libc::c_int;
}

use crate::cint::CINTOpt;
use crate::cint::CINTEnvVars;

#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_kin(
    mut gout: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut idx: *mut libc::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: libc::c_int,
) {
    let mut nf: libc::c_int = (*envs).nf;
    let mut ix: libc::c_int = 0;
    let mut iy: libc::c_int = 0;
    let mut iz: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut g0: *mut libc::c_double = g;
    let mut g1: *mut libc::c_double = g0
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g2: *mut libc::c_double = g1
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g3: *mut libc::c_double = g2
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut s: [libc::c_double; 9] = [0.; 9];
    CINTnabla1j_1e(
        g1,
        g0,
        (*envs).i_l + 0 as libc::c_int,
        (*envs).j_l + 0 as libc::c_int,
        0 as libc::c_int,
        envs,
    );
    CINTnabla1j_1e(
        g2,
        g0,
        (*envs).i_l + 0 as libc::c_int,
        (*envs).j_l + 1 as libc::c_int,
        0 as libc::c_int,
        envs,
    );
    CINTnabla1j_1e(
        g3,
        g2,
        (*envs).i_l + 0 as libc::c_int,
        (*envs).j_l + 0 as libc::c_int,
        0 as libc::c_int,
        envs,
    );
    n = 0 as libc::c_int;
    while n < nf {
        ix = *idx.offset((0 as libc::c_int + n * 3 as libc::c_int) as isize);
        iy = *idx.offset((1 as libc::c_int + n * 3 as libc::c_int) as isize);
        iz = *idx.offset((2 as libc::c_int + n * 3 as libc::c_int) as isize);
        s[0 as libc::c_int
            as usize] = *g3.offset((ix + 0 as libc::c_int) as isize)
            * *g0.offset((iy + 0 as libc::c_int) as isize)
            * *g0.offset((iz + 0 as libc::c_int) as isize);
        s[1 as libc::c_int
            as usize] = *g2.offset((ix + 0 as libc::c_int) as isize)
            * *g1.offset((iy + 0 as libc::c_int) as isize)
            * *g0.offset((iz + 0 as libc::c_int) as isize);
        s[2 as libc::c_int
            as usize] = *g2.offset((ix + 0 as libc::c_int) as isize)
            * *g0.offset((iy + 0 as libc::c_int) as isize)
            * *g1.offset((iz + 0 as libc::c_int) as isize);
        s[3 as libc::c_int
            as usize] = *g1.offset((ix + 0 as libc::c_int) as isize)
            * *g2.offset((iy + 0 as libc::c_int) as isize)
            * *g0.offset((iz + 0 as libc::c_int) as isize);
        s[4 as libc::c_int
            as usize] = *g0.offset((ix + 0 as libc::c_int) as isize)
            * *g3.offset((iy + 0 as libc::c_int) as isize)
            * *g0.offset((iz + 0 as libc::c_int) as isize);
        s[5 as libc::c_int
            as usize] = *g0.offset((ix + 0 as libc::c_int) as isize)
            * *g2.offset((iy + 0 as libc::c_int) as isize)
            * *g1.offset((iz + 0 as libc::c_int) as isize);
        s[6 as libc::c_int
            as usize] = *g1.offset((ix + 0 as libc::c_int) as isize)
            * *g0.offset((iy + 0 as libc::c_int) as isize)
            * *g2.offset((iz + 0 as libc::c_int) as isize);
        s[7 as libc::c_int
            as usize] = *g0.offset((ix + 0 as libc::c_int) as isize)
            * *g1.offset((iy + 0 as libc::c_int) as isize)
            * *g2.offset((iz + 0 as libc::c_int) as isize);
        s[8 as libc::c_int
            as usize] = *g0.offset((ix + 0 as libc::c_int) as isize)
            * *g0.offset((iy + 0 as libc::c_int) as isize)
            * *g3.offset((iz + 0 as libc::c_int) as isize);
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 1 as libc::c_int + 0 as libc::c_int) as isize,
                ) = -s[0 as libc::c_int as usize] - s[4 as libc::c_int as usize]
                - s[8 as libc::c_int as usize];
        } else {
            *gout.offset((n * 1 as libc::c_int + 0 as libc::c_int) as isize)
                += -s[0 as libc::c_int as usize] - s[4 as libc::c_int as usize]
                    - s[8 as libc::c_int as usize];
        }
        n += 1;
        n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_kin_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
) {
    let mut ng: [libc::c_int; 8] = [
        0 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    CINTall_1e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int1e_kin_cart(
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
        0 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int1e_EnvVars(&mut envs, ng.as_mut_ptr(), shls, atm, natm, bas, nbas, env);
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
            CINTgout1e_int1e_kin
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    return CINT1e_drv(
        out,
        dims,
        &mut envs,
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
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut libc::c_double,
                        *mut libc::c_int,
                        *mut CINTEnvVars,
                        *mut libc::c_double,
                    ) -> (),
            ),
        ),
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_kin_sph(
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
        0 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int1e_EnvVars(&mut envs, ng.as_mut_ptr(), shls, atm, natm, bas, nbas, env);
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
            CINTgout1e_int1e_kin
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    return CINT1e_drv(
        out,
        dims,
        &mut envs,
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
                c2s_sph_1e
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut libc::c_double,
                        *mut libc::c_int,
                        *mut CINTEnvVars,
                        *mut libc::c_double,
                    ) -> (),
            ),
        ),
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_kin_spinor(
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
        0 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int1e_EnvVars(&mut envs, ng.as_mut_ptr(), shls, atm, natm, bas, nbas, env);
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
            CINTgout1e_int1e_kin
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    panic!("Reached end of non-void function without returning");
}
