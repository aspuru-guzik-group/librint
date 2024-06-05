#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::optimizer::CINTOpt_log_max_pgto_coeff;
use crate::optimizer::CINTOpt_non0coeff_byshell;
use crate::optimizer::CINTset_pairdata;
use crate::g1e::CINTg1e_index_xyz;
use crate::g1e::CINTprim_to_ctr_0;
use crate::g1e::CINTprim_to_ctr_1;
use crate::g1e_grids::CINTinit_int1e_grids_EnvVars;
use crate::g1e_grids::CINTg0_1e_grids;
use crate::g1e_grids::CINTg0_1e_grids;
use crate::g1e_grids::CINTgout1e_grids;
use crate::cart2sph::c2s_sph_1e_grids;
use crate::cart2sph::c2s_cart_1e_grids;
use crate::cart2sph::c2s_grids_dset0;

use crate::cint::PairData;
use crate::cint::CINTOPt;
use crate::cint::CINTEnvVars;

pub type size_t = libc::c_ulong;
pub type uintptr_t = libc::c_ulong;

extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn CINT1e_grids_loop(
    mut gctr: *mut f64,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut f64,
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
    let mut nf: i32 = (*envs).nf;
    let mut n_comp: i32 = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut ngrids: i32 = (*envs).c2rust_unnamed_0.ngrids;
    let mut grids: *mut f64 = (*envs).c2rust_unnamed_1.grids;
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
    let mut cutoff: f64 = 0.;
    let mut rij: *mut f64 = 0 as *mut f64;
    let mut ip: i32 = 0;
    let mut jp: i32 = 0;
    let mut i: i32 = 0;
    let mut grids_offset: i32 = 0;
    let mut bgrids: i32 = 0;
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
    let mut all_empty: i32 = 1 as i32;
    let mut idx: *mut i32 = 0 as *mut i32;
    idx = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut i32;
    cache = idx.offset((nf * 3 as i32) as isize) as *mut f64;
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
    let lenj: i32 = 104 as i32 * nf * nc * n_comp;
    let leni: i32 = 104 as i32 * nf * i_ctr * n_comp;
    let len0: i32 = 104 as i32 * nf * n_comp;
    let len: i32 = leng + lenj + leni + len0;
    let mut gridsT: *mut f64 = 0 as *mut f64;
    gridsT = ((cache as uintptr_t).wrapping_add(63 as i32 as libc::c_ulong)
        & (64 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut f64;
    cache = gridsT.offset((len + 104 as i32 * 3 as i32) as isize);
    let mut g: *mut f64 = gridsT
        .offset((104 as i32 * 3 as i32) as isize);
    let mut g1: *mut f64 = g.offset(leng as isize);
    let mut gout: *mut f64 = 0 as *mut f64;
    let mut gctri: *mut f64 = 0 as *mut f64;
    let mut gctrj: *mut f64 = 0 as *mut f64;
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
    grids_offset = 0 as i32;
    while grids_offset < ngrids {
        (*envs).c2rust_unnamed.grids_offset = grids_offset;
        bgrids = if ngrids - grids_offset < 104 as i32 {
            ngrids - grids_offset
        } else {
            104 as i32
        };
        i = 0 as i32;
        while i < bgrids {
            *gridsT
                .offset(
                    (i + 104 as i32 * 0 as i32) as isize,
                ) = *grids
                .offset(
                    ((grids_offset + i) * 3 as i32 + 0 as i32) as isize,
                );
            *gridsT
                .offset(
                    (i + 104 as i32 * 1 as i32) as isize,
                ) = *grids
                .offset(
                    ((grids_offset + i) * 3 as i32 + 1 as i32) as isize,
                );
            *gridsT
                .offset(
                    (i + 104 as i32 * 2 as i32) as isize,
                ) = *grids
                .offset(
                    ((grids_offset + i) * 3 as i32 + 2 as i32) as isize,
                );
            i += 1;
            i;
        }
        empty[0 as i32 as usize] = 1 as i32;
        empty[1 as i32 as usize] = 1 as i32;
        empty[2 as i32 as usize] = 1 as i32;
        if n_comp == 1 as i32 {
            gctrj = gctr.offset((grids_offset * nf * nc) as isize);
            if j_ctr == 1 as i32 {
                gctri = gctrj;
            }
            if i_ctr == 1 as i32 {
                gout = gctri;
            }
        }
        pdata_ij = pdata_base;
        jp = 0 as i32;
        while jp < j_prim {
            (*envs).aj[0 as i32 as usize] = *aj.offset(jp as isize);
            if j_ctr == 1 as i32 {
                fac1j = (*envs).common_factor * *cj.offset(jp as isize);
            } else {
                fac1j = (*envs).common_factor;
                *iempty = 1 as i32;
            }
            ip = 0 as i32;
            while ip < i_prim {
                if !((*pdata_ij).cceij > expcutoff) {
                    (*envs).ai[0 as i32 as usize] = *ai.offset(ip as isize);
                    expij = (*pdata_ij).eij;
                    cutoff = expcutoff - (*pdata_ij).cceij;
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
                    CINTg0_1e_grids(g, cutoff, envs, cache, gridsT);
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
                                (bgrids * nf * n_comp) as size_t,
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
                                (bgrids * nf * n_comp) as size_t,
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
                pdata_ij = pdata_ij.offset(1);
                pdata_ij;
            }
            if *iempty == 0 {
                if j_ctr > 1 as i32 {
                    if *jempty != 0 {
                        CINTprim_to_ctr_0(
                            gctrj,
                            gctri,
                            cj.offset(jp as isize),
                            (bgrids * nf * i_ctr * n_comp) as size_t,
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
                            (bgrids * nf * i_ctr * n_comp) as size_t,
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
            jp;
        }
        if n_comp > 1 as i32 && *jempty == 0 {
            _transpose_comps(
                gctr.offset((grids_offset * nf * nc) as isize),
                gctrj,
                bgrids,
                nf * nc,
                ngrids,
                n_comp,
            );
        }
        all_empty &= *jempty;
        grids_offset += 104 as i32;
    }
    return (all_empty == 0) as i32;
}
unsafe extern "C" fn _transpose_comps(
    mut gctr: *mut f64,
    mut gctrj: *mut f64,
    mut bgrids: i32,
    mut dij: i32,
    mut ngrids: i32,
    mut n_comp: i32,
) {
    let mut n: i32 = 0;
    let mut ic: i32 = 0;
    let mut ig: i32 = 0;
    let mut pgctr: *mut f64 = 0 as *mut f64;
    let mut pgctrj: *mut f64 = 0 as *mut f64;
    ic = 0 as i32;
    while ic < n_comp {
        pgctr = gctr.offset((ic * dij * ngrids) as isize);
        n = 0 as i32;
        while n < dij {
            pgctrj = gctrj.offset(((n * n_comp + ic) * bgrids) as isize);
            ig = 0 as i32;
            while ig < bgrids {
                *pgctr.offset((ig + n * bgrids) as isize) = *pgctrj.offset(ig as isize);
                ig += 1;
                ig;
            }
            n += 1;
            n;
        }
        ic += 1;
        ic;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_grids_cache_size(mut envs: *mut CINTEnvVars) -> size_t {
    let mut bas: *mut i32 = (*envs).bas;
    let mut shls: *mut i32 = (*envs).shls;
    let mut x_ctr: *mut i32 = ((*envs).x_ctr).as_mut_ptr();
    let mut ngrids: i32 = (*envs).c2rust_unnamed_0.ngrids;
    let mut nroots: i32 = (*envs).nrys_roots;
    let mut nf: i32 = (*envs).nf;
    let mut nc: i32 = ngrids * nf * *x_ctr.offset(0 as i32 as isize)
        * *x_ctr.offset(1 as i32 as isize);
    let mut n_comp: i32 = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
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
    let mut pdata_size: i32 = i_prim * j_prim * 5 as i32
        + i_prim * *x_ctr.offset(0 as i32 as isize)
        + j_prim * *x_ctr.offset(1 as i32 as isize)
        + (i_prim + j_prim) * 2 as i32 + (*envs).nf * 3 as i32;
    let mut leng: size_t = ((*envs).g_size * 3 as i32
        * (((1 as i32) << (*envs).gbits) + 1 as i32)) as size_t;
    let mut len0: size_t = (104 as i32 * nf * n_comp) as size_t;
    let mut leni: size_t = len0
        .wrapping_mul(*x_ctr.offset(0 as i32 as isize) as libc::c_ulong);
    let mut lenj: size_t = leni
        .wrapping_mul(*x_ctr.offset(1 as i32 as isize) as libc::c_ulong);
    let mut cache_size: size_t = if ((nc * n_comp) as libc::c_ulong)
        .wrapping_add(leng)
        .wrapping_add(len0)
        .wrapping_add(leni)
        .wrapping_add(lenj)
        .wrapping_add(pdata_size as libc::c_ulong)
        .wrapping_add(
            (104 as i32
                * (if n_comp > nroots + 10 as i32 {
                    n_comp
                } else {
                    nroots + 10 as i32
                })) as libc::c_ulong,
        )
        > (nc * n_comp + 104 as i32 * nf * 8 as i32 * 2 as i32)
            as libc::c_ulong
    {
        ((nc * n_comp) as libc::c_ulong)
            .wrapping_add(leng)
            .wrapping_add(len0)
            .wrapping_add(leni)
            .wrapping_add(lenj)
            .wrapping_add(pdata_size as libc::c_ulong)
            .wrapping_add(
                (104 as i32
                    * (if n_comp > nroots + 10 as i32 {
                        n_comp
                    } else {
                        nroots + 10 as i32
                    })) as libc::c_ulong,
            )
    } else {
        (nc * n_comp + 104 as i32 * nf * 8 as i32 * 2 as i32)
            as libc::c_ulong
    };
    return cache_size.wrapping_add(32 as i32 as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn CINT1e_grids_drv(
    mut out: *mut f64,
    mut dims: *mut i32,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut f64,
    mut f_c2s: Option::<unsafe extern "C" fn() -> ()>,
) -> i32 {
    if out.is_null() {
        return int1e_grids_cache_size(envs) as i32;
    }
    let mut x_ctr: *mut i32 = ((*envs).x_ctr).as_mut_ptr();
    let mut ngrids_nf: i32 = (*envs).c2rust_unnamed_0.ngrids * (*envs).nf;
    let mut nc: i32 = ngrids_nf * *x_ctr.offset(0 as i32 as isize)
        * *x_ctr.offset(1 as i32 as isize);
    let mut n_comp: i32 = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut stack: *mut f64 = 0 as *mut f64;
    if cache.is_null() {
        let mut cache_size: size_t = int1e_grids_cache_size(envs);
        stack = malloc(
            (::core::mem::size_of::<f64>() as libc::c_ulong)
                .wrapping_mul(cache_size),
        ) as *mut f64;
        cache = stack;
    }
    let mut gctr: *mut f64 = 0 as *mut f64;
    gctr = ((cache as uintptr_t).wrapping_add(63 as i32 as libc::c_ulong)
        & (64 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut f64;
    cache = gctr.offset((nc * n_comp) as isize);
    let mut has_value: i32 = CINT1e_grids_loop(gctr, envs, cache);
    let mut counts: [i32; 4] = [0; 4];
    if dims.is_null() {
        dims = counts.as_mut_ptr();
    }
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
                c2s_sph_1e_grids
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
        counts[2 as i32 as usize] = (*envs).c2rust_unnamed_0.ngrids;
        counts[3 as i32 as usize] = 1 as i32;
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
                c2s_cart_1e_grids
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
        counts[2 as i32 as usize] = (*envs).c2rust_unnamed_0.ngrids;
        counts[3 as i32 as usize] = 1 as i32;
    }
    let mut nout: i32 = *dims.offset(0 as i32 as isize)
        * *dims.offset(1 as i32 as isize)
        * *dims.offset(2 as i32 as isize);
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
            n;
        }
    } else {
        n = 0 as i32;
        while n < n_comp {
            c2s_grids_dset0(out.offset((nout * n) as isize), dims, counts.as_mut_ptr());
            n += 1;
            n;
        }
    }
    if !stack.is_null() {
        free(stack as *mut libc::c_void);
    }
    return has_value;
}
#[no_mangle]
pub unsafe extern "C" fn int1e_grids_sph(
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
    CINTinit_int1e_grids_EnvVars(
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
            CINTgout1e_grids
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT1e_grids_drv(
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
                c2s_sph_1e_grids
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
pub unsafe extern "C" fn int1e_grids_optimizer(
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
pub unsafe extern "C" fn int1e_grids_cart(
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
    CINTinit_int1e_grids_EnvVars(
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
            CINTgout1e_grids
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    i32,
                ) -> (),
        ),
    );
    return CINT1e_grids_drv(
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
                c2s_cart_1e_grids
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
