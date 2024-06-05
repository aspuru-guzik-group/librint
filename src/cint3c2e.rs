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
    fn log(_: f64) -> f64;
    fn sqrt(_: f64) -> f64;
}

#[no_mangle]
pub unsafe extern "C" fn CINT3c2e_loop_nopt(
    mut gctr: *mut f64,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut f64,
    mut empty: *mut i32,
) -> i32 {
    let mut shls: *mut i32 = (*envs).shls;
    let mut bas: *mut i32 = (*envs).bas;
    let mut env: *mut f64 = (*envs).env;
    let mut i_sh: i32 = *shls.offset(0 as i32 as isize);
    let mut j_sh: i32 = *shls.offset(1 as i32 as isize);
    let mut k_sh: i32 = *shls.offset(2 as i32 as isize);
    let mut i_ctr: i32 = (*envs).x_ctr[0 as i32 as usize];
    let mut j_ctr: i32 = (*envs).x_ctr[1 as i32 as usize];
    let mut k_ctr: i32 = (*envs).x_ctr[2 as i32 as usize];
    let mut i_prim: i32 = *bas
        .offset((8 as i32 * i_sh + 2 as i32) as isize);
    let mut j_prim: i32 = *bas
        .offset((8 as i32 * j_sh + 2 as i32) as isize);
    let mut k_prim: i32 = *bas
        .offset((8 as i32 * k_sh + 2 as i32) as isize);
    let mut ai: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * i_sh + 5 as i32) as isize) as isize,
        );
    let mut aj: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * j_sh + 5 as i32) as isize) as isize,
        );
    let mut ak: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * k_sh + 5 as i32) as isize) as isize,
        );
    let mut ci: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * i_sh + 6 as i32) as isize) as isize,
        );
    let mut cj: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * j_sh + 6 as i32) as isize) as isize,
        );
    let mut ck: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * k_sh + 6 as i32) as isize) as isize,
        );
    let mut expcutoff: f64 = (*envs).expcutoff;
    let mut rr_ij: f64 = (*envs).rirj[0 as i32 as usize]
        * (*envs).rirj[0 as i32 as usize]
        + (*envs).rirj[1 as i32 as usize]
            * (*envs).rirj[1 as i32 as usize]
        + (*envs).rirj[2 as i32 as usize]
            * (*envs).rirj[2 as i32 as usize];
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
        rr_ij,
        expcutoff,
        env,
    ) != 0
    {
        return 0 as i32;
    }
    let mut n_comp: i32 = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: f64 = 0.;
    let mut fac1j: f64 = 0.;
    let mut fac1k: f64 = 0.;
    let mut ip: i32 = 0;
    let mut jp: i32 = 0;
    let mut kp: i32 = 0;
    let mut _empty: [i32; 4] = [
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    let mut iempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(0 as i32 as isize);
    let mut jempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(1 as i32 as isize);
    let mut kempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(2 as i32 as isize);
    let mut gempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(3 as i32 as isize);
    let mut expij: f64 = 0.;
    let mut cutoff: f64 = 0.;
    let mut rij: *mut f64 = 0 as *mut f64;
    let mut rkl: *mut f64 = (*envs).rk;
    let mut omega: f64 = *env.offset(8 as i32 as isize);
    if omega < 0 as i32 as f64 && (*envs).rys_order > 1 as i32
    {
        let mut r_guess: f64 = 8.0f64;
        let mut omega2: f64 = omega * omega;
        let mut lij: i32 = (*envs).li_ceil + (*envs).lj_ceil;
        if lij > 0 as i32 {
            let mut dist_ij: f64 = sqrt(rr_ij);
            let mut aij: f64 = *ai
                .offset((i_prim - 1 as i32) as isize)
                + *aj.offset((j_prim - 1 as i32) as isize);
            let mut theta: f64 = omega2 / (omega2 + aij);
            expcutoff
                += lij as f64
                    * log((dist_ij + theta * r_guess + 1.0f64) / (dist_ij + 1.0f64));
        }
        if (*envs).lk_ceil > 0 as i32 {
            let mut theta_0: f64 = omega2
                / (omega2 + *ak.offset((k_prim - 1 as i32) as isize));
            expcutoff
                += (*envs).lk_ceil as f64 * log(theta_0 * r_guess + 1.0f64);
        }
    }
    let mut idx: *mut i32 = 0 as *mut i32;
    idx = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut i32;
    cache = idx.offset(nf.wrapping_mul(3 as i32 as libc::c_ulong) as isize)
        as *mut f64;
    CINTg2e_index_xyz(idx, envs);
    let mut non0ctri: *mut i32 = 0 as *mut i32;
    let mut non0ctrj: *mut i32 = 0 as *mut i32;
    let mut non0ctrk: *mut i32 = 0 as *mut i32;
    let mut non0idxi: *mut i32 = 0 as *mut i32;
    let mut non0idxj: *mut i32 = 0 as *mut i32;
    let mut non0idxk: *mut i32 = 0 as *mut i32;
    non0ctri = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut i32;
    cache = non0ctri
        .offset(
            (i_prim + j_prim + k_prim + i_prim * i_ctr + j_prim * j_ctr + k_prim * k_ctr)
                as isize,
        ) as *mut f64;
    non0ctrj = non0ctri.offset(i_prim as isize);
    non0ctrk = non0ctrj.offset(j_prim as isize);
    non0idxi = non0ctrk.offset(k_prim as isize);
    non0idxj = non0idxi.offset((i_prim * i_ctr) as isize);
    non0idxk = non0idxj.offset((j_prim * j_ctr) as isize);
    CINTOpt_non0coeff_byshell(non0idxi, non0ctri, ci, i_prim, i_ctr);
    CINTOpt_non0coeff_byshell(non0idxj, non0ctrj, cj, j_prim, j_ctr);
    CINTOpt_non0coeff_byshell(non0idxk, non0ctrk, ck, k_prim, k_ctr);
    let mut nc: i32 = i_ctr * j_ctr * k_ctr;
    let mut leng: size_t = ((*envs).g_size * 3 as i32
        * (((1 as i32) << (*envs).gbits) + 1 as i32)) as size_t;
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
    let mut g: *mut f64 = 0 as *mut f64;
    g = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut f64;
    cache = g.offset(len as isize);
    let mut g1: *mut f64 = g.offset(leng as isize);
    let mut gout: *mut f64 = 0 as *mut f64;
    let mut gctri: *mut f64 = 0 as *mut f64;
    let mut gctrj: *mut f64 = 0 as *mut f64;
    let mut gctrk: *mut f64 = 0 as *mut f64;
    if n_comp == 1 as i32 {
        gctrk = gctr;
        kempty = empty;
    } else {
        gctrk = g1;
        g1 = g1.offset(lenk as isize);
    }
    if k_ctr == 1 as i32 {
        gctrj = gctrk;
        jempty = kempty;
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
        g1 = g1.offset(leng as isize);
    }
    kp = 0 as i32;
    while kp < k_prim {
        (*envs).ak[0 as i32 as usize] = *ak.offset(kp as isize);
        if k_ctr == 1 as i32 {
            fac1k = (*envs).common_factor * *ck.offset(kp as isize);
        } else {
            fac1k = (*envs).common_factor;
            *jempty = 1 as i32;
        }
        pdata_ij = pdata_base;
        jp = 0 as i32;
        while jp < j_prim {
            (*envs).aj[0 as i32 as usize] = *aj.offset(jp as isize);
            if j_ctr == 1 as i32 {
                fac1j = fac1k * *cj.offset(jp as isize);
            } else {
                fac1j = fac1k;
                *iempty = 1 as i32;
            }
            ip = 0 as i32;
            while ip < i_prim {
                if !((*pdata_ij).cceij > expcutoff) {
                    (*envs).ai[0 as i32 as usize] = *ai.offset(ip as isize);
                    expij = (*pdata_ij).eij;
                    rij = ((*pdata_ij).rij).as_mut_ptr();
                    cutoff = expcutoff - (*pdata_ij).cceij;
                    if i_ctr == 1 as i32 {
                        fac1i = fac1j * *ci.offset(ip as isize) * expij;
                    } else {
                        fac1i = fac1j * expij;
                    }
                    (*envs).fac[0 as i32 as usize] = fac1i;
                    if ::core::mem::transmute::<
                        _,
                        fn(_, _, _, _, _) -> i32,
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
                        if i_ctr > 1 as i32 {
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
                        *iempty = 0 as i32;
                    }
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
                *jempty = 0 as i32;
            }
            jp += 1;
            jp;
        }
        if *jempty == 0 {
            if k_ctr > 1 as i32 {
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
pub unsafe extern "C" fn CINT3c2e_111_loop(
    mut gctr: *mut f64,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut f64,
    mut empty: *mut i32,
) -> i32 {
    let mut shls: *mut i32 = (*envs).shls;
    let mut bas: *mut i32 = (*envs).bas;
    let mut env: *mut f64 = (*envs).env;
    let mut i_sh: i32 = *shls.offset(0 as i32 as isize);
    let mut j_sh: i32 = *shls.offset(1 as i32 as isize);
    let mut opt: *mut CINTOpt = (*envs).opt;
    if !((*opt).pairdata).is_null()
        && *((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize)
            == 0xffffffffffffffff as libc::c_ulong as *mut libc::c_void as *mut PairData
    {
        return 0 as i32;
    }
    let mut k_sh: i32 = *shls.offset(2 as i32 as isize);
    let mut i_ctr: i32 = (*envs).x_ctr[0 as i32 as usize];
    let mut j_ctr: i32 = (*envs).x_ctr[1 as i32 as usize];
    let mut k_ctr: i32 = (*envs).x_ctr[2 as i32 as usize];
    let mut i_prim: i32 = *bas
        .offset((8 as i32 * i_sh + 2 as i32) as isize);
    let mut j_prim: i32 = *bas
        .offset((8 as i32 * j_sh + 2 as i32) as isize);
    let mut k_prim: i32 = *bas
        .offset((8 as i32 * k_sh + 2 as i32) as isize);
    let mut ai: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * i_sh + 5 as i32) as isize) as isize,
        );
    let mut aj: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * j_sh + 5 as i32) as isize) as isize,
        );
    let mut ak: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * k_sh + 5 as i32) as isize) as isize,
        );
    let mut ci: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * i_sh + 6 as i32) as isize) as isize,
        );
    let mut cj: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * j_sh + 6 as i32) as isize) as isize,
        );
    let mut ck: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * k_sh + 6 as i32) as isize) as isize,
        );
    let mut expcutoff: f64 = (*envs).expcutoff;
    let mut rr_ij: f64 = (*envs).rirj[0 as i32 as usize]
        * (*envs).rirj[0 as i32 as usize]
        + (*envs).rirj[1 as i32 as usize]
            * (*envs).rirj[1 as i32 as usize]
        + (*envs).rirj[2 as i32 as usize]
            * (*envs).rirj[2 as i32 as usize];
    let mut pdata_base: *mut PairData = 0 as *mut PairData;
    let mut pdata_ij: *mut PairData = 0 as *mut PairData;
    if !((*opt).pairdata).is_null() {
        pdata_base = *((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize);
    } else {
        let mut log_maxci: *mut f64 = *((*opt).log_max_coeff)
            .offset(i_sh as isize);
        let mut log_maxcj: *mut f64 = *((*opt).log_max_coeff)
            .offset(j_sh as isize);
        pdata_base = ((cache as uintptr_t)
            .wrapping_add(7 as i32 as libc::c_ulong)
            & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut PairData;
        cache = pdata_base.offset((i_prim * j_prim) as isize) as *mut f64;
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
            return 0 as i32;
        }
    }
    let mut n_comp: i32 = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: f64 = 0.;
    let mut fac1j: f64 = 0.;
    let mut fac1k: f64 = 0.;
    let mut ip: i32 = 0;
    let mut jp: i32 = 0;
    let mut kp: i32 = 0;
    let mut _empty: [i32; 4] = [
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    let mut iempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(0 as i32 as isize);
    let mut jempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(1 as i32 as isize);
    let mut kempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(2 as i32 as isize);
    let mut gempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(3 as i32 as isize);
    let mut non0ctri: *mut i32 = *((*opt).non0ctr).offset(i_sh as isize);
    let mut non0ctrj: *mut i32 = *((*opt).non0ctr).offset(j_sh as isize);
    let mut non0idxi: *mut i32 = *((*opt).sortedidx).offset(i_sh as isize);
    let mut non0idxj: *mut i32 = *((*opt).sortedidx).offset(j_sh as isize);
    let mut non0ctrk: *mut i32 = 0 as *mut i32;
    let mut non0idxk: *mut i32 = 0 as *mut i32;
    non0ctrk = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut i32;
    cache = non0ctrk.offset((k_prim + k_prim * k_ctr) as isize) as *mut f64;
    non0idxk = non0ctrk.offset(k_prim as isize);
    CINTOpt_non0coeff_byshell(non0idxk, non0ctrk, ck, k_prim, k_ctr);
    let mut expij: f64 = 0.;
    let mut cutoff: f64 = 0.;
    let mut rij: *mut f64 = 0 as *mut f64;
    let mut rkl: *mut f64 = ((*envs).rkl).as_mut_ptr();
    let mut idx: *mut i32 = *((*opt).index_xyz_array)
        .offset(
            ((*envs).i_l * 16 as i32 * 16 as i32
                + (*envs).j_l * 16 as i32 + (*envs).k_l) as isize,
        );
    if idx.is_null() {
        idx = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
            & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut i32;
        cache = idx.offset(nf.wrapping_mul(3 as i32 as libc::c_ulong) as isize)
            as *mut f64;
        CINTg2e_index_xyz(idx, envs);
    }
    let mut omega: f64 = *env.offset(8 as i32 as isize);
    if omega < 0 as i32 as f64 && (*envs).rys_order > 1 as i32
    {
        let mut r_guess: f64 = 8.0f64;
        let mut omega2: f64 = omega * omega;
        let mut lij: i32 = (*envs).li_ceil + (*envs).lj_ceil;
        if lij > 0 as i32 {
            let mut dist_ij: f64 = sqrt(rr_ij);
            let mut aij: f64 = *ai
                .offset((i_prim - 1 as i32) as isize)
                + *aj.offset((j_prim - 1 as i32) as isize);
            let mut theta: f64 = omega2 / (omega2 + aij);
            expcutoff
                += lij as f64
                    * log((dist_ij + theta * r_guess + 1.0f64) / (dist_ij + 1.0f64));
        }
        if (*envs).lk_ceil > 0 as i32 {
            let mut theta_0: f64 = omega2
                / (omega2 + *ak.offset((k_prim - 1 as i32) as isize));
            expcutoff
                += (*envs).lk_ceil as f64 * log(theta_0 * r_guess + 1.0f64);
        }
    }
    let mut nc: i32 = 1 as i32;
    let mut leng: size_t = ((*envs).g_size * 3 as i32
        * (((1 as i32) << (*envs).gbits) + 1 as i32)) as size_t;
    let mut len0: size_t = ((*envs).nf * n_comp) as size_t;
    let mut len: size_t = leng.wrapping_add(len0);
    let mut g: *mut f64 = 0 as *mut f64;
    g = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut f64;
    cache = g.offset(len as isize);
    let mut gout: *mut f64 = 0 as *mut f64;
    if n_comp == 1 as i32 {
        gout = gctr;
        gempty = empty;
    } else {
        gout = g.offset(leng as isize);
    }
    kp = 0 as i32;
    while kp < k_prim {
        (*envs).ak[0 as i32 as usize] = *ak.offset(kp as isize);
        fac1k = (*envs).common_factor * *ck.offset(kp as isize);
        pdata_ij = pdata_base;
        jp = 0 as i32;
        while jp < j_prim {
            (*envs).aj[0 as i32 as usize] = *aj.offset(jp as isize);
            fac1j = fac1k * *cj.offset(jp as isize);
            ip = 0 as i32;
            while ip < i_prim {
                if !((*pdata_ij).cceij > expcutoff) {
                    (*envs).ai[0 as i32 as usize] = *ai.offset(ip as isize);
                    expij = (*pdata_ij).eij;
                    rij = ((*pdata_ij).rij).as_mut_ptr();
                    cutoff = expcutoff - (*pdata_ij).cceij;
                    fac1i = fac1j * *ci.offset(ip as isize) * expij;
                    (*envs).fac[0 as i32 as usize] = fac1i;
                    if ::core::mem::transmute::<
                        _,
                        fn(_, _, _, _, _) -> i32,
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
                        *gempty = 0 as i32;
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
    if n_comp > 1 as i32 && *gempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gout,
                nf.wrapping_mul(nc as libc::c_ulong) as i32,
                n_comp,
            );
        } else {
            CINTdplus_transpose(
                gctr,
                gout,
                nf.wrapping_mul(nc as libc::c_ulong) as i32,
                n_comp,
            );
        }
        *empty = 0 as i32;
    }
    return (*empty == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn CINT3c2e_n11_loop(
    mut gctr: *mut f64,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut f64,
    mut empty: *mut i32,
) -> i32 {
    let mut shls: *mut i32 = (*envs).shls;
    let mut bas: *mut i32 = (*envs).bas;
    let mut env: *mut f64 = (*envs).env;
    let mut i_sh: i32 = *shls.offset(0 as i32 as isize);
    let mut j_sh: i32 = *shls.offset(1 as i32 as isize);
    let mut opt: *mut CINTOpt = (*envs).opt;
    if !((*opt).pairdata).is_null()
        && *((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize)
            == 0xffffffffffffffff as libc::c_ulong as *mut libc::c_void as *mut PairData
    {
        return 0 as i32;
    }
    let mut k_sh: i32 = *shls.offset(2 as i32 as isize);
    let mut i_ctr: i32 = (*envs).x_ctr[0 as i32 as usize];
    let mut j_ctr: i32 = (*envs).x_ctr[1 as i32 as usize];
    let mut k_ctr: i32 = (*envs).x_ctr[2 as i32 as usize];
    let mut i_prim: i32 = *bas
        .offset((8 as i32 * i_sh + 2 as i32) as isize);
    let mut j_prim: i32 = *bas
        .offset((8 as i32 * j_sh + 2 as i32) as isize);
    let mut k_prim: i32 = *bas
        .offset((8 as i32 * k_sh + 2 as i32) as isize);
    let mut ai: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * i_sh + 5 as i32) as isize) as isize,
        );
    let mut aj: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * j_sh + 5 as i32) as isize) as isize,
        );
    let mut ak: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * k_sh + 5 as i32) as isize) as isize,
        );
    let mut ci: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * i_sh + 6 as i32) as isize) as isize,
        );
    let mut cj: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * j_sh + 6 as i32) as isize) as isize,
        );
    let mut ck: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * k_sh + 6 as i32) as isize) as isize,
        );
    let mut expcutoff: f64 = (*envs).expcutoff;
    let mut rr_ij: f64 = (*envs).rirj[0 as i32 as usize]
        * (*envs).rirj[0 as i32 as usize]
        + (*envs).rirj[1 as i32 as usize]
            * (*envs).rirj[1 as i32 as usize]
        + (*envs).rirj[2 as i32 as usize]
            * (*envs).rirj[2 as i32 as usize];
    let mut pdata_base: *mut PairData = 0 as *mut PairData;
    let mut pdata_ij: *mut PairData = 0 as *mut PairData;
    if !((*opt).pairdata).is_null() {
        pdata_base = *((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize);
    } else {
        let mut log_maxci: *mut f64 = *((*opt).log_max_coeff)
            .offset(i_sh as isize);
        let mut log_maxcj: *mut f64 = *((*opt).log_max_coeff)
            .offset(j_sh as isize);
        pdata_base = ((cache as uintptr_t)
            .wrapping_add(7 as i32 as libc::c_ulong)
            & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut PairData;
        cache = pdata_base.offset((i_prim * j_prim) as isize) as *mut f64;
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
            return 0 as i32;
        }
    }
    let mut n_comp: i32 = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: f64 = 0.;
    let mut fac1j: f64 = 0.;
    let mut fac1k: f64 = 0.;
    let mut ip: i32 = 0;
    let mut jp: i32 = 0;
    let mut kp: i32 = 0;
    let mut _empty: [i32; 4] = [
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    let mut iempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(0 as i32 as isize);
    let mut jempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(1 as i32 as isize);
    let mut kempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(2 as i32 as isize);
    let mut gempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(3 as i32 as isize);
    let mut non0ctri: *mut i32 = *((*opt).non0ctr).offset(i_sh as isize);
    let mut non0ctrj: *mut i32 = *((*opt).non0ctr).offset(j_sh as isize);
    let mut non0idxi: *mut i32 = *((*opt).sortedidx).offset(i_sh as isize);
    let mut non0idxj: *mut i32 = *((*opt).sortedidx).offset(j_sh as isize);
    let mut non0ctrk: *mut i32 = 0 as *mut i32;
    let mut non0idxk: *mut i32 = 0 as *mut i32;
    non0ctrk = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut i32;
    cache = non0ctrk.offset((k_prim + k_prim * k_ctr) as isize) as *mut f64;
    non0idxk = non0ctrk.offset(k_prim as isize);
    CINTOpt_non0coeff_byshell(non0idxk, non0ctrk, ck, k_prim, k_ctr);
    let mut expij: f64 = 0.;
    let mut cutoff: f64 = 0.;
    let mut rij: *mut f64 = 0 as *mut f64;
    let mut rkl: *mut f64 = ((*envs).rkl).as_mut_ptr();
    let mut idx: *mut i32 = *((*opt).index_xyz_array)
        .offset(
            ((*envs).i_l * 16 as i32 * 16 as i32
                + (*envs).j_l * 16 as i32 + (*envs).k_l) as isize,
        );
    if idx.is_null() {
        idx = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
            & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut i32;
        cache = idx.offset(nf.wrapping_mul(3 as i32 as libc::c_ulong) as isize)
            as *mut f64;
        CINTg2e_index_xyz(idx, envs);
    }
    let mut omega: f64 = *env.offset(8 as i32 as isize);
    if omega < 0 as i32 as f64 && (*envs).rys_order > 1 as i32
    {
        let mut r_guess: f64 = 8.0f64;
        let mut omega2: f64 = omega * omega;
        let mut lij: i32 = (*envs).li_ceil + (*envs).lj_ceil;
        if lij > 0 as i32 {
            let mut dist_ij: f64 = sqrt(rr_ij);
            let mut aij: f64 = *ai
                .offset((i_prim - 1 as i32) as isize)
                + *aj.offset((j_prim - 1 as i32) as isize);
            let mut theta: f64 = omega2 / (omega2 + aij);
            expcutoff
                += lij as f64
                    * log((dist_ij + theta * r_guess + 1.0f64) / (dist_ij + 1.0f64));
        }
        if (*envs).lk_ceil > 0 as i32 {
            let mut theta_0: f64 = omega2
                / (omega2 + *ak.offset((k_prim - 1 as i32) as isize));
            expcutoff
                += (*envs).lk_ceil as f64 * log(theta_0 * r_guess + 1.0f64);
        }
    }
    let mut nc: i32 = i_ctr;
    let mut leng: size_t = ((*envs).g_size * 3 as i32
        * (((1 as i32) << (*envs).gbits) + 1 as i32)) as size_t;
    let mut leni: size_t = nf
        .wrapping_mul(i_ctr as libc::c_ulong)
        .wrapping_mul(n_comp as libc::c_ulong);
    let mut len0: size_t = nf.wrapping_mul(n_comp as libc::c_ulong);
    let mut len: size_t = leng.wrapping_add(leni).wrapping_add(len0);
    let mut g: *mut f64 = 0 as *mut f64;
    g = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut f64;
    cache = g.offset(len as isize);
    let mut g1: *mut f64 = g.offset(leng as isize);
    let mut gout: *mut f64 = 0 as *mut f64;
    let mut gctri: *mut f64 = 0 as *mut f64;
    if n_comp == 1 as i32 {
        gctri = gctr;
        iempty = empty;
    } else {
        gctri = g1;
        g1 = g1.offset(leni as isize);
    }
    gout = g1;
    kp = 0 as i32;
    while kp < k_prim {
        (*envs).ak[0 as i32 as usize] = *ak.offset(kp as isize);
        fac1k = (*envs).common_factor * *ck.offset(kp as isize);
        pdata_ij = pdata_base;
        jp = 0 as i32;
        while jp < j_prim {
            (*envs).aj[0 as i32 as usize] = *aj.offset(jp as isize);
            fac1j = fac1k * *cj.offset(jp as isize);
            ip = 0 as i32;
            while ip < i_prim {
                if !((*pdata_ij).cceij > expcutoff) {
                    (*envs).ai[0 as i32 as usize] = *ai.offset(ip as isize);
                    expij = (*pdata_ij).eij;
                    rij = ((*pdata_ij).rij).as_mut_ptr();
                    cutoff = expcutoff - (*pdata_ij).cceij;
                    fac1i = fac1j * expij;
                    (*envs).fac[0 as i32 as usize] = fac1i;
                    if ::core::mem::transmute::<
                        _,
                        fn(_, _, _, _, _) -> i32,
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
                        )(gout, g, idx, envs, 1 as i32);
                        if i_ctr > 1 as i32 {
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
                        *iempty = 0 as i32;
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
    if n_comp > 1 as i32 && *iempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gctri,
                nf.wrapping_mul(nc as libc::c_ulong) as i32,
                n_comp,
            );
        } else {
            CINTdplus_transpose(
                gctr,
                gctri,
                nf.wrapping_mul(nc as libc::c_ulong) as i32,
                n_comp,
            );
        }
        *empty = 0 as i32;
    }
    return (*empty == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn CINT3c2e_1n1_loop(
    mut gctr: *mut f64,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut f64,
    mut empty: *mut i32,
) -> i32 {
    let mut shls: *mut i32 = (*envs).shls;
    let mut bas: *mut i32 = (*envs).bas;
    let mut env: *mut f64 = (*envs).env;
    let mut i_sh: i32 = *shls.offset(0 as i32 as isize);
    let mut j_sh: i32 = *shls.offset(1 as i32 as isize);
    let mut opt: *mut CINTOpt = (*envs).opt;
    if !((*opt).pairdata).is_null()
        && *((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize)
            == 0xffffffffffffffff as libc::c_ulong as *mut libc::c_void as *mut PairData
    {
        return 0 as i32;
    }
    let mut k_sh: i32 = *shls.offset(2 as i32 as isize);
    let mut i_ctr: i32 = (*envs).x_ctr[0 as i32 as usize];
    let mut j_ctr: i32 = (*envs).x_ctr[1 as i32 as usize];
    let mut k_ctr: i32 = (*envs).x_ctr[2 as i32 as usize];
    let mut i_prim: i32 = *bas
        .offset((8 as i32 * i_sh + 2 as i32) as isize);
    let mut j_prim: i32 = *bas
        .offset((8 as i32 * j_sh + 2 as i32) as isize);
    let mut k_prim: i32 = *bas
        .offset((8 as i32 * k_sh + 2 as i32) as isize);
    let mut ai: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * i_sh + 5 as i32) as isize) as isize,
        );
    let mut aj: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * j_sh + 5 as i32) as isize) as isize,
        );
    let mut ak: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * k_sh + 5 as i32) as isize) as isize,
        );
    let mut ci: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * i_sh + 6 as i32) as isize) as isize,
        );
    let mut cj: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * j_sh + 6 as i32) as isize) as isize,
        );
    let mut ck: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * k_sh + 6 as i32) as isize) as isize,
        );
    let mut expcutoff: f64 = (*envs).expcutoff;
    let mut rr_ij: f64 = (*envs).rirj[0 as i32 as usize]
        * (*envs).rirj[0 as i32 as usize]
        + (*envs).rirj[1 as i32 as usize]
            * (*envs).rirj[1 as i32 as usize]
        + (*envs).rirj[2 as i32 as usize]
            * (*envs).rirj[2 as i32 as usize];
    let mut pdata_base: *mut PairData = 0 as *mut PairData;
    let mut pdata_ij: *mut PairData = 0 as *mut PairData;
    if !((*opt).pairdata).is_null() {
        pdata_base = *((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize);
    } else {
        let mut log_maxci: *mut f64 = *((*opt).log_max_coeff)
            .offset(i_sh as isize);
        let mut log_maxcj: *mut f64 = *((*opt).log_max_coeff)
            .offset(j_sh as isize);
        pdata_base = ((cache as uintptr_t)
            .wrapping_add(7 as i32 as libc::c_ulong)
            & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut PairData;
        cache = pdata_base.offset((i_prim * j_prim) as isize) as *mut f64;
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
            return 0 as i32;
        }
    }
    let mut n_comp: i32 = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: f64 = 0.;
    let mut fac1j: f64 = 0.;
    let mut fac1k: f64 = 0.;
    let mut ip: i32 = 0;
    let mut jp: i32 = 0;
    let mut kp: i32 = 0;
    let mut _empty: [i32; 4] = [
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    let mut iempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(0 as i32 as isize);
    let mut jempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(1 as i32 as isize);
    let mut kempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(2 as i32 as isize);
    let mut gempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(3 as i32 as isize);
    let mut non0ctri: *mut i32 = *((*opt).non0ctr).offset(i_sh as isize);
    let mut non0ctrj: *mut i32 = *((*opt).non0ctr).offset(j_sh as isize);
    let mut non0idxi: *mut i32 = *((*opt).sortedidx).offset(i_sh as isize);
    let mut non0idxj: *mut i32 = *((*opt).sortedidx).offset(j_sh as isize);
    let mut non0ctrk: *mut i32 = 0 as *mut i32;
    let mut non0idxk: *mut i32 = 0 as *mut i32;
    non0ctrk = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut i32;
    cache = non0ctrk.offset((k_prim + k_prim * k_ctr) as isize) as *mut f64;
    non0idxk = non0ctrk.offset(k_prim as isize);
    CINTOpt_non0coeff_byshell(non0idxk, non0ctrk, ck, k_prim, k_ctr);
    let mut expij: f64 = 0.;
    let mut cutoff: f64 = 0.;
    let mut rij: *mut f64 = 0 as *mut f64;
    let mut rkl: *mut f64 = ((*envs).rkl).as_mut_ptr();
    let mut idx: *mut i32 = *((*opt).index_xyz_array)
        .offset(
            ((*envs).i_l * 16 as i32 * 16 as i32
                + (*envs).j_l * 16 as i32 + (*envs).k_l) as isize,
        );
    if idx.is_null() {
        idx = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
            & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut i32;
        cache = idx.offset(nf.wrapping_mul(3 as i32 as libc::c_ulong) as isize)
            as *mut f64;
        CINTg2e_index_xyz(idx, envs);
    }
    let mut omega: f64 = *env.offset(8 as i32 as isize);
    if omega < 0 as i32 as f64 && (*envs).rys_order > 1 as i32
    {
        let mut r_guess: f64 = 8.0f64;
        let mut omega2: f64 = omega * omega;
        let mut lij: i32 = (*envs).li_ceil + (*envs).lj_ceil;
        if lij > 0 as i32 {
            let mut dist_ij: f64 = sqrt(rr_ij);
            let mut aij: f64 = *ai
                .offset((i_prim - 1 as i32) as isize)
                + *aj.offset((j_prim - 1 as i32) as isize);
            let mut theta: f64 = omega2 / (omega2 + aij);
            expcutoff
                += lij as f64
                    * log((dist_ij + theta * r_guess + 1.0f64) / (dist_ij + 1.0f64));
        }
        if (*envs).lk_ceil > 0 as i32 {
            let mut theta_0: f64 = omega2
                / (omega2 + *ak.offset((k_prim - 1 as i32) as isize));
            expcutoff
                += (*envs).lk_ceil as f64 * log(theta_0 * r_guess + 1.0f64);
        }
    }
    let mut nc: i32 = j_ctr;
    let mut leng: size_t = ((*envs).g_size * 3 as i32
        * (((1 as i32) << (*envs).gbits) + 1 as i32)) as size_t;
    let mut lenj: size_t = nf
        .wrapping_mul(j_ctr as libc::c_ulong)
        .wrapping_mul(n_comp as libc::c_ulong);
    let mut len0: size_t = nf.wrapping_mul(n_comp as libc::c_ulong);
    let mut len: size_t = leng.wrapping_add(lenj).wrapping_add(len0);
    let mut g: *mut f64 = 0 as *mut f64;
    g = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut f64;
    cache = g.offset(len as isize);
    let mut g1: *mut f64 = g.offset(leng as isize);
    let mut gout: *mut f64 = 0 as *mut f64;
    let mut gctrj: *mut f64 = 0 as *mut f64;
    if n_comp == 1 as i32 {
        gctrj = gctr;
        jempty = empty;
    } else {
        gctrj = g1;
        g1 = g1.offset(lenj as isize);
    }
    gout = g1;
    kp = 0 as i32;
    while kp < k_prim {
        (*envs).ak[0 as i32 as usize] = *ak.offset(kp as isize);
        fac1k = (*envs).common_factor * *ck.offset(kp as isize);
        pdata_ij = pdata_base;
        jp = 0 as i32;
        while jp < j_prim {
            (*envs).aj[0 as i32 as usize] = *aj.offset(jp as isize);
            fac1j = fac1k;
            *iempty = 1 as i32;
            ip = 0 as i32;
            while ip < i_prim {
                if !((*pdata_ij).cceij > expcutoff) {
                    (*envs).ai[0 as i32 as usize] = *ai.offset(ip as isize);
                    expij = (*pdata_ij).eij;
                    rij = ((*pdata_ij).rij).as_mut_ptr();
                    cutoff = expcutoff - (*pdata_ij).cceij;
                    fac1i = fac1j * *ci.offset(ip as isize) * expij;
                    (*envs).fac[0 as i32 as usize] = fac1i;
                    if ::core::mem::transmute::<
                        _,
                        fn(_, _, _, _, _) -> i32,
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
                        *iempty = 0 as i32;
                    }
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
                *jempty = 0 as i32;
            }
            jp += 1;
            jp;
        }
        kp += 1;
        kp;
    }
    if n_comp > 1 as i32 && *jempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gctrj,
                nf.wrapping_mul(nc as libc::c_ulong) as i32,
                n_comp,
            );
        } else {
            CINTdplus_transpose(
                gctr,
                gctrj,
                nf.wrapping_mul(nc as libc::c_ulong) as i32,
                n_comp,
            );
        }
        *empty = 0 as i32;
    }
    return (*empty == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn CINT3c2e_loop(
    mut gctr: *mut f64,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut f64,
    mut empty: *mut i32,
) -> i32 {
    let mut shls: *mut i32 = (*envs).shls;
    let mut bas: *mut i32 = (*envs).bas;
    let mut env: *mut f64 = (*envs).env;
    let mut i_sh: i32 = *shls.offset(0 as i32 as isize);
    let mut j_sh: i32 = *shls.offset(1 as i32 as isize);
    let mut opt: *mut CINTOpt = (*envs).opt;
    if !((*opt).pairdata).is_null()
        && *((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize)
            == 0xffffffffffffffff as libc::c_ulong as *mut libc::c_void as *mut PairData
    {
        return 0 as i32;
    }
    let mut k_sh: i32 = *shls.offset(2 as i32 as isize);
    let mut i_ctr: i32 = (*envs).x_ctr[0 as i32 as usize];
    let mut j_ctr: i32 = (*envs).x_ctr[1 as i32 as usize];
    let mut k_ctr: i32 = (*envs).x_ctr[2 as i32 as usize];
    let mut i_prim: i32 = *bas
        .offset((8 as i32 * i_sh + 2 as i32) as isize);
    let mut j_prim: i32 = *bas
        .offset((8 as i32 * j_sh + 2 as i32) as isize);
    let mut k_prim: i32 = *bas
        .offset((8 as i32 * k_sh + 2 as i32) as isize);
    let mut ai: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * i_sh + 5 as i32) as isize) as isize,
        );
    let mut aj: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * j_sh + 5 as i32) as isize) as isize,
        );
    let mut ak: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * k_sh + 5 as i32) as isize) as isize,
        );
    let mut ci: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * i_sh + 6 as i32) as isize) as isize,
        );
    let mut cj: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * j_sh + 6 as i32) as isize) as isize,
        );
    let mut ck: *mut f64 = env
        .offset(
            *bas.offset((8 as i32 * k_sh + 6 as i32) as isize) as isize,
        );
    let mut expcutoff: f64 = (*envs).expcutoff;
    let mut rr_ij: f64 = (*envs).rirj[0 as i32 as usize]
        * (*envs).rirj[0 as i32 as usize]
        + (*envs).rirj[1 as i32 as usize]
            * (*envs).rirj[1 as i32 as usize]
        + (*envs).rirj[2 as i32 as usize]
            * (*envs).rirj[2 as i32 as usize];
    let mut pdata_base: *mut PairData = 0 as *mut PairData;
    let mut pdata_ij: *mut PairData = 0 as *mut PairData;
    if !((*opt).pairdata).is_null() {
        pdata_base = *((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize);
    } else {
        let mut log_maxci: *mut f64 = *((*opt).log_max_coeff)
            .offset(i_sh as isize);
        let mut log_maxcj: *mut f64 = *((*opt).log_max_coeff)
            .offset(j_sh as isize);
        pdata_base = ((cache as uintptr_t)
            .wrapping_add(7 as i32 as libc::c_ulong)
            & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut PairData;
        cache = pdata_base.offset((i_prim * j_prim) as isize) as *mut f64;
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
            return 0 as i32;
        }
    }
    let mut n_comp: i32 = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: f64 = 0.;
    let mut fac1j: f64 = 0.;
    let mut fac1k: f64 = 0.;
    let mut ip: i32 = 0;
    let mut jp: i32 = 0;
    let mut kp: i32 = 0;
    let mut _empty: [i32; 4] = [
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    let mut iempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(0 as i32 as isize);
    let mut jempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(1 as i32 as isize);
    let mut kempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(2 as i32 as isize);
    let mut gempty: *mut i32 = _empty
        .as_mut_ptr()
        .offset(3 as i32 as isize);
    let mut non0ctri: *mut i32 = *((*opt).non0ctr).offset(i_sh as isize);
    let mut non0ctrj: *mut i32 = *((*opt).non0ctr).offset(j_sh as isize);
    let mut non0idxi: *mut i32 = *((*opt).sortedidx).offset(i_sh as isize);
    let mut non0idxj: *mut i32 = *((*opt).sortedidx).offset(j_sh as isize);
    let mut non0ctrk: *mut i32 = 0 as *mut i32;
    let mut non0idxk: *mut i32 = 0 as *mut i32;
    non0ctrk = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut i32;
    cache = non0ctrk.offset((k_prim + k_prim * k_ctr) as isize) as *mut f64;
    non0idxk = non0ctrk.offset(k_prim as isize);
    CINTOpt_non0coeff_byshell(non0idxk, non0ctrk, ck, k_prim, k_ctr);
    let mut expij: f64 = 0.;
    let mut cutoff: f64 = 0.;
    let mut rij: *mut f64 = 0 as *mut f64;
    let mut rkl: *mut f64 = ((*envs).rkl).as_mut_ptr();
    let mut idx: *mut i32 = *((*opt).index_xyz_array)
        .offset(
            ((*envs).i_l * 16 as i32 * 16 as i32
                + (*envs).j_l * 16 as i32 + (*envs).k_l) as isize,
        );
    if idx.is_null() {
        idx = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
            & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut i32;
        cache = idx.offset(nf.wrapping_mul(3 as i32 as libc::c_ulong) as isize)
            as *mut f64;
        CINTg2e_index_xyz(idx, envs);
    }
    let mut omega: f64 = *env.offset(8 as i32 as isize);
    if omega < 0 as i32 as f64 && (*envs).rys_order > 1 as i32
    {
        let mut r_guess: f64 = 8.0f64;
        let mut omega2: f64 = omega * omega;
        let mut lij: i32 = (*envs).li_ceil + (*envs).lj_ceil;
        if lij > 0 as i32 {
            let mut dist_ij: f64 = sqrt(rr_ij);
            let mut aij: f64 = *ai
                .offset((i_prim - 1 as i32) as isize)
                + *aj.offset((j_prim - 1 as i32) as isize);
            let mut theta: f64 = omega2 / (omega2 + aij);
            expcutoff
                += lij as f64
                    * log((dist_ij + theta * r_guess + 1.0f64) / (dist_ij + 1.0f64));
        }
        if (*envs).lk_ceil > 0 as i32 {
            let mut theta_0: f64 = omega2
                / (omega2 + *ak.offset((k_prim - 1 as i32) as isize));
            expcutoff
                += (*envs).lk_ceil as f64 * log(theta_0 * r_guess + 1.0f64);
        }
    }
    let mut nc: i32 = i_ctr * j_ctr * k_ctr;
    let mut leng: size_t = ((*envs).g_size * 3 as i32
        * (((1 as i32) << (*envs).gbits) + 1 as i32)) as size_t;
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
    let mut g: *mut f64 = 0 as *mut f64;
    g = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut f64;
    cache = g.offset(len as isize);
    let mut g1: *mut f64 = g.offset(leng as isize);
    let mut gout: *mut f64 = 0 as *mut f64;
    let mut gctri: *mut f64 = 0 as *mut f64;
    let mut gctrj: *mut f64 = 0 as *mut f64;
    let mut gctrk: *mut f64 = 0 as *mut f64;
    if n_comp == 1 as i32 {
        gctrk = gctr;
        kempty = empty;
    } else {
        gctrk = g1;
        g1 = g1.offset(lenk as isize);
    }
    if k_ctr == 1 as i32 {
        gctrj = gctrk;
        jempty = kempty;
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
        g1 = g1.offset(leng as isize);
    }
    kp = 0 as i32;
    while kp < k_prim {
        (*envs).ak[0 as i32 as usize] = *ak.offset(kp as isize);
        if k_ctr == 1 as i32 {
            fac1k = (*envs).common_factor * *ck.offset(kp as isize);
        } else {
            fac1k = (*envs).common_factor;
            *jempty = 1 as i32;
        }
        pdata_ij = pdata_base;
        jp = 0 as i32;
        while jp < j_prim {
            (*envs).aj[0 as i32 as usize] = *aj.offset(jp as isize);
            if j_ctr == 1 as i32 {
                fac1j = fac1k * *cj.offset(jp as isize);
            } else {
                fac1j = fac1k;
                *iempty = 1 as i32;
            }
            ip = 0 as i32;
            while ip < i_prim {
                if !((*pdata_ij).cceij > expcutoff) {
                    (*envs).ai[0 as i32 as usize] = *ai.offset(ip as isize);
                    expij = (*pdata_ij).eij;
                    rij = ((*pdata_ij).rij).as_mut_ptr();
                    cutoff = expcutoff - (*pdata_ij).cceij;
                    if i_ctr == 1 as i32 {
                        fac1i = fac1j * *ci.offset(ip as isize) * expij;
                    } else {
                        fac1i = fac1j * expij;
                    }
                    (*envs).fac[0 as i32 as usize] = fac1i;
                    if ::core::mem::transmute::<
                        _,
                        fn(_, _, _, _, _) -> i32,
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
                        if i_ctr > 1 as i32 {
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
                        *iempty = 0 as i32;
                    }
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
                *jempty = 0 as i32;
            }
            jp += 1;
            jp;
        }
        if *jempty == 0 {
            if k_ctr > 1 as i32 {
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
static mut CINTf_3c2e_loop: [Option::<
    unsafe extern "C" fn(
        *mut f64,
        *mut CINTEnvVars,
        *mut f64,
        *mut i32,
    ) -> i32,
>; 8] = unsafe {
    [
        Some(
            CINT3c2e_loop
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut CINTEnvVars,
                    *mut f64,
                    *mut i32,
                ) -> i32,
        ),
        Some(
            CINT3c2e_loop
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut CINTEnvVars,
                    *mut f64,
                    *mut i32,
                ) -> i32,
        ),
        Some(
            CINT3c2e_loop
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut CINTEnvVars,
                    *mut f64,
                    *mut i32,
                ) -> i32,
        ),
        Some(
            CINT3c2e_n11_loop
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut CINTEnvVars,
                    *mut f64,
                    *mut i32,
                ) -> i32,
        ),
        Some(
            CINT3c2e_loop
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut CINTEnvVars,
                    *mut f64,
                    *mut i32,
                ) -> i32,
        ),
        Some(
            CINT3c2e_1n1_loop
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut CINTEnvVars,
                    *mut f64,
                    *mut i32,
                ) -> i32,
        ),
        Some(
            CINT3c2e_loop
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut CINTEnvVars,
                    *mut f64,
                    *mut i32,
                ) -> i32,
        ),
        Some(
            CINT3c2e_111_loop
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut CINTEnvVars,
                    *mut f64,
                    *mut i32,
                ) -> i32,
        ),
    ]
};
#[no_mangle]
pub unsafe extern "C" fn CINT3c2e_drv(
    mut out: *mut f64,
    mut dims: *mut i32,
    mut envs: *mut CINTEnvVars,
    mut opt: *mut CINTOpt,
    mut cache: *mut f64,
    mut f_e1_c2s: Option::<unsafe extern "C" fn() -> ()>,
    mut is_ssc: i32,
) -> i32 {
    let mut x_ctr: *mut i32 = ((*envs).x_ctr).as_mut_ptr();
    let mut nc: size_t = ((*envs).nf * *x_ctr.offset(0 as i32 as isize)
        * *x_ctr.offset(1 as i32 as isize)
        * *x_ctr.offset(2 as i32 as isize)) as size_t;
    let mut n_comp: i32 = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    if out.is_null() {
        let mut bas: *mut i32 = (*envs).bas;
        let mut shls: *mut i32 = (*envs).shls;
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
        let mut k_prim: i32 = *bas
            .offset(
                (8 as i32 * *shls.offset(2 as i32 as isize)
                    + 2 as i32) as isize,
            );
        let mut pdata_size: i32 = i_prim * j_prim * 5 as i32
            + i_prim * *x_ctr.offset(0 as i32 as isize)
            + j_prim * *x_ctr.offset(1 as i32 as isize)
            + k_prim * *x_ctr.offset(2 as i32 as isize)
            + (i_prim + j_prim) * 2 as i32 + k_prim
            + (*envs).nf * 3 as i32 + 16 as i32;
        let mut leng: i32 = (*envs).g_size * 3 as i32
            * (((1 as i32) << (*envs).gbits) + 1 as i32);
        let mut len0: i32 = (*envs).nf * n_comp;
        let mut cache_size: i32 = (if ((leng + len0) as libc::c_ulong)
            .wrapping_add(
                nc
                    .wrapping_mul(n_comp as libc::c_ulong)
                    .wrapping_mul(3 as i32 as libc::c_ulong),
            )
            .wrapping_add(pdata_size as libc::c_ulong)
            > nc
                .wrapping_mul(n_comp as libc::c_ulong)
                .wrapping_add(((*envs).nf * 3 as i32) as libc::c_ulong)
        {
            ((leng + len0) as libc::c_ulong)
                .wrapping_add(
                    nc
                        .wrapping_mul(n_comp as libc::c_ulong)
                        .wrapping_mul(3 as i32 as libc::c_ulong),
                )
                .wrapping_add(pdata_size as libc::c_ulong)
        } else {
            nc.wrapping_mul(n_comp as libc::c_ulong)
                .wrapping_add(((*envs).nf * 3 as i32) as libc::c_ulong)
        }) as i32;
        return cache_size;
    }
    let mut stack: *mut f64 = 0 as *mut f64;
    if cache.is_null() {
        let mut bas_0: *mut i32 = (*envs).bas;
        let mut shls_0: *mut i32 = (*envs).shls;
        let mut i_prim_0: i32 = *bas_0
            .offset(
                (8 as i32 * *shls_0.offset(0 as i32 as isize)
                    + 2 as i32) as isize,
            );
        let mut j_prim_0: i32 = *bas_0
            .offset(
                (8 as i32 * *shls_0.offset(1 as i32 as isize)
                    + 2 as i32) as isize,
            );
        let mut k_prim_0: i32 = *bas_0
            .offset(
                (8 as i32 * *shls_0.offset(2 as i32 as isize)
                    + 2 as i32) as isize,
            );
        let mut pdata_size_0: i32 = i_prim_0 * j_prim_0 * 5 as i32
            + i_prim_0 * *x_ctr.offset(0 as i32 as isize)
            + j_prim_0 * *x_ctr.offset(1 as i32 as isize)
            + k_prim_0 * *x_ctr.offset(2 as i32 as isize)
            + (i_prim_0 + j_prim_0) * 2 as i32 + k_prim_0
            + (*envs).nf * 3 as i32 + 16 as i32;
        let mut leng_0: size_t = ((*envs).g_size * 3 as i32
            * (((1 as i32) << (*envs).gbits) + 1 as i32)) as size_t;
        let mut len0_0: size_t = ((*envs).nf * n_comp) as size_t;
        let mut cache_size_0: size_t = if leng_0
            .wrapping_add(len0_0)
            .wrapping_add(
                nc
                    .wrapping_mul(n_comp as libc::c_ulong)
                    .wrapping_mul(3 as i32 as libc::c_ulong),
            )
            .wrapping_add(pdata_size_0 as libc::c_ulong)
            > nc
                .wrapping_mul(n_comp as libc::c_ulong)
                .wrapping_add(((*envs).nf * 3 as i32) as libc::c_ulong)
        {
            leng_0
                .wrapping_add(len0_0)
                .wrapping_add(
                    nc
                        .wrapping_mul(n_comp as libc::c_ulong)
                        .wrapping_mul(3 as i32 as libc::c_ulong),
                )
                .wrapping_add(pdata_size_0 as libc::c_ulong)
        } else {
            nc.wrapping_mul(n_comp as libc::c_ulong)
                .wrapping_add(((*envs).nf * 3 as i32) as libc::c_ulong)
        };
        stack = malloc(
            (::core::mem::size_of::<f64>() as libc::c_ulong)
                .wrapping_mul(cache_size_0),
        ) as *mut f64;
        cache = stack;
    }
    let mut gctr: *mut f64 = 0 as *mut f64;
    gctr = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut f64;
    cache = gctr.offset(nc.wrapping_mul(n_comp as libc::c_ulong) as isize);
    let mut n: i32 = 0;
    let mut empty: i32 = 1 as i32;
    if !opt.is_null() {
        (*envs).opt = opt;
        n = ((((*envs).x_ctr[0 as i32 as usize] == 1 as i32)
            as i32) << 2 as i32)
            + ((((*envs).x_ctr[1 as i32 as usize] == 1 as i32)
                as i32) << 1 as i32)
            + ((*envs).x_ctr[2 as i32 as usize] == 1 as i32)
                as i32;
        (CINTf_3c2e_loop[n as usize])
            .expect("non-null function pointer")(gctr, envs, cache, &mut empty);
    } else {
        CINT3c2e_loop_nopt(gctr, envs, cache, &mut empty);
    }
    let mut counts: [i32; 4] = [0; 4];
    if f_e1_c2s
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
                c2s_sph_3c2e1
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
        if is_ssc != 0 {
            counts[2 as i32
                as usize] = (*envs).c2rust_unnamed.nfk
                * *x_ctr.offset(2 as i32 as isize);
        } else {
            counts[2 as i32
                as usize] = ((*envs).k_l * 2 as i32 + 1 as i32)
                * *x_ctr.offset(2 as i32 as isize);
        }
    } else {
        counts[0 as i32
            as usize] = (*envs).nfi * *x_ctr.offset(0 as i32 as isize);
        counts[1 as i32
            as usize] = (*envs).nfj * *x_ctr.offset(1 as i32 as isize);
        counts[2 as i32
            as usize] = (*envs).c2rust_unnamed.nfk
            * *x_ctr.offset(2 as i32 as isize);
    }
    counts[3 as i32 as usize] = 1 as i32;
    if dims.is_null() {
        dims = counts.as_mut_ptr();
    }
    let mut nout: i32 = *dims.offset(0 as i32 as isize)
        * *dims.offset(1 as i32 as isize)
        * *dims.offset(2 as i32 as isize);
    if empty == 0 {
        n = 0 as i32;
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
pub unsafe extern "C" fn int3c2e_sph(
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
    return CINT3c2e_drv(
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
                c2s_sph_3c2e1
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
pub unsafe extern "C" fn int3c2e_optimizer(
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
    CINTall_3c2e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int3c2e_cart(
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
    return CINT3c2e_drv(
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
                c2s_cart_3c2e1
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
pub unsafe extern "C" fn int3c2e_sph_ssc(
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
    return CINT3c2e_drv(
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
                c2s_sph_3c2e1_ssc
                    as unsafe extern "C" fn(
                        *mut f64,
                        *mut f64,
                        *mut i32,
                        *mut CINTEnvVars,
                        *mut f64,
                    ) -> (),
            ),
        ),
        1 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int3c2e_ssc_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
) {
    int3c2e_optimizer(opt, atm, natm, bas, nbas, env);
}
