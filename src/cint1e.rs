#![allow(dead_code, unused, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::optimizer::CINTOpt_log_max_pgto_coeff;
use crate::optimizer::CINTOpt_non0coeff_byshell;
use crate::optimizer::CINTset_pairdata;
use crate::g1e::CINTinit_int1e_EnvVars;
use crate::g1e::CINTg1e_index_xyz;
use crate::g1e::CINTg1e_ovlp;
use crate::g1e::CINTg1e_nuc;
use crate::g1e::CINTcommon_fac_sp;
use crate::g1e::CINTprim_to_ctr_0;
use crate::g1e::CINTprim_to_ctr_1;
use crate::fblas::CINTdmat_transpose;
use crate::cart2sph::c2s_sph_1e;
use crate::cart2sph::c2s_cart_1e;
use crate::cart2sph::c2s_dset0;

use crate::cint::PairData;
use crate::cint::CINTOpt;
use crate::cint::CINTEnvVars;

pub type uintptr_t = u64;

extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}

// Manuel up
#[no_mangle]
pub unsafe extern "C" fn CINT1e_loop(
    mut gctr: *mut f64,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut f64,
    mut int1e_type: i32,
) -> i32 {
    let mut shls: *mut i32 = (*envs).shls;
    let mut bas: *mut i32 = (*envs).bas;
    let mut env: *mut f64 = (*envs).env;
    let mut i_sh: i32 = *shls.offset(0 as isize);
    let mut j_sh: i32 = *shls.offset(1 as isize);
    let mut i_ctr: i32 = (*envs).x_ctr[0 as usize];
    let mut j_ctr: i32 = (*envs).x_ctr[1 as usize];
    let mut i_prim: i32 = *bas
        .offset((8 as i32 * i_sh + 2 as i32) as isize);
    let mut j_prim: i32 = *bas
        .offset((8 as i32 * j_sh + 2 as i32) as isize);
    let mut ai: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * i_sh + 5 as i32) as isize) as isize,
        );
    let mut aj: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * j_sh + 5 as i32) as isize) as isize,
        );
    let mut ci: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * i_sh + 6 as i32) as isize) as isize,
        );
    let mut cj: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * j_sh + 6 as i32) as isize) as isize,
        );
    let mut n_comp: i32 = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut expcutoff: f64 = (*envs).expcutoff;
    let mut log_maxci: *mut f64 = 0 as *mut f64;
    let mut log_maxcj: *mut f64 = 0 as *mut f64;
    let mut pdata_base: *mut PairData = 0 as *mut PairData;
    let mut pdata_ij: *mut PairData = 0 as *mut PairData;
    log_maxci = ((cache as uintptr_t).wrapping_add(7 as u64)
        & (8 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut f64;
    cache = log_maxci.offset((i_prim + j_prim) as isize);
    pdata_base = ((cache as uintptr_t).wrapping_add(7 as u64)
        & (8 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut PairData;
    cache = pdata_base.offset((i_prim * j_prim) as isize) as *mut f64;
    log_maxcj = log_maxci.offset(i_prim as isize);
    CINTOpt_log_max_pgto_coeff(log_maxci, ci, i_prim, i_ctr);
    CINTOpt_log_max_pgto_coeff(log_maxcj, cj, j_prim, j_ctr);
    let mut empty: [i32; 4] = [
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    let mut jempty: *mut i32 = empty
        .as_mut_ptr()
        .offset(2 as isize);
    let mut idx: *mut i32 = 0 as *mut i32;
    let nc: i32 = i_ctr * j_ctr;
    let mut g: *mut f64 = 0 as *mut f64;
    let mut gout: *mut f64 = 0 as *mut f64;
    let mut gctri: *mut f64 = 0 as *mut f64;
    let mut gctrj: *mut f64 = 0 as *mut f64;
    g = ((cache as uintptr_t).wrapping_add(7 as u64)
        & (8 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut f64;
    CINTg1e_nuc(g, envs, -(1 as i32));
    ::core::mem::transmute::<
        _,
        fn(_, _, _, _, _),
    >(
        (Some(((*envs).f_gout).expect("non-null function pointer")))
            .expect("non-null function pointer"),
    )(gout, g, idx, envs, empty);
    if n_comp > 1 as i32 && *jempty == 0 {
        CINTdmat_transpose(gctr, gctrj, (*envs).nf * nc, n_comp);
    }
    return (*jempty == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn int1e_cache_size(mut envs: *mut CINTEnvVars) -> i32 {
    let mut shls: *mut i32 = (*envs).shls;
    let mut bas: *mut i32 = (*envs).bas;
    let mut i_prim: i32 = *bas
        .offset(
            (8 as i32 * *shls.offset(0 as isize)
                + 2 as i32) as isize,
        );
    let mut j_prim: i32 = *bas
        .offset(
            (8 as i32 * *shls.offset(1 as isize)
                + 2 as i32) as isize,
        );
    let mut x_ctr: *mut i32 = ((*envs).x_ctr).as_mut_ptr();
    let mut nc: i32 = (*envs).nf * *x_ctr.offset(0 as isize)
        * *x_ctr.offset(1 as isize);
    let mut n_comp: i32 = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut leng: i32 = (*envs).g_size * 3 as i32
        * (((1 as i32) << (*envs).gbits) + 1 as i32);
    let mut lenj: i32 = (*envs).nf * nc * n_comp;
    let mut leni: i32 = (*envs).nf * *x_ctr.offset(0 as isize)
        * n_comp;
    let mut len0: i32 = (*envs).nf * n_comp;
    let mut pdata_size: i32 = i_prim * j_prim * 5 as i32
        + i_prim * *x_ctr.offset(0 as isize)
        + j_prim * *x_ctr.offset(1 as isize)
        + (i_prim + j_prim) * 2 as i32 + (*envs).nf * 3 as i32;
    let mut cache_size: i32 = if nc * n_comp + leng + lenj + leni + len0
        + pdata_size > nc * n_comp + (*envs).nf * 8 as i32 * 2 as i32
    {
        nc * n_comp + leng + lenj + leni + len0 + pdata_size
    } else {
        nc * n_comp + (*envs).nf * 8 as i32 * 2 as i32
    };
    return cache_size;
}

// Manuel down
#[no_mangle]
pub unsafe extern "C" fn CINT1e_drv(
    mut out: *mut f64,
    mut dims: *mut i32,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut f64,
    mut f_c2s: Option::<unsafe extern "C" fn() -> ()>,
    mut int1e_type: i32,
) -> i32 {
    if out.is_null() {
        return int1e_cache_size(envs);
    }
    let mut x_ctr: *mut i32 = ((*envs).x_ctr).as_mut_ptr();
    let mut nc: i32 = (*envs).nf * *x_ctr.offset(0 as isize)
        * *x_ctr.offset(1 as isize);
    let mut n_comp: i32 = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut stack: *mut f64 = 0 as *mut f64;
    if cache.is_null() {
        let mut cache_size: u64 = int1e_cache_size(envs) as u64;
        stack = malloc(
            (::core::mem::size_of::<f64>() as u64)
                .wrapping_mul(cache_size),
        ) as *mut f64;
        cache = stack;
    }
    let mut gctr: *mut f64 = 0 as *mut f64;
    gctr = ((cache as uintptr_t).wrapping_add(7 as u64)
        & (8 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut f64;
    cache = gctr.offset((nc * n_comp) as isize);
    let mut has_value: i32 = CINT1e_loop(gctr, envs, cache, int1e_type);
    return has_value;
}
// Manuel 
//unsafe extern "C" fn make_g1e_gout(
//    mut gout: *mut f64,
//    mut g: *mut f64,
//    mut idx: *mut i32,
//    mut envs: *mut CINTEnvVars,
//    mut empty: i32,
//    mut int1e_type: i32,
//) {
//    //let mut ia: i32 = 0;
//    // new
//    CINTg1e_nuc(g, envs, -(1 as i32));
//    ::core::mem::transmute::<
//        _,
//        fn(_, _, _, _, _),
//    >(
//        (Some(((*envs).f_gout).expect("non-null function pointer")))
//            .expect("non-null function pointer"),
//    )(gout, g, idx, envs, empty);
//}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e(
    mut gout: *mut f64,
    mut g: *mut f64,
    mut idx: *mut i32,
    mut envs: *mut CINTEnvVars,
    mut empty: i32,
) {
    let mut nf: i32 = (*envs).nf;
    let mut n: i32 = 0;
    let mut ix: i32 = 0;
    let mut iy: i32 = 0;
    let mut iz: i32 = 0;
    if empty != 0 {
        n = 0 as i32;
        while n < nf {
            ix = *idx.offset((n * 3 as i32 + 0 as i32) as isize);
            iy = *idx.offset((n * 3 as i32 + 1 as i32) as isize);
            iz = *idx.offset((n * 3 as i32 + 2 as i32) as isize);
            *gout
                .offset(
                    n as isize,
                ) = *g.offset(ix as isize) * *g.offset(iy as isize)
                * *g.offset(iz as isize);
            n += 1;
            n;
        }
    } else {
        n = 0 as i32;
        while n < nf {
            ix = *idx.offset((n * 3 as i32 + 0 as i32) as isize);
            iy = *idx.offset((n * 3 as i32 + 1 as i32) as isize);
            iz = *idx.offset((n * 3 as i32 + 2 as i32) as isize);
            *gout.offset(n as isize)
                += *g.offset(ix as isize) * *g.offset(iy as isize)
                    * *g.offset(iz as isize);
            n += 1;
            n;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_nuc(
    mut gout: *mut f64,
    mut g: *mut f64,
    mut idx: *mut i32,
    mut envs: *mut CINTEnvVars,
    mut empty: i32,
) {
    let mut nf: i32 = (*envs).nf;
    let mut nrys_roots: i32 = (*envs).nrys_roots;
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    let mut gx: *mut f64 = 0 as *mut f64;
    let mut gy: *mut f64 = 0 as *mut f64;
    let mut gz: *mut f64 = 0 as *mut f64;
    let mut s: f64 = 0.;
    if empty != 0 {
        n = 0 as i32;
        while n < nf {
            gx = g
                .offset(
                    *idx.offset((n * 3 as i32 + 0 as i32) as isize)
                        as isize,
                );
            gy = g
                .offset(
                    *idx.offset((n * 3 as i32 + 1 as i32) as isize)
                        as isize,
                );
            gz = g
                .offset(
                    *idx.offset((n * 3 as i32 + 2 as i32) as isize)
                        as isize,
                );
            s = 0 as f64;
            i = 0 as i32;
            while i < nrys_roots {
                s
                    += *gx.offset(i as isize) * *gy.offset(i as isize)
                        * *gz.offset(i as isize);
                i += 1;
                i;
            }
            *gout.offset(n as isize) = s;
            n += 1;
            n;
        }
    } else {
        n = 0 as i32;
        while n < nf {
            gx = g
                .offset(
                    *idx.offset((n * 3 as i32 + 0 as i32) as isize)
                        as isize,
                );
            gy = g
                .offset(
                    *idx.offset((n * 3 as i32 + 1 as i32) as isize)
                        as isize,
                );
            gz = g
                .offset(
                    *idx.offset((n * 3 as i32 + 2 as i32) as isize)
                        as isize,
                );
            s = 0 as f64;
            i = 0 as i32;
            while i < nrys_roots {
                s
                    += *gx.offset(i as isize) * *gy.offset(i as isize)
                        * *gz.offset(i as isize);
                i += 1;
                i;
            }
            *gout.offset(n as isize) += s;
            n += 1;
            n;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn int1e_ovlp_sph(
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
        0 as i32,
        0 as i32,
        0 as i32,
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
            CINTgout1e
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
pub unsafe extern "C" fn int1e_ovlp_cart(
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
        0 as i32,
        0 as i32,
        0 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
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
pub unsafe extern "C" fn int1e_ovlp_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
) {
    *opt = 0 as *mut CINTOpt;
}
#[no_mangle]
pub unsafe extern "C" fn int1e_nuc_sph(
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
        0 as i32,
        0 as i32,
        0 as i32,
        1 as i32,
        0 as i32,
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
            CINTgout1e_nuc
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
        2 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_nuc_cart(
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
        0 as i32,
        0 as i32,
        0 as i32,
        1 as i32,
        0 as i32,
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
            CINTgout1e_nuc
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
        2 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_nuc_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
) {
    *opt = 0 as *mut CINTOpt;
}
// #[no_mangle]
// pub unsafe extern "C" fn cint1e_ovlp_cart(
//     mut out: *mut f64,
//     mut shls: *mut i32,
//     mut atm: *mut i32,
//     mut natm: i32,
//     mut bas: *mut i32,
//     mut nbas: i32,
//     mut env: *mut f64,
//     mut opt: *mut CINTOpt,
// ) -> i32 {
//     return int1e_ovlp_cart(
//         out,
//         0 as *mut i32,
//         shls,
//         atm,
//         natm,
//         bas,
//         nbas,
//         env,
//         opt,
//         0 as *mut f64,
//     );
// }
#[no_mangle]
pub unsafe extern "C" fn cint1e_ovlp_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
) {
    int1e_ovlp_optimizer(opt, atm, natm, bas, nbas, env);
}
// #[no_mangle]
// pub unsafe extern "C" fn cint1e_ovlp_sph(
//     mut out: *mut f64,
//     mut shls: *mut i32,
//     mut atm: *mut i32,
//     mut natm: i32,
//     mut bas: *mut i32,
//     mut nbas: i32,
//     mut env: *mut f64,
//     mut opt: *mut CINTOpt,
// ) -> i32 {
//     return int1e_ovlp_sph(
//         out,
//         0 as *mut i32,
//         shls,
//         atm,
//         natm,
//         bas,
//         nbas,
//         env,
//         opt,
//         0 as *mut f64,
//     );
// }
#[no_mangle]
pub unsafe extern "C" fn cint1e_ovlp_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
) {
    int1e_ovlp_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_ovlp_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
) {
    int1e_ovlp_optimizer(opt, atm, natm, bas, nbas, env);
}

#[no_mangle]
pub unsafe extern "C" fn cint1e_nuc_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
) {
    int1e_nuc_optimizer(opt, atm, natm, bas, nbas, env);
}
// #[no_mangle]
// pub unsafe extern "C" fn cint1e_nuc_sph(
//     mut out: *mut f64,
//     mut shls: *mut i32,
//     mut atm: *mut i32,
//     mut natm: i32,
//     mut bas: *mut i32,
//     mut nbas: i32,
//     mut env: *mut f64,
//     mut opt: *mut CINTOpt,
// ) -> i32 {
//     return int1e_nuc_sph(
//         out,
//         0 as *mut i32,
//         shls,
//         atm,
//         natm,
//         bas,
//         nbas,
//         env,
//         opt,
//         0 as *mut f64,
//     );
// }
#[no_mangle]
pub unsafe extern "C" fn cint1e_nuc_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
) {
    int1e_nuc_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_nuc_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
) {
    int1e_nuc_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_ovlp_sph_(
    mut out: *mut f64,
    mut shls: *mut i32,
    mut atm: *mut i32,
    mut natm: *mut i32,
    mut bas: *mut i32,
    mut nbas: *mut i32,
    mut env: *mut f64,
    mut optptr_as_integer8: u64,
) -> i32 {
    let mut opt: *mut *mut CINTOpt = optptr_as_integer8 as *mut *mut CINTOpt;
    return int1e_ovlp_sph(
        out,
        0 as *mut i32,
        shls,
        atm,
        *natm,
        bas,
        *nbas,
        env,
        *opt,
        0 as *mut f64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_ovlp_sph_optimizer_(
    mut optptr_as_integer8: u64,
    mut atm: *mut i32,
    mut natm: *mut i32,
    mut bas: *mut i32,
    mut nbas: *mut i32,
    mut env: *mut f64,
) {
    let mut opt: *mut *mut CINTOpt = optptr_as_integer8 as *mut *mut CINTOpt;
    int1e_ovlp_optimizer(opt, atm, *natm, bas, *nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_ovlp_cart_(
    mut out: *mut f64,
    mut shls: *mut i32,
    mut atm: *mut i32,
    mut natm: *mut i32,
    mut bas: *mut i32,
    mut nbas: *mut i32,
    mut env: *mut f64,
    mut optptr_as_integer8: u64,
) -> i32 {
    let mut opt: *mut *mut CINTOpt = optptr_as_integer8 as *mut *mut CINTOpt;
    return int1e_ovlp_cart(
        out,
        0 as *mut i32,
        shls,
        atm,
        *natm,
        bas,
        *nbas,
        env,
        *opt,
        0 as *mut f64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_ovlp_cart_optimizer_(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut i32,
    mut natm: *mut i32,
    mut bas: *mut i32,
    mut nbas: *mut i32,
    mut env: *mut f64,
) {
    int1e_ovlp_optimizer(opt, atm, *natm, bas, *nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_ovlp_optimizer_(
    mut optptr_as_integer8: u64,
    mut atm: *mut i32,
    mut natm: *mut i32,
    mut bas: *mut i32,
    mut nbas: *mut i32,
    mut env: *mut f64,
) {
    let mut opt: *mut *mut CINTOpt = optptr_as_integer8 as *mut *mut CINTOpt;
    int1e_ovlp_optimizer(opt, atm, *natm, bas, *nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_nuc_sph_(
    mut out: *mut f64,
    mut shls: *mut i32,
    mut atm: *mut i32,
    mut natm: *mut i32,
    mut bas: *mut i32,
    mut nbas: *mut i32,
    mut env: *mut f64,
    mut optptr_as_integer8: u64,
) -> i32 {
    let mut opt: *mut *mut CINTOpt = optptr_as_integer8 as *mut *mut CINTOpt;
    return int1e_nuc_sph(
        out,
        0 as *mut i32,
        shls,
        atm,
        *natm,
        bas,
        *nbas,
        env,
        *opt,
        0 as *mut f64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_nuc_sph_optimizer_(
    mut optptr_as_integer8: u64,
    mut atm: *mut i32,
    mut natm: *mut i32,
    mut bas: *mut i32,
    mut nbas: *mut i32,
    mut env: *mut f64,
) {
    let mut opt: *mut *mut CINTOpt = optptr_as_integer8 as *mut *mut CINTOpt;
    int1e_nuc_optimizer(opt, atm, *natm, bas, *nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_nuc_cart_(
    mut out: *mut f64,
    mut shls: *mut i32,
    mut atm: *mut i32,
    mut natm: *mut i32,
    mut bas: *mut i32,
    mut nbas: *mut i32,
    mut env: *mut f64,
    mut optptr_as_integer8: u64,
) -> i32 {
    let mut opt: *mut *mut CINTOpt = optptr_as_integer8 as *mut *mut CINTOpt;
    return int1e_nuc_cart(
        out,
        0 as *mut i32,
        shls,
        atm,
        *natm,
        bas,
        *nbas,
        env,
        *opt,
        0 as *mut f64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_nuc_cart_optimizer_(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut i32,
    mut natm: *mut i32,
    mut bas: *mut i32,
    mut nbas: *mut i32,
    mut env: *mut f64,
) {
    int1e_nuc_optimizer(opt, atm, *natm, bas, *nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_nuc_optimizer_(
    mut optptr_as_integer8: u64,
    mut atm: *mut i32,
    mut natm: *mut i32,
    mut bas: *mut i32,
    mut nbas: *mut i32,
    mut env: *mut f64,
) {
    let mut opt: *mut *mut CINTOpt = optptr_as_integer8 as *mut *mut CINTOpt;
    int1e_nuc_optimizer(opt, atm, *natm, bas, *nbas, env);
}


#[no_mangle]
pub fn cint1e_ovlp_cart(
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
        return int1e_ovlp_cart(
            out.as_mut_ptr(),
            0 as *mut i32,
            shls.as_mut_ptr(),
            atm.as_mut_ptr(),
            natm,
            bas.as_mut_ptr(),
            nbas,
            env.as_mut_ptr(),
            opt,
            0 as *mut f64,
        );
    }
}
#[no_mangle]
pub fn cint1e_ovlp_sph(
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
        return int1e_ovlp_sph(
            out.as_mut_ptr(),
            0 as *mut i32,
            shls.as_mut_ptr(),
            atm.as_mut_ptr(),
            natm,
            bas.as_mut_ptr(),
            nbas,
            env.as_mut_ptr(),
            opt,
            0 as *mut f64,
        );
    }
}

#[no_mangle]
pub fn cint1e_nuc_cart(
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
        return int1e_nuc_cart(
            out.as_mut_ptr(),
            0 as *mut i32,
            shls.as_mut_ptr(),
            atm.as_mut_ptr(),
            natm,
            bas.as_mut_ptr(),
            nbas,
            env.as_mut_ptr(),
            opt,
            0 as *mut f64,
        );
    }
}
#[no_mangle]
pub fn cint1e_nuc_sph(
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
        return int1e_nuc_sph(
            out.as_mut_ptr(),
            0 as *mut i32,
            shls.as_mut_ptr(),
            atm.as_mut_ptr(),
            natm,
            bas.as_mut_ptr(),
            nbas,
            env.as_mut_ptr(),
            opt,
            0 as *mut f64,
        );
    }
}
