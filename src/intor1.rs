#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments
)]

use crate::cart2sph::c2s_cart_1e;
use crate::cart2sph::c2s_sph_1e;
use crate::cint1e::CINT1e_drv;
use crate::g1e::CINTinit_int1e_EnvVars;
use crate::g1e::CINTnabla1j_1e;
use crate::optimizer::CINTall_1e_optimizer;

use crate::cint::CINTEnvVars;
use crate::cint::CINTOpt;
use crate::cint::Gout;

// Plain Rust fn: see the note on CINTgout2e in cint2e.rs.
pub unsafe fn CINTgout1e_int1e_kin(
    gout: *mut f64,
    g: *mut f64,
    idx: *mut i32,
    envs: *mut CINTEnvVars,
    gout_empty: i32,
) {
    let nf: i32 = (*envs).nf;
    let mut ix: i32 = 0;
    let mut iy: i32 = 0;
    let mut iz: i32 = 0;
    let mut n: i32 = 0;
    let g0: *mut f64 = g;
    let g1: *mut f64 = g0.offset(((*envs).g_size * 3_i32) as isize);
    let g2: *mut f64 = g1.offset(((*envs).g_size * 3_i32) as isize);
    let g3: *mut f64 = g2.offset(((*envs).g_size * 3_i32) as isize);
    let mut s: [f64; 9] = [0.; 9];
    CINTnabla1j_1e(g1, g0, (*envs).i_l, (*envs).j_l, 0_i32, envs);
    CINTnabla1j_1e(g2, g0, (*envs).i_l, (*envs).j_l + 1_i32, 0_i32, envs);
    CINTnabla1j_1e(g3, g2, (*envs).i_l, (*envs).j_l, 0_i32, envs);
    n = 0_i32;
    while n < nf {
        ix = *idx.offset((n * 3_i32) as isize);
        iy = *idx.offset((1_i32 + n * 3_i32) as isize);
        iz = *idx.offset((2_i32 + n * 3_i32) as isize);
        s[0_i32 as usize] =
            *g3.offset(ix as isize) * *g0.offset(iy as isize) * *g0.offset(iz as isize);
        s[1_i32 as usize] =
            *g2.offset(ix as isize) * *g1.offset(iy as isize) * *g0.offset(iz as isize);
        s[2_i32 as usize] =
            *g2.offset(ix as isize) * *g0.offset(iy as isize) * *g1.offset(iz as isize);
        s[3_i32 as usize] =
            *g1.offset(ix as isize) * *g2.offset(iy as isize) * *g0.offset(iz as isize);
        s[4_i32 as usize] =
            *g0.offset(ix as isize) * *g3.offset(iy as isize) * *g0.offset(iz as isize);
        s[5_i32 as usize] =
            *g0.offset(ix as isize) * *g2.offset(iy as isize) * *g1.offset(iz as isize);
        s[6_i32 as usize] =
            *g1.offset(ix as isize) * *g0.offset(iy as isize) * *g2.offset(iz as isize);
        s[7_i32 as usize] =
            *g0.offset(ix as isize) * *g1.offset(iy as isize) * *g2.offset(iz as isize);
        s[8_i32 as usize] =
            *g0.offset(ix as isize) * *g0.offset(iy as isize) * *g3.offset(iz as isize);
        if gout_empty != 0 {
            *gout.offset(n as isize) = -s[0_usize] - s[4_usize] - s[8_usize];
        } else {
            *gout.offset(n as isize) += -s[0_usize] - s[4_usize] - s[8_usize];
        }
        n += 1;
        n;
    }
}
pub unsafe fn int1e_kin_optimizer(
    opt: *mut *mut CINTOpt,
    atm: *mut i32,
    natm: i32,
    bas: *mut i32,
    nbas: i32,
    env: *mut f64,
) {
    let ng: [i32; 8] = [0_i32, 2_i32, 0_i32, 0_i32, 2_i32, 1_i32, 1_i32, 1_i32];
    CINTall_1e_optimizer(opt, &ng, atm, natm, bas, nbas, env);
}
pub unsafe fn int1e_kin_cart(
    out: *mut f64,
    dims: *mut i32,
    shls: *mut i32,
    atm: *mut i32,
    natm: i32,
    bas: *mut i32,
    nbas: i32,
    env: *mut f64,
    _opt: *mut CINTOpt,
    cache: *mut f64,
) -> i32 {
    let ng: [i32; 8] = [0_i32, 2_i32, 0_i32, 0_i32, 2_i32, 1_i32, 1_i32, 1_i32];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int1e_EnvVars(&mut envs, &ng, shls, atm, natm, bas, nbas, env);
    envs.f_gout = Some(Gout::E1Kin);
    envs.common_factor *= 0.5f64;
    CINT1e_drv(
        out,
        dims,
        &mut envs,
        cache,
        ::core::mem::transmute::<
            Option<unsafe fn(*mut f64, *mut f64, *mut i32, *mut CINTEnvVars, *mut f64) -> ()>,
            Option<unsafe fn() -> ()>,
        >(Some(
            c2s_cart_1e
                as unsafe fn(*mut f64, *mut f64, *mut i32, *mut CINTEnvVars, *mut f64) -> (),
        )),
        0_i32,
    )
}
pub unsafe fn int1e_kin_sph(
    out: *mut f64,
    dims: *mut i32,
    shls: *mut i32,
    atm: *mut i32,
    natm: i32,
    bas: *mut i32,
    nbas: i32,
    env: *mut f64,
    _opt: *mut CINTOpt,
    cache: *mut f64,
) -> i32 {
    let ng: [i32; 8] = [0_i32, 2_i32, 0_i32, 0_i32, 2_i32, 1_i32, 1_i32, 1_i32];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int1e_EnvVars(&mut envs, &ng, shls, atm, natm, bas, nbas, env);
    envs.f_gout = Some(Gout::E1Kin);
    envs.common_factor *= 0.5f64;
    CINT1e_drv(
        out,
        dims,
        &mut envs,
        cache,
        ::core::mem::transmute::<
            Option<unsafe fn(*mut f64, *mut f64, *mut i32, *mut CINTEnvVars, *mut f64) -> ()>,
            Option<unsafe fn() -> ()>,
        >(Some(
            c2s_sph_1e as unsafe fn(*mut f64, *mut f64, *mut i32, *mut CINTEnvVars, *mut f64) -> (),
        )),
        0_i32,
    )
}
pub unsafe fn int1e_kin_spinor(
    _out: *mut f64,
    _dims: *mut i32,
    shls: *mut i32,
    atm: *mut i32,
    natm: i32,
    bas: *mut i32,
    nbas: i32,
    env: *mut f64,
    _opt: *mut CINTOpt,
    _cache: *mut f64,
) -> i32 {
    let ng: [i32; 8] = [0_i32, 2_i32, 0_i32, 0_i32, 2_i32, 1_i32, 1_i32, 1_i32];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int1e_EnvVars(&mut envs, &ng, shls, atm, natm, bas, nbas, env);
    envs.f_gout = Some(Gout::E1Kin);
    envs.common_factor *= 0.5f64;
    panic!("Reached end of non-void function without returning");
}

pub fn cint1e_kin_cart(
    out: &mut [f64],
    shls: &mut [i32],
    atm: &mut [i32],
    natm: i32,
    bas: &mut [i32],
    nbas: i32,
    env: &mut [f64],
    opt: *mut CINTOpt,
) -> i32 {
    unsafe {
        int1e_kin_cart(
            out.as_mut_ptr(),
            std::ptr::null_mut::<i32>(),
            shls.as_mut_ptr(),
            atm.as_mut_ptr(),
            natm,
            bas.as_mut_ptr(),
            nbas,
            env.as_mut_ptr(),
            opt,
            std::ptr::null_mut::<f64>(),
        )
    }
}

pub fn cint1e_kin_sph(
    out: &mut [f64],
    shls: &mut [i32],
    atm: &mut [i32],
    natm: i32,
    bas: &mut [i32],
    nbas: i32,
    env: &mut [f64],
    opt: *mut CINTOpt,
) -> i32 {
    unsafe {
        int1e_kin_sph(
            out.as_mut_ptr(),
            std::ptr::null_mut::<i32>(),
            shls.as_mut_ptr(),
            atm.as_mut_ptr(),
            natm,
            bas.as_mut_ptr(),
            nbas,
            env.as_mut_ptr(),
            opt,
            std::ptr::null_mut::<f64>(),
        )
    }
}
