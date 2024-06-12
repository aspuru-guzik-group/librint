#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

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

pub type size_t = libc::c_ulong;
pub type uintptr_t = libc::c_ulong;

extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}

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
    let mut i_sh: i32 = *shls.offset(0 as i32 as isize);
    let mut j_sh: i32 = *shls.offset(1 as i32 as isize);
    let mut i_ctr: i32 = (*envs).x_ctr[0 as i32 as usize];
    let mut j_ctr: i32 = (*envs).x_ctr[1 as i32 as usize];
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
    let mut n_comp: i32 = (*envs).ncomp_e1 * (*envs).ncomp_tensor; // cannot handle (augmented) unknown intrinsic
    let mut expcutoff: f64 = (*envs).expcutoff;
    let mut log_maxci: *mut f64 = 0 as *mut f64;
    let mut log_maxcj: *mut f64 = 0 as *mut f64;
    let mut pdata_base: *mut PairData = 0 as *mut PairData;
    let mut pdata_ij: *mut PairData = 0 as *mut PairData;
    log_maxci = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut f64;
    cache = log_maxci.offset((i_prim + j_prim) as isize);
    pdata_base = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut PairData;
    cache = pdata_base.offset((i_prim * j_prim) as isize) as *mut f64;
    log_maxcj = log_maxci.offset(i_prim as isize);
    CINTOpt_log_max_pgto_coeff(log_maxci, ci, i_prim, i_ctr);
    CINTOpt_log_max_pgto_coeff(log_maxcj, cj, j_prim, j_ctr);
    if CINTset_pairdata(
        pdata_base,
        ai,
        aj,
        (*envs).ri,
        (*envs).rj,
        log_maxci,
        log_maxcj,
        (*envs).li_ceil,
        (*envs).lj_ceil,
        i_prim,
        j_prim,
        (*envs).rirj[0 as i32 as usize] * (*envs).rirj[0 as i32 as usize]
            + (*envs).rirj[1 as i32 as usize]
                * (*envs).rirj[1 as i32 as usize]
            + (*envs).rirj[2 as i32 as usize]
                * (*envs).rirj[2 as i32 as usize],
        expcutoff,
        env,
    ) != 0
    {
        return 0 as i32;
    }
    let mut fac1i: f64 = 0.;
    let mut fac1j: f64 = 0.;
    let mut expij: f64 = 0.;
    let mut ip: i32 = 0;
    let mut jp: i32 = 0;
    let mut empty: [i32; 4] = [
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    let mut gempty: *mut i32 = empty
        .as_mut_ptr()
        .offset(0 as i32 as isize);
    let mut iempty: *mut i32 = empty
        .as_mut_ptr()
        .offset(1 as i32 as isize);
    let mut jempty: *mut i32 = empty
        .as_mut_ptr()
        .offset(2 as i32 as isize);
    let mut rij: *mut f64 = 0 as *mut f64;
    let mut idx: *mut i32 = 0 as *mut i32;
    idx = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut i32;
    cache = idx.offset(((*envs).nf * 3 as i32) as isize) as *mut f64;
    CINTg1e_index_xyz(idx, envs);
    let mut non0ctri: *mut i32 = 0 as *mut i32;
    let mut non0ctrj: *mut i32 = 0 as *mut i32;
    let mut non0idxi: *mut i32 = 0 as *mut i32;
    let mut non0idxj: *mut i32 = 0 as *mut i32;
    non0ctri = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut i32;
    cache = non0ctri.offset((i_prim + j_prim + i_prim * i_ctr + j_prim * j_ctr) as isize)
        as *mut f64;
    non0ctrj = non0ctri.offset(i_prim as isize);
    non0idxi = non0ctrj.offset(j_prim as isize);
    non0idxj = non0idxi.offset((i_prim * i_ctr) as isize);
    CINTOpt_non0coeff_byshell(non0idxi, non0ctri, ci, i_prim, i_ctr);
    CINTOpt_non0coeff_byshell(non0idxj, non0ctrj, cj, j_prim, j_ctr);
    let nc: i32 = i_ctr * j_ctr;
    let leng: i32 = (*envs).g_size * 3 as i32
        * (((1 as i32) << (*envs).gbits) + 1 as i32);
    let lenj: i32 = (*envs).nf * nc * n_comp;
    let leni: i32 = (*envs).nf * i_ctr * n_comp;
    let len0: i32 = (*envs).nf * n_comp;
    let len: i32 = leng + lenj + leni + len0;
    let mut g: *mut f64 = 0 as *mut f64;
    let mut gout: *mut f64 = 0 as *mut f64;
    let mut gctri: *mut f64 = 0 as *mut f64;
    let mut gctrj: *mut f64 = 0 as *mut f64;
    g = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut f64;
    cache = g.offset(len as isize);
    let mut g1: *mut f64 = g.offset(leng as isize);
    if n_comp == 1 as i32 {
        gctrj = gctr;
    } else {
        gctrj = g1;
        g1 = g1.offset(lenj as isize);
    }
    if j_ctr == 1 as i32 {
        gctri = gctrj;
        iempty = jempty;
    } else {
        gctri = g1;
        g1 = g1.offset(leni as isize);
    }
    if i_ctr == 1 as i32 {
        gout = gctri;
        gempty = iempty;
    } else {
        gout = g1;
    }
    let mut common_factor: f64 = (*envs).common_factor
        * CINTcommon_fac_sp((*envs).i_l) * CINTcommon_fac_sp((*envs).j_l);
    pdata_ij = pdata_base;
    jp = 0 as i32;
    while jp < j_prim {
        (*envs).aj[0 as i32 as usize] = *aj.offset(jp as isize);
        if j_ctr == 1 as i32 {
            fac1j = common_factor * *cj.offset(jp as isize);
        } else {
            fac1j = common_factor;
            *iempty = 1 as i32;
        }
        ip = 0 as i32;
        while ip < i_prim {
            if !((*pdata_ij).cceij > expcutoff) {
                (*envs).ai[0 as i32 as usize] = *ai.offset(ip as isize);
                expij = (*pdata_ij).eij;
                rij = ((*pdata_ij).rij).as_mut_ptr();
                (*envs)
                    .rij[0 as i32
                    as usize] = *rij.offset(0 as i32 as isize);
                (*envs)
                    .rij[1 as i32
                    as usize] = *rij.offset(1 as i32 as isize);
                (*envs)
                    .rij[2 as i32
                    as usize] = *rij.offset(2 as i32 as isize);
                if i_ctr == 1 as i32 {
                    fac1i = fac1j * *ci.offset(ip as isize) * expij;
                } else {
                    fac1i = fac1j * expij;
                }
                (*envs).fac[0 as i32 as usize] = fac1i;
                make_g1e_gout(gout, g, idx, envs, *gempty, int1e_type);
                if i_ctr > 1 as i32 {
                    if *iempty != 0 {
                        CINTprim_to_ctr_0(
                            gctri,
                            gout,
                            ci.offset(ip as isize),
                            ((*envs).nf * n_comp) as size_t,
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
                            ((*envs).nf * n_comp) as size_t,
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
            pdata_ij = pdata_ij.offset(1);
        }
        if *iempty == 0 {
            if j_ctr > 1 as i32 {
                if *jempty != 0 {
                    CINTprim_to_ctr_0(
                        gctrj,
                        gctri,
                        cj.offset(jp as isize),
                        ((*envs).nf * i_ctr * n_comp) as size_t,
                        j_prim,
                        j_ctr,
                        *non0ctrj.offset(jp as isize),
                        non0idxj.offset((jp * j_ctr) as isize),
                    );
                } else {
                    CINTprim_to_ctr_1(
                        gctrj,
                        gctri,
                        cj.offset(jp as isize),
                        ((*envs).nf * i_ctr * n_comp) as size_t,
                        j_prim,
                        j_ctr,
                        *non0ctrj.offset(jp as isize),
                        non0idxj.offset((jp * j_ctr) as isize),
                    );
                }
            }
            *jempty = 0 as i32;
        }
        jp += 1;
    }
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
            (8 as i32 * *shls.offset(0 as i32 as isize)
                + 2 as i32) as isize,
        );
    let mut j_prim: i32 = *bas
        .offset(
            (8 as i32 * *shls.offset(1 as i32 as isize)
                + 2 as i32) as isize,
        );
    let mut x_ctr: *mut i32 = ((*envs).x_ctr).as_mut_ptr();
    let mut nc: i32 = (*envs).nf * *x_ctr.offset(0 as i32 as isize)
        * *x_ctr.offset(1 as i32 as isize);
    let mut n_comp: i32 = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut leng: i32 = (*envs).g_size * 3 as i32
        * (((1 as i32) << (*envs).gbits) + 1 as i32);
    let mut lenj: i32 = (*envs).nf * nc * n_comp;
    let mut leni: i32 = (*envs).nf * *x_ctr.offset(0 as i32 as isize)
        * n_comp;
    let mut len0: i32 = (*envs).nf * n_comp;
    let mut pdata_size: i32 = i_prim * j_prim * 5 as i32
        + i_prim * *x_ctr.offset(0 as i32 as isize)
        + j_prim * *x_ctr.offset(1 as i32 as isize)
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
    let mut nc: i32 = (*envs).nf * *x_ctr.offset(0 as i32 as isize)
        * *x_ctr.offset(1 as i32 as isize);
    let mut n_comp: i32 = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut stack: *mut f64 = 0 as *mut f64;
    // if cache.is_null() {
    let mut cache_size: size_t = int1e_cache_size(envs) as size_t;
    stack = malloc(
        (::core::mem::size_of::<f64>() as libc::c_ulong)
            .wrapping_mul(cache_size),
    ) as *mut f64;
    cache = stack;
    // }
    let mut gctr: *mut f64 = 0 as *mut f64;
    gctr = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut f64;
    cache = gctr.offset((nc * n_comp) as isize);
    let mut has_value: i32 = CINT1e_loop(gctr, envs, cache, int1e_type);
    let mut counts: [i32; 4] = [0; 4];
    // if dims.is_null() {
    dims = counts.as_mut_ptr();
    // }
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
            as usize] = ((*envs).j_l * 2 as i32 + 1 as i32)
            * *x_ctr.offset(1 as i32 as isize);
    } else if f_c2s
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
                c2s_cart_1e
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
            as usize] = (*envs).nfi * *x_ctr.offset(0 as i32 as isize);
        counts[1 as i32
            as usize] = (*envs).nfj * *x_ctr.offset(1 as i32 as isize);
    }
    counts[2 as i32 as usize] = 1 as i32;
    counts[3 as i32 as usize] = 1 as i32;
    let mut nout: i32 = *dims.offset(0 as i32 as isize)
        * *dims.offset(1 as i32 as isize);
    let mut n: i32 = 0;
    if has_value != 0 {
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
        }
    } else {
        n = 0 as i32;
        while n < n_comp {
            c2s_dset0(out.offset((nout * n) as isize), dims, counts.as_mut_ptr());
            n += 1;
        }
    }
    if !stack.is_null() {
        free(stack as *mut libc::c_void);
    }
    return has_value;
}
unsafe extern "C" fn make_g1e_gout(
    mut gout: *mut f64,
    mut g: *mut f64,
    mut idx: *mut i32,
    mut envs: *mut CINTEnvVars,
    mut empty: i32,
    mut int1e_type: i32,
) {
    let mut ia: i32 = 0;
    match int1e_type {
        0 => {
            CINTg1e_ovlp(g, envs);
            ::core::mem::transmute::<
                _,
                fn(_, _, _, _, _),
            >(
                (Some(((*envs).f_gout).expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )(gout, g, idx, envs, empty);
        }
        1 => {
            CINTg1e_nuc(g, envs, -(1 as i32));
            ::core::mem::transmute::<
                _,
                fn(_, _, _, _, _),
            >(
                (Some(((*envs).f_gout).expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )(gout, g, idx, envs, empty);
        }
        2 => {
            ia = 0 as i32;
            while ia < (*envs).natm {
                CINTg1e_nuc(g, envs, ia);
                ::core::mem::transmute::<
                    _,
                    fn(_, _, _, _, _),
                >(
                    (Some(((*envs).f_gout).expect("non-null function pointer")))
                        .expect("non-null function pointer"),
                )(
                    gout,
                    g,
                    idx,
                    envs,
                    (empty != 0 && ia == 0 as i32) as i32,
                );
                ia += 1;
            }
        }
        _ => {}
    };
}
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
            s = 0 as i32 as f64;
            i = 0 as i32;
            while i < nrys_roots {
                s
                    += *gx.offset(i as isize) * *gy.offset(i as isize)
                        * *gz.offset(i as isize);
                i += 1;
            }
            *gout.offset(n as isize) = s;
            n += 1;
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
            s = 0 as i32 as f64;
            i = 0 as i32;
            while i < nrys_roots {
                s
                    += *gx.offset(i as isize) * *gy.offset(i as isize)
                        * *gz.offset(i as isize);
                i += 1;
            }
            *gout.offset(n as isize) += s;
            n += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn int1e_ovlp_sph(
    out: &mut [f64],
    dims: &mut [i32],
    shls: &mut [i32],
    atm: &mut [i32],
    natm: i32,
    bas: &mut [i32],
    nbas: i32,
    env: &mut [f64],
    cache: &mut [f64],
) -> i32 {
    let mut ng = [0, 0, 0, 0, 0, 1, 1, 1];
    let mut envs = CINTEnvVars::new();
    CINTinit_int1e_EnvVars(&mut envs as *mut CINTEnvVars, ng.as_mut_ptr(), shls.as_mut_ptr(), atm.as_mut_ptr(), natm, bas.as_mut_ptr(), nbas, env.as_mut_ptr());
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
pub unsafe fn int1e_ovlp_cart(
    out: &mut [f64],
    dims: &mut [i32],
    shls: &mut [i32],
    atm: &mut [i32],
    natm: i32,
    bas: &mut [i32],
    nbas: i32,
    env: &mut [f64],
    cache: &mut [f64],
) -> i32 {
    let mut ng = [0, 0, 0, 0, 0, 1, 1, 1];
    let mut envs = CINTEnvVars::new();
    CINTinit_int1e_EnvVars(&mut envs as *mut CINTEnvVars, ng.as_mut_ptr(), shls.as_mut_ptr(), atm.as_mut_ptr(), natm, bas.as_mut_ptr(), nbas, env.as_mut_ptr());
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
// this function does nothing
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
    out: &mut [f64],
    dims: &mut [i32],
    shls: &mut [i32],
    atm: &mut [i32],
    natm: i32,
    bas: &mut [i32],
    nbas: i32,
    env: &mut [f64],
    cache: &mut [f64],
) -> i32 {
    let mut ng = [0, 0, 0, 0, 0, 1, 0, 1];
    let mut envs = CINTEnvVars::new();
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
pub unsafe fn int1e_nuc_cart(
    out: &mut [f64],
    dims: &mut [i32],
    shls: &mut [i32],
    atm: &mut [i32],
    natm: i32,
    bas: &mut [i32],
    nbas: i32,
    env: &mut [f64],
    cache: &mut [f64],
) -> i32 {
    let mut ng = [0, 0, 0, 0, 0, 1, 0, 1];
    let mut envs = CINTEnvVars::new();
    CINTinit_int1e_EnvVars(&mut envs as *mut CINTEnvVars, ng.as_mut_ptr(), shls.as_mut_ptr(), atm.as_mut_ptr(), natm, bas.as_mut_ptr(), nbas, env.as_mut_ptr());
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
#[no_mangle]
pub fn cint1e_ovlp_cart(
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
        return int1e_ovlp_cart(
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
#[no_mangle]
pub fn cint1e_ovlp_sph(
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
        return int1e_ovlp_sph(
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
pub fn cint1e_nuc_cart(
    mut out: &mut [f64],
    mut shls: &mut [i32],
    mut atm: &mut [i32],
    mut natm: i32,
    mut bas: &mut [i32],
    mut nbas: i32,
    mut env: &mut [f64],
) -> i32 {
    let mut dims = [0;0];
    let mut cache = [0.0;0];
    unsafe {
        return int1e_nuc_cart(
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
#[no_mangle]
pub fn cint1e_nuc_sph(
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
        return int1e_nuc_sph(
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
    mut optptr_as_integer8: size_t,
) -> i32 {
    let mut opt: *mut *mut CINTOpt = optptr_as_integer8 as *mut *mut CINTOpt;
    // return int1e_ovlp_sph(
    //     out,
    //     0 as *mut i32,
    //     shls,
    //     atm,
    //     *natm,
    //     bas,
    //     *nbas,
    //     env,
    //     *opt,
    //     0 as *mut f64,
    // );
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_ovlp_sph_optimizer_(
    mut optptr_as_integer8: size_t,
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
    mut optptr_as_integer8: size_t,
) -> i32 {
    let mut opt: *mut *mut CINTOpt = optptr_as_integer8 as *mut *mut CINTOpt;
    unimplemented!("why do we have the fn_name_ versions of fn_name functions?");
    //return int1e_ovlp_cart(
    //    out,
    //    0 as *mut i32,
    //    shls,
    //    atm,
    //    *natm,
    //    bas,
    //    *nbas,
    //    env,
    //    *opt,
    //    0 as *mut f64,
    //);
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
    mut optptr_as_integer8: size_t,
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
    mut optptr_as_integer8: size_t,
) -> i32 {
    let mut opt: *mut *mut CINTOpt = optptr_as_integer8 as *mut *mut CINTOpt;
    // return int1e_nuc_sph(
    //     out,
    //     0 as *mut i32,
    //     shls,
    //     atm,
    //     *natm,
    //     bas,
    //     *nbas,
    //     env,
    //     *opt,
    //     0 as *mut f64,
    // );
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_nuc_sph_optimizer_(
    mut optptr_as_integer8: size_t,
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
    mut optptr_as_integer8: size_t,
) -> i32 {
    let mut opt: *mut *mut CINTOpt = optptr_as_integer8 as *mut *mut CINTOpt;
    todo!();
    // return int1e_nuc_cart(
    //     out,
    //     0 as *mut i32,
    //     shls,
    //     atm,
    //     *natm,
    //     bas,
    //     *nbas,
    //     env,
    //     *opt,
    //     0 as *mut f64,
    // );
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
    mut optptr_as_integer8: size_t,
    mut atm: *mut i32,
    mut natm: *mut i32,
    mut bas: *mut i32,
    mut nbas: *mut i32,
    mut env: *mut f64,
) {
    let mut opt: *mut *mut CINTOpt = optptr_as_integer8 as *mut *mut CINTOpt;
    int1e_nuc_optimizer(opt, atm, *natm, bas, *nbas, env);
}
