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
    let mut x_ctr: *mut i32 = ((*envs).x_ctr).as_mut_ptr();
    let mut nc: i32 = (*envs).nf * *x_ctr.offset(0 as isize)
        * *x_ctr.offset(1 as isize);
    let mut n_comp: i32 = 42;
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
    let mut g: *mut f64 = 0 as *mut f64;
    let mut gout: *mut f64 = 0 as *mut f64;
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
    return (*jempty == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn CINTg1e_nuc(
    mut g: *mut f64,
    mut envs: *mut CINTEnvVars,
    mut nuc_id: i32,
) -> i32 {
    let mut nrys_roots: i32 = (*envs).nrys_roots;
    let mut atm: *mut i32 = (*envs).atm;
    let mut env: *mut f64 = (*envs).env;
    let mut rij: *mut f64 = ((*envs).rij).as_mut_ptr();
    let mut gx: *mut f64 = g;
    let mut gy: *mut f64 = g.offset((*envs).g_size as isize);
    let mut gz: *mut f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut u: [f64; 32] = [0.; 32];
    let mut w: *mut f64 = gz;
    let mut cr: *mut f64 = 0 as *mut f64;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    let mut crij: [f64; 3] = [0.; 3];
    let mut x: f64 = 0.;
    let mut fac1: f64 = 0.;
    let mut aij: f64 = (*envs).ai[0 as usize]
        + (*envs).aj[0 as usize];
    let mut tau: f64 = CINTnuc_mod(aij, nuc_id, atm, env);
    if nuc_id < 0 as i32 {
        fac1 = 2 as f64 * 3.14159265358979323846f64
            * (*envs).fac[0 as usize] * tau / aij;
        cr = env.offset(4 as isize);
    } else if *atm.offset((6 as i32 * nuc_id + 2 as i32) as isize)
        == 3 as i32
    {
        fac1 = 2 as f64 * 3.14159265358979323846f64
            * -*env
                .offset(
                    *atm.offset((4 as i32 + nuc_id * 6 as i32) as isize)
                        as isize,
                ) * (*envs).fac[0 as usize] * tau / aij;
        cr = env
            .offset(
                *atm.offset((6 as i32 * nuc_id + 1 as i32) as isize)
                    as isize,
            );
    } else {
        fac1 = 2 as f64 * 3.14159265358979323846f64
            * -(*atm.offset((0 as i32 + nuc_id * 6 as i32) as isize)).abs()
                as f64 * (*envs).fac[0 as usize] * tau / aij;
        cr = env
            .offset(
                *atm.offset((6 as i32 * nuc_id + 1 as i32) as isize)
                    as isize,
            );
    }
    crij[0 as i32
        as usize] = *cr.offset(0 as isize)
        - *rij.offset(0 as isize);
    crij[1 as i32
        as usize] = *cr.offset(1 as isize)
        - *rij.offset(1 as isize);
    crij[2 as i32
        as usize] = *cr.offset(2 as isize)
        - *rij.offset(2 as isize);
    x = aij * tau * tau * SQUARE(crij.as_mut_ptr()) as f64;
    CINTrys_roots(nrys_roots, x, u.as_mut_ptr(), w);
    i = 0 as i32;
    while i < nrys_roots {
        *gx.offset(i as isize) = 1 as f64;
        *gy.offset(i as isize) = 1 as f64;
        *gz.offset(i as isize) *= fac1;
        i += 1;
        i;
    }
    let mut nmax: i32 = (*envs).li_ceil + (*envs).lj_ceil;
    if nmax == 0 as i32 {
        return 1 as i32;
    }
    let mut p0x: *mut f64 = 0 as *mut f64;
    let mut p0y: *mut f64 = 0 as *mut f64;
    let mut p0z: *mut f64 = 0 as *mut f64;
    let mut p1x: *mut f64 = 0 as *mut f64;
    let mut p1y: *mut f64 = 0 as *mut f64;
    let mut p1z: *mut f64 = 0 as *mut f64;
    let mut p2x: *mut f64 = 0 as *mut f64;
    let mut p2y: *mut f64 = 0 as *mut f64;
    let mut p2z: *mut f64 = 0 as *mut f64;
    let mut lj: i32 = 0;
    let mut di: i32 = 0;
    let mut dj: i32 = 0;
    let mut rx: *mut f64 = 0 as *mut f64;
    if (*envs).li_ceil > (*envs).lj_ceil {
        lj = (*envs).lj_ceil;
        di = (*envs).g_stride_i;
        dj = (*envs).g_stride_j;
        rx = (*envs).ri;
    } else {
        lj = (*envs).li_ceil;
        di = (*envs).g_stride_j;
        dj = (*envs).g_stride_i;
        rx = (*envs).rj;
    }
    let mut rijrx: f64 = *rij.offset(0 as isize)
        - *rx.offset(0 as isize);
    let mut rijry: f64 = *rij.offset(1 as isize)
        - *rx.offset(1 as isize);
    let mut rijrz: f64 = *rij.offset(2 as isize)
        - *rx.offset(2 as isize);
    let mut aij2: f64 = 0.5f64 / aij;
    let mut ru: f64 = 0.;
    let mut rt: f64 = 0.;
    let mut r0: f64 = 0.;
    let mut r1: f64 = 0.;
    let mut r2: f64 = 0.;
    p0x = gx.offset(di as isize);
    p0y = gy.offset(di as isize);
    p0z = gz.offset(di as isize);
    p1x = gx.offset(-(di as isize));
    p1y = gy.offset(-(di as isize));
    p1z = gz.offset(-(di as isize));
    n = 0 as i32;
    while n < nrys_roots {
        ru = tau * tau * u[n as usize]
            / (1 as f64 + u[n as usize]);
        rt = aij2 - aij2 * ru;
        r0 = rijrx + ru * crij[0 as usize];
        r1 = rijry + ru * crij[1 as usize];
        r2 = rijrz + ru * crij[2 as usize];
        *p0x.offset(n as isize) = r0 * *gx.offset(n as isize);
        *p0y.offset(n as isize) = r1 * *gy.offset(n as isize);
        *p0z.offset(n as isize) = r2 * *gz.offset(n as isize);
        i = 1 as i32;
        while i < nmax {
            *p0x
                .offset(
                    (n + i * di) as isize,
                ) = i as f64 * rt * *p1x.offset((n + i * di) as isize)
                + r0 * *gx.offset((n + i * di) as isize);
            *p0y
                .offset(
                    (n + i * di) as isize,
                ) = i as f64 * rt * *p1y.offset((n + i * di) as isize)
                + r1 * *gy.offset((n + i * di) as isize);
            *p0z
                .offset(
                    (n + i * di) as isize,
                ) = i as f64 * rt * *p1z.offset((n + i * di) as isize)
                + r2 * *gz.offset((n + i * di) as isize);
            i += 1;
            i;
        }
        n += 1;
        n;
    }
    let mut rirjx: f64 = (*envs).rirj[0 as usize];
    let mut rirjy: f64 = (*envs).rirj[1 as usize];
    let mut rirjz: f64 = (*envs).rirj[2 as usize];
    j = 1 as i32;
    while j <= lj {
        p0x = gx.offset((j * dj) as isize);
        p0y = gy.offset((j * dj) as isize);
        p0z = gz.offset((j * dj) as isize);
        p1x = p0x.offset(-(dj as isize));
        p1y = p0y.offset(-(dj as isize));
        p1z = p0z.offset(-(dj as isize));
        p2x = p1x.offset(di as isize);
        p2y = p1y.offset(di as isize);
        p2z = p1z.offset(di as isize);
        i = 0 as i32;
        while i <= nmax - j {
            n = 0 as i32;
            while n < nrys_roots {
                *p0x
                    .offset(
                        (n + i * di) as isize,
                    ) = *p2x.offset((n + i * di) as isize)
                    + rirjx * *p1x.offset((n + i * di) as isize);
                *p0y
                    .offset(
                        (n + i * di) as isize,
                    ) = *p2y.offset((n + i * di) as isize)
                    + rirjy * *p1y.offset((n + i * di) as isize);
                *p0z
                    .offset(
                        (n + i * di) as isize,
                    ) = *p2z.offset((n + i * di) as isize)
                    + rirjz * *p1z.offset((n + i * di) as isize);
                n += 1;
                n;
            }
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    return 1 as i32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CINTEnvVars {
    pub atm: *mut i32,
    pub bas: *mut i32,
    pub env: *mut f64,
    pub shls: *mut i32,
    pub natm: i32,
    pub nbas: i32,
    pub i_l: i32,
    pub j_l: i32,
    pub k_l: i32,
    pub l_l: i32,
    pub nfi: i32,
    pub nfj: i32,
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed_0,
    pub nf: i32,
    pub rys_order: i32,
    pub x_ctr: [i32; 4],
    pub gbits: i32,
    pub ncomp_e1: i32,
    pub ncomp_e2: i32,
    pub ncomp_tensor: i32,
    pub li_ceil: i32,
    pub lj_ceil: i32,
    pub lk_ceil: i32,
    pub ll_ceil: i32,
    pub g_stride_i: i32,
    pub g_stride_k: i32,
    pub g_stride_l: i32,
    pub g_stride_j: i32,
    pub nrys_roots: i32,
    pub g_size: i32,
    pub g2d_ijmax: i32,
    pub g2d_klmax: i32,
    pub common_factor: f64,
    pub expcutoff: f64,
    pub rirj: [f64; 3],
    pub rkrl: [f64; 3],
    pub rx_in_rijrx: *mut f64,
    pub rx_in_rklrx: *mut f64,
    pub ri: *mut f64,
    pub rj: *mut f64,
    pub rk: *mut f64,
    pub c2rust_unnamed_1: C2RustUnnamed,
    pub f_g0_2e: Option::<unsafe extern "C" fn() -> i32>,
    pub f_g0_2d4d: Option::<unsafe extern "C" fn() -> ()>,
    pub f_gout: Option::<unsafe extern "C" fn() -> ()>,
    pub opt: *mut CINTOpt,
    pub idx: *mut i32,
    pub ai: [f64; 1],
    pub aj: [f64; 1],
    pub ak: [f64; 1],
    pub al: [f64; 1],
    pub fac: [f64; 1],
    pub rij: [f64; 3],
    pub rkl: [f64; 3],
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
