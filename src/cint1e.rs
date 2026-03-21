#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::cart2sph::c2s_cart_1e;
use crate::cart2sph::c2s_dset0;
use crate::cart2sph::c2s_sph_1e;
use crate::fblas::CINTdmat_transpose;
use crate::g1e::CINTcommon_fac_sp;
use crate::g1e::CINTg1e_index_xyz;
use crate::g1e::CINTg1e_nuc;
use crate::g1e::CINTg1e_ovlp;
use crate::g1e::CINTinit_int1e_EnvVars;
use crate::g1e::CINTprim_to_ctr_0;
use crate::g1e::CINTprim_to_ctr_1;
use crate::optimizer::CINTOpt_log_max_pgto_coeff;
use crate::optimizer::{CINTOpt_non0coeff_byshell, CINTOpt_non0coeff_byshell_safe};
use crate::optimizer::CINTset_pairdata;

use crate::cint::CINTEnvVars;
use crate::cint::CINTOpt;
use crate::cint::PairData;

pub type uintptr_t = u64;

extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}

#[no_mangle]
pub unsafe fn CINT1e_loop(
    gctr: *mut f64,
    envs: *mut CINTEnvVars,
    mut cache: *mut f64,
    int1e_type: i32,
) -> i32 {
    let shls: *const i32 = (*envs).shls;
    let bas: *const i32 = (*envs).bas;
    let env: *const f64 = (*envs).env;
    let i_sh: i32 = *shls.offset(0 as isize);
    let j_sh: i32 = *shls.offset(1 as isize);
    let i_ctr: i32 = (*envs).x_ctr[0 as usize];
    let j_ctr: i32 = (*envs).x_ctr[1 as usize];
    let i_prim: i32 = *bas.offset((8 * i_sh + 2) as isize);
    let j_prim: i32 = *bas.offset((8 * j_sh + 2) as isize);
    let ai: *const f64 = env.offset(*bas.offset((8 * i_sh + 5) as isize) as isize);
    let aj: *const f64 = env.offset(*bas.offset((8 * j_sh + 5) as isize) as isize);
    let ci: *const f64 = env.offset(*bas.offset((8 * i_sh + 6) as isize) as isize);
    let cj: *const f64 = env.offset(*bas.offset((8 * j_sh + 6) as isize) as isize);
    let n_comp: i32 = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let expcutoff: f64 = (*envs).expcutoff;
    //let mut pdata_ij: *mut PairData = 0 as *mut PairData;
    let log_maxci = ((cache as uintptr_t).wrapping_add(7 as u64) & (8 as uintptr_t).wrapping_neg())
        as *mut libc::c_void as *mut f64;
    cache = log_maxci.offset((i_prim + j_prim) as isize);
    let pdata_base = ((cache as uintptr_t).wrapping_add(7 as u64) & (8 as uintptr_t).wrapping_neg())
        as *mut libc::c_void as *mut PairData;
    cache = pdata_base.offset((i_prim * j_prim) as isize) as *mut f64;
    let log_maxcj = log_maxci.offset(i_prim as isize);
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
        (*envs).rirj[0 as usize] * (*envs).rirj[0 as usize]
            + (*envs).rirj[1 as usize] * (*envs).rirj[1 as usize]
            + (*envs).rirj[2 as usize] * (*envs).rirj[2 as usize],
        expcutoff,
        env,
    ) != 0
    {
        return 0;
    }
    let mut empty: [i32; 4] = [1, 1, 1, 1];
    let mut gempty: *mut i32 = empty.as_mut_ptr().offset(0 as isize);
    let mut iempty: *mut i32 = empty.as_mut_ptr().offset(1 as isize);
    let mut jempty: *mut i32 = empty.as_mut_ptr().offset(2 as isize);
    let mut idx: *mut i32 = 0 as *mut i32;
    idx = ((cache as uintptr_t).wrapping_add(7 as u64) & (8 as uintptr_t).wrapping_neg())
        as *mut libc::c_void as *mut i32;
    cache = idx.offset(((*envs).nf * 3) as isize) as *mut f64;
    CINTg1e_index_xyz(idx, &*envs);
    let mut non0ctri: *mut i32 = 0 as *mut i32;
    let mut non0ctrj: *mut i32 = 0 as *mut i32;
    let mut non0idxi: *mut i32 = 0 as *mut i32;
    let mut non0idxj: *mut i32 = 0 as *mut i32;
    non0ctri = ((cache as uintptr_t).wrapping_add(7 as u64) & (8 as uintptr_t).wrapping_neg())
        as *mut libc::c_void as *mut i32;
    cache =
        non0ctri.offset((i_prim + j_prim + i_prim * i_ctr + j_prim * j_ctr) as isize) as *mut f64;
    non0ctrj = non0ctri.offset(i_prim as isize);
    non0idxi = non0ctrj.offset(j_prim as isize);
    non0idxj = non0idxi.offset((i_prim * i_ctr) as isize);
    CINTOpt_non0coeff_byshell_safe(non0idxi, std::slice::from_raw_parts_mut(non0ctri, 999999), std::slice::from_raw_parts(ci, 999999), i_prim, i_ctr);
    CINTOpt_non0coeff_byshell_safe(non0idxj, std::slice::from_raw_parts_mut(non0ctrj, 999999), std::slice::from_raw_parts(cj, 999999), j_prim, j_ctr);
    let nc: i32 = i_ctr * j_ctr;
    let leng: i32 = (*envs).g_size * 3 * (((1) << (*envs).gbits) + 1);
    let lenj: i32 = (*envs).nf * nc * n_comp;
    let leni: i32 = (*envs).nf * i_ctr * n_comp;
    let len0: i32 = (*envs).nf * n_comp;
    let len: i32 = leng + lenj + leni + len0;
    let mut g: *mut f64 = 0 as *mut f64;
    let mut gout: *mut f64 = 0 as *mut f64;
    let mut gctri: *mut f64 = 0 as *mut f64;
    let mut gctrj: *mut f64 = 0 as *mut f64;
    g = ((cache as uintptr_t).wrapping_add(7 as u64) & (8 as uintptr_t).wrapping_neg())
        as *mut libc::c_void as *mut f64;
    cache = g.offset(len as isize);
    let mut g1: *mut f64 = g.offset(leng as isize);
    if n_comp == 1 {
        gctrj = gctr;
    } else {
        gctrj = g1;
        g1 = g1.offset(lenj as isize);
    }
    if j_ctr == 1 {
        gctri = gctrj;
        iempty = jempty;
    } else {
        gctri = g1;
        g1 = g1.offset(leni as isize);
    }
    if i_ctr == 1 {
        gout = gctri;
        gempty = iempty;
    } else {
        gout = g1;
    }
    let mut common_factor: f64 =
        (*envs).common_factor * CINTcommon_fac_sp((*envs).i_l) * CINTcommon_fac_sp((*envs).j_l);
    let mut pdata_ij = pdata_base;
    for jp in 0..(j_prim as isize) {
        (*envs).aj[0] = *aj.offset(jp);
        let fac1j: f64 = if j_ctr == 1 {
            common_factor * *cj.offset(jp)
        } else {
            *iempty = 1;
            common_factor
        };
        for ip in 0..(i_prim as isize) {
            if !((*pdata_ij).cceij > expcutoff) {
                (*envs).ai[0 as usize] = *ai.offset(ip);
                let expij = (*pdata_ij).eij;
                let rij = ((*pdata_ij).rij).as_ptr();
                (*envs).rij[0] = *rij.offset(0);
                (*envs).rij[1] = *rij.offset(1);
                (*envs).rij[2] = *rij.offset(2);
                (*envs).fac[0] = if i_ctr == 1 {
                    fac1j * *ci.offset(ip) * expij
                } else {
                    fac1j * expij
                };
                //make_g1e_gout(gout, g, idx, &*envs, *gempty, int1e_type);
                assert_eq!(int1e_type, 0);
                CINTg1e_ovlp(g, &*envs);
                CINTgout1e(gout, g, idx, &*envs, *gempty);
                if i_ctr > 1 {
                    if *iempty != 0 {
                        CINTprim_to_ctr_0(
                            gctri,
                            gout,
                            ci.offset(ip),
                            ((*envs).nf * n_comp) as u64,
                            i_prim,
                            i_ctr,
                            *non0ctri.offset(ip),
                            non0idxi.offset(ip * i_ctr as isize),
                        );
                    } else {
                        CINTprim_to_ctr_1(
                            gctri,
                            gout,
                            ci.offset(ip),
                            ((*envs).nf * n_comp) as u64,
                            i_prim,
                            i_ctr,
                            *non0ctri.offset(ip),
                            non0idxi.offset(ip * i_ctr as isize),
                        );
                    }
                }
                *iempty = 0;
            }
            pdata_ij = pdata_ij.offset(1);
        }
        if *iempty == 0 {
            if j_ctr > 1 {
                if *jempty != 0 {
                    CINTprim_to_ctr_0(
                        gctrj,
                        gctri,
                        cj.offset(jp),
                        ((*envs).nf * i_ctr * n_comp) as u64,
                        j_prim,
                        j_ctr,
                        *non0ctrj.offset(jp),
                        non0idxj.offset(jp * j_ctr as isize),
                    );
                } else {
                    CINTprim_to_ctr_1(
                        gctrj,
                        gctri,
                        cj.offset(jp),
                        ((*envs).nf * i_ctr * n_comp) as u64,
                        j_prim,
                        j_ctr,
                        *non0ctrj.offset(jp),
                        non0idxj.offset(jp * j_ctr as isize),
                    );
                }
            }
            *jempty = 0;
        }
    }
    if n_comp > 1 && *jempty == 0 {
        CINTdmat_transpose(gctr, gctrj, (*envs).nf * nc, n_comp);
    }
    return (*jempty == 0) as i32;
}
#[no_mangle]
pub unsafe fn int1e_cache_size(envs: &CINTEnvVars) -> i32 {
    let shls: *const i32 = envs.shls;
    let bas: *const i32 = envs.bas;
    let i_prim: i32 = *bas.offset((8 * *shls.offset(0 as isize) + 2) as isize);
    let mut j_prim: i32 = *bas.offset((8 * *shls.offset(1 as isize) + 2) as isize);
    let x_ctr: *const i32 = (envs.x_ctr).as_ptr();
    let nc: i32 = envs.nf * *x_ctr.offset(0 as isize) * *x_ctr.offset(1 as isize);
    let mut n_comp: i32 = envs.ncomp_e1 * envs.ncomp_tensor;
    let mut leng: i32 = envs.g_size * 3 * (((1) << envs.gbits) + 1);
    let mut lenj: i32 = envs.nf * nc * n_comp;
    let mut leni: i32 = envs.nf * *x_ctr.offset(0 as isize) * n_comp;
    let mut len0: i32 = envs.nf * n_comp;
    let mut pdata_size: i32 = i_prim * j_prim * 5
        + i_prim * *x_ctr.offset(0 as isize)
        + j_prim * *x_ctr.offset(1 as isize)
        + (i_prim + j_prim) * 2
        + envs.nf * 3;
    let mut cache_size: i32 =
        if nc * n_comp + leng + lenj + leni + len0 + pdata_size > nc * n_comp + envs.nf * 8 * 2 {
            nc * n_comp + leng + lenj + leni + len0 + pdata_size
        } else {
            nc * n_comp + envs.nf * 8 * 2
        };
    return cache_size;
}
#[no_mangle]
pub unsafe fn CINT1e_drv(
    out: *mut f64,
    mut dims: *mut i32,
    envs: *mut CINTEnvVars,
    mut cache: *mut f64,
    f_c2s: Option<unsafe fn() -> ()>,
    int1e_type: i32,
) -> i32 {
    if out.is_null() {
        return int1e_cache_size(&*envs);
    }
    let mut x_ctr: *const i32 = ((*envs).x_ctr).as_ptr();
    let mut nc: i32 = (*envs).nf * *x_ctr.offset(0 as isize) * *x_ctr.offset(1 as isize);
    let mut n_comp: i32 = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut stack: *mut f64 = 0 as *mut f64;
    if cache.is_null() {
        let mut cache_size: u64 = int1e_cache_size(&*envs) as u64;
        dbg!(&cache_size);
        stack = malloc((::core::mem::size_of::<f64>() as u64).wrapping_mul(cache_size)) as *mut f64;
        cache = stack;
    }
    let mut gctr: *mut f64 = 0 as *mut f64;
    gctr = ((cache as uintptr_t).wrapping_add(7 as u64) & (8 as uintptr_t).wrapping_neg())
        as *mut libc::c_void as *mut f64;
    cache = gctr.offset((nc * n_comp) as isize);
    let mut has_value: i32 = CINT1e_loop(gctr, envs, cache, int1e_type);
    let mut counts: [i32; 4] = [0; 4];
    if dims.is_null() {
        dims = counts.as_mut_ptr();
    }
    if f_c2s
        == ::core::mem::transmute::<
            Option<unsafe fn(*mut f64, *mut f64, *const i32, *const CINTEnvVars, *mut f64) -> ()>,
            Option<unsafe fn() -> ()>,
        >(Some(
            c2s_cart_1e
                as unsafe fn(*mut f64, *mut f64, *const i32, *const CINTEnvVars, *mut f64) -> (),
        ))
    {
        counts[0 as usize] = (*envs).nfi * *x_ctr.offset(0 as isize);
        counts[1 as usize] = (*envs).nfj * *x_ctr.offset(1 as isize);
    }
    counts[2 as usize] = 1;
    counts[3 as usize] = 1;
    let mut nout: i32 = *dims.offset(0 as isize) * *dims.offset(1 as isize);
    let mut n: i32 = 0;
    if has_value != 0 {
        n = 0;
        while n < n_comp {
            //::core::mem::transmute::<_, fn(_, _, _, _, _)>(
            //    (Some(f_c2s.expect("non-null function pointer")))
            //        .expect("non-null function pointer"),
            c2s_cart_1e
            (
                out.offset((nout * n) as isize),
                gctr.offset((nc * n) as isize),
                dims,
                envs,
                cache,
            );
            n += 1;
        }
    } else {
        n = 0;
        while n < n_comp {
            c2s_dset0(out.offset((nout * n) as isize), dims, counts.as_mut_ptr());
            n += 1;
            n;
        }
    }
    if !stack.is_null() {
        free(stack as *mut libc::c_void);
    }
    return has_value;
}
unsafe extern "C" fn make_g1e_gout(
    gout: *mut f64,
    g: *mut f64,
    idx: *mut i32,
    envs: &CINTEnvVars,
    empty: i32,
    int1e_type: i32,
) {
    assert_eq!(int1e_type, 0);
    CINTg1e_ovlp(g, &*envs);
    CINTgout1e(gout, g, idx, envs, empty);
}
#[no_mangle]
#[inline(never)]
pub unsafe fn CINTgout1e(
    gout: *mut f64,
    g: *const f64,
    idx: *const i32,
    envs: &CINTEnvVars,
    empty: i32,
) {
    let nf: i32 = envs.nf;
    let mut n: i32 = 0;
    let mut ix: i32 = 0;
    let mut iy: i32 = 0;
    let mut iz: i32 = 0;
    if empty != 0 {
        n = 0;
        while n < nf {
            ix = *idx.offset((n * 3 + 0) as isize);
            iy = *idx.offset((n * 3 + 1) as isize);
            iz = *idx.offset((n * 3 + 2) as isize);
            *gout.offset(n as isize) =
                *g.offset(ix as isize) * *g.offset(iy as isize) * *g.offset(iz as isize);
            n += 1;
        }
    } else {
        n = 0;
        while n < nf {
            ix = *idx.offset((n * 3 + 0) as isize);
            iy = *idx.offset((n * 3 + 1) as isize);
            iz = *idx.offset((n * 3 + 2) as isize);
            *gout.offset(n as isize) +=
                *g.offset(ix as isize) * *g.offset(iy as isize) * *g.offset(iz as isize);
            n += 1;
        }
    };
}
#[no_mangle]
pub unsafe fn CINTgout1e_nuc(
    gout: *mut f64,
    g: *mut f64,
    idx: *mut i32,
    envs: &CINTEnvVars,
    empty: i32,
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
        n = 0;
        while n < nf {
            gx = g.offset(*idx.offset((n * 3 + 0) as isize) as isize);
            gy = g.offset(*idx.offset((n * 3 + 1) as isize) as isize);
            gz = g.offset(*idx.offset((n * 3 + 2) as isize) as isize);
            s = 0 as f64;
            i = 0;
            while i < nrys_roots {
                s += *gx.offset(i as isize) * *gy.offset(i as isize) * *gz.offset(i as isize);
                i += 1;
                i;
            }
            *gout.offset(n as isize) = s;
            n += 1;
            n;
        }
    } else {
        n = 0;
        while n < nf {
            gx = g.offset(*idx.offset((n * 3 + 0) as isize) as isize);
            gy = g.offset(*idx.offset((n * 3 + 1) as isize) as isize);
            gz = g.offset(*idx.offset((n * 3 + 2) as isize) as isize);
            s = 0 as f64;
            i = 0;
            while i < nrys_roots {
                s += *gx.offset(i as isize) * *gy.offset(i as isize) * *gz.offset(i as isize);
                i += 1;
                i;
            }
            *gout.offset(n as isize) += s;
            n += 1;
            n;
        }
    };
}
//#[no_mangle]
//pub unsafe extern "C" fn int1e_ovlp_sph(
//    out: *mut f64,
//    dims: *mut i32,
//    shls: *mut i32,
//    atm: *mut i32,
//    natm: i32,
//    bas: *mut i32,
//    nbas: i32,
//    env: *mut f64,
//    _opt: *mut CINTOpt,
//    cache: *mut f64,
//) -> i32 {
//    let ng: [i32; 8] = [0, 0, 0, 0, 0, 1, 1, 1];
//    let mut envs: CINTEnvVars = CINTEnvVars::new();
//    CINTinit_int1e_EnvVars(&mut envs, &ng, shls, atm, natm, bas, nbas, env);
//    envs.f_gout = ::core::mem::transmute::<
//        Option<unsafe fn(*mut f64, *const f64, *const i32, &CINTEnvVars, i32) -> ()>,
//        Option<unsafe fn() -> ()>,
//    >(Some(
//        CINTgout1e as unsafe fn(*mut f64, *const f64, *const i32, &CINTEnvVars, i32) -> (),
//    ));
//    return CINT1e_drv(
//        out,
//        dims,
//        &mut envs,
//        cache,
//        ::core::mem::transmute::<
//            Option<unsafe fn(*mut f64, *mut f64, *const i32, *const CINTEnvVars, *mut f64) -> ()>,
//            Option<unsafe fn() -> ()>,
//        >(Some(
//            c2s_sph_1e
//                as unsafe fn(*mut f64, *mut f64, *const i32, *const CINTEnvVars, *mut f64) -> (),
//        )),
//        0,
//    );
//}
#[no_mangle]
pub unsafe fn int1e_ovlp_cart(
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
    let ng: [i32; 8] = [0, 0, 0, 0, 0, 1, 1, 1];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int1e_EnvVars(&mut envs, &ng, shls, atm, natm, bas, nbas, env);
    envs.f_gout = ::core::mem::transmute::<
        Option<unsafe fn(*mut f64, *const f64, *const i32, &CINTEnvVars, i32) -> ()>,
        Option<unsafe fn() -> ()>,
    >(Some(
        CINTgout1e as unsafe fn(*mut f64, *const f64, *const i32, &CINTEnvVars, i32) -> (),
    ));
    return CINT1e_drv(
        out,
        dims,
        &mut envs,
        cache,
        ::core::mem::transmute::<
            Option<unsafe fn(*mut f64, *mut f64, *const i32, *const CINTEnvVars, *mut f64) -> ()>,
            Option<unsafe fn() -> ()>,
        >(Some(
            c2s_cart_1e
                as unsafe fn(*mut f64, *mut f64, *const i32, *const CINTEnvVars, *mut f64) -> (),
        )),
        0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_ovlp_optimizer(
    opt: *mut *mut CINTOpt,
    _atm: *mut i32,
    _natm: i32,
    _bas: *mut i32,
    _nbas: i32,
    _env: *mut f64,
) {
    *opt = 0 as *mut CINTOpt;
}
#[no_mangle]
pub unsafe extern "C" fn int1e_nuc_sph(
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
    let ng: [i32; 8] = [0, 0, 0, 0, 0, 1, 0, 1];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int1e_EnvVars(&mut envs, &ng, shls, atm, natm, bas, nbas, env);
    envs.f_gout = ::core::mem::transmute::<
        Option<unsafe fn(*mut f64, *mut f64, *mut i32, &CINTEnvVars, i32) -> ()>,
        Option<unsafe fn() -> ()>,
    >(Some(
        CINTgout1e_nuc as unsafe fn(*mut f64, *mut f64, *mut i32, &CINTEnvVars, i32) -> (),
    ));
    return CINT1e_drv(
        out,
        dims,
        &mut envs,
        cache,
        ::core::mem::transmute::<
            Option<unsafe fn(*mut f64, *mut f64, *const i32, *const CINTEnvVars, *mut f64) -> ()>,
            Option<unsafe fn() -> ()>,
        >(Some(
            c2s_sph_1e
                as unsafe fn(*mut f64, *mut f64, *const i32, *const CINTEnvVars, *mut f64) -> (),
        )),
        2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_nuc_cart(
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
    let ng: [i32; 8] = [0, 0, 0, 0, 0, 1, 0, 1];
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    CINTinit_int1e_EnvVars(&mut envs, &ng, shls, atm, natm, bas, nbas, env);
    envs.f_gout = ::core::mem::transmute::<
        Option<unsafe fn(*mut f64, *mut f64, *mut i32, &CINTEnvVars, i32) -> ()>,
        Option<unsafe fn() -> ()>,
    >(Some(
        CINTgout1e_nuc as unsafe fn(*mut f64, *mut f64, *mut i32, &CINTEnvVars, i32) -> (),
    ));
    return CINT1e_drv(
        out,
        dims,
        &mut envs,
        cache,
        ::core::mem::transmute::<
            Option<unsafe fn(*mut f64, *mut f64, *const i32, *const CINTEnvVars, *mut f64) -> ()>,
            Option<unsafe fn() -> ()>,
        >(Some(
            c2s_cart_1e
                as unsafe fn(*mut f64, *mut f64, *const i32, *const CINTEnvVars, *mut f64) -> (),
        )),
        2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_nuc_optimizer(
    opt: *mut *mut CINTOpt,
    _atm: *mut i32,
    _natm: i32,
    _bas: *mut i32,
    _nbas: i32,
    _env: *mut f64,
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
    opt: *mut *mut CINTOpt,
    atm: *mut i32,
    natm: i32,
    bas: *mut i32,
    nbas: i32,
    env: *mut f64,
) {
    int1e_nuc_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_nuc_optimizer(
    opt: *mut *mut CINTOpt,
    atm: *mut i32,
    natm: i32,
    bas: *mut i32,
    nbas: i32,
    env: *mut f64,
) {
    int1e_nuc_optimizer(opt, atm, natm, bas, nbas, env);
}
//#[no_mangle]
//pub unsafe extern "C" fn cint1e_ovlp_sph_(
//    out: *mut f64,
//    shls: *mut i32,
//    atm: *mut i32,
//    natm: *mut i32,
//    bas: *mut i32,
//    nbas: *mut i32,
//    env: *mut f64,
//    optptr_as_integer8: u64,
//) -> i32 {
//    let mut opt: *mut *mut CINTOpt = optptr_as_integer8 as *mut *mut CINTOpt;
//    return int1e_ovlp_sph(
//        out,
//        0 as *mut i32,
//        shls,
//        atm,
//        *natm,
//        bas,
//        *nbas,
//        env,
//        *opt,
//        0 as *mut f64,
//    );
//}
#[no_mangle]
pub unsafe extern "C" fn cint1e_ovlp_sph_optimizer_(
    optptr_as_integer8: u64,
    atm: *mut i32,
    natm: *mut i32,
    bas: *mut i32,
    nbas: *mut i32,
    env: *mut f64,
) {
    let mut opt: *mut *mut CINTOpt = optptr_as_integer8 as *mut *mut CINTOpt;
    int1e_ovlp_optimizer(opt, atm, *natm, bas, *nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_ovlp_cart_(
    out: *mut f64,
    shls: *mut i32,
    atm: *mut i32,
    natm: *mut i32,
    bas: *mut i32,
    nbas: *mut i32,
    env: *mut f64,
    optptr_as_integer8: u64,
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
#[inline(never)]
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
//#[no_mangle]
//pub fn cint1e_ovlp_sph(
//    out: &mut [f64],
//    shls: &mut [i32],
//    atm: &mut [i32],
//    natm: i32,
//    bas: &mut [i32],
//    nbas: i32,
//    env: &mut [f64],
//    opt: *mut CINTOpt,
//) -> i32 {
//    unsafe {
//        return int1e_ovlp_sph(
//            out.as_mut_ptr(),
//            0 as *mut i32,
//            shls.as_mut_ptr(),
//            atm.as_mut_ptr(),
//            natm,
//            bas.as_mut_ptr(),
//            nbas,
//            env.as_mut_ptr(),
//            opt,
//            0 as *mut f64,
//        );
//    }
//}

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
