#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::g1e::CINTinit_int1e_EnvVars;
use crate::g1e::CINTnabla1j_1e;
use crate::cart2sph::c2s_sph_1e;
use crate::cart2sph::c2s_cart_1e;
use crate::optimizer::CINTall_1e_optimizer;
use crate::cint1e::CINT1e_drv;

use crate::cint::CINTOpt;
use crate::cint::CINTEnvVars;


#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_kin(
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
    let mut s: [f64; 9] = [0.; 9];
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
                ) = -s[0 as i32 as usize] - s[4 as i32 as usize]
                - s[8 as i32 as usize];
        } else {
            *gout.offset((n * 1 as i32 + 0 as i32) as isize)
                += -s[0 as i32 as usize] - s[4 as i32 as usize]
                    - s[8 as i32 as usize];
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
    mut env: *mut f64,
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
    mut out: &mut [f64],
    mut dims: &mut [i32],
    mut shls: &mut [i32],
    mut atm: &mut [i32],
    mut natm: i32,
    mut bas: &mut [i32],
    mut nbas: i32,
    mut env: &mut [f64],
    mut cache: &mut [f64],
) -> i32 {
    let mut ng: [i32; 8] = [0, 2, 0, 0, 2, 1, 1, 1];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int1e_EnvVars(&mut envs, ng.as_mut_ptr(), shls.as_mut_ptr(), atm.as_mut_ptr(), natm, bas.as_mut_ptr(), nbas, env.as_mut_ptr());
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
            CINTgout1e_int1e_kin
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    return CINT1e_drv(
        out.as_mut_ptr(),
        dims.as_mut_ptr(),
        &mut envs as *mut CINTEnvVars,
        cache.as_mut_ptr(),
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
pub unsafe extern "C" fn int1e_kin_sph(
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
            CINTgout1e_int1e_kin
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
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
pub unsafe extern "C" fn int1e_kin_spinor(
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
            CINTgout1e_int1e_kin
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    panic!("Reached end of non-void function without returning");
}


#[no_mangle]
pub fn cint1e_kin_cart(
    out: &mut [f64],
    shls: &mut [i32],
    atm: &mut [i32],
    natm: i32,
    bas: &mut [i32],
    nbas: i32,
    env: &mut [f64],
) -> i32 {
    let mut dims = [0;0];
    let mut cache = [0.0;0];
    unsafe {
        return int1e_kin_cart(
            out,
            &mut dims,
            shls,
            atm,
            natm,
            bas,
            nbas,
            env,
            &mut cache,
        );
    }
}