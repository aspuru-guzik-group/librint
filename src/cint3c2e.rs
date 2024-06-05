#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::optimizer::CINTOpt_log_max_pgto_coeff;
use crate::optimizer::CINTOpt_non0coeff_byshell;
use crate::optimizer::CINTset_pairdata;
use crate::optimizer::CINTall_3c2e_optimizer;
use crate::g1e::CINTprim_to_ctr_0;
use crate::g1e::CINTprim_to_ctr_1;
use crate::g3c2e::CINTinit_int3c2e_EnvVars;
use crate::g2e::CINTg2e_index_xyz;
use crate::cint2e::CINTgout2e;
use crate::fblas::CINTdmat_transpose;
use crate::fblas::CINTdplus_transpose;
use crate::cart2sph::c2s_sph_3c2e1;
use crate::cart2sph::c2s_cart_3c2e1;
use crate::cart2sph::c2s_sph_3c2e1_ssc;
use crate::cart2sph::c2s_dset0;

use crate::cint::PairData;
use crate::cint::CINTOpt;
use crate::cint::CINTEnvVars;

pub type size_t = libc::c_ulong;
pub type uintptr_t = libc::c_ulong;

extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
}

#[no_mangle]
pub unsafe extern "C" fn CINT3c2e_loop_nopt(
    mut gctr: *mut libc::c_double,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut libc::c_double,
    mut empty: *mut libc::c_int,
) -> libc::c_int {
    let mut shls: *mut libc::c_int = (*envs).shls;
    let mut bas: *mut libc::c_int = (*envs).bas;
    let mut env: *mut libc::c_double = (*envs).env;
    let mut i_sh: libc::c_int = *shls.offset(0 as libc::c_int as isize);
    let mut j_sh: libc::c_int = *shls.offset(1 as libc::c_int as isize);
    let mut k_sh: libc::c_int = *shls.offset(2 as libc::c_int as isize);
    let mut i_ctr: libc::c_int = (*envs).x_ctr[0 as libc::c_int as usize];
    let mut j_ctr: libc::c_int = (*envs).x_ctr[1 as libc::c_int as usize];
    let mut k_ctr: libc::c_int = (*envs).x_ctr[2 as libc::c_int as usize];
    let mut i_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * i_sh + 2 as libc::c_int) as isize);
    let mut j_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * j_sh + 2 as libc::c_int) as isize);
    let mut k_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * k_sh + 2 as libc::c_int) as isize);
    let mut ai: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * i_sh + 5 as libc::c_int) as isize) as isize,
        );
    let mut aj: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * j_sh + 5 as libc::c_int) as isize) as isize,
        );
    let mut ak: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * k_sh + 5 as libc::c_int) as isize) as isize,
        );
    let mut ci: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * i_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut cj: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * j_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut ck: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * k_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut expcutoff: libc::c_double = (*envs).expcutoff;
    let mut rr_ij: libc::c_double = (*envs).rirj[0 as libc::c_int as usize]
        * (*envs).rirj[0 as libc::c_int as usize]
        + (*envs).rirj[1 as libc::c_int as usize]
            * (*envs).rirj[1 as libc::c_int as usize]
        + (*envs).rirj[2 as libc::c_int as usize]
            * (*envs).rirj[2 as libc::c_int as usize];
    let mut log_maxci: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut log_maxcj: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut pdata_base: *mut PairData = 0 as *mut PairData;
    let mut pdata_ij: *mut PairData = 0 as *mut PairData;
    log_maxci = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_double;
    cache = log_maxci.offset((i_prim + j_prim) as isize);
    pdata_base = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut PairData;
    cache = pdata_base.offset((i_prim * j_prim) as isize) as *mut libc::c_double;
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
        rr_ij,
        expcutoff,
        env,
    ) != 0
    {
        return 0 as libc::c_int;
    }
    let mut n_comp: libc::c_int = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: libc::c_double = 0.;
    let mut fac1j: libc::c_double = 0.;
    let mut fac1k: libc::c_double = 0.;
    let mut ip: libc::c_int = 0;
    let mut jp: libc::c_int = 0;
    let mut kp: libc::c_int = 0;
    let mut _empty: [libc::c_int; 4] = [
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    let mut iempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize);
    let mut jempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize);
    let mut kempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize);
    let mut gempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(3 as libc::c_int as isize);
    let mut expij: libc::c_double = 0.;
    let mut cutoff: libc::c_double = 0.;
    let mut rij: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rkl: *mut libc::c_double = (*envs).rk;
    let mut omega: libc::c_double = *env.offset(8 as libc::c_int as isize);
    if omega < 0 as libc::c_int as libc::c_double && (*envs).rys_order > 1 as libc::c_int
    {
        let mut r_guess: libc::c_double = 8.0f64;
        let mut omega2: libc::c_double = omega * omega;
        let mut lij: libc::c_int = (*envs).li_ceil + (*envs).lj_ceil;
        if lij > 0 as libc::c_int {
            let mut dist_ij: libc::c_double = sqrt(rr_ij);
            let mut aij: libc::c_double = *ai
                .offset((i_prim - 1 as libc::c_int) as isize)
                + *aj.offset((j_prim - 1 as libc::c_int) as isize);
            let mut theta: libc::c_double = omega2 / (omega2 + aij);
            expcutoff
                += lij as libc::c_double
                    * log((dist_ij + theta * r_guess + 1.0f64) / (dist_ij + 1.0f64));
        }
        if (*envs).lk_ceil > 0 as libc::c_int {
            let mut theta_0: libc::c_double = omega2
                / (omega2 + *ak.offset((k_prim - 1 as libc::c_int) as isize));
            expcutoff
                += (*envs).lk_ceil as libc::c_double * log(theta_0 * r_guess + 1.0f64);
        }
    }
    let mut idx: *mut libc::c_int = 0 as *mut libc::c_int;
    idx = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_int;
    cache = idx.offset(nf.wrapping_mul(3 as libc::c_int as libc::c_ulong) as isize)
        as *mut libc::c_double;
    CINTg2e_index_xyz(idx, envs);
    let mut non0ctri: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut non0ctrj: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut non0ctrk: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut non0idxi: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut non0idxj: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut non0idxk: *mut libc::c_int = 0 as *mut libc::c_int;
    non0ctri = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_int;
    cache = non0ctri
        .offset(
            (i_prim + j_prim + k_prim + i_prim * i_ctr + j_prim * j_ctr + k_prim * k_ctr)
                as isize,
        ) as *mut libc::c_double;
    non0ctrj = non0ctri.offset(i_prim as isize);
    non0ctrk = non0ctrj.offset(j_prim as isize);
    non0idxi = non0ctrk.offset(k_prim as isize);
    non0idxj = non0idxi.offset((i_prim * i_ctr) as isize);
    non0idxk = non0idxj.offset((j_prim * j_ctr) as isize);
    CINTOpt_non0coeff_byshell(non0idxi, non0ctri, ci, i_prim, i_ctr);
    CINTOpt_non0coeff_byshell(non0idxj, non0ctrj, cj, j_prim, j_ctr);
    CINTOpt_non0coeff_byshell(non0idxk, non0ctrk, ck, k_prim, k_ctr);
    let mut nc: libc::c_int = i_ctr * j_ctr * k_ctr;
    let mut leng: size_t = ((*envs).g_size * 3 as libc::c_int
        * (((1 as libc::c_int) << (*envs).gbits) + 1 as libc::c_int)) as size_t;
    let mut lenk: size_t = nf
        .wrapping_mul(nc as libc::c_ulong)
        .wrapping_mul(n_comp as libc::c_ulong);
    let mut lenj: size_t = nf
        .wrapping_mul(i_ctr as libc::c_ulong)
        .wrapping_mul(j_ctr as libc::c_ulong)
        .wrapping_mul(n_comp as libc::c_ulong);
    let mut leni: size_t = nf
        .wrapping_mul(i_ctr as libc::c_ulong)
        .wrapping_mul(n_comp as libc::c_ulong);
    let mut len0: size_t = nf.wrapping_mul(n_comp as libc::c_ulong);
    let mut len: size_t = leng
        .wrapping_add(lenk)
        .wrapping_add(lenj)
        .wrapping_add(leni)
        .wrapping_add(len0);
    let mut g: *mut libc::c_double = 0 as *mut libc::c_double;
    g = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_double;
    cache = g.offset(len as isize);
    let mut g1: *mut libc::c_double = g.offset(leng as isize);
    let mut gout: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut gctri: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut gctrj: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut gctrk: *mut libc::c_double = 0 as *mut libc::c_double;
    if n_comp == 1 as libc::c_int {
        gctrk = gctr;
        kempty = empty;
    } else {
        gctrk = g1;
        g1 = g1.offset(lenk as isize);
    }
    if k_ctr == 1 as libc::c_int {
        gctrj = gctrk;
        jempty = kempty;
    } else {
        gctrj = g1;
        g1 = g1.offset(lenj as isize);
    }
    if j_ctr == 1 as libc::c_int {
        gctri = gctrj;
        iempty = jempty;
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
            *jempty = 1 as libc::c_int;
        }
        pdata_ij = pdata_base;
        jp = 0 as libc::c_int;
        while jp < j_prim {
            (*envs).aj[0 as libc::c_int as usize] = *aj.offset(jp as isize);
            if j_ctr == 1 as libc::c_int {
                fac1j = fac1k * *cj.offset(jp as isize);
            } else {
                fac1j = fac1k;
                *iempty = 1 as libc::c_int;
            }
            ip = 0 as libc::c_int;
            while ip < i_prim {
                if !((*pdata_ij).cceij > expcutoff) {
                    (*envs).ai[0 as libc::c_int as usize] = *ai.offset(ip as isize);
                    expij = (*pdata_ij).eij;
                    rij = ((*pdata_ij).rij).as_mut_ptr();
                    cutoff = expcutoff - (*pdata_ij).cceij;
                    if i_ctr == 1 as libc::c_int {
                        fac1i = fac1j * *ci.offset(ip as isize) * expij;
                    } else {
                        fac1i = fac1j * expij;
                    }
                    (*envs).fac[0 as libc::c_int as usize] = fac1i;
                    if ::core::mem::transmute::<
                        _,
                        fn(_, _, _, _, _) -> libc::c_int,
                    >(
                        (Some(((*envs).f_g0_2e).expect("non-null function pointer")))
                            .expect("non-null function pointer"),
                    )(g, rij, rkl, cutoff, envs) != 0
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
                                    len0,
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
                                    len0,
                                    i_prim,
                                    i_ctr,
                                    *non0ctri.offset(ip as isize),
                                    non0idxi.offset((ip * i_ctr) as isize),
                                );
                            }
                        }
                        *iempty = 0 as libc::c_int;
                    }
                }
                ip += 1;
                ip;
                pdata_ij = pdata_ij.offset(1);
                pdata_ij;
            }
            if *iempty == 0 {
                if j_ctr > 1 as libc::c_int {
                    if *jempty != 0 {
                        CINTprim_to_ctr_0(
                            gctrj,
                            gctri,
                            cj.offset(jp as isize),
                            leni,
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
                            leni,
                            j_prim,
                            j_ctr,
                            *non0ctrj.offset(jp as isize),
                            non0idxj.offset((jp * j_ctr) as isize),
                        );
                    }
                }
                *jempty = 0 as libc::c_int;
            }
            jp += 1;
            jp;
        }
        if *jempty == 0 {
            if k_ctr > 1 as libc::c_int {
                if *kempty != 0 {
                    CINTprim_to_ctr_0(
                        gctrk,
                        gctrj,
                        ck.offset(kp as isize),
                        lenj,
                        k_prim,
                        k_ctr,
                        *non0ctrk.offset(kp as isize),
                        non0idxk.offset((kp * k_ctr) as isize),
                    );
                } else {
                    CINTprim_to_ctr_1(
                        gctrk,
                        gctrj,
                        ck.offset(kp as isize),
                        lenj,
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
pub unsafe extern "C" fn CINT3c2e_111_loop(
    mut gctr: *mut libc::c_double,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut libc::c_double,
    mut empty: *mut libc::c_int,
) -> libc::c_int {
    let mut shls: *mut libc::c_int = (*envs).shls;
    let mut bas: *mut libc::c_int = (*envs).bas;
    let mut env: *mut libc::c_double = (*envs).env;
    let mut i_sh: libc::c_int = *shls.offset(0 as libc::c_int as isize);
    let mut j_sh: libc::c_int = *shls.offset(1 as libc::c_int as isize);
    let mut opt: *mut CINTOpt = (*envs).opt;
    if !((*opt).pairdata).is_null()
        && *((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize)
            == 0xffffffffffffffff as libc::c_ulong as *mut libc::c_void as *mut PairData
    {
        return 0 as libc::c_int;
    }
    let mut k_sh: libc::c_int = *shls.offset(2 as libc::c_int as isize);
    let mut i_ctr: libc::c_int = (*envs).x_ctr[0 as libc::c_int as usize];
    let mut j_ctr: libc::c_int = (*envs).x_ctr[1 as libc::c_int as usize];
    let mut k_ctr: libc::c_int = (*envs).x_ctr[2 as libc::c_int as usize];
    let mut i_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * i_sh + 2 as libc::c_int) as isize);
    let mut j_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * j_sh + 2 as libc::c_int) as isize);
    let mut k_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * k_sh + 2 as libc::c_int) as isize);
    let mut ai: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * i_sh + 5 as libc::c_int) as isize) as isize,
        );
    let mut aj: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * j_sh + 5 as libc::c_int) as isize) as isize,
        );
    let mut ak: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * k_sh + 5 as libc::c_int) as isize) as isize,
        );
    let mut ci: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * i_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut cj: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * j_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut ck: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * k_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut expcutoff: libc::c_double = (*envs).expcutoff;
    let mut rr_ij: libc::c_double = (*envs).rirj[0 as libc::c_int as usize]
        * (*envs).rirj[0 as libc::c_int as usize]
        + (*envs).rirj[1 as libc::c_int as usize]
            * (*envs).rirj[1 as libc::c_int as usize]
        + (*envs).rirj[2 as libc::c_int as usize]
            * (*envs).rirj[2 as libc::c_int as usize];
    let mut pdata_base: *mut PairData = 0 as *mut PairData;
    let mut pdata_ij: *mut PairData = 0 as *mut PairData;
    if !((*opt).pairdata).is_null() {
        pdata_base = *((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize);
    } else {
        let mut log_maxci: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(i_sh as isize);
        let mut log_maxcj: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(j_sh as isize);
        pdata_base = ((cache as uintptr_t)
            .wrapping_add(7 as libc::c_int as libc::c_ulong)
            & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut PairData;
        cache = pdata_base.offset((i_prim * j_prim) as isize) as *mut libc::c_double;
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
            rr_ij,
            expcutoff,
            env,
        ) != 0
        {
            return 0 as libc::c_int;
        }
    }
    let mut n_comp: libc::c_int = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: libc::c_double = 0.;
    let mut fac1j: libc::c_double = 0.;
    let mut fac1k: libc::c_double = 0.;
    let mut ip: libc::c_int = 0;
    let mut jp: libc::c_int = 0;
    let mut kp: libc::c_int = 0;
    let mut _empty: [libc::c_int; 4] = [
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    let mut iempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize);
    let mut jempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize);
    let mut kempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize);
    let mut gempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(3 as libc::c_int as isize);
    let mut non0ctri: *mut libc::c_int = *((*opt).non0ctr).offset(i_sh as isize);
    let mut non0ctrj: *mut libc::c_int = *((*opt).non0ctr).offset(j_sh as isize);
    let mut non0idxi: *mut libc::c_int = *((*opt).sortedidx).offset(i_sh as isize);
    let mut non0idxj: *mut libc::c_int = *((*opt).sortedidx).offset(j_sh as isize);
    let mut non0ctrk: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut non0idxk: *mut libc::c_int = 0 as *mut libc::c_int;
    non0ctrk = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_int;
    cache = non0ctrk.offset((k_prim + k_prim * k_ctr) as isize) as *mut libc::c_double;
    non0idxk = non0ctrk.offset(k_prim as isize);
    CINTOpt_non0coeff_byshell(non0idxk, non0ctrk, ck, k_prim, k_ctr);
    let mut expij: libc::c_double = 0.;
    let mut cutoff: libc::c_double = 0.;
    let mut rij: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rkl: *mut libc::c_double = ((*envs).rkl).as_mut_ptr();
    let mut idx: *mut libc::c_int = *((*opt).index_xyz_array)
        .offset(
            ((*envs).i_l * 16 as libc::c_int * 16 as libc::c_int
                + (*envs).j_l * 16 as libc::c_int + (*envs).k_l) as isize,
        );
    if idx.is_null() {
        idx = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
            & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut libc::c_int;
        cache = idx.offset(nf.wrapping_mul(3 as libc::c_int as libc::c_ulong) as isize)
            as *mut libc::c_double;
        CINTg2e_index_xyz(idx, envs);
    }
    let mut omega: libc::c_double = *env.offset(8 as libc::c_int as isize);
    if omega < 0 as libc::c_int as libc::c_double && (*envs).rys_order > 1 as libc::c_int
    {
        let mut r_guess: libc::c_double = 8.0f64;
        let mut omega2: libc::c_double = omega * omega;
        let mut lij: libc::c_int = (*envs).li_ceil + (*envs).lj_ceil;
        if lij > 0 as libc::c_int {
            let mut dist_ij: libc::c_double = sqrt(rr_ij);
            let mut aij: libc::c_double = *ai
                .offset((i_prim - 1 as libc::c_int) as isize)
                + *aj.offset((j_prim - 1 as libc::c_int) as isize);
            let mut theta: libc::c_double = omega2 / (omega2 + aij);
            expcutoff
                += lij as libc::c_double
                    * log((dist_ij + theta * r_guess + 1.0f64) / (dist_ij + 1.0f64));
        }
        if (*envs).lk_ceil > 0 as libc::c_int {
            let mut theta_0: libc::c_double = omega2
                / (omega2 + *ak.offset((k_prim - 1 as libc::c_int) as isize));
            expcutoff
                += (*envs).lk_ceil as libc::c_double * log(theta_0 * r_guess + 1.0f64);
        }
    }
    let mut nc: libc::c_int = 1 as libc::c_int;
    let mut leng: size_t = ((*envs).g_size * 3 as libc::c_int
        * (((1 as libc::c_int) << (*envs).gbits) + 1 as libc::c_int)) as size_t;
    let mut len0: size_t = ((*envs).nf * n_comp) as size_t;
    let mut len: size_t = leng.wrapping_add(len0);
    let mut g: *mut libc::c_double = 0 as *mut libc::c_double;
    g = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_double;
    cache = g.offset(len as isize);
    let mut gout: *mut libc::c_double = 0 as *mut libc::c_double;
    if n_comp == 1 as libc::c_int {
        gout = gctr;
        gempty = empty;
    } else {
        gout = g.offset(leng as isize);
    }
    kp = 0 as libc::c_int;
    while kp < k_prim {
        (*envs).ak[0 as libc::c_int as usize] = *ak.offset(kp as isize);
        fac1k = (*envs).common_factor * *ck.offset(kp as isize);
        pdata_ij = pdata_base;
        jp = 0 as libc::c_int;
        while jp < j_prim {
            (*envs).aj[0 as libc::c_int as usize] = *aj.offset(jp as isize);
            fac1j = fac1k * *cj.offset(jp as isize);
            ip = 0 as libc::c_int;
            while ip < i_prim {
                if !((*pdata_ij).cceij > expcutoff) {
                    (*envs).ai[0 as libc::c_int as usize] = *ai.offset(ip as isize);
                    expij = (*pdata_ij).eij;
                    rij = ((*pdata_ij).rij).as_mut_ptr();
                    cutoff = expcutoff - (*pdata_ij).cceij;
                    fac1i = fac1j * *ci.offset(ip as isize) * expij;
                    (*envs).fac[0 as libc::c_int as usize] = fac1i;
                    if ::core::mem::transmute::<
                        _,
                        fn(_, _, _, _, _) -> libc::c_int,
                    >(
                        (Some(((*envs).f_g0_2e).expect("non-null function pointer")))
                            .expect("non-null function pointer"),
                    )(g, rij, rkl, cutoff, envs) != 0
                    {
                        ::core::mem::transmute::<
                            _,
                            fn(_, _, _, _, _),
                        >(
                            (Some(((*envs).f_gout).expect("non-null function pointer")))
                                .expect("non-null function pointer"),
                        )(gout, g, idx, envs, *gempty);
                        *gempty = 0 as libc::c_int;
                    }
                }
                ip += 1;
                ip;
                pdata_ij = pdata_ij.offset(1);
                pdata_ij;
            }
            jp += 1;
            jp;
        }
        kp += 1;
        kp;
    }
    if n_comp > 1 as libc::c_int && *gempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gout,
                nf.wrapping_mul(nc as libc::c_ulong) as libc::c_int,
                n_comp,
            );
        } else {
            CINTdplus_transpose(
                gctr,
                gout,
                nf.wrapping_mul(nc as libc::c_ulong) as libc::c_int,
                n_comp,
            );
        }
        *empty = 0 as libc::c_int;
    }
    return (*empty == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINT3c2e_n11_loop(
    mut gctr: *mut libc::c_double,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut libc::c_double,
    mut empty: *mut libc::c_int,
) -> libc::c_int {
    let mut shls: *mut libc::c_int = (*envs).shls;
    let mut bas: *mut libc::c_int = (*envs).bas;
    let mut env: *mut libc::c_double = (*envs).env;
    let mut i_sh: libc::c_int = *shls.offset(0 as libc::c_int as isize);
    let mut j_sh: libc::c_int = *shls.offset(1 as libc::c_int as isize);
    let mut opt: *mut CINTOpt = (*envs).opt;
    if !((*opt).pairdata).is_null()
        && *((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize)
            == 0xffffffffffffffff as libc::c_ulong as *mut libc::c_void as *mut PairData
    {
        return 0 as libc::c_int;
    }
    let mut k_sh: libc::c_int = *shls.offset(2 as libc::c_int as isize);
    let mut i_ctr: libc::c_int = (*envs).x_ctr[0 as libc::c_int as usize];
    let mut j_ctr: libc::c_int = (*envs).x_ctr[1 as libc::c_int as usize];
    let mut k_ctr: libc::c_int = (*envs).x_ctr[2 as libc::c_int as usize];
    let mut i_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * i_sh + 2 as libc::c_int) as isize);
    let mut j_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * j_sh + 2 as libc::c_int) as isize);
    let mut k_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * k_sh + 2 as libc::c_int) as isize);
    let mut ai: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * i_sh + 5 as libc::c_int) as isize) as isize,
        );
    let mut aj: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * j_sh + 5 as libc::c_int) as isize) as isize,
        );
    let mut ak: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * k_sh + 5 as libc::c_int) as isize) as isize,
        );
    let mut ci: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * i_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut cj: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * j_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut ck: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * k_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut expcutoff: libc::c_double = (*envs).expcutoff;
    let mut rr_ij: libc::c_double = (*envs).rirj[0 as libc::c_int as usize]
        * (*envs).rirj[0 as libc::c_int as usize]
        + (*envs).rirj[1 as libc::c_int as usize]
            * (*envs).rirj[1 as libc::c_int as usize]
        + (*envs).rirj[2 as libc::c_int as usize]
            * (*envs).rirj[2 as libc::c_int as usize];
    let mut pdata_base: *mut PairData = 0 as *mut PairData;
    let mut pdata_ij: *mut PairData = 0 as *mut PairData;
    if !((*opt).pairdata).is_null() {
        pdata_base = *((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize);
    } else {
        let mut log_maxci: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(i_sh as isize);
        let mut log_maxcj: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(j_sh as isize);
        pdata_base = ((cache as uintptr_t)
            .wrapping_add(7 as libc::c_int as libc::c_ulong)
            & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut PairData;
        cache = pdata_base.offset((i_prim * j_prim) as isize) as *mut libc::c_double;
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
            rr_ij,
            expcutoff,
            env,
        ) != 0
        {
            return 0 as libc::c_int;
        }
    }
    let mut n_comp: libc::c_int = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: libc::c_double = 0.;
    let mut fac1j: libc::c_double = 0.;
    let mut fac1k: libc::c_double = 0.;
    let mut ip: libc::c_int = 0;
    let mut jp: libc::c_int = 0;
    let mut kp: libc::c_int = 0;
    let mut _empty: [libc::c_int; 4] = [
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    let mut iempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize);
    let mut jempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize);
    let mut kempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize);
    let mut gempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(3 as libc::c_int as isize);
    let mut non0ctri: *mut libc::c_int = *((*opt).non0ctr).offset(i_sh as isize);
    let mut non0ctrj: *mut libc::c_int = *((*opt).non0ctr).offset(j_sh as isize);
    let mut non0idxi: *mut libc::c_int = *((*opt).sortedidx).offset(i_sh as isize);
    let mut non0idxj: *mut libc::c_int = *((*opt).sortedidx).offset(j_sh as isize);
    let mut non0ctrk: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut non0idxk: *mut libc::c_int = 0 as *mut libc::c_int;
    non0ctrk = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_int;
    cache = non0ctrk.offset((k_prim + k_prim * k_ctr) as isize) as *mut libc::c_double;
    non0idxk = non0ctrk.offset(k_prim as isize);
    CINTOpt_non0coeff_byshell(non0idxk, non0ctrk, ck, k_prim, k_ctr);
    let mut expij: libc::c_double = 0.;
    let mut cutoff: libc::c_double = 0.;
    let mut rij: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rkl: *mut libc::c_double = ((*envs).rkl).as_mut_ptr();
    let mut idx: *mut libc::c_int = *((*opt).index_xyz_array)
        .offset(
            ((*envs).i_l * 16 as libc::c_int * 16 as libc::c_int
                + (*envs).j_l * 16 as libc::c_int + (*envs).k_l) as isize,
        );
    if idx.is_null() {
        idx = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
            & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut libc::c_int;
        cache = idx.offset(nf.wrapping_mul(3 as libc::c_int as libc::c_ulong) as isize)
            as *mut libc::c_double;
        CINTg2e_index_xyz(idx, envs);
    }
    let mut omega: libc::c_double = *env.offset(8 as libc::c_int as isize);
    if omega < 0 as libc::c_int as libc::c_double && (*envs).rys_order > 1 as libc::c_int
    {
        let mut r_guess: libc::c_double = 8.0f64;
        let mut omega2: libc::c_double = omega * omega;
        let mut lij: libc::c_int = (*envs).li_ceil + (*envs).lj_ceil;
        if lij > 0 as libc::c_int {
            let mut dist_ij: libc::c_double = sqrt(rr_ij);
            let mut aij: libc::c_double = *ai
                .offset((i_prim - 1 as libc::c_int) as isize)
                + *aj.offset((j_prim - 1 as libc::c_int) as isize);
            let mut theta: libc::c_double = omega2 / (omega2 + aij);
            expcutoff
                += lij as libc::c_double
                    * log((dist_ij + theta * r_guess + 1.0f64) / (dist_ij + 1.0f64));
        }
        if (*envs).lk_ceil > 0 as libc::c_int {
            let mut theta_0: libc::c_double = omega2
                / (omega2 + *ak.offset((k_prim - 1 as libc::c_int) as isize));
            expcutoff
                += (*envs).lk_ceil as libc::c_double * log(theta_0 * r_guess + 1.0f64);
        }
    }
    let mut nc: libc::c_int = i_ctr;
    let mut leng: size_t = ((*envs).g_size * 3 as libc::c_int
        * (((1 as libc::c_int) << (*envs).gbits) + 1 as libc::c_int)) as size_t;
    let mut leni: size_t = nf
        .wrapping_mul(i_ctr as libc::c_ulong)
        .wrapping_mul(n_comp as libc::c_ulong);
    let mut len0: size_t = nf.wrapping_mul(n_comp as libc::c_ulong);
    let mut len: size_t = leng.wrapping_add(leni).wrapping_add(len0);
    let mut g: *mut libc::c_double = 0 as *mut libc::c_double;
    g = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_double;
    cache = g.offset(len as isize);
    let mut g1: *mut libc::c_double = g.offset(leng as isize);
    let mut gout: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut gctri: *mut libc::c_double = 0 as *mut libc::c_double;
    if n_comp == 1 as libc::c_int {
        gctri = gctr;
        iempty = empty;
    } else {
        gctri = g1;
        g1 = g1.offset(leni as isize);
    }
    gout = g1;
    kp = 0 as libc::c_int;
    while kp < k_prim {
        (*envs).ak[0 as libc::c_int as usize] = *ak.offset(kp as isize);
        fac1k = (*envs).common_factor * *ck.offset(kp as isize);
        pdata_ij = pdata_base;
        jp = 0 as libc::c_int;
        while jp < j_prim {
            (*envs).aj[0 as libc::c_int as usize] = *aj.offset(jp as isize);
            fac1j = fac1k * *cj.offset(jp as isize);
            ip = 0 as libc::c_int;
            while ip < i_prim {
                if !((*pdata_ij).cceij > expcutoff) {
                    (*envs).ai[0 as libc::c_int as usize] = *ai.offset(ip as isize);
                    expij = (*pdata_ij).eij;
                    rij = ((*pdata_ij).rij).as_mut_ptr();
                    cutoff = expcutoff - (*pdata_ij).cceij;
                    fac1i = fac1j * expij;
                    (*envs).fac[0 as libc::c_int as usize] = fac1i;
                    if ::core::mem::transmute::<
                        _,
                        fn(_, _, _, _, _) -> libc::c_int,
                    >(
                        (Some(((*envs).f_g0_2e).expect("non-null function pointer")))
                            .expect("non-null function pointer"),
                    )(g, rij, rkl, cutoff, envs) != 0
                    {
                        ::core::mem::transmute::<
                            _,
                            fn(_, _, _, _, _),
                        >(
                            (Some(((*envs).f_gout).expect("non-null function pointer")))
                                .expect("non-null function pointer"),
                        )(gout, g, idx, envs, 1 as libc::c_int);
                        if i_ctr > 1 as libc::c_int {
                            if *iempty != 0 {
                                CINTprim_to_ctr_0(
                                    gctri,
                                    gout,
                                    ci.offset(ip as isize),
                                    len0,
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
                                    len0,
                                    i_prim,
                                    i_ctr,
                                    *non0ctri.offset(ip as isize),
                                    non0idxi.offset((ip * i_ctr) as isize),
                                );
                            }
                        }
                        *iempty = 0 as libc::c_int;
                    }
                }
                ip += 1;
                ip;
                pdata_ij = pdata_ij.offset(1);
                pdata_ij;
            }
            jp += 1;
            jp;
        }
        kp += 1;
        kp;
    }
    if n_comp > 1 as libc::c_int && *iempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gctri,
                nf.wrapping_mul(nc as libc::c_ulong) as libc::c_int,
                n_comp,
            );
        } else {
            CINTdplus_transpose(
                gctr,
                gctri,
                nf.wrapping_mul(nc as libc::c_ulong) as libc::c_int,
                n_comp,
            );
        }
        *empty = 0 as libc::c_int;
    }
    return (*empty == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINT3c2e_1n1_loop(
    mut gctr: *mut libc::c_double,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut libc::c_double,
    mut empty: *mut libc::c_int,
) -> libc::c_int {
    let mut shls: *mut libc::c_int = (*envs).shls;
    let mut bas: *mut libc::c_int = (*envs).bas;
    let mut env: *mut libc::c_double = (*envs).env;
    let mut i_sh: libc::c_int = *shls.offset(0 as libc::c_int as isize);
    let mut j_sh: libc::c_int = *shls.offset(1 as libc::c_int as isize);
    let mut opt: *mut CINTOpt = (*envs).opt;
    if !((*opt).pairdata).is_null()
        && *((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize)
            == 0xffffffffffffffff as libc::c_ulong as *mut libc::c_void as *mut PairData
    {
        return 0 as libc::c_int;
    }
    let mut k_sh: libc::c_int = *shls.offset(2 as libc::c_int as isize);
    let mut i_ctr: libc::c_int = (*envs).x_ctr[0 as libc::c_int as usize];
    let mut j_ctr: libc::c_int = (*envs).x_ctr[1 as libc::c_int as usize];
    let mut k_ctr: libc::c_int = (*envs).x_ctr[2 as libc::c_int as usize];
    let mut i_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * i_sh + 2 as libc::c_int) as isize);
    let mut j_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * j_sh + 2 as libc::c_int) as isize);
    let mut k_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * k_sh + 2 as libc::c_int) as isize);
    let mut ai: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * i_sh + 5 as libc::c_int) as isize) as isize,
        );
    let mut aj: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * j_sh + 5 as libc::c_int) as isize) as isize,
        );
    let mut ak: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * k_sh + 5 as libc::c_int) as isize) as isize,
        );
    let mut ci: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * i_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut cj: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * j_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut ck: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * k_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut expcutoff: libc::c_double = (*envs).expcutoff;
    let mut rr_ij: libc::c_double = (*envs).rirj[0 as libc::c_int as usize]
        * (*envs).rirj[0 as libc::c_int as usize]
        + (*envs).rirj[1 as libc::c_int as usize]
            * (*envs).rirj[1 as libc::c_int as usize]
        + (*envs).rirj[2 as libc::c_int as usize]
            * (*envs).rirj[2 as libc::c_int as usize];
    let mut pdata_base: *mut PairData = 0 as *mut PairData;
    let mut pdata_ij: *mut PairData = 0 as *mut PairData;
    if !((*opt).pairdata).is_null() {
        pdata_base = *((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize);
    } else {
        let mut log_maxci: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(i_sh as isize);
        let mut log_maxcj: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(j_sh as isize);
        pdata_base = ((cache as uintptr_t)
            .wrapping_add(7 as libc::c_int as libc::c_ulong)
            & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut PairData;
        cache = pdata_base.offset((i_prim * j_prim) as isize) as *mut libc::c_double;
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
            rr_ij,
            expcutoff,
            env,
        ) != 0
        {
            return 0 as libc::c_int;
        }
    }
    let mut n_comp: libc::c_int = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: libc::c_double = 0.;
    let mut fac1j: libc::c_double = 0.;
    let mut fac1k: libc::c_double = 0.;
    let mut ip: libc::c_int = 0;
    let mut jp: libc::c_int = 0;
    let mut kp: libc::c_int = 0;
    let mut _empty: [libc::c_int; 4] = [
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    let mut iempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize);
    let mut jempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize);
    let mut kempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize);
    let mut gempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(3 as libc::c_int as isize);
    let mut non0ctri: *mut libc::c_int = *((*opt).non0ctr).offset(i_sh as isize);
    let mut non0ctrj: *mut libc::c_int = *((*opt).non0ctr).offset(j_sh as isize);
    let mut non0idxi: *mut libc::c_int = *((*opt).sortedidx).offset(i_sh as isize);
    let mut non0idxj: *mut libc::c_int = *((*opt).sortedidx).offset(j_sh as isize);
    let mut non0ctrk: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut non0idxk: *mut libc::c_int = 0 as *mut libc::c_int;
    non0ctrk = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_int;
    cache = non0ctrk.offset((k_prim + k_prim * k_ctr) as isize) as *mut libc::c_double;
    non0idxk = non0ctrk.offset(k_prim as isize);
    CINTOpt_non0coeff_byshell(non0idxk, non0ctrk, ck, k_prim, k_ctr);
    let mut expij: libc::c_double = 0.;
    let mut cutoff: libc::c_double = 0.;
    let mut rij: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rkl: *mut libc::c_double = ((*envs).rkl).as_mut_ptr();
    let mut idx: *mut libc::c_int = *((*opt).index_xyz_array)
        .offset(
            ((*envs).i_l * 16 as libc::c_int * 16 as libc::c_int
                + (*envs).j_l * 16 as libc::c_int + (*envs).k_l) as isize,
        );
    if idx.is_null() {
        idx = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
            & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut libc::c_int;
        cache = idx.offset(nf.wrapping_mul(3 as libc::c_int as libc::c_ulong) as isize)
            as *mut libc::c_double;
        CINTg2e_index_xyz(idx, envs);
    }
    let mut omega: libc::c_double = *env.offset(8 as libc::c_int as isize);
    if omega < 0 as libc::c_int as libc::c_double && (*envs).rys_order > 1 as libc::c_int
    {
        let mut r_guess: libc::c_double = 8.0f64;
        let mut omega2: libc::c_double = omega * omega;
        let mut lij: libc::c_int = (*envs).li_ceil + (*envs).lj_ceil;
        if lij > 0 as libc::c_int {
            let mut dist_ij: libc::c_double = sqrt(rr_ij);
            let mut aij: libc::c_double = *ai
                .offset((i_prim - 1 as libc::c_int) as isize)
                + *aj.offset((j_prim - 1 as libc::c_int) as isize);
            let mut theta: libc::c_double = omega2 / (omega2 + aij);
            expcutoff
                += lij as libc::c_double
                    * log((dist_ij + theta * r_guess + 1.0f64) / (dist_ij + 1.0f64));
        }
        if (*envs).lk_ceil > 0 as libc::c_int {
            let mut theta_0: libc::c_double = omega2
                / (omega2 + *ak.offset((k_prim - 1 as libc::c_int) as isize));
            expcutoff
                += (*envs).lk_ceil as libc::c_double * log(theta_0 * r_guess + 1.0f64);
        }
    }
    let mut nc: libc::c_int = j_ctr;
    let mut leng: size_t = ((*envs).g_size * 3 as libc::c_int
        * (((1 as libc::c_int) << (*envs).gbits) + 1 as libc::c_int)) as size_t;
    let mut lenj: size_t = nf
        .wrapping_mul(j_ctr as libc::c_ulong)
        .wrapping_mul(n_comp as libc::c_ulong);
    let mut len0: size_t = nf.wrapping_mul(n_comp as libc::c_ulong);
    let mut len: size_t = leng.wrapping_add(lenj).wrapping_add(len0);
    let mut g: *mut libc::c_double = 0 as *mut libc::c_double;
    g = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_double;
    cache = g.offset(len as isize);
    let mut g1: *mut libc::c_double = g.offset(leng as isize);
    let mut gout: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut gctrj: *mut libc::c_double = 0 as *mut libc::c_double;
    if n_comp == 1 as libc::c_int {
        gctrj = gctr;
        jempty = empty;
    } else {
        gctrj = g1;
        g1 = g1.offset(lenj as isize);
    }
    gout = g1;
    kp = 0 as libc::c_int;
    while kp < k_prim {
        (*envs).ak[0 as libc::c_int as usize] = *ak.offset(kp as isize);
        fac1k = (*envs).common_factor * *ck.offset(kp as isize);
        pdata_ij = pdata_base;
        jp = 0 as libc::c_int;
        while jp < j_prim {
            (*envs).aj[0 as libc::c_int as usize] = *aj.offset(jp as isize);
            fac1j = fac1k;
            *iempty = 1 as libc::c_int;
            ip = 0 as libc::c_int;
            while ip < i_prim {
                if !((*pdata_ij).cceij > expcutoff) {
                    (*envs).ai[0 as libc::c_int as usize] = *ai.offset(ip as isize);
                    expij = (*pdata_ij).eij;
                    rij = ((*pdata_ij).rij).as_mut_ptr();
                    cutoff = expcutoff - (*pdata_ij).cceij;
                    fac1i = fac1j * *ci.offset(ip as isize) * expij;
                    (*envs).fac[0 as libc::c_int as usize] = fac1i;
                    if ::core::mem::transmute::<
                        _,
                        fn(_, _, _, _, _) -> libc::c_int,
                    >(
                        (Some(((*envs).f_g0_2e).expect("non-null function pointer")))
                            .expect("non-null function pointer"),
                    )(g, rij, rkl, cutoff, envs) != 0
                    {
                        ::core::mem::transmute::<
                            _,
                            fn(_, _, _, _, _),
                        >(
                            (Some(((*envs).f_gout).expect("non-null function pointer")))
                                .expect("non-null function pointer"),
                        )(gout, g, idx, envs, *iempty);
                        *iempty = 0 as libc::c_int;
                    }
                }
                ip += 1;
                ip;
                pdata_ij = pdata_ij.offset(1);
                pdata_ij;
            }
            if *iempty == 0 {
                if j_ctr > 1 as libc::c_int {
                    if *jempty != 0 {
                        CINTprim_to_ctr_0(
                            gctrj,
                            gout,
                            cj.offset(jp as isize),
                            len0,
                            j_prim,
                            j_ctr,
                            *non0ctrj.offset(jp as isize),
                            non0idxj.offset((jp * j_ctr) as isize),
                        );
                    } else {
                        CINTprim_to_ctr_1(
                            gctrj,
                            gout,
                            cj.offset(jp as isize),
                            len0,
                            j_prim,
                            j_ctr,
                            *non0ctrj.offset(jp as isize),
                            non0idxj.offset((jp * j_ctr) as isize),
                        );
                    }
                }
                *jempty = 0 as libc::c_int;
            }
            jp += 1;
            jp;
        }
        kp += 1;
        kp;
    }
    if n_comp > 1 as libc::c_int && *jempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gctrj,
                nf.wrapping_mul(nc as libc::c_ulong) as libc::c_int,
                n_comp,
            );
        } else {
            CINTdplus_transpose(
                gctr,
                gctrj,
                nf.wrapping_mul(nc as libc::c_ulong) as libc::c_int,
                n_comp,
            );
        }
        *empty = 0 as libc::c_int;
    }
    return (*empty == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINT3c2e_loop(
    mut gctr: *mut libc::c_double,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut libc::c_double,
    mut empty: *mut libc::c_int,
) -> libc::c_int {
    let mut shls: *mut libc::c_int = (*envs).shls;
    let mut bas: *mut libc::c_int = (*envs).bas;
    let mut env: *mut libc::c_double = (*envs).env;
    let mut i_sh: libc::c_int = *shls.offset(0 as libc::c_int as isize);
    let mut j_sh: libc::c_int = *shls.offset(1 as libc::c_int as isize);
    let mut opt: *mut CINTOpt = (*envs).opt;
    if !((*opt).pairdata).is_null()
        && *((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize)
            == 0xffffffffffffffff as libc::c_ulong as *mut libc::c_void as *mut PairData
    {
        return 0 as libc::c_int;
    }
    let mut k_sh: libc::c_int = *shls.offset(2 as libc::c_int as isize);
    let mut i_ctr: libc::c_int = (*envs).x_ctr[0 as libc::c_int as usize];
    let mut j_ctr: libc::c_int = (*envs).x_ctr[1 as libc::c_int as usize];
    let mut k_ctr: libc::c_int = (*envs).x_ctr[2 as libc::c_int as usize];
    let mut i_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * i_sh + 2 as libc::c_int) as isize);
    let mut j_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * j_sh + 2 as libc::c_int) as isize);
    let mut k_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * k_sh + 2 as libc::c_int) as isize);
    let mut ai: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * i_sh + 5 as libc::c_int) as isize) as isize,
        );
    let mut aj: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * j_sh + 5 as libc::c_int) as isize) as isize,
        );
    let mut ak: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * k_sh + 5 as libc::c_int) as isize) as isize,
        );
    let mut ci: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * i_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut cj: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * j_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut ck: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * k_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut expcutoff: libc::c_double = (*envs).expcutoff;
    let mut rr_ij: libc::c_double = (*envs).rirj[0 as libc::c_int as usize]
        * (*envs).rirj[0 as libc::c_int as usize]
        + (*envs).rirj[1 as libc::c_int as usize]
            * (*envs).rirj[1 as libc::c_int as usize]
        + (*envs).rirj[2 as libc::c_int as usize]
            * (*envs).rirj[2 as libc::c_int as usize];
    let mut pdata_base: *mut PairData = 0 as *mut PairData;
    let mut pdata_ij: *mut PairData = 0 as *mut PairData;
    if !((*opt).pairdata).is_null() {
        pdata_base = *((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize);
    } else {
        let mut log_maxci: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(i_sh as isize);
        let mut log_maxcj: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(j_sh as isize);
        pdata_base = ((cache as uintptr_t)
            .wrapping_add(7 as libc::c_int as libc::c_ulong)
            & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut PairData;
        cache = pdata_base.offset((i_prim * j_prim) as isize) as *mut libc::c_double;
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
            rr_ij,
            expcutoff,
            env,
        ) != 0
        {
            return 0 as libc::c_int;
        }
    }
    let mut n_comp: libc::c_int = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: libc::c_double = 0.;
    let mut fac1j: libc::c_double = 0.;
    let mut fac1k: libc::c_double = 0.;
    let mut ip: libc::c_int = 0;
    let mut jp: libc::c_int = 0;
    let mut kp: libc::c_int = 0;
    let mut _empty: [libc::c_int; 4] = [
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    let mut iempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize);
    let mut jempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize);
    let mut kempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize);
    let mut gempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(3 as libc::c_int as isize);
    let mut non0ctri: *mut libc::c_int = *((*opt).non0ctr).offset(i_sh as isize);
    let mut non0ctrj: *mut libc::c_int = *((*opt).non0ctr).offset(j_sh as isize);
    let mut non0idxi: *mut libc::c_int = *((*opt).sortedidx).offset(i_sh as isize);
    let mut non0idxj: *mut libc::c_int = *((*opt).sortedidx).offset(j_sh as isize);
    let mut non0ctrk: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut non0idxk: *mut libc::c_int = 0 as *mut libc::c_int;
    non0ctrk = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_int;
    cache = non0ctrk.offset((k_prim + k_prim * k_ctr) as isize) as *mut libc::c_double;
    non0idxk = non0ctrk.offset(k_prim as isize);
    CINTOpt_non0coeff_byshell(non0idxk, non0ctrk, ck, k_prim, k_ctr);
    let mut expij: libc::c_double = 0.;
    let mut cutoff: libc::c_double = 0.;
    let mut rij: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rkl: *mut libc::c_double = ((*envs).rkl).as_mut_ptr();
    let mut idx: *mut libc::c_int = *((*opt).index_xyz_array)
        .offset(
            ((*envs).i_l * 16 as libc::c_int * 16 as libc::c_int
                + (*envs).j_l * 16 as libc::c_int + (*envs).k_l) as isize,
        );
    if idx.is_null() {
        idx = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
            & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut libc::c_int;
        cache = idx.offset(nf.wrapping_mul(3 as libc::c_int as libc::c_ulong) as isize)
            as *mut libc::c_double;
        CINTg2e_index_xyz(idx, envs);
    }
    let mut omega: libc::c_double = *env.offset(8 as libc::c_int as isize);
    if omega < 0 as libc::c_int as libc::c_double && (*envs).rys_order > 1 as libc::c_int
    {
        let mut r_guess: libc::c_double = 8.0f64;
        let mut omega2: libc::c_double = omega * omega;
        let mut lij: libc::c_int = (*envs).li_ceil + (*envs).lj_ceil;
        if lij > 0 as libc::c_int {
            let mut dist_ij: libc::c_double = sqrt(rr_ij);
            let mut aij: libc::c_double = *ai
                .offset((i_prim - 1 as libc::c_int) as isize)
                + *aj.offset((j_prim - 1 as libc::c_int) as isize);
            let mut theta: libc::c_double = omega2 / (omega2 + aij);
            expcutoff
                += lij as libc::c_double
                    * log((dist_ij + theta * r_guess + 1.0f64) / (dist_ij + 1.0f64));
        }
        if (*envs).lk_ceil > 0 as libc::c_int {
            let mut theta_0: libc::c_double = omega2
                / (omega2 + *ak.offset((k_prim - 1 as libc::c_int) as isize));
            expcutoff
                += (*envs).lk_ceil as libc::c_double * log(theta_0 * r_guess + 1.0f64);
        }
    }
    let mut nc: libc::c_int = i_ctr * j_ctr * k_ctr;
    let mut leng: size_t = ((*envs).g_size * 3 as libc::c_int
        * (((1 as libc::c_int) << (*envs).gbits) + 1 as libc::c_int)) as size_t;
    let mut lenk: size_t = nf
        .wrapping_mul(nc as libc::c_ulong)
        .wrapping_mul(n_comp as libc::c_ulong);
    let mut lenj: size_t = nf
        .wrapping_mul(i_ctr as libc::c_ulong)
        .wrapping_mul(j_ctr as libc::c_ulong)
        .wrapping_mul(n_comp as libc::c_ulong);
    let mut leni: size_t = nf
        .wrapping_mul(i_ctr as libc::c_ulong)
        .wrapping_mul(n_comp as libc::c_ulong);
    let mut len0: size_t = nf.wrapping_mul(n_comp as libc::c_ulong);
    let mut len: size_t = leng
        .wrapping_add(lenk)
        .wrapping_add(lenj)
        .wrapping_add(leni)
        .wrapping_add(len0);
    let mut g: *mut libc::c_double = 0 as *mut libc::c_double;
    g = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_double;
    cache = g.offset(len as isize);
    let mut g1: *mut libc::c_double = g.offset(leng as isize);
    let mut gout: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut gctri: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut gctrj: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut gctrk: *mut libc::c_double = 0 as *mut libc::c_double;
    if n_comp == 1 as libc::c_int {
        gctrk = gctr;
        kempty = empty;
    } else {
        gctrk = g1;
        g1 = g1.offset(lenk as isize);
    }
    if k_ctr == 1 as libc::c_int {
        gctrj = gctrk;
        jempty = kempty;
    } else {
        gctrj = g1;
        g1 = g1.offset(lenj as isize);
    }
    if j_ctr == 1 as libc::c_int {
        gctri = gctrj;
        iempty = jempty;
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
            *jempty = 1 as libc::c_int;
        }
        pdata_ij = pdata_base;
        jp = 0 as libc::c_int;
        while jp < j_prim {
            (*envs).aj[0 as libc::c_int as usize] = *aj.offset(jp as isize);
            if j_ctr == 1 as libc::c_int {
                fac1j = fac1k * *cj.offset(jp as isize);
            } else {
                fac1j = fac1k;
                *iempty = 1 as libc::c_int;
            }
            ip = 0 as libc::c_int;
            while ip < i_prim {
                if !((*pdata_ij).cceij > expcutoff) {
                    (*envs).ai[0 as libc::c_int as usize] = *ai.offset(ip as isize);
                    expij = (*pdata_ij).eij;
                    rij = ((*pdata_ij).rij).as_mut_ptr();
                    cutoff = expcutoff - (*pdata_ij).cceij;
                    if i_ctr == 1 as libc::c_int {
                        fac1i = fac1j * *ci.offset(ip as isize) * expij;
                    } else {
                        fac1i = fac1j * expij;
                    }
                    (*envs).fac[0 as libc::c_int as usize] = fac1i;
                    if ::core::mem::transmute::<
                        _,
                        fn(_, _, _, _, _) -> libc::c_int,
                    >(
                        (Some(((*envs).f_g0_2e).expect("non-null function pointer")))
                            .expect("non-null function pointer"),
                    )(g, rij, rkl, cutoff, envs) != 0
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
                                    len0,
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
                                    len0,
                                    i_prim,
                                    i_ctr,
                                    *non0ctri.offset(ip as isize),
                                    non0idxi.offset((ip * i_ctr) as isize),
                                );
                            }
                        }
                        *iempty = 0 as libc::c_int;
                    }
                }
                ip += 1;
                ip;
                pdata_ij = pdata_ij.offset(1);
                pdata_ij;
            }
            if *iempty == 0 {
                if j_ctr > 1 as libc::c_int {
                    if *jempty != 0 {
                        CINTprim_to_ctr_0(
                            gctrj,
                            gctri,
                            cj.offset(jp as isize),
                            leni,
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
                            leni,
                            j_prim,
                            j_ctr,
                            *non0ctrj.offset(jp as isize),
                            non0idxj.offset((jp * j_ctr) as isize),
                        );
                    }
                }
                *jempty = 0 as libc::c_int;
            }
            jp += 1;
            jp;
        }
        if *jempty == 0 {
            if k_ctr > 1 as libc::c_int {
                if *kempty != 0 {
                    CINTprim_to_ctr_0(
                        gctrk,
                        gctrj,
                        ck.offset(kp as isize),
                        lenj,
                        k_prim,
                        k_ctr,
                        *non0ctrk.offset(kp as isize),
                        non0idxk.offset((kp * k_ctr) as isize),
                    );
                } else {
                    CINTprim_to_ctr_1(
                        gctrk,
                        gctrj,
                        ck.offset(kp as isize),
                        lenj,
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
static mut CINTf_3c2e_loop: [Option::<
    unsafe extern "C" fn(
        *mut libc::c_double,
        *mut CINTEnvVars,
        *mut libc::c_double,
        *mut libc::c_int,
    ) -> libc::c_int,
>; 8] = unsafe {
    [
        Some(
            CINT3c2e_loop
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            CINT3c2e_loop
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            CINT3c2e_loop
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            CINT3c2e_n11_loop
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            CINT3c2e_loop
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            CINT3c2e_1n1_loop
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            CINT3c2e_loop
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            CINT3c2e_111_loop
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
    ]
};
#[no_mangle]
pub unsafe extern "C" fn CINT3c2e_drv(
    mut out: *mut libc::c_double,
    mut dims: *mut libc::c_int,
    mut envs: *mut CINTEnvVars,
    mut opt: *mut CINTOpt,
    mut cache: *mut libc::c_double,
    mut f_e1_c2s: Option::<unsafe extern "C" fn() -> ()>,
    mut is_ssc: libc::c_int,
) -> libc::c_int {
    let mut x_ctr: *mut libc::c_int = ((*envs).x_ctr).as_mut_ptr();
    let mut nc: size_t = ((*envs).nf * *x_ctr.offset(0 as libc::c_int as isize)
        * *x_ctr.offset(1 as libc::c_int as isize)
        * *x_ctr.offset(2 as libc::c_int as isize)) as size_t;
    let mut n_comp: libc::c_int = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    if out.is_null() {
        let mut bas: *mut libc::c_int = (*envs).bas;
        let mut shls: *mut libc::c_int = (*envs).shls;
        let mut i_prim: libc::c_int = *bas
            .offset(
                (8 as libc::c_int * *shls.offset(0 as libc::c_int as isize)
                    + 2 as libc::c_int) as isize,
            );
        let mut j_prim: libc::c_int = *bas
            .offset(
                (8 as libc::c_int * *shls.offset(1 as libc::c_int as isize)
                    + 2 as libc::c_int) as isize,
            );
        let mut k_prim: libc::c_int = *bas
            .offset(
                (8 as libc::c_int * *shls.offset(2 as libc::c_int as isize)
                    + 2 as libc::c_int) as isize,
            );
        let mut pdata_size: libc::c_int = i_prim * j_prim * 5 as libc::c_int
            + i_prim * *x_ctr.offset(0 as libc::c_int as isize)
            + j_prim * *x_ctr.offset(1 as libc::c_int as isize)
            + k_prim * *x_ctr.offset(2 as libc::c_int as isize)
            + (i_prim + j_prim) * 2 as libc::c_int + k_prim
            + (*envs).nf * 3 as libc::c_int + 16 as libc::c_int;
        let mut leng: libc::c_int = (*envs).g_size * 3 as libc::c_int
            * (((1 as libc::c_int) << (*envs).gbits) + 1 as libc::c_int);
        let mut len0: libc::c_int = (*envs).nf * n_comp;
        let mut cache_size: libc::c_int = (if ((leng + len0) as libc::c_ulong)
            .wrapping_add(
                nc
                    .wrapping_mul(n_comp as libc::c_ulong)
                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
            )
            .wrapping_add(pdata_size as libc::c_ulong)
            > nc
                .wrapping_mul(n_comp as libc::c_ulong)
                .wrapping_add(((*envs).nf * 3 as libc::c_int) as libc::c_ulong)
        {
            ((leng + len0) as libc::c_ulong)
                .wrapping_add(
                    nc
                        .wrapping_mul(n_comp as libc::c_ulong)
                        .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(pdata_size as libc::c_ulong)
        } else {
            nc.wrapping_mul(n_comp as libc::c_ulong)
                .wrapping_add(((*envs).nf * 3 as libc::c_int) as libc::c_ulong)
        }) as libc::c_int;
        return cache_size;
    }
    let mut stack: *mut libc::c_double = 0 as *mut libc::c_double;
    if cache.is_null() {
        let mut bas_0: *mut libc::c_int = (*envs).bas;
        let mut shls_0: *mut libc::c_int = (*envs).shls;
        let mut i_prim_0: libc::c_int = *bas_0
            .offset(
                (8 as libc::c_int * *shls_0.offset(0 as libc::c_int as isize)
                    + 2 as libc::c_int) as isize,
            );
        let mut j_prim_0: libc::c_int = *bas_0
            .offset(
                (8 as libc::c_int * *shls_0.offset(1 as libc::c_int as isize)
                    + 2 as libc::c_int) as isize,
            );
        let mut k_prim_0: libc::c_int = *bas_0
            .offset(
                (8 as libc::c_int * *shls_0.offset(2 as libc::c_int as isize)
                    + 2 as libc::c_int) as isize,
            );
        let mut pdata_size_0: libc::c_int = i_prim_0 * j_prim_0 * 5 as libc::c_int
            + i_prim_0 * *x_ctr.offset(0 as libc::c_int as isize)
            + j_prim_0 * *x_ctr.offset(1 as libc::c_int as isize)
            + k_prim_0 * *x_ctr.offset(2 as libc::c_int as isize)
            + (i_prim_0 + j_prim_0) * 2 as libc::c_int + k_prim_0
            + (*envs).nf * 3 as libc::c_int + 16 as libc::c_int;
        let mut leng_0: size_t = ((*envs).g_size * 3 as libc::c_int
            * (((1 as libc::c_int) << (*envs).gbits) + 1 as libc::c_int)) as size_t;
        let mut len0_0: size_t = ((*envs).nf * n_comp) as size_t;
        let mut cache_size_0: size_t = if leng_0
            .wrapping_add(len0_0)
            .wrapping_add(
                nc
                    .wrapping_mul(n_comp as libc::c_ulong)
                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
            )
            .wrapping_add(pdata_size_0 as libc::c_ulong)
            > nc
                .wrapping_mul(n_comp as libc::c_ulong)
                .wrapping_add(((*envs).nf * 3 as libc::c_int) as libc::c_ulong)
        {
            leng_0
                .wrapping_add(len0_0)
                .wrapping_add(
                    nc
                        .wrapping_mul(n_comp as libc::c_ulong)
                        .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(pdata_size_0 as libc::c_ulong)
        } else {
            nc.wrapping_mul(n_comp as libc::c_ulong)
                .wrapping_add(((*envs).nf * 3 as libc::c_int) as libc::c_ulong)
        };
        stack = malloc(
            (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(cache_size_0),
        ) as *mut libc::c_double;
        cache = stack;
    }
    let mut gctr: *mut libc::c_double = 0 as *mut libc::c_double;
    gctr = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_double;
    cache = gctr.offset(nc.wrapping_mul(n_comp as libc::c_ulong) as isize);
    let mut n: libc::c_int = 0;
    let mut empty: libc::c_int = 1 as libc::c_int;
    if !opt.is_null() {
        (*envs).opt = opt;
        n = ((((*envs).x_ctr[0 as libc::c_int as usize] == 1 as libc::c_int)
            as libc::c_int) << 2 as libc::c_int)
            + ((((*envs).x_ctr[1 as libc::c_int as usize] == 1 as libc::c_int)
                as libc::c_int) << 1 as libc::c_int)
            + ((*envs).x_ctr[2 as libc::c_int as usize] == 1 as libc::c_int)
                as libc::c_int;
        (CINTf_3c2e_loop[n as usize])
            .expect("non-null function pointer")(gctr, envs, cache, &mut empty);
    } else {
        CINT3c2e_loop_nopt(gctr, envs, cache, &mut empty);
    }
    let mut counts: [libc::c_int; 4] = [0; 4];
    if f_e1_c2s
        == ::core::mem::transmute::<
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
                c2s_sph_3c2e1
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut libc::c_double,
                        *mut libc::c_int,
                        *mut CINTEnvVars,
                        *mut libc::c_double,
                    ) -> (),
            ),
        )
    {
        counts[0 as libc::c_int
            as usize] = ((*envs).i_l * 2 as libc::c_int + 1 as libc::c_int)
            * *x_ctr.offset(0 as libc::c_int as isize);
        counts[1 as libc::c_int
            as usize] = ((*envs).j_l * 2 as libc::c_int + 1 as libc::c_int)
            * *x_ctr.offset(1 as libc::c_int as isize);
        if is_ssc != 0 {
            counts[2 as libc::c_int
                as usize] = (*envs).c2rust_unnamed.nfk
                * *x_ctr.offset(2 as libc::c_int as isize);
        } else {
            counts[2 as libc::c_int
                as usize] = ((*envs).k_l * 2 as libc::c_int + 1 as libc::c_int)
                * *x_ctr.offset(2 as libc::c_int as isize);
        }
    } else {
        counts[0 as libc::c_int
            as usize] = (*envs).nfi * *x_ctr.offset(0 as libc::c_int as isize);
        counts[1 as libc::c_int
            as usize] = (*envs).nfj * *x_ctr.offset(1 as libc::c_int as isize);
        counts[2 as libc::c_int
            as usize] = (*envs).c2rust_unnamed.nfk
            * *x_ctr.offset(2 as libc::c_int as isize);
    }
    counts[3 as libc::c_int as usize] = 1 as libc::c_int;
    if dims.is_null() {
        dims = counts.as_mut_ptr();
    }
    let mut nout: libc::c_int = *dims.offset(0 as libc::c_int as isize)
        * *dims.offset(1 as libc::c_int as isize)
        * *dims.offset(2 as libc::c_int as isize);
    if empty == 0 {
        n = 0 as libc::c_int;
        while n < n_comp {
            ::core::mem::transmute::<
                _,
                fn(_, _, _, _, _),
            >(
                (Some(f_e1_c2s.expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )(
                out.offset((nout * n) as isize),
                gctr.offset(nc.wrapping_mul(n as libc::c_ulong) as isize),
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
pub unsafe extern "C" fn int3c2e_sph(
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
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int3c2e_EnvVars(
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
            CINTgout2e
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
                ) -> (),
        ),
    );
    return CINT3c2e_drv(
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
                c2s_sph_3c2e1
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
pub unsafe extern "C" fn int3c2e_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
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
    CINTall_3c2e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int3c2e_cart(
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
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int3c2e_EnvVars(
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
            CINTgout2e
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
                ) -> (),
        ),
    );
    return CINT3c2e_drv(
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
                c2s_cart_3c2e1
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
pub unsafe extern "C" fn int3c2e_sph_ssc(
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
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int3c2e_EnvVars(
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
            CINTgout2e
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
                ) -> (),
        ),
    );
    return CINT3c2e_drv(
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
                c2s_sph_3c2e1_ssc
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut libc::c_double,
                        *mut libc::c_int,
                        *mut CINTEnvVars,
                        *mut libc::c_double,
                    ) -> (),
            ),
        ),
        1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int3c2e_ssc_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
) {
    int3c2e_optimizer(opt, atm, natm, bas, nbas, env);
}
