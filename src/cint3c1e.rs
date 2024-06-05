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
    fn abs(_: libc::c_int) -> libc::c_int;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
}

#[no_mangle]
pub unsafe extern "C" fn CINT3c1e_loop_nopt(
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
    let mut ri: *mut libc::c_double = (*envs).ri;
    let mut rj: *mut libc::c_double = (*envs).rj;
    let mut rk: *mut libc::c_double = (*envs).rk;
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
    let mut idx: *mut libc::c_int = 0 as *mut libc::c_int;
    idx = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_int;
    cache = idx.offset(((*envs).nf * 3 as libc::c_int) as isize) as *mut libc::c_double;
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
    let mut lenk: size_t = ((*envs).nf * nc * n_comp) as size_t;
    let mut lenj: size_t = ((*envs).nf * i_ctr * j_ctr * n_comp) as size_t;
    let mut leni: size_t = ((*envs).nf * i_ctr * n_comp) as size_t;
    let mut len0: size_t = ((*envs).nf * n_comp) as size_t;
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
    let mut eijk: libc::c_double = 0.;
    let mut dijk: libc::c_double = 0.;
    let mut aijk: libc::c_double = 0.;
    let mut aiajrr: libc::c_double = 0.;
    let mut aiakrr: libc::c_double = 0.;
    let mut ajakrr: libc::c_double = 0.;
    let mut rirk: [libc::c_double; 3] = [0.; 3];
    let mut rjrk: [libc::c_double; 3] = [0.; 3];
    rirk[0 as libc::c_int
        as usize] = *ri.offset(0 as libc::c_int as isize)
        - *rk.offset(0 as libc::c_int as isize);
    rirk[1 as libc::c_int
        as usize] = *ri.offset(1 as libc::c_int as isize)
        - *rk.offset(1 as libc::c_int as isize);
    rirk[2 as libc::c_int
        as usize] = *ri.offset(2 as libc::c_int as isize)
        - *rk.offset(2 as libc::c_int as isize);
    rjrk[0 as libc::c_int
        as usize] = *rj.offset(0 as libc::c_int as isize)
        - *rk.offset(0 as libc::c_int as isize);
    rjrk[1 as libc::c_int
        as usize] = *rj.offset(1 as libc::c_int as isize)
        - *rk.offset(1 as libc::c_int as isize);
    rjrk[2 as libc::c_int
        as usize] = *rj.offset(2 as libc::c_int as isize)
        - *rk.offset(2 as libc::c_int as isize);
    let mut rr_ij: libc::c_double = (*envs).rirj[0 as libc::c_int as usize]
        * (*envs).rirj[0 as libc::c_int as usize]
        + (*envs).rirj[1 as libc::c_int as usize]
            * (*envs).rirj[1 as libc::c_int as usize]
        + (*envs).rirj[2 as libc::c_int as usize]
            * (*envs).rirj[2 as libc::c_int as usize];
    let mut rr_ik: libc::c_double = rirk[0 as libc::c_int as usize]
        * rirk[0 as libc::c_int as usize]
        + rirk[1 as libc::c_int as usize] * rirk[1 as libc::c_int as usize]
        + rirk[2 as libc::c_int as usize] * rirk[2 as libc::c_int as usize];
    let mut rr_jk: libc::c_double = rjrk[0 as libc::c_int as usize]
        * rjrk[0 as libc::c_int as usize]
        + rjrk[1 as libc::c_int as usize] * rjrk[1 as libc::c_int as usize]
        + rjrk[2 as libc::c_int as usize] * rjrk[2 as libc::c_int as usize];
    kp = 0 as libc::c_int;
    while kp < k_prim {
        (*envs).ak[0 as libc::c_int as usize] = *ak.offset(kp as isize);
        if k_ctr == 1 as libc::c_int {
            fac1k = (*envs).common_factor * *ck.offset(kp as isize);
        } else {
            fac1k = (*envs).common_factor;
            *jempty = 1 as libc::c_int;
        }
        jp = 0 as libc::c_int;
        while jp < j_prim {
            (*envs).aj[0 as libc::c_int as usize] = *aj.offset(jp as isize);
            if j_ctr == 1 as libc::c_int {
                fac1j = fac1k * *cj.offset(jp as isize);
            } else {
                fac1j = fac1k;
                *iempty = 1 as libc::c_int;
            }
            ajakrr = *aj.offset(jp as isize) * *ak.offset(kp as isize) * rr_jk;
            ip = 0 as libc::c_int;
            while ip < i_prim {
                (*envs).ai[0 as libc::c_int as usize] = *ai.offset(ip as isize);
                aijk = *ai.offset(ip as isize) + *aj.offset(jp as isize)
                    + *ak.offset(kp as isize);
                aiakrr = *ai.offset(ip as isize) * *ak.offset(kp as isize) * rr_ik;
                aiajrr = *ai.offset(ip as isize) * *aj.offset(jp as isize) * rr_ij;
                eijk = (aiajrr + aiakrr + ajakrr) / aijk;
                if !(eijk > 60 as libc::c_int as libc::c_double) {
                    if i_ctr == 1 as libc::c_int {
                        fac1i = fac1j * *ci.offset(ip as isize) * exp(-eijk);
                    } else {
                        fac1i = fac1j * exp(-eijk);
                    }
                    dijk = fac1i / (aijk * sqrt(aijk));
                    (*envs).fac[0 as libc::c_int as usize] = dijk;
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
                ip += 1;
                ip;
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
pub unsafe extern "C" fn CINT3c1e_nuc_loop_nopt(
    mut gctr: *mut libc::c_double,
    mut envs: *mut CINTEnvVars,
    mut fac: libc::c_double,
    mut nuc_id: libc::c_int,
    mut cache: *mut libc::c_double,
    mut empty: *mut libc::c_int,
) -> libc::c_int {
    let mut shls: *mut libc::c_int = (*envs).shls;
    let mut atm: *mut libc::c_int = (*envs).atm;
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
    let mut ri: *mut libc::c_double = (*envs).ri;
    let mut rj: *mut libc::c_double = (*envs).rj;
    let mut rk: *mut libc::c_double = (*envs).rk;
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
    let mut n_comp: libc::c_int = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: libc::c_double = 0.;
    let mut fac1j: libc::c_double = 0.;
    let mut fac1k: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
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
    let mut rys_empty: libc::c_int = 0;
    let mut idx: *mut libc::c_int = 0 as *mut libc::c_int;
    idx = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_int;
    cache = idx.offset(((*envs).nf * 3 as libc::c_int) as isize) as *mut libc::c_double;
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
    let mut cr: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut t2: libc::c_double = 0.;
    let mut tau: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut u: [libc::c_double; 32] = [0.; 32];
    let mut w: [libc::c_double; 32] = [0.; 32];
    let mut nc: libc::c_int = i_ctr * j_ctr * k_ctr;
    let mut leng: size_t = ((*envs).g_size * 3 as libc::c_int
        * (((1 as libc::c_int) << (*envs).gbits) + 1 as libc::c_int)) as size_t;
    let mut lenk: size_t = ((*envs).nf * nc * n_comp) as size_t;
    let mut lenj: size_t = ((*envs).nf * i_ctr * j_ctr * n_comp) as size_t;
    let mut leni: size_t = ((*envs).nf * i_ctr * n_comp) as size_t;
    let mut len0: size_t = ((*envs).nf * n_comp) as size_t;
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
    if nuc_id < 0 as libc::c_int {
        cr = &mut *env.offset(4 as libc::c_int as isize) as *mut libc::c_double;
    } else {
        cr = &mut *env
            .offset(
                *atm.offset((6 as libc::c_int * nuc_id + 1 as libc::c_int) as isize)
                    as isize,
            ) as *mut libc::c_double;
    }
    let mut eijk: libc::c_double = 0.;
    let mut dijk: libc::c_double = 0.;
    let mut aijk: libc::c_double = 0.;
    let mut aiajrr: libc::c_double = 0.;
    let mut aiakrr: libc::c_double = 0.;
    let mut ajakrr: libc::c_double = 0.;
    let mut rirk: [libc::c_double; 3] = [0.; 3];
    let mut rjrk: [libc::c_double; 3] = [0.; 3];
    let mut rijk: [libc::c_double; 3] = [0.; 3];
    rirk[0 as libc::c_int
        as usize] = *ri.offset(0 as libc::c_int as isize)
        - *rk.offset(0 as libc::c_int as isize);
    rirk[1 as libc::c_int
        as usize] = *ri.offset(1 as libc::c_int as isize)
        - *rk.offset(1 as libc::c_int as isize);
    rirk[2 as libc::c_int
        as usize] = *ri.offset(2 as libc::c_int as isize)
        - *rk.offset(2 as libc::c_int as isize);
    rjrk[0 as libc::c_int
        as usize] = *rj.offset(0 as libc::c_int as isize)
        - *rk.offset(0 as libc::c_int as isize);
    rjrk[1 as libc::c_int
        as usize] = *rj.offset(1 as libc::c_int as isize)
        - *rk.offset(1 as libc::c_int as isize);
    rjrk[2 as libc::c_int
        as usize] = *rj.offset(2 as libc::c_int as isize)
        - *rk.offset(2 as libc::c_int as isize);
    let mut rr_ij: libc::c_double = (*envs).rirj[0 as libc::c_int as usize]
        * (*envs).rirj[0 as libc::c_int as usize]
        + (*envs).rirj[1 as libc::c_int as usize]
            * (*envs).rirj[1 as libc::c_int as usize]
        + (*envs).rirj[2 as libc::c_int as usize]
            * (*envs).rirj[2 as libc::c_int as usize];
    let mut rr_ik: libc::c_double = rirk[0 as libc::c_int as usize]
        * rirk[0 as libc::c_int as usize]
        + rirk[1 as libc::c_int as usize] * rirk[1 as libc::c_int as usize]
        + rirk[2 as libc::c_int as usize] * rirk[2 as libc::c_int as usize];
    let mut rr_jk: libc::c_double = rjrk[0 as libc::c_int as usize]
        * rjrk[0 as libc::c_int as usize]
        + rjrk[1 as libc::c_int as usize] * rjrk[1 as libc::c_int as usize]
        + rjrk[2 as libc::c_int as usize] * rjrk[2 as libc::c_int as usize];
    fac *= (*envs).common_factor;
    kp = 0 as libc::c_int;
    while kp < k_prim {
        (*envs).ak[0 as libc::c_int as usize] = *ak.offset(kp as isize);
        if k_ctr == 1 as libc::c_int {
            fac1k = fac * *ck.offset(kp as isize);
        } else {
            fac1k = fac;
            *jempty = 1 as libc::c_int;
        }
        jp = 0 as libc::c_int;
        while jp < j_prim {
            (*envs).aj[0 as libc::c_int as usize] = *aj.offset(jp as isize);
            if j_ctr == 1 as libc::c_int {
                fac1j = fac1k * *cj.offset(jp as isize);
            } else {
                fac1j = fac1k;
                *iempty = 1 as libc::c_int;
            }
            ajakrr = *aj.offset(jp as isize) * *ak.offset(kp as isize) * rr_jk;
            ip = 0 as libc::c_int;
            while ip < i_prim {
                (*envs).ai[0 as libc::c_int as usize] = *ai.offset(ip as isize);
                aijk = *ai.offset(ip as isize) + *aj.offset(jp as isize)
                    + *ak.offset(kp as isize);
                aiakrr = *ai.offset(ip as isize) * *ak.offset(kp as isize) * rr_ik;
                aiajrr = *ai.offset(ip as isize) * *aj.offset(jp as isize) * rr_ij;
                eijk = (aiajrr + aiakrr + ajakrr) / aijk;
                if !(eijk > 60 as libc::c_int as libc::c_double) {
                    if i_ctr == 1 as libc::c_int {
                        fac1i = fac1j * *ci.offset(ip as isize) * exp(-eijk);
                    } else {
                        fac1i = fac1j * exp(-eijk);
                    }
                    dijk = fac1i / aijk;
                    rijk[0 as libc::c_int
                        as usize] = (*ai.offset(ip as isize)
                        * *ri.offset(0 as libc::c_int as isize)
                        + *aj.offset(jp as isize) * *rj.offset(0 as libc::c_int as isize)
                        + *ak.offset(kp as isize)
                            * *rk.offset(0 as libc::c_int as isize)) / aijk;
                    rijk[1 as libc::c_int
                        as usize] = (*ai.offset(ip as isize)
                        * *ri.offset(1 as libc::c_int as isize)
                        + *aj.offset(jp as isize) * *rj.offset(1 as libc::c_int as isize)
                        + *ak.offset(kp as isize)
                            * *rk.offset(1 as libc::c_int as isize)) / aijk;
                    rijk[2 as libc::c_int
                        as usize] = (*ai.offset(ip as isize)
                        * *ri.offset(2 as libc::c_int as isize)
                        + *aj.offset(jp as isize) * *rj.offset(2 as libc::c_int as isize)
                        + *ak.offset(kp as isize)
                            * *rk.offset(2 as libc::c_int as isize)) / aijk;
                    tau = CINTnuc_mod(aijk, nuc_id, atm, env);
                    x = aijk * CINTsquare_dist(rijk.as_mut_ptr(), cr) * tau * tau;
                    CINTrys_roots((*envs).nrys_roots, x, u.as_mut_ptr(), w.as_mut_ptr());
                    rys_empty = *gempty;
                    i = 0 as libc::c_int;
                    while i < (*envs).nrys_roots {
                        t2 = u[i as usize]
                            / (1 as libc::c_int as libc::c_double + u[i as usize]) * tau
                            * tau;
                        (*envs)
                            .fac[0 as libc::c_int as usize] = dijk * w[i as usize] * tau;
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
                        rys_empty = 0 as libc::c_int;
                        i += 1;
                        i;
                    }
                    if i_ctr > 1 as libc::c_int {
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
                    *iempty = 0 as libc::c_int;
                }
                ip += 1;
                ip;
            }
            if *iempty == 0 {
                if j_ctr > 1 as libc::c_int {
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
pub unsafe extern "C" fn CINT3c1e_drv(
    mut out: *mut libc::c_double,
    mut dims: *mut libc::c_int,
    mut envs: *mut CINTEnvVars,
    mut opt: *mut CINTOpt,
    mut cache: *mut libc::c_double,
    mut f_e1_c2s: Option::<unsafe extern "C" fn() -> ()>,
    mut int_type: libc::c_int,
    mut is_ssc: libc::c_int,
) -> libc::c_int {
    let mut x_ctr: *mut libc::c_int = ((*envs).x_ctr).as_mut_ptr();
    let mut nc: libc::c_int = (*envs).nf * *x_ctr.offset(0 as libc::c_int as isize)
        * *x_ctr.offset(1 as libc::c_int as isize)
        * *x_ctr.offset(2 as libc::c_int as isize);
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
        let mut pdata_size: libc::c_int = i_prim
            * *x_ctr.offset(0 as libc::c_int as isize)
            + j_prim * *x_ctr.offset(1 as libc::c_int as isize)
            + k_prim * *x_ctr.offset(2 as libc::c_int as isize)
            + (*envs).nf * 3 as libc::c_int;
        let mut leng: size_t = ((*envs).g_size * 3 as libc::c_int
            * (((1 as libc::c_int) << (*envs).gbits) + 1 as libc::c_int)) as size_t;
        let mut len0: size_t = ((*envs).nf * n_comp) as size_t;
        let mut cache_size: libc::c_int = (if leng
            .wrapping_add(len0)
            .wrapping_add((nc * n_comp * 4 as libc::c_int) as libc::c_ulong)
            .wrapping_add(pdata_size as libc::c_ulong)
            > (nc * n_comp + (*envs).nf * 3 as libc::c_int) as libc::c_ulong
        {
            leng.wrapping_add(len0)
                .wrapping_add((nc * n_comp * 4 as libc::c_int) as libc::c_ulong)
                .wrapping_add(pdata_size as libc::c_ulong)
        } else {
            (nc * n_comp + (*envs).nf * 3 as libc::c_int) as libc::c_ulong
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
        let mut pdata_size_0: libc::c_int = i_prim_0
            * *x_ctr.offset(0 as libc::c_int as isize)
            + j_prim_0 * *x_ctr.offset(1 as libc::c_int as isize)
            + k_prim_0 * *x_ctr.offset(2 as libc::c_int as isize)
            + (*envs).nf * 3 as libc::c_int;
        let mut leng_0: size_t = ((*envs).g_size * 3 as libc::c_int
            * (((1 as libc::c_int) << (*envs).gbits) + 1 as libc::c_int)) as size_t;
        let mut len0_0: size_t = ((*envs).nf * n_comp) as size_t;
        let mut cache_size_0: libc::c_int = (if leng_0
            .wrapping_add(len0_0)
            .wrapping_add((nc * n_comp * 4 as libc::c_int) as libc::c_ulong)
            .wrapping_add(pdata_size_0 as libc::c_ulong)
            > (nc * n_comp + (*envs).nf * 3 as libc::c_int) as libc::c_ulong
        {
            leng_0
                .wrapping_add(len0_0)
                .wrapping_add((nc * n_comp * 4 as libc::c_int) as libc::c_ulong)
                .wrapping_add(pdata_size_0 as libc::c_ulong)
        } else {
            (nc * n_comp + (*envs).nf * 3 as libc::c_int) as libc::c_ulong
        }) as libc::c_int;
        stack = malloc(
            (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(cache_size_0 as libc::c_ulong),
        ) as *mut libc::c_double;
        cache = stack;
    }
    let mut gctr: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut n: libc::c_int = 0;
    let mut empty: libc::c_int = 1 as libc::c_int;
    if int_type == 0 as libc::c_int {
        gctr = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
            & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut libc::c_double;
        cache = gctr.offset((nc * n_comp) as isize);
        CINT3c1e_loop_nopt(gctr, envs, cache, &mut empty);
    } else if int_type == 1 as libc::c_int {
        gctr = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
            & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut libc::c_double;
        cache = gctr.offset((nc * n_comp) as isize);
        CINT3c1e_nuc_loop_nopt(
            gctr,
            envs,
            1 as libc::c_int as libc::c_double,
            -(1 as libc::c_int),
            cache,
            &mut empty,
        );
    } else {
        let mut atm: *mut libc::c_int = (*envs).atm;
        let mut i: libc::c_int = 0;
        let mut fac: libc::c_double = 0.;
        let mut buf: *mut libc::c_double = 0 as *mut libc::c_double;
        gctr = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
            & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut libc::c_double;
        cache = gctr.offset((nc * n_comp) as isize);
        buf = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
            & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut libc::c_double;
        cache = buf.offset((nc * n_comp) as isize);
        n = 0 as libc::c_int;
        while n < (*envs).natm {
            if *atm.offset((6 as libc::c_int * n + 0 as libc::c_int) as isize)
                != 0 as libc::c_int
            {
                fac = -abs(
                    *atm.offset((6 as libc::c_int * n + 0 as libc::c_int) as isize),
                ) as libc::c_double;
                CINT3c1e_nuc_loop_nopt(buf, envs, fac, n, cache, &mut empty);
            }
            n += 1;
            n;
        }
        cache = buf;
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
                c2s_sph_3c1e
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut libc::c_double,
                        *mut libc::c_int,
                        *mut CINTEnvVars,
                        *mut libc::c_double,
                    ) -> (),
            ),
        )
        || f_e1_c2s
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
pub unsafe extern "C" fn int3c1e_sph(
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
            CINTgout1e
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
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
                c2s_sph_3c1e
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
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
) {
    *opt = 0 as *mut CINTOpt;
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_cart(
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
            CINTgout1e
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
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
                c2s_cart_3c1e
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
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_rinv_sph(
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
            CINTgout1e
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
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
                c2s_sph_3c1e
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
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_rinv_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
) {
    *opt = 0 as *mut CINTOpt;
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_rinv_cart(
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
            CINTgout1e
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
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
                c2s_cart_3c1e
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
        0 as libc::c_int,
    );
}
