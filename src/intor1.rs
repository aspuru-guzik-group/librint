#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn CINTinit_int1e_EnvVars(
        envs: *mut CINTEnvVars,
        ng: *mut i32,
        shls: *mut i32,
        atm: *mut i32,
        natm: i32,
        bas: *mut i32,
        nbas: i32,
        env: *mut libc::c_double,
    );
    fn CINTnabla1j_1e(
        f: *mut libc::c_double,
        g: *mut libc::c_double,
        li: i32,
        lj: i32,
        lk: i32,
        envs: *mut CINTEnvVars,
    );
    fn c2s_sph_1e(
        opij: *mut libc::c_double,
        gctr: *mut libc::c_double,
        dims: *mut i32,
        envs: *mut CINTEnvVars,
        cache: *mut libc::c_double,
    );
    fn c2s_cart_1e(
        opij: *mut libc::c_double,
        gctr: *mut libc::c_double,
        dims: *mut i32,
        envs: *mut CINTEnvVars,
        cache: *mut libc::c_double,
    );
    fn CINTall_1e_optimizer(
        opt: *mut *mut CINTOpt,
        ng: *mut i32,
        atm: *mut i32,
        natm: i32,
        bas: *mut i32,
        nbas: i32,
        env: *mut libc::c_double,
    );
    fn CINT1e_drv(
        out: *mut libc::c_double,
        dims: *mut i32,
        envs: *mut CINTEnvVars,
        cache: *mut libc::c_double,
        f_c2s: Option::<unsafe extern "C" fn() -> ()>,
        int1e_type: i32,
    ) -> i32;
}

use crate::cint::CINTOpt;
use crate::cint::CINTEnvVars;

#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_kin(
    mut gout: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut idx: *mut i32,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: i32,
) {
    let mut nf: i32 = (*envs).nf;
    let mut ix: i32 = 0;
    let mut iy: i32 = 0;
    let mut iz: i32 = 0;
    let mut n: i32 = 0;
    let mut g0: *mut libc::c_double = g;
    let mut g1: *mut libc::c_double = g0
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g2: *mut libc::c_double = g1
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut g3: *mut libc::c_double = g2
        .offset(((*envs).g_size * 3 as i32) as isize);
    let mut s: [libc::c_double; 9] = [0.; 9];
    CINTnabla1j_1e(
        g1,
        g0,
        (*envs).i_l + 0 as i32,
        (*envs).j_l + 0 as i32,
        0 as i32,
        envs,
    );
    CINTnabla1j_1e(
        g2,
        g0,
        (*envs).i_l + 0 as i32,
        (*envs).j_l + 1 as i32,
        0 as i32,
        envs,
    );
    CINTnabla1j_1e(
        g3,
        g2,
        (*envs).i_l + 0 as i32,
        (*envs).j_l + 0 as i32,
        0 as i32,
        envs,
    );
    n = 0 as i32;
    while n < nf {
        ix = *idx.offset((0 as i32 + n * 3 as i32) as isize);
        iy = *idx.offset((1 as i32 + n * 3 as i32) as isize);
        iz = *idx.offset((2 as i32 + n * 3 as i32) as isize);
        s[0 as i32
            as usize] = *g3.offset((ix + 0 as i32) as isize)
            * *g0.offset((iy + 0 as i32) as isize)
            * *g0.offset((iz + 0 as i32) as isize);
        s[1 as i32
            as usize] = *g2.offset((ix + 0 as i32) as isize)
            * *g1.offset((iy + 0 as i32) as isize)
            * *g0.offset((iz + 0 as i32) as isize);
        s[2 as i32
            as usize] = *g2.offset((ix + 0 as i32) as isize)
            * *g0.offset((iy + 0 as i32) as isize)
            * *g1.offset((iz + 0 as i32) as isize);
        s[3 as i32
            as usize] = *g1.offset((ix + 0 as i32) as isize)
            * *g2.offset((iy + 0 as i32) as isize)
            * *g0.offset((iz + 0 as i32) as isize);
        s[4 as i32
            as usize] = *g0.offset((ix + 0 as i32) as isize)
            * *g3.offset((iy + 0 as i32) as isize)
            * *g0.offset((iz + 0 as i32) as isize);
        s[5 as i32
            as usize] = *g0.offset((ix + 0 as i32) as isize)
            * *g2.offset((iy + 0 as i32) as isize)
            * *g1.offset((iz + 0 as i32) as isize);
        s[6 as i32
            as usize] = *g1.offset((ix + 0 as i32) as isize)
            * *g0.offset((iy + 0 as i32) as isize)
            * *g2.offset((iz + 0 as i32) as isize);
        s[7 as i32
            as usize] = *g0.offset((ix + 0 as i32) as isize)
            * *g1.offset((iy + 0 as i32) as isize)
            * *g2.offset((iz + 0 as i32) as isize);
        s[8 as i32
            as usize] = *g0.offset((ix + 0 as i32) as isize)
            * *g0.offset((iy + 0 as i32) as isize)
            * *g3.offset((iz + 0 as i32) as isize);
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 1 as i32 + 0 as i32) as isize,
                ) = -s[0 as usize] - s[4 as usize]
                - s[8 as usize];
        } else {
            *gout.offset((n * 1 as i32 + 0 as i32) as isize)
                += -s[0 as usize] - s[4 as usize]
                    - s[8 as usize];
        }
        n += 1;
        n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_kin_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut libc::c_double,
) {
    let mut ng: [i32; 8] = [
        0 as i32,
        2 as i32,
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
pub unsafe extern "C" fn int1e_kin_cart(
    mut out: *mut libc::c_double,
    mut dims: *mut i32,
    mut shls: *mut i32,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut libc::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut libc::c_double,
) -> i32 {
    let mut ng: [i32; 8] = [
        0 as i32,
        2 as i32,
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
                *mut libc::c_double,
                *mut libc::c_double,
                *mut i32,
                *mut CINTEnvVars,
                i32,
            ) -> (),
        >,
        Option::<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_kin
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
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
                    *mut i32,
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
                        *mut i32,
                        *mut CINTEnvVars,
                        *mut libc::c_double,
                    ) -> (),
            ),
        ),
        0 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_kin_sph(
    mut out: *mut libc::c_double,
    mut dims: *mut i32,
    mut shls: *mut i32,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut libc::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut libc::c_double,
) -> i32 {
    let mut ng: [i32; 8] = [
        0 as i32,
        2 as i32,
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
                *mut libc::c_double,
                *mut libc::c_double,
                *mut i32,
                *mut CINTEnvVars,
                i32,
            ) -> (),
        >,
        Option::<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_kin
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
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
                    *mut i32,
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
                        *mut i32,
                        *mut CINTEnvVars,
                        *mut libc::c_double,
                    ) -> (),
            ),
        ),
        0 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_kin_spinor(
    mut out: *mut libc::c_double,
    mut dims: *mut i32,
    mut shls: *mut i32,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut libc::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut libc::c_double,
) -> i32 {
    let mut ng: [i32; 8] = [
        0 as i32,
        2 as i32,
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
                *mut libc::c_double,
                *mut libc::c_double,
                *mut i32,
                *mut CINTEnvVars,
                i32,
            ) -> (),
        >,
        Option::<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_kin
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    panic!("Reached end of non-void function without returning");
}
