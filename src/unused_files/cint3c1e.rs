#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::g1e::CINTnuc_mod;
use crate::g1e::CINTprim_to_ctr_0;
use crate::g1e::CINTprim_to_ctr_1;
use crate::g2e::CINTg2e_index_xyz;
use crate::g3c1e::CINTinit_int3c1e_EnvVars;
use crate::g3c1e::CINTg3c1e_ovlp;
use crate::g3c1e::CINTg3c1e_ovlp;
use crate::g3c1e::CINTg3c1e_nuc;
use crate::optimizer::CINTOpt_non0coeff_byshell;
use crate::misc::CINTsquare_dist;
use crate::fblas::CINTdmat_transpose;
use crate::fblas::CINTdplus_transpose;
use crate::cart2sph::c2s_sph_3c2e1;
use crate::cart2sph::c2s_sph_3c1e;
use crate::cart2sph::c2s_cart_3c1e;
use crate::cart2sph::c2s_dset0;
use crate::rys_roots::CINTrys_roots;
use crate::g1e_grids::CINTgout1e;

use crate::cint::CINTOpt;
use crate::cint::CINTEnvVars;


pub type size_t = libc::c_ulong;
pub type uintptr_t = libc::c_ulong;

extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abs(_: i32) -> i32;
    fn exp(_: f64) -> f64;
    fn sqrt(_: f64) -> f64;
}

#[no_mangle]
pub unsafe extern "C" fn CINT3c1e_loop_nopt(
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
    let mut ri: *mut f64 = (*envs).ri;
    let mut rj: *mut f64 = (*envs).rj;
    let mut rk: *mut f64 = (*envs).rk;
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
    let mut idx: *mut i32 = 0 as *mut i32;
    idx = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut i32;
    cache = idx.offset(((*envs).nf * 3 as i32) as isize) as *mut f64;
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
    let mut lenk: size_t = ((*envs).nf * nc * n_comp) as size_t;
    let mut lenj: size_t = ((*envs).nf * i_ctr * j_ctr * n_comp) as size_t;
    let mut leni: size_t = ((*envs).nf * i_ctr * n_comp) as size_t;
    let mut len0: size_t = ((*envs).nf * n_comp) as size_t;
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
    let mut eijk: f64 = 0.;
    let mut dijk: f64 = 0.;
    let mut aijk: f64 = 0.;
    let mut aiajrr: f64 = 0.;
    let mut aiakrr: f64 = 0.;
    let mut ajakrr: f64 = 0.;
    let mut rirk: [f64; 3] = [0.; 3];
    let mut rjrk: [f64; 3] = [0.; 3];
    rirk[0 as i32
        as usize] = *ri.offset(0 as i32 as isize)
        - *rk.offset(0 as i32 as isize);
    rirk[1 as i32
        as usize] = *ri.offset(1 as i32 as isize)
        - *rk.offset(1 as i32 as isize);
    rirk[2 as i32
        as usize] = *ri.offset(2 as i32 as isize)
        - *rk.offset(2 as i32 as isize);
    rjrk[0 as i32
        as usize] = *rj.offset(0 as i32 as isize)
        - *rk.offset(0 as i32 as isize);
    rjrk[1 as i32
        as usize] = *rj.offset(1 as i32 as isize)
        - *rk.offset(1 as i32 as isize);
    rjrk[2 as i32
        as usize] = *rj.offset(2 as i32 as isize)
        - *rk.offset(2 as i32 as isize);
    let mut rr_ij: f64 = (*envs).rirj[0 as i32 as usize]
        * (*envs).rirj[0 as i32 as usize]
        + (*envs).rirj[1 as i32 as usize]
            * (*envs).rirj[1 as i32 as usize]
        + (*envs).rirj[2 as i32 as usize]
            * (*envs).rirj[2 as i32 as usize];
    let mut rr_ik: f64 = rirk[0 as i32 as usize]
        * rirk[0 as i32 as usize]
        + rirk[1 as i32 as usize] * rirk[1 as i32 as usize]
        + rirk[2 as i32 as usize] * rirk[2 as i32 as usize];
    let mut rr_jk: f64 = rjrk[0 as i32 as usize]
        * rjrk[0 as i32 as usize]
        + rjrk[1 as i32 as usize] * rjrk[1 as i32 as usize]
        + rjrk[2 as i32 as usize] * rjrk[2 as i32 as usize];
    kp = 0 as i32;
    while kp < k_prim {
        (*envs).ak[0 as i32 as usize] = *ak.offset(kp as isize);
        if k_ctr == 1 as i32 {
            fac1k = (*envs).common_factor * *ck.offset(kp as isize);
        } else {
            fac1k = (*envs).common_factor;
            *jempty = 1 as i32;
        }
        jp = 0 as i32;
        while jp < j_prim {
            (*envs).aj[0 as i32 as usize] = *aj.offset(jp as isize);
            if j_ctr == 1 as i32 {
                fac1j = fac1k * *cj.offset(jp as isize);
            } else {
                fac1j = fac1k;
                *iempty = 1 as i32;
            }
            ajakrr = *aj.offset(jp as isize) * *ak.offset(kp as isize) * rr_jk;
            ip = 0 as i32;
            while ip < i_prim {
                (*envs).ai[0 as i32 as usize] = *ai.offset(ip as isize);
                aijk = *ai.offset(ip as isize) + *aj.offset(jp as isize)
                    + *ak.offset(kp as isize);
                aiakrr = *ai.offset(ip as isize) * *ak.offset(kp as isize) * rr_ik;
                aiajrr = *ai.offset(ip as isize) * *aj.offset(jp as isize) * rr_ij;
                eijk = (aiajrr + aiakrr + ajakrr) / aijk;
                if !(eijk > 60 as i32 as f64) {
                    if i_ctr == 1 as i32 {
                        fac1i = fac1j * *ci.offset(ip as isize) * exp(-eijk);
                    } else {
                        fac1i = fac1j * exp(-eijk);
                    }
                    dijk = fac1i / (aijk * sqrt(aijk));
                    (*envs).fac[0 as i32 as usize] = dijk;
                    CINTg3c1e_ovlp(
                        g,
                        *ai.offset(ip as isize),
                        *aj.offset(jp as isize),
                        *ak.offset(kp as isize),
                        envs,
                    );
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
                ip += 1;
                ip;
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
pub unsafe extern "C" fn CINT3c1e_nuc_loop_nopt(
    mut gctr: *mut f64,
    mut envs: *mut CINTEnvVars,
    mut fac: f64,
    mut nuc_id: i32,
    mut cache: *mut f64,
    mut empty: *mut i32,
) -> i32 {
    let mut shls: *mut i32 = (*envs).shls;
    let mut atm: *mut i32 = (*envs).atm;
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
    let mut ri: *mut f64 = (*envs).ri;
    let mut rj: *mut f64 = (*envs).rj;
    let mut rk: *mut f64 = (*envs).rk;
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
    let mut n_comp: i32 = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: f64 = 0.;
    let mut fac1j: f64 = 0.;
    let mut fac1k: f64 = 0.;
    let mut i: i32 = 0;
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
    let mut rys_empty: i32 = 0;
    let mut idx: *mut i32 = 0 as *mut i32;
    idx = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
        & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut i32;
    cache = idx.offset(((*envs).nf * 3 as i32) as isize) as *mut f64;
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
    let mut cr: *mut f64 = 0 as *mut f64;
    let mut t2: f64 = 0.;
    let mut tau: f64 = 0.;
    let mut x: f64 = 0.;
    let mut u: [f64; 32] = [0.; 32];
    let mut w: [f64; 32] = [0.; 32];
    let mut nc: i32 = i_ctr * j_ctr * k_ctr;
    let mut leng: size_t = ((*envs).g_size * 3 as i32
        * (((1 as i32) << (*envs).gbits) + 1 as i32)) as size_t;
    let mut lenk: size_t = ((*envs).nf * nc * n_comp) as size_t;
    let mut lenj: size_t = ((*envs).nf * i_ctr * j_ctr * n_comp) as size_t;
    let mut leni: size_t = ((*envs).nf * i_ctr * n_comp) as size_t;
    let mut len0: size_t = ((*envs).nf * n_comp) as size_t;
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
    if nuc_id < 0 as i32 {
        cr = &mut *env.offset(4 as i32 as isize) as *mut f64;
    } else {
        cr = &mut *env
            .offset(
                *atm.offset((6 as i32 * nuc_id + 1 as i32) as isize)
                    as isize,
            ) as *mut f64;
    }
    let mut eijk: f64 = 0.;
    let mut dijk: f64 = 0.;
    let mut aijk: f64 = 0.;
    let mut aiajrr: f64 = 0.;
    let mut aiakrr: f64 = 0.;
    let mut ajakrr: f64 = 0.;
    let mut rirk: [f64; 3] = [0.; 3];
    let mut rjrk: [f64; 3] = [0.; 3];
    let mut rijk: [f64; 3] = [0.; 3];
    rirk[0 as i32
        as usize] = *ri.offset(0 as i32 as isize)
        - *rk.offset(0 as i32 as isize);
    rirk[1 as i32
        as usize] = *ri.offset(1 as i32 as isize)
        - *rk.offset(1 as i32 as isize);
    rirk[2 as i32
        as usize] = *ri.offset(2 as i32 as isize)
        - *rk.offset(2 as i32 as isize);
    rjrk[0 as i32
        as usize] = *rj.offset(0 as i32 as isize)
        - *rk.offset(0 as i32 as isize);
    rjrk[1 as i32
        as usize] = *rj.offset(1 as i32 as isize)
        - *rk.offset(1 as i32 as isize);
    rjrk[2 as i32
        as usize] = *rj.offset(2 as i32 as isize)
        - *rk.offset(2 as i32 as isize);
    let mut rr_ij: f64 = (*envs).rirj[0 as i32 as usize]
        * (*envs).rirj[0 as i32 as usize]
        + (*envs).rirj[1 as i32 as usize]
            * (*envs).rirj[1 as i32 as usize]
        + (*envs).rirj[2 as i32 as usize]
            * (*envs).rirj[2 as i32 as usize];
    let mut rr_ik: f64 = rirk[0 as i32 as usize]
        * rirk[0 as i32 as usize]
        + rirk[1 as i32 as usize] * rirk[1 as i32 as usize]
        + rirk[2 as i32 as usize] * rirk[2 as i32 as usize];
    let mut rr_jk: f64 = rjrk[0 as i32 as usize]
        * rjrk[0 as i32 as usize]
        + rjrk[1 as i32 as usize] * rjrk[1 as i32 as usize]
        + rjrk[2 as i32 as usize] * rjrk[2 as i32 as usize];
    fac *= (*envs).common_factor;
    kp = 0 as i32;
    while kp < k_prim {
        (*envs).ak[0 as i32 as usize] = *ak.offset(kp as isize);
        if k_ctr == 1 as i32 {
            fac1k = fac * *ck.offset(kp as isize);
        } else {
            fac1k = fac;
            *jempty = 1 as i32;
        }
        jp = 0 as i32;
        while jp < j_prim {
            (*envs).aj[0 as i32 as usize] = *aj.offset(jp as isize);
            if j_ctr == 1 as i32 {
                fac1j = fac1k * *cj.offset(jp as isize);
            } else {
                fac1j = fac1k;
                *iempty = 1 as i32;
            }
            ajakrr = *aj.offset(jp as isize) * *ak.offset(kp as isize) * rr_jk;
            ip = 0 as i32;
            while ip < i_prim {
                (*envs).ai[0 as i32 as usize] = *ai.offset(ip as isize);
                aijk = *ai.offset(ip as isize) + *aj.offset(jp as isize)
                    + *ak.offset(kp as isize);
                aiakrr = *ai.offset(ip as isize) * *ak.offset(kp as isize) * rr_ik;
                aiajrr = *ai.offset(ip as isize) * *aj.offset(jp as isize) * rr_ij;
                eijk = (aiajrr + aiakrr + ajakrr) / aijk;
                if !(eijk > 60 as i32 as f64) {
                    if i_ctr == 1 as i32 {
                        fac1i = fac1j * *ci.offset(ip as isize) * exp(-eijk);
                    } else {
                        fac1i = fac1j * exp(-eijk);
                    }
                    dijk = fac1i / aijk;
                    rijk[0 as i32
                        as usize] = (*ai.offset(ip as isize)
                        * *ri.offset(0 as i32 as isize)
                        + *aj.offset(jp as isize) * *rj.offset(0 as i32 as isize)
                        + *ak.offset(kp as isize)
                            * *rk.offset(0 as i32 as isize)) / aijk;
                    rijk[1 as i32
                        as usize] = (*ai.offset(ip as isize)
                        * *ri.offset(1 as i32 as isize)
                        + *aj.offset(jp as isize) * *rj.offset(1 as i32 as isize)
                        + *ak.offset(kp as isize)
                            * *rk.offset(1 as i32 as isize)) / aijk;
                    rijk[2 as i32
                        as usize] = (*ai.offset(ip as isize)
                        * *ri.offset(2 as i32 as isize)
                        + *aj.offset(jp as isize) * *rj.offset(2 as i32 as isize)
                        + *ak.offset(kp as isize)
                            * *rk.offset(2 as i32 as isize)) / aijk;
                    tau = CINTnuc_mod(aijk, nuc_id, atm, env);
                    x = aijk * CINTsquare_dist(rijk.as_mut_ptr(), cr) * tau * tau;
                    CINTrys_roots((*envs).nrys_roots, x, u.as_mut_ptr(), w.as_mut_ptr());
                    rys_empty = *gempty;
                    i = 0 as i32;
                    while i < (*envs).nrys_roots {
                        t2 = u[i as usize]
                            / (1 as i32 as f64 + u[i as usize]) * tau
                            * tau;
                        (*envs)
                            .fac[0 as i32 as usize] = dijk * w[i as usize] * tau;
                        CINTg3c1e_nuc(
                            g,
                            *ai.offset(ip as isize),
                            *aj.offset(jp as isize),
                            *ak.offset(kp as isize),
                            rijk.as_mut_ptr(),
                            cr,
                            t2,
                            envs,
                        );
                        ::core::mem::transmute::<
                            _,
                            fn(_, _, _, _, _),
                        >(
                            (Some(((*envs).f_gout).expect("non-null function pointer")))
                                .expect("non-null function pointer"),
                        )(gout, g, idx, envs, rys_empty);
                        rys_empty = 0 as i32;
                        i += 1;
                        i;
                    }
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
                ip;
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
            jp;
        }
        if *jempty == 0 {
            if k_ctr > 1 as i32 {
                if *kempty != 0 {
                    CINTprim_to_ctr_0(
                        gctrk,
                        gctrj,
                        ck.offset(kp as isize),
                        ((*envs).nf * i_ctr * j_ctr * n_comp) as size_t,
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
                        ((*envs).nf * i_ctr * j_ctr * n_comp) as size_t,
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
pub unsafe extern "C" fn CINT3c1e_drv(
    mut out: *mut f64,
    mut dims: *mut i32,
    mut envs: *mut CINTEnvVars,
    mut opt: *mut CINTOpt,
    mut cache: *mut f64,
    mut f_e1_c2s: Option::<unsafe extern "C" fn() -> ()>,
    mut int_type: i32,
    mut is_ssc: i32,
) -> i32 {
    let mut x_ctr: *mut i32 = ((*envs).x_ctr).as_mut_ptr();
    let mut nc: i32 = (*envs).nf * *x_ctr.offset(0 as i32 as isize)
        * *x_ctr.offset(1 as i32 as isize)
        * *x_ctr.offset(2 as i32 as isize);
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
        let mut pdata_size: i32 = i_prim
            * *x_ctr.offset(0 as i32 as isize)
            + j_prim * *x_ctr.offset(1 as i32 as isize)
            + k_prim * *x_ctr.offset(2 as i32 as isize)
            + (*envs).nf * 3 as i32;
        let mut leng: size_t = ((*envs).g_size * 3 as i32
            * (((1 as i32) << (*envs).gbits) + 1 as i32)) as size_t;
        let mut len0: size_t = ((*envs).nf * n_comp) as size_t;
        let mut cache_size: i32 = (if leng
            .wrapping_add(len0)
            .wrapping_add((nc * n_comp * 4 as i32) as libc::c_ulong)
            .wrapping_add(pdata_size as libc::c_ulong)
            > (nc * n_comp + (*envs).nf * 3 as i32) as libc::c_ulong
        {
            leng.wrapping_add(len0)
                .wrapping_add((nc * n_comp * 4 as i32) as libc::c_ulong)
                .wrapping_add(pdata_size as libc::c_ulong)
        } else {
            (nc * n_comp + (*envs).nf * 3 as i32) as libc::c_ulong
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
        let mut pdata_size_0: i32 = i_prim_0
            * *x_ctr.offset(0 as i32 as isize)
            + j_prim_0 * *x_ctr.offset(1 as i32 as isize)
            + k_prim_0 * *x_ctr.offset(2 as i32 as isize)
            + (*envs).nf * 3 as i32;
        let mut leng_0: size_t = ((*envs).g_size * 3 as i32
            * (((1 as i32) << (*envs).gbits) + 1 as i32)) as size_t;
        let mut len0_0: size_t = ((*envs).nf * n_comp) as size_t;
        let mut cache_size_0: i32 = (if leng_0
            .wrapping_add(len0_0)
            .wrapping_add((nc * n_comp * 4 as i32) as libc::c_ulong)
            .wrapping_add(pdata_size_0 as libc::c_ulong)
            > (nc * n_comp + (*envs).nf * 3 as i32) as libc::c_ulong
        {
            leng_0
                .wrapping_add(len0_0)
                .wrapping_add((nc * n_comp * 4 as i32) as libc::c_ulong)
                .wrapping_add(pdata_size_0 as libc::c_ulong)
        } else {
            (nc * n_comp + (*envs).nf * 3 as i32) as libc::c_ulong
        }) as i32;
        stack = malloc(
            (::core::mem::size_of::<f64>() as libc::c_ulong)
                .wrapping_mul(cache_size_0 as libc::c_ulong),
        ) as *mut f64;
        cache = stack;
    }
    let mut gctr: *mut f64 = 0 as *mut f64;
    let mut n: i32 = 0;
    let mut empty: i32 = 1 as i32;
    if int_type == 0 as i32 {
        gctr = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
            & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut f64;
        cache = gctr.offset((nc * n_comp) as isize);
        CINT3c1e_loop_nopt(gctr, envs, cache, &mut empty);
    } else if int_type == 1 as i32 {
        gctr = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
            & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut f64;
        cache = gctr.offset((nc * n_comp) as isize);
        CINT3c1e_nuc_loop_nopt(
            gctr,
            envs,
            1 as i32 as f64,
            -(1 as i32),
            cache,
            &mut empty,
        );
    } else {
        let mut atm: *mut i32 = (*envs).atm;
        let mut i: i32 = 0;
        let mut fac: f64 = 0.;
        let mut buf: *mut f64 = 0 as *mut f64;
        gctr = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
            & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut f64;
        cache = gctr.offset((nc * n_comp) as isize);
        buf = ((cache as uintptr_t).wrapping_add(7 as i32 as libc::c_ulong)
            & (8 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut f64;
        cache = buf.offset((nc * n_comp) as isize);
        n = 0 as i32;
        while n < (*envs).natm {
            if *atm.offset((6 as i32 * n + 0 as i32) as isize)
                != 0 as i32
            {
                fac = -abs(
                    *atm.offset((6 as i32 * n + 0 as i32) as isize),
                ) as f64;
                CINT3c1e_nuc_loop_nopt(buf, envs, fac, n, cache, &mut empty);
            }
            n += 1;
            n;
        }
        cache = buf;
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
                c2s_sph_3c1e
                    as unsafe extern "C" fn(
                        *mut f64,
                        *mut f64,
                        *mut i32,
                        *mut CINTEnvVars,
                        *mut f64,
                    ) -> (),
            ),
        )
        || f_e1_c2s
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
pub unsafe extern "C" fn int3c1e_sph(
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
    CINTinit_int3c1e_EnvVars(
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
    return CINT3c1e_drv(
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
                c2s_sph_3c1e
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
        0 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_optimizer(
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
pub unsafe extern "C" fn int3c1e_cart(
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
    CINTinit_int3c1e_EnvVars(
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
    return CINT3c1e_drv(
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
                c2s_cart_3c1e
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
        0 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_rinv_sph(
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
    CINTinit_int3c1e_EnvVars(
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
    return CINT3c1e_drv(
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
                c2s_sph_3c1e
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
        0 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_rinv_optimizer(
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
pub unsafe extern "C" fn int3c1e_rinv_cart(
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
    CINTinit_int3c1e_EnvVars(
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
    return CINT3c1e_drv(
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
                c2s_cart_3c1e
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
        0 as i32,
    );
}
