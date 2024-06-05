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
    mut empty: *mut libc::c_int,
) -> libc::c_int {
    let mut shls: *mut libc::c_int = (*envs).shls;
    let mut bas: *mut libc::c_int = (*envs).bas;
    let mut env: *mut f64 = (*envs).env;
    let mut i_sh: libc::c_int = *shls.offset(0 as libc::c_int as isize);
    let mut k_sh: libc::c_int = *shls.offset(1 as libc::c_int as isize);
    let mut i_ctr: libc::c_int = (*envs).x_ctr[0 as libc::c_int as usize];
    let mut k_ctr: libc::c_int = (*envs).x_ctr[1 as libc::c_int as usize];
    let mut i_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * i_sh + 2 as libc::c_int) as isize);
    let mut k_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * k_sh + 2 as libc::c_int) as isize);
    let mut ai: *mut f64 = env
        .offset(
            *bas.offset((8 as libc::c_int * i_sh + 5 as libc::c_int) as isize) as isize,
        );
    let mut ak: *mut f64 = env
        .offset(
            *bas.offset((8 as libc::c_int * k_sh + 5 as libc::c_int) as isize) as isize,
        );
    let mut ci: *mut f64 = env
        .offset(
            *bas.offset((8 as libc::c_int * i_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut ck: *mut f64 = env
        .offset(
            *bas.offset((8 as libc::c_int * k_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut expcutoff: f64 = (*envs).expcutoff;
    let mut ri: *mut f64 = (*envs).ri;
    let mut rk: *mut f64 = (*envs).rk;
    let mut n_comp: libc::c_int = (*envs).ncomp_tensor;
    let mut fac1i: f64 = 0.;
    let mut fac1k: f64 = 0.;
    let mut ip: libc::c_int = 0;
    let mut kp: libc::c_int = 0;
    let mut _empty: [libc::c_int; 3] = [
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    let mut iempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize);
    let mut kempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize);
    let mut gempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize);
    let mut nf: size_t = (*envs).nf as size_t;
    let nc: libc::c_int = i_ctr * k_ctr;
    let leng: libc::c_int = (*envs).g_size * 3 as libc::c_int
        * (((1 as libc::c_int) << (*envs).gbits) + 1 as libc::c_int);
    let lenk: libc::c_int = nf
        .wrapping_mul(nc as libc::c_ulong)
        .wrapping_mul(n_comp as libc::c_ulong) as libc::c_int;
    let leni: libc::c_int = nf
        .wrapping_mul(i_ctr as libc::c_ulong)
        .wrapping_mul(n_comp as libc::c_ulong) as libc::c_int;
    let len0: libc::c_int = nf.wrapping_mul(n_comp as libc::c_ulong) as libc::c_int;
    let len: libc::c_int = leng + lenk + leni + len0;
    let mut g: *mut f64 = 0 as *mut f64;
    g = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut f64;
    cache = g.offset(len as isize);
    let mut g1: *mut f64 = g.offset(leng as isize);
    let mut gout: *mut f64 = 0 as *mut f64;
    let mut gctri: *mut f64 = 0 as *mut f64;
    let mut gctrk: *mut f64 = 0 as *mut f64;
    if n_comp == 1 as libc::c_int {
        gctrk = gctr;
        kempty = empty;
    } else {
        gctrk = g1;
        g1 = g1.offset(lenk as isize);
    }
    if k_ctr == 1 as libc::c_int {
        gctri = gctrk;
        iempty = kempty;
    } else {
        gctri = g1;
        g1 = g1.offset(leni as isize);
    }
    if i_ctr == 1 as libc::c_int {
        gout = gctri;
        gempty = iempty;
    } else {
        gout = g1;
        g1 = g1.offset(leng as isize);
    }
    let mut idx: *mut libc::c_int = 0 as *mut libc::c_int;
    idx = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_int;
    cache = idx.offset(((*envs).nf * 3 as libc::c_int) as isize) as *mut f64;
    CINTg1e_index_xyz(idx, envs);
    let mut non0ctri: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut non0ctrk: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut non0idxi: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut non0idxk: *mut libc::c_int = 0 as *mut libc::c_int;
    non0ctri = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_int;
    cache = non0ctri.offset((i_prim + k_prim + i_prim * i_ctr + k_prim * k_ctr) as isize)
        as *mut f64;
    non0ctrk = non0ctri.offset(i_prim as isize);
    non0idxi = non0ctrk.offset(k_prim as isize);
    non0idxk = non0idxi.offset((i_prim * i_ctr) as isize);
    if i_ctr > 1 as libc::c_int {
        CINTOpt_non0coeff_byshell(non0idxi, non0ctri, ci, i_prim, i_ctr);
    }
    if k_ctr > 1 as libc::c_int {
        CINTOpt_non0coeff_byshell(non0idxk, non0ctrk, ck, k_prim, k_ctr);
    }
    kp = 0 as libc::c_int;
    while kp < k_prim {
        (*envs).ak[0 as libc::c_int as usize] = *ak.offset(kp as isize);
        (*envs).al[0 as libc::c_int as usize] = 0 as libc::c_int as f64;
        if k_ctr == 1 as libc::c_int {
            fac1k = (*envs).common_factor * *ck.offset(kp as isize);
        } else {
            fac1k = (*envs).common_factor;
            *iempty = 1 as libc::c_int;
        }
        ip = 0 as libc::c_int;
        while ip < i_prim {
            (*envs).ai[0 as libc::c_int as usize] = *ai.offset(ip as isize);
            (*envs).aj[0 as libc::c_int as usize] = 0 as libc::c_int as f64;
            if i_ctr == 1 as libc::c_int {
                fac1i = fac1k * *ci.offset(ip as isize);
            } else {
                fac1i = fac1k;
            }
            (*envs).fac[0 as libc::c_int as usize] = fac1i;
            if ::core::mem::transmute::<
                _,
                fn(_, _, _, _, _) -> libc::c_int,
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
                if i_ctr > 1 as libc::c_int {
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
                *iempty = 0 as libc::c_int;
            }
            ip += 1;
            ip;
        }
        if *iempty == 0 {
            if k_ctr > 1 as libc::c_int {
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
            *kempty = 0 as libc::c_int;
        }
        kp += 1;
        kp;
    }
    if n_comp > 1 as libc::c_int && *kempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gctrk,
                nf.wrapping_mul(nc as libc::c_ulong) as libc::c_int,
                n_comp,
            );
        } else {
            CINTdplus_transpose(
                gctr,
                gctrk,
                nf.wrapping_mul(nc as libc::c_ulong) as libc::c_int,
                n_comp,
            );
        }
        *empty = 0 as libc::c_int;
    }
    return (*empty == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINT2c2e_loop(
    mut gctr: *mut f64,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut f64,
    mut empty: *mut libc::c_int,
) -> libc::c_int {
    let mut shls: *mut libc::c_int = (*envs).shls;
    let mut bas: *mut libc::c_int = (*envs).bas;
    let mut env: *mut f64 = (*envs).env;
    let mut i_sh: libc::c_int = *shls.offset(0 as libc::c_int as isize);
    let mut k_sh: libc::c_int = *shls.offset(1 as libc::c_int as isize);
    let mut i_ctr: libc::c_int = (*envs).x_ctr[0 as libc::c_int as usize];
    let mut k_ctr: libc::c_int = (*envs).x_ctr[1 as libc::c_int as usize];
    let mut i_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * i_sh + 2 as libc::c_int) as isize);
    let mut k_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * k_sh + 2 as libc::c_int) as isize);
    let mut ai: *mut f64 = env
        .offset(
            *bas.offset((8 as libc::c_int * i_sh + 5 as libc::c_int) as isize) as isize,
        );
    let mut ak: *mut f64 = env
        .offset(
            *bas.offset((8 as libc::c_int * k_sh + 5 as libc::c_int) as isize) as isize,
        );
    let mut ci: *mut f64 = env
        .offset(
            *bas.offset((8 as libc::c_int * i_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut ck: *mut f64 = env
        .offset(
            *bas.offset((8 as libc::c_int * k_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut expcutoff: f64 = (*envs).expcutoff;
    let mut ri: *mut f64 = (*envs).ri;
    let mut rk: *mut f64 = (*envs).rk;
    let mut n_comp: libc::c_int = (*envs).ncomp_tensor;
    let mut fac1i: f64 = 0.;
    let mut fac1k: f64 = 0.;
    let mut ip: libc::c_int = 0;
    let mut kp: libc::c_int = 0;
    let mut _empty: [libc::c_int; 3] = [
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    let mut iempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize);
    let mut kempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize);
    let mut gempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize);
    let mut non0ctri: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut non0ctrk: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut non0idxi: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut non0idxk: *mut libc::c_int = 0 as *mut libc::c_int;
    non0ctri = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_int;
    cache = non0ctri.offset((i_prim + k_prim + i_prim * i_ctr + k_prim * k_ctr) as isize)
        as *mut f64;
    non0ctrk = non0ctri.offset(i_prim as isize);
    non0idxi = non0ctrk.offset(k_prim as isize);
    non0idxk = non0idxi.offset((i_prim * i_ctr) as isize);
    if i_ctr > 1 as libc::c_int {
        CINTOpt_non0coeff_byshell(non0idxi, non0ctri, ci, i_prim, i_ctr);
    }
    if k_ctr > 1 as libc::c_int {
        CINTOpt_non0coeff_byshell(non0idxk, non0ctrk, ck, k_prim, k_ctr);
    }
    let mut idx: *mut libc::c_int = *((*(*envs).opt).index_xyz_array)
        .offset(((*envs).i_l * 16 as libc::c_int + (*envs).k_l) as isize);
    let mut nf: size_t = (*envs).nf as size_t;
    let nc: libc::c_int = i_ctr * k_ctr;
    let leng: libc::c_int = (*envs).g_size * 3 as libc::c_int
        * (((1 as libc::c_int) << (*envs).gbits) + 1 as libc::c_int);
    let lenk: libc::c_int = nf
        .wrapping_mul(nc as libc::c_ulong)
        .wrapping_mul(n_comp as libc::c_ulong) as libc::c_int;
    let leni: libc::c_int = nf
        .wrapping_mul(i_ctr as libc::c_ulong)
        .wrapping_mul(n_comp as libc::c_ulong) as libc::c_int;
    let len0: libc::c_int = nf.wrapping_mul(n_comp as libc::c_ulong) as libc::c_int;
    let len: libc::c_int = leng + lenk + leni + len0;
    let mut g: *mut f64 = 0 as *mut f64;
    g = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut f64;
    cache = g.offset(len as isize);
    let mut g1: *mut f64 = g.offset(leng as isize);
    let mut gout: *mut f64 = 0 as *mut f64;
    let mut gctri: *mut f64 = 0 as *mut f64;
    let mut gctrk: *mut f64 = 0 as *mut f64;
    if n_comp == 1 as libc::c_int {
        gctrk = gctr;
        kempty = empty;
    } else {
        gctrk = g1;
        g1 = g1.offset(lenk as isize);
    }
    if k_ctr == 1 as libc::c_int {
        gctri = gctrk;
        iempty = kempty;
    } else {
        gctri = g1;
        g1 = g1.offset(leni as isize);
    }
    if i_ctr == 1 as libc::c_int {
        gout = gctri;
        gempty = iempty;
    } else {
        gout = g1;
        g1 = g1.offset(leng as isize);
    }
    kp = 0 as libc::c_int;
    while kp < k_prim {
        (*envs).ak[0 as libc::c_int as usize] = *ak.offset(kp as isize);
        if k_ctr == 1 as libc::c_int {
            fac1k = (*envs).common_factor * *ck.offset(kp as isize);
        } else {
            fac1k = (*envs).common_factor;
            *iempty = 1 as libc::c_int;
        }
        ip = 0 as libc::c_int;
        while ip < i_prim {
            (*envs).ai[0 as libc::c_int as usize] = *ai.offset(ip as isize);
            if i_ctr == 1 as libc::c_int {
                fac1i = fac1k * *ci.offset(ip as isize);
            } else {
                fac1i = fac1k;
            }
            (*envs).fac[0 as libc::c_int as usize] = fac1i;
            if ::core::mem::transmute::<
                _,
                fn(_, _, _, _, _) -> libc::c_int,
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
                if i_ctr > 1 as libc::c_int {
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
                *iempty = 0 as libc::c_int;
            }
            ip += 1;
            ip;
        }
        if *iempty == 0 {
            if k_ctr > 1 as libc::c_int {
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
            *kempty = 0 as libc::c_int;
        }
        kp += 1;
        kp;
    }
    if n_comp > 1 as libc::c_int && *kempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gctrk,
                nf.wrapping_mul(nc as libc::c_ulong) as libc::c_int,
                n_comp,
            );
        } else {
            CINTdplus_transpose(
                gctr,
                gctrk,
                nf.wrapping_mul(nc as libc::c_ulong) as libc::c_int,
                n_comp,
            );
        }
        *empty = 0 as libc::c_int;
    }
    return (*empty == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINT2c2e_drv(
    mut out: *mut f64,
    mut dims: *mut libc::c_int,
    mut envs: *mut CINTEnvVars,
    mut opt: *mut CINTOpt,
    mut cache: *mut f64,
    mut f_c2s: Option::<unsafe extern "C" fn() -> ()>,
) -> libc::c_int {
    let mut x_ctr: *mut libc::c_int = ((*envs).x_ctr).as_mut_ptr();
    let mut nc: libc::c_int = (*envs).nf * *x_ctr.offset(0 as libc::c_int as isize)
        * *x_ctr.offset(1 as libc::c_int as isize);
    let mut n_comp: libc::c_int = (*envs).ncomp_e1 * (*envs).ncomp_e2
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
    gctr = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut f64;
    cache = gctr.offset((nc * n_comp) as isize);
    let mut n: libc::c_int = 0;
    let mut empty: libc::c_int = 1 as libc::c_int;
    if !opt.is_null() {
        (*envs).opt = opt;
        CINT2c2e_loop(gctr, envs, cache, &mut empty);
    } else {
        CINT2c2e_loop_nopt(gctr, envs, cache, &mut empty);
    }
    let mut counts: [libc::c_int; 4] = [0; 4];
    if f_c2s
        == ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut libc::c_int,
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
                        *mut libc::c_int,
                        *mut CINTEnvVars,
                        *mut f64,
                    ) -> (),
            ),
        )
    {
        counts[0 as libc::c_int
            as usize] = ((*envs).i_l * 2 as libc::c_int + 1 as libc::c_int)
            * *x_ctr.offset(0 as libc::c_int as isize);
        counts[1 as libc::c_int
            as usize] = ((*envs).k_l * 2 as libc::c_int + 1 as libc::c_int)
            * *x_ctr.offset(1 as libc::c_int as isize);
    } else {
        counts[0 as libc::c_int
            as usize] = (*envs).nfi * *x_ctr.offset(0 as libc::c_int as isize);
        counts[1 as libc::c_int
            as usize] = (*envs).c2rust_unnamed.nfk
            * *x_ctr.offset(1 as libc::c_int as isize);
    }
    counts[2 as libc::c_int as usize] = 1 as libc::c_int;
    counts[3 as libc::c_int as usize] = 1 as libc::c_int;
    if dims.is_null() {
        dims = counts.as_mut_ptr();
    }
    let mut nout: libc::c_int = *dims.offset(0 as libc::c_int as isize)
        * *dims.offset(1 as libc::c_int as isize);
    if empty == 0 {
        n = 0 as libc::c_int;
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
        n = 0 as libc::c_int;
        while n < n_comp {
            c2s_dset0(out.offset((nout * n) as isize), dims, counts.as_mut_ptr());
            n += 1;
            n;
        }
    }
    if !stack.is_null() {
        free(stack as *mut libc::c_void);
    }
    return (empty == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn int2c2e_sph(
    mut out: *mut f64,
    mut dims: *mut libc::c_int,
    mut shls: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut f64,
    mut opt: *mut CINTOpt,
    mut cache: *mut f64,
) -> libc::c_int {
    let mut ng: [libc::c_int; 8] = [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
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
                *mut libc::c_int,
                *mut CINTEnvVars,
                libc::c_int,
            ) -> (),
        >,
        Option::<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout2e
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
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
                    *mut libc::c_int,
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
                        *mut libc::c_int,
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
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut f64,
) {
    let mut ng: [libc::c_int; 8] = [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    CINTall_2c2e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int2c2e_cart(
    mut out: *mut f64,
    mut dims: *mut libc::c_int,
    mut shls: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut f64,
    mut opt: *mut CINTOpt,
    mut cache: *mut f64,
) -> libc::c_int {
    let mut ng: [libc::c_int; 8] = [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
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
                *mut libc::c_int,
                *mut CINTEnvVars,
                libc::c_int,
            ) -> (),
        >,
        Option::<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout2e
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
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
                    *mut libc::c_int,
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
                        *mut libc::c_int,
                        *mut CINTEnvVars,
                        *mut f64,
                    ) -> (),
            ),
        ),
    );
}
