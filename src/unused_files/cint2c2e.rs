#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::g1e::CINTg1e_index_xyz;
use crate::g1e::CINTprim_to_ctr_0;
use crate::g1e::CINTprim_to_ctr_1;
use crate::g2c2e::CINTinit_int2c2e_EnvVars;
use crate::optimizer::CINTOpt_non0coeff_byshell;
use crate::optimizer::CINTall_2c2e_optimizer;
use crate::cint1e::int1e_cache_size;
use crate::cint2e::CINTgout2e;
use crate::fblas::CINTdmat_transpose;
use crate::fblas::CINTdplus_transpose;
use crate::cart2sph::c2s_sph_1e;
use crate::cart2sph::c2s_cart_1e;
use crate::cart2sph::c2s_cart_1e;

use crate::cint::CINTOpt;
use crate::cint::CINTEnvVars;

pub type size_t = libc::c_ulong;
pub type uintptr_t = libc::c_ulong;

extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn CINT2c2e_loop_nopt(
    mut gctr: *mut f64,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut f64,
    mut empty: *mut i32,
) -> i32 {
    let mut shls: *mut i32 = (*envs).shls;
    let mut bas: *mut i32 = (*envs).bas;
    let mut env: *mut f64 = (*envs).env;
    let mut i_sh: i32 = *shls.offset(0 as i32 as isize);
    let mut k_sh: i32 = *shls.offset(1 as i32 as isize);
    let mut i_ctr: i32 = (*envs).x_ctr[0 as i32 as usize];
    let mut k_ctr: i32 = (*envs).x_ctr[1 as i32 as usize];
    let mut i_prim: i32 = *bas
        .offset((8 as i32 * i_sh + 2 as i32) as isize);
    let mut k_prim: i32 = *bas
        .offset((8 as i32 * k_sh + 2 as i32) as isize);
    let mut ai: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * i_sh + 5 as i32) as isize) as isize,
        );
    let mut ak: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * k_sh + 5 as i32) as isize) as isize,
        );
    let mut ci: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * i_sh + 6 as i32) as isize) as isize,
        );
    let mut ck: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * k_sh + 6 as i32) as isize) as isize,
        );
    let mut expcutoff: f64 = (*envs).expcutoff;
    let mut ri: *mut f64 = (*envs).ri;
    let mut rk: *mut f64 = (*envs).rk;
    let mut n_comp: i32 = (*envs).ncomp_tensor;
    let mut fac1i: f64 = 0.;
    let mut fac1k: f64 = 0.;
    let mut ip: i32 = 0;
    let mut kp: i32 = 0;
    let mut _empty: [i32; 3] = [
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    let mut iempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(0 as i32 as isize);
    let mut kempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(1 as i32 as isize);
    let mut gempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(2 as i32 as isize);
    let mut nf: size_t = (*envs).nf as size_t;
    let nc: i32 = i_ctr * k_ctr;
    let leng: i32 = (*envs).g_size * 3 as i32
        * (((1 as i32) << (*envs).gbits) + 1 as i32);
    let lenk: i32 = nf
        .wrapping_mul(nc as libc::c_ulong)
        .wrapping_mul(n_comp as libc::c_ulong) as i32;
    let leni: i32 = nf
        .wrapping_mul(i_ctr as libc::c_ulong)
        .wrapping_mul(n_comp as libc::c_ulong) as i32;
    let len0: i32 = nf.wrapping_mul(n_comp as libc::c_ulong) as i32;
    let len: i32 = leng + lenk + leni + len0;
    let mut g: *mut f64 = 0 as *mut f64;
    g = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut f64;
    cache = g.offset(len as isize);
    let mut g1: *mut f64 = g.offset(leng as isize);
    let mut gout: *mut f64 = 0 as *mut f64;
    let mut gctri: *mut f64 = 0 as *mut f64;
    let mut gctrk: *mut f64 = 0 as *mut f64;
    if n_comp == 1 as i32 {
        gctrk = gctr;
        kempty = empty;
    } else {
        gctrk = g1;
        g1 = g1.offset(lenk as isize);
    }
    if k_ctr == 1 as i32 {
        gctri = gctrk;
        iempty = kempty;
    } else {
        gctri = g1;
        g1 = g1.offset(leni as isize);
    }
    if i_ctr == 1 as i32 {
        gout = gctri;
        gempty = iempty;
    } else {
        gout = g1;
        g1 = g1.offset(leng as isize);
    }
    let mut idx: *mut i32 = 0 as *mut i32;
    idx = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut i32;
    cache = idx.offset(((*envs).nf * 3 as i32) as isize) as *mut f64;
    CINTg1e_index_xyz(idx, envs);
    let mut non0ctri: *mut i32 = 0 as *mut i32;
    let mut non0ctrk: *mut i32 = 0 as *mut i32;
    let mut non0idxi: *mut i32 = 0 as *mut i32;
    let mut non0idxk: *mut i32 = 0 as *mut i32;
    non0ctri = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut i32;
    cache = non0ctri.offset((i_prim + k_prim + i_prim * i_ctr + k_prim * k_ctr) as isize)
        as *mut f64;
    non0ctrk = non0ctri.offset(i_prim as isize);
    non0idxi = non0ctrk.offset(k_prim as isize);
    non0idxk = non0idxi.offset((i_prim * i_ctr) as isize);
    if i_ctr > 1 as i32 {
        CINTOpt_non0coeff_byshell(non0idxi, non0ctri, ci, i_prim, i_ctr);
    }
    if k_ctr > 1 as i32 {
        CINTOpt_non0coeff_byshell(non0idxk, non0ctrk, ck, k_prim, k_ctr);
    }
    kp = 0 as i32;
    while kp < k_prim {
        (*envs).ak[0 as i32 as usize] = *ak.offset(kp as isize);
        (*envs).al[0 as i32 as usize] = 0 as i32 as f64;
        if k_ctr == 1 as i32 {
            fac1k = (*envs).common_factor * *ck.offset(kp as isize);
        } else {
            fac1k = (*envs).common_factor;
            *iempty = 1 as i32;
        }
        ip = 0 as i32;
        while ip < i_prim {
            (*envs).ai[0 as i32 as usize] = *ai.offset(ip as isize);
            (*envs).aj[0 as i32 as usize] = 0 as i32 as f64;
            if i_ctr == 1 as i32 {
                fac1i = fac1k * *ci.offset(ip as isize);
            } else {
                fac1i = fac1k;
            }
            (*envs).fac[0 as i32 as usize] = fac1i;
            if ::core::mem::transmute::<
                _,
                fn(_, _, _, _, _) -> i32,
            >(
                (Some(((*envs).f_g0_2e).expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )(g, ri, rk, expcutoff, envs) != 0
            {
                ::core::mem::transmute::<
                    _,
                    fn(_, _, _, _, _),
                >(
                    (Some(((*envs).f_gout).expect("non-null function pointer")))
                        .expect("non-null function pointer"),
                )(gout, g, idx, envs, *gempty);
                if i_ctr > 1 as i32 {
                    if *iempty != 0 {
                        CINTprim_to_ctr_0(
                            gctri,
                            gout,
                            ci.offset(ip as isize),
                            len0 as size_t,
                            i_prim,
                            i_ctr,
                            *non0ctri.offset(ip as isize),
                            non0idxi.offset((ip * i_ctr) as isize),
                        );
                    } else {
                        CINTprim_to_ctr_1(
                            gctri,
                            gout,
                            ci.offset(ip as isize),
                            len0 as size_t,
                            i_prim,
                            i_ctr,
                            *non0ctri.offset(ip as isize),
                            non0idxi.offset((ip * i_ctr) as isize),
                        );
                    }
                }
                *iempty = 0 as i32;
            }
            ip += 1;
            ip;
        }
        if *iempty == 0 {
            if k_ctr > 1 as i32 {
                if *kempty != 0 {
                    CINTprim_to_ctr_0(
                        gctrk,
                        gctri,
                        ck.offset(kp as isize),
                        leni as size_t,
                        k_prim,
                        k_ctr,
                        *non0ctrk.offset(kp as isize),
                        non0idxk.offset((kp * k_ctr) as isize),
                    );
                } else {
                    CINTprim_to_ctr_1(
                        gctrk,
                        gctri,
                        ck.offset(kp as isize),
                        leni as size_t,
                        k_prim,
                        k_ctr,
                        *non0ctrk.offset(kp as isize),
                        non0idxk.offset((kp * k_ctr) as isize),
                    );
                }
            }
            *kempty = 0 as i32;
        }
        kp += 1;
        kp;
    }
    if n_comp > 1 as i32 && *kempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gctrk,
                nf.wrapping_mul(nc as libc::c_ulong) as i32,
                n_comp,
            );
        } else {
            CINTdplus_transpose(
                gctr,
                gctrk,
                nf.wrapping_mul(nc as libc::c_ulong) as i32,
                n_comp,
            );
        }
        *empty = 0 as i32;
    }
    return (*empty == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn CINT2c2e_loop(
    mut gctr: *mut f64,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut f64,
    mut empty: *mut i32,
) -> i32 {
    let mut shls: *mut i32 = (*envs).shls;
    let mut bas: *mut i32 = (*envs).bas;
    let mut env: *mut f64 = (*envs).env;
    let mut i_sh: i32 = *shls.offset(0 as i32 as isize);
    let mut k_sh: i32 = *shls.offset(1 as i32 as isize);
    let mut i_ctr: i32 = (*envs).x_ctr[0 as i32 as usize];
    let mut k_ctr: i32 = (*envs).x_ctr[1 as i32 as usize];
    let mut i_prim: i32 = *bas
        .offset((8 as i32 * i_sh + 2 as i32) as isize);
    let mut k_prim: i32 = *bas
        .offset((8 as i32 * k_sh + 2 as i32) as isize);
    let mut ai: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * i_sh + 5 as i32) as isize) as isize,
        );
    let mut ak: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * k_sh + 5 as i32) as isize) as isize,
        );
    let mut ci: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * i_sh + 6 as i32) as isize) as isize,
        );
    let mut ck: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * k_sh + 6 as i32) as isize) as isize,
        );
    let mut expcutoff: f64 = (*envs).expcutoff;
    let mut ri: *mut f64 = (*envs).ri;
    let mut rk: *mut f64 = (*envs).rk;
    let mut n_comp: i32 = (*envs).ncomp_tensor;
    let mut fac1i: f64 = 0.;
    let mut fac1k: f64 = 0.;
    let mut ip: i32 = 0;
    let mut kp: i32 = 0;
    let mut _empty: [i32; 3] = [
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    let mut iempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(0 as i32 as isize);
    let mut kempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(1 as i32 as isize);
    let mut gempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(2 as i32 as isize);
    let mut non0ctri: *mut i32 = 0 as *mut i32;
    let mut non0ctrk: *mut i32 = 0 as *mut i32;
    let mut non0idxi: *mut i32 = 0 as *mut i32;
    let mut non0idxk: *mut i32 = 0 as *mut i32;
    non0ctri = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut i32;
    cache = non0ctri.offset((i_prim + k_prim + i_prim * i_ctr + k_prim * k_ctr) as isize)
        as *mut f64;
    non0ctrk = non0ctri.offset(i_prim as isize);
    non0idxi = non0ctrk.offset(k_prim as isize);
    non0idxk = non0idxi.offset((i_prim * i_ctr) as isize);
    if i_ctr > 1 as i32 {
        CINTOpt_non0coeff_byshell(non0idxi, non0ctri, ci, i_prim, i_ctr);
    }
    if k_ctr > 1 as i32 {
        CINTOpt_non0coeff_byshell(non0idxk, non0ctrk, ck, k_prim, k_ctr);
    }
    let mut idx: *mut i32 = *((*(*envs).opt).index_xyz_array)
        .offset(((*envs).i_l * 16 as i32 + (*envs).k_l) as isize);
    let mut nf: size_t = (*envs).nf as size_t;
    let nc: i32 = i_ctr * k_ctr;
    let leng: i32 = (*envs).g_size * 3 as i32
        * (((1 as i32) << (*envs).gbits) + 1 as i32);
    let lenk: i32 = nf
        .wrapping_mul(nc as libc::c_ulong)
        .wrapping_mul(n_comp as libc::c_ulong) as i32;
    let leni: i32 = nf
        .wrapping_mul(i_ctr as libc::c_ulong)
        .wrapping_mul(n_comp as libc::c_ulong) as i32;
    let len0: i32 = nf.wrapping_mul(n_comp as libc::c_ulong) as i32;
    let len: i32 = leng + lenk + leni + len0;
    let mut g: *mut f64 = 0 as *mut f64;
    g = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut f64;
    cache = g.offset(len as isize);
    let mut g1: *mut f64 = g.offset(leng as isize);
    let mut gout: *mut f64 = 0 as *mut f64;
    let mut gctri: *mut f64 = 0 as *mut f64;
    let mut gctrk: *mut f64 = 0 as *mut f64;
    if n_comp == 1 as i32 {
        gctrk = gctr;
        kempty = empty;
    } else {
        gctrk = g1;
        g1 = g1.offset(lenk as isize);
    }
    if k_ctr == 1 as i32 {
        gctri = gctrk;
        iempty = kempty;
    } else {
        gctri = g1;
        g1 = g1.offset(leni as isize);
    }
    if i_ctr == 1 as i32 {
        gout = gctri;
        gempty = iempty;
    } else {
        gout = g1;
        g1 = g1.offset(leng as isize);
    }
    kp = 0 as i32;
    while kp < k_prim {
        (*envs).ak[0 as i32 as usize] = *ak.offset(kp as isize);
        if k_ctr == 1 as i32 {
            fac1k = (*envs).common_factor * *ck.offset(kp as isize);
        } else {
            fac1k = (*envs).common_factor;
            *iempty = 1 as i32;
        }
        ip = 0 as i32;
        while ip < i_prim {
            (*envs).ai[0 as i32 as usize] = *ai.offset(ip as isize);
            if i_ctr == 1 as i32 {
                fac1i = fac1k * *ci.offset(ip as isize);
            } else {
                fac1i = fac1k;
            }
            (*envs).fac[0 as i32 as usize] = fac1i;
            if ::core::mem::transmute::<
                _,
                fn(_, _, _, _, _) -> i32,
            >(
                (Some(((*envs).f_g0_2e).expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )(g, ri, rk, expcutoff, envs) != 0
            {
                ::core::mem::transmute::<
                    _,
                    fn(_, _, _, _, _),
                >(
                    (Some(((*envs).f_gout).expect("non-null function pointer")))
                        .expect("non-null function pointer"),
                )(gout, g, idx, envs, *gempty);
                if i_ctr > 1 as i32 {
                    if *iempty != 0 {
                        CINTprim_to_ctr_0(
                            gctri,
                            gout,
                            ci.offset(ip as isize),
                            len0 as size_t,
                            i_prim,
                            i_ctr,
                            *non0ctri.offset(ip as isize),
                            non0idxi.offset((ip * i_ctr) as isize),
                        );
                    } else {
                        CINTprim_to_ctr_1(
                            gctri,
                            gout,
                            ci.offset(ip as isize),
                            len0 as size_t,
                            i_prim,
                            i_ctr,
                            *non0ctri.offset(ip as isize),
                            non0idxi.offset((ip * i_ctr) as isize),
                        );
                    }
                }
                *iempty = 0 as i32;
            }
            ip += 1;
            ip;
        }
        if *iempty == 0 {
            if k_ctr > 1 as i32 {
                if *kempty != 0 {
                    CINTprim_to_ctr_0(
                        gctrk,
                        gctri,
                        ck.offset(kp as isize),
                        leni as size_t,
                        k_prim,
                        k_ctr,
                        *non0ctrk.offset(kp as isize),
                        non0idxk.offset((kp * k_ctr) as isize),
                    );
                } else {
                    CINTprim_to_ctr_1(
                        gctrk,
                        gctri,
                        ck.offset(kp as isize),
                        leni as size_t,
                        k_prim,
                        k_ctr,
                        *non0ctrk.offset(kp as isize),
                        non0idxk.offset((kp * k_ctr) as isize),
                    );
                }
            }
            *kempty = 0 as i32;
        }
        kp += 1;
        kp;
    }
    if n_comp > 1 as i32 && *kempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gctrk,
                nf.wrapping_mul(nc as libc::c_ulong) as i32,
                n_comp,
            );
        } else {
            CINTdplus_transpose(
                gctr,
                gctrk,
                nf.wrapping_mul(nc as libc::c_ulong) as i32,
                n_comp,
            );
        }
        *empty = 0 as i32;
    }
    return (*empty == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn CINT2c2e_drv(
    mut out: *mut f64,
    mut dims: *mut i32,
    mut envs: *mut CINTEnvVars,
    mut opt: *mut CINTOpt,
    mut cache: *mut f64,
    mut f_c2s: Option::<unsafe extern "C" fn() -> ()>,
) -> i32 {
    let mut x_ctr: *mut i32 = ((*envs).x_ctr).as_mut_ptr();
    let mut nc: i32 = (*envs).nf * *x_ctr.offset(0 as i32 as isize)
        * *x_ctr.offset(1 as i32 as isize);
    let mut n_comp: i32 = (*envs).ncomp_e1 * (*envs).ncomp_e2
        * (*envs).ncomp_tensor;
    if out.is_null() {
        return int1e_cache_size(envs);
    }
    let mut stack: *mut f64 = 0 as *mut f64;
    if cache.is_null() {
        let mut cache_size: size_t = int1e_cache_size(envs) as size_t;
        stack = malloc(
            (::core::mem::size_of::<f64>() as libc::c_ulong)
                .wrapping_mul(cache_size),
        ) as *mut f64;
        cache = stack;
    }
    let mut gctr: *mut f64 = 0 as *mut f64;
    gctr = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut f64;
    cache = gctr.offset((nc * n_comp) as isize);
    let mut n: i32 = 0;
    let mut empty: i32 = 1 as i32;
    if !opt.is_null() {
        (*envs).opt = opt;
        CINT2c2e_loop(gctr, envs, cache, &mut empty);
    } else {
        CINT2c2e_loop_nopt(gctr, envs, cache, &mut empty);
    }
    let mut counts: [i32; 4] = [0; 4];
    if f_c2s
        == ::core::mem::transmute::<
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
        )
    {
        counts[0 as i32
            as usize] = ((*envs).i_l * 2 as i32 + 1 as i32)
            * *x_ctr.offset(0 as i32 as isize);
        counts[1 as i32
            as usize] = ((*envs).k_l * 2 as i32 + 1 as i32)
            * *x_ctr.offset(1 as i32 as isize);
    } else {
        counts[0 as i32
            as usize] = (*envs).nfi * *x_ctr.offset(0 as i32 as isize);
        counts[1 as i32
            as usize] = (*envs).c2rust_unnamed.nfk
            * *x_ctr.offset(1 as i32 as isize);
    }
    counts[2 as i32 as usize] = 1 as i32;
    counts[3 as i32 as usize] = 1 as i32;
    if dims.is_null() {
        dims = counts.as_mut_ptr();
    }
    let mut nout: i32 = *dims.offset(0 as i32 as isize)
        * *dims.offset(1 as i32 as isize);
    if empty == 0 {
        n = 0 as i32;
        while n < n_comp {
            ::core::mem::transmute::<
                _,
                fn(_, _, _, _, _),
            >(
                (Some(f_c2s.expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )(
                out.offset((nout * n) as isize),
                gctr.offset((nc * n) as isize),
                dims,
                envs,
                cache,
            );
            n += 1;
            n;
        }
    } else {
        n = 0 as i32;
        while n < n_comp {
            c2s_dset0(out.offset((nout * n) as isize), dims, counts.as_mut_ptr());
            n += 1;
            n;
        }
    }
    if !stack.is_null() {
        free(stack as *mut libc::c_void);
    }
    return (empty == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn int2c2e_sph(
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
    CINTinit_int2c2e_EnvVars(
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
            CINTgout2e
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT2c2e_drv(
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
    );
}
#[no_mangle]
pub unsafe extern "C" fn int2c2e_optimizer(
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
        0 as i32,
        0 as i32,
        0 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    CINTall_2c2e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int2c2e_cart(
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
    CINTinit_int2c2e_EnvVars(
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
            CINTgout2e
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT2c2e_drv(
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
    );
}
