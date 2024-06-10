#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::g1e::CINTinit_int1e_EnvVars;
use crate::g1e::CINTcommon_fac_sp;
use crate::rys_roots::CINTsr_rys_roots;
use crate::rys_roots::CINTrys_roots;

use crate::cint::CINTEnvVars;

pub type uintptr_t = libc::c_ulong;
pub type size_t = libc::c_ulong;

extern "C" {
    fn sqrt(_: f64) -> f64;
}

#[no_mangle]
pub unsafe extern "C" fn CINTinit_int1e_grids_EnvVars(
    mut envs: *mut CINTEnvVars,
    mut ng: *mut i32,
    mut shls: *mut i32,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
) {
    CINTinit_int1e_EnvVars(envs, ng, shls, atm, natm, bas, nbas, env);
    let mut ngrids: i32 = *shls.offset(3 as i32 as isize)
        - *shls.offset(2 as i32 as isize);
    let mut grids: *mut f64 = env
        .offset(*env.offset(12 as i32 as isize) as size_t as isize)
        .offset((*shls.offset(2 as i32 as isize) * 3 as i32) as isize);
    (*envs).c2rust_unnamed_0.ngrids = ngrids;
    (*envs).c2rust_unnamed_1.grids = grids;
    (*envs)
        .common_factor = 2 as i32 as f64 * 3.14159265358979323846f64
        * CINTcommon_fac_sp((*envs).i_l) * CINTcommon_fac_sp((*envs).j_l);
    let mut rys_order: i32 = (*envs).nrys_roots;
    let mut nroots: i32 = rys_order;
    let mut omega: f64 = *env.offset(8 as i32 as isize);
    if omega < 0 as i32 as f64 && rys_order <= 3 as i32 {
        nroots *= 2 as i32;
    }
    (*envs).rys_order = rys_order;
    (*envs).nrys_roots = nroots;
    let mut dli: i32 = 0;
    let mut dlj: i32 = 0;
    let mut ibase: i32 = ((*envs).li_ceil > (*envs).lj_ceil) as i32;
    if ibase != 0 {
        dli = (*envs).li_ceil + (*envs).lj_ceil + 1 as i32;
        dlj = (*envs).lj_ceil + 1 as i32;
        (*envs)
            .rirj[0 as i32
            as usize] = *((*envs).ri).offset(0 as i32 as isize)
            - *((*envs).rj).offset(0 as i32 as isize);
        (*envs)
            .rirj[1 as i32
            as usize] = *((*envs).ri).offset(1 as i32 as isize)
            - *((*envs).rj).offset(1 as i32 as isize);
        (*envs)
            .rirj[2 as i32
            as usize] = *((*envs).ri).offset(2 as i32 as isize)
            - *((*envs).rj).offset(2 as i32 as isize);
    } else {
        dli = (*envs).li_ceil + 1 as i32;
        dlj = (*envs).li_ceil + (*envs).lj_ceil + 1 as i32;
        (*envs)
            .rirj[0 as i32
            as usize] = *((*envs).rj).offset(0 as i32 as isize)
            - *((*envs).ri).offset(0 as i32 as isize);
        (*envs)
            .rirj[1 as i32
            as usize] = *((*envs).rj).offset(1 as i32 as isize)
            - *((*envs).ri).offset(1 as i32 as isize);
        (*envs)
            .rirj[2 as i32
            as usize] = *((*envs).rj).offset(2 as i32 as isize)
            - *((*envs).ri).offset(2 as i32 as isize);
    }
    (*envs).g_stride_i = 104 as i32 * nroots;
    (*envs).g_stride_j = 104 as i32 * nroots * dli;
    (*envs).g_size = 104 as i32 * nroots * dli * dlj;
    (*envs).g_stride_k = (*envs).g_size;
    (*envs).g_stride_l = (*envs).g_size;
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_1e_grids(
    mut g: *mut f64,
    mut cutoff: f64,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut f64,
    mut gridsT: *mut f64,
) -> i32 {
    let mut ngrids: i32 = (*envs).c2rust_unnamed_0.ngrids;
    let mut bgrids: i32 = if ngrids - (*envs).c2rust_unnamed.grids_offset
        < 104 as i32
    {
        ngrids - (*envs).c2rust_unnamed.grids_offset
    } else {
        104 as i32
    };
    let mut nroots: i32 = (*envs).nrys_roots;
    let mut gx: *mut f64 = g;
    let mut gy: *mut f64 = g.offset((*envs).g_size as isize);
    let mut gz: *mut f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut w: *mut f64 = gz;
    let mut rij: *mut f64 = ((*envs).rij).as_mut_ptr();
    let mut ubuf: [f64; 32] = [0.; 32];
    let mut wbuf: [f64; 32] = [0.; 32];
    let mut u: *mut f64 = 0 as *mut f64;
    u = ((cache as uintptr_t).wrapping_add(63 as i32 as libc::c_ulong)
        & (64 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut f64;
    cache = u.offset((104 as i32 * nroots) as isize);
    let mut rijrg: *mut f64 = 0 as *mut f64;
    rijrg = ((cache as uintptr_t).wrapping_add(63 as i32 as libc::c_ulong)
        & (64 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut f64;
    cache = rijrg.offset((104 as i32 * 3 as i32) as isize);
    let mut aij: f64 = (*envs).ai[0 as i32 as usize]
        + (*envs).aj[0 as i32 as usize];
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut ig: i32 = 0;
    let mut x: f64 = 0.;
    let mut fac1: f64 = 0.;
    i = 0 as i32;
    while i < nroots {
        ig = 0 as i32;
        while ig < bgrids {
            *gx
                .offset(
                    (ig + 104 as i32 * i) as isize,
                ) = 1 as i32 as f64;
            *gy
                .offset(
                    (ig + 104 as i32 * i) as isize,
                ) = 1 as i32 as f64;
            ig += 1;
        }
        i += 1;
    }
    ig = 0 as i32;
    while ig < bgrids {
        *rijrg
            .offset(
                (ig + 104 as i32 * 0 as i32) as isize,
            ) = *gridsT.offset((ig + 104 as i32 * 0 as i32) as isize)
            - *rij.offset(0 as i32 as isize);
        *rijrg
            .offset(
                (ig + 104 as i32 * 1 as i32) as isize,
            ) = *gridsT.offset((ig + 104 as i32 * 1 as i32) as isize)
            - *rij.offset(1 as i32 as isize);
        *rijrg
            .offset(
                (ig + 104 as i32 * 2 as i32) as isize,
            ) = *gridsT.offset((ig + 104 as i32 * 2 as i32) as isize)
            - *rij.offset(2 as i32 as isize);
        ig += 1;
    }
    let mut omega: f64 = *((*envs).env).offset(8 as i32 as isize);
    let mut zeta: f64 = *((*envs).env).offset(7 as i32 as isize);
    let mut omega2: f64 = 0.;
    let mut theta: f64 = 0.;
    let mut sqrt_theta: f64 = 0.;
    let mut a0: f64 = 0.;
    let mut tau2: f64 = 0.;
    if omega == 0.0f64 && zeta == 0.0f64 {
        fac1 = (*envs).fac[0 as i32 as usize] / aij;
        ig = 0 as i32;
        while ig < bgrids {
            x = aij
                * (*rijrg.offset((ig + 104 as i32 * 0 as i32) as isize)
                    * *rijrg
                        .offset((ig + 104 as i32 * 0 as i32) as isize)
                    + *rijrg
                        .offset((ig + 104 as i32 * 1 as i32) as isize)
                        * *rijrg
                            .offset(
                                (ig + 104 as i32 * 1 as i32) as isize,
                            )
                    + *rijrg
                        .offset((ig + 104 as i32 * 2 as i32) as isize)
                        * *rijrg
                            .offset(
                                (ig + 104 as i32 * 2 as i32) as isize,
                            ));
            CINTrys_roots(nroots, x, ubuf.as_mut_ptr(), wbuf.as_mut_ptr());
            i = 0 as i32;
            while i < nroots {
                *u
                    .offset(
                        (ig + 104 as i32 * i) as isize,
                    ) = ubuf[i as usize]
                    / (ubuf[i as usize] + 1 as i32 as f64);
                *w
                    .offset(
                        (ig + 104 as i32 * i) as isize,
                    ) = wbuf[i as usize] * fac1;
                i += 1;
            }
            ig += 1;
        }
    } else if omega < 0.0f64 {
        a0 = aij;
        fac1 = (*envs).fac[0 as i32 as usize] / aij;
        if zeta == 0.0f64 {
            tau2 = 1.0f64;
            omega2 = omega * omega;
            theta = omega2 / (omega2 + aij);
        } else {
            tau2 = zeta / (zeta + aij);
            a0 *= tau2;
            fac1 *= sqrt(tau2);
            omega2 = omega * omega;
            theta = omega2 / (omega2 + a0);
        }
        sqrt_theta = sqrt(theta);
        let mut temp_cutoff: f64 = if cutoff
            < 40 as i32 as f64
        {
            cutoff
        } else {
            40 as i32 as f64
        };
        let mut rorder: i32 = (*envs).rys_order;
        let mut tau_theta: f64 = 0.;
        let mut fac_theta: f64 = 0.;
        ig = 0 as i32;
        while ig < bgrids {
            x = a0
                * (*rijrg.offset((ig + 104 as i32 * 0 as i32) as isize)
                    * *rijrg
                        .offset((ig + 104 as i32 * 0 as i32) as isize)
                    + *rijrg
                        .offset((ig + 104 as i32 * 1 as i32) as isize)
                        * *rijrg
                            .offset(
                                (ig + 104 as i32 * 1 as i32) as isize,
                            )
                    + *rijrg
                        .offset((ig + 104 as i32 * 2 as i32) as isize)
                        * *rijrg
                            .offset(
                                (ig + 104 as i32 * 2 as i32) as isize,
                            ));
            if theta * x > temp_cutoff {
                i = 0 as i32;
                while i < nroots {
                    *u
                        .offset(
                            (ig + 104 as i32 * i) as isize,
                        ) = 0 as i32 as f64;
                    *w
                        .offset(
                            (ig + 104 as i32 * i) as isize,
                        ) = 0 as i32 as f64;
                    i += 1;
                }
            } else if rorder == nroots {
                CINTsr_rys_roots(
                    nroots,
                    x,
                    sqrt_theta,
                    ubuf.as_mut_ptr(),
                    wbuf.as_mut_ptr(),
                );
                i = 0 as i32;
                while i < nroots {
                    *u
                        .offset(
                            (ig + 104 as i32 * i) as isize,
                        ) = ubuf[i as usize]
                        / (ubuf[i as usize] + 1 as i32 as f64) * tau2;
                    *w
                        .offset(
                            (ig + 104 as i32 * i) as isize,
                        ) = wbuf[i as usize] * fac1;
                    i += 1;
                }
            } else {
                tau_theta = tau2 * theta;
                fac_theta = fac1 * -sqrt_theta;
                CINTrys_roots(rorder, x, ubuf.as_mut_ptr(), wbuf.as_mut_ptr());
                CINTrys_roots(
                    rorder,
                    theta * x,
                    ubuf.as_mut_ptr().offset(rorder as isize),
                    wbuf.as_mut_ptr().offset(rorder as isize),
                );
                i = 0 as i32;
                while i < rorder {
                    *u
                        .offset(
                            (ig + 104 as i32 * i) as isize,
                        ) = ubuf[i as usize]
                        / (ubuf[i as usize] + 1 as i32 as f64) * tau2;
                    *w
                        .offset(
                            (ig + 104 as i32 * i) as isize,
                        ) = wbuf[i as usize] * fac1;
                    *u
                        .offset(
                            (ig + 104 as i32 * (i + rorder)) as isize,
                        ) = ubuf[(i + rorder) as usize]
                        / (ubuf[(i + rorder) as usize]
                            + 1 as i32 as f64) * tau_theta;
                    *w
                        .offset(
                            (ig + 104 as i32 * (i + rorder)) as isize,
                        ) = wbuf[(i + rorder) as usize] * fac_theta;
                    i += 1;
                }
            }
            ig += 1;
        }
    } else {
        a0 = aij;
        fac1 = (*envs).fac[0 as i32 as usize] / aij;
        if zeta == 0.0f64 {
            omega2 = omega * omega;
            theta = omega2 / (omega2 + aij);
            a0 *= theta;
            fac1 *= sqrt(theta);
        } else if omega == 0.0f64 {
            theta = zeta / (zeta + aij);
            a0 *= theta;
            fac1 *= sqrt(theta);
        } else {
            omega2 = omega * omega;
            theta = omega2 * zeta / (omega2 * zeta + (zeta + omega2) * aij);
            a0 *= theta;
            fac1 *= sqrt(theta);
        }
        ig = 0 as i32;
        while ig < bgrids {
            x = a0
                * (*rijrg.offset((ig + 104 as i32 * 0 as i32) as isize)
                    * *rijrg
                        .offset((ig + 104 as i32 * 0 as i32) as isize)
                    + *rijrg
                        .offset((ig + 104 as i32 * 1 as i32) as isize)
                        * *rijrg
                            .offset(
                                (ig + 104 as i32 * 1 as i32) as isize,
                            )
                    + *rijrg
                        .offset((ig + 104 as i32 * 2 as i32) as isize)
                        * *rijrg
                            .offset(
                                (ig + 104 as i32 * 2 as i32) as isize,
                            ));
            CINTrys_roots(nroots, x, ubuf.as_mut_ptr(), wbuf.as_mut_ptr());
            i = 0 as i32;
            while i < nroots {
                *u
                    .offset(
                        (ig + 104 as i32 * i) as isize,
                    ) = ubuf[i as usize]
                    / (ubuf[i as usize] + 1 as i32 as f64) * theta;
                *w
                    .offset(
                        (ig + 104 as i32 * i) as isize,
                    ) = wbuf[i as usize] * fac1;
                i += 1;
            }
            ig += 1;
        }
    }
    let mut nmax: i32 = (*envs).li_ceil + (*envs).lj_ceil;
    if nmax == 0 as i32 {
        return 1 as i32;
    }
    let mut rirj: *mut f64 = ((*envs).rirj).as_mut_ptr();
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
    let mut rijrx: [f64; 3] = [0.; 3];
    rijrx[0 as i32
        as usize] = *rij.offset(0 as i32 as isize)
        - *rx.offset(0 as i32 as isize);
    rijrx[1 as i32
        as usize] = *rij.offset(1 as i32 as isize)
        - *rx.offset(1 as i32 as isize);
    rijrx[2 as i32
        as usize] = *rij.offset(2 as i32 as isize)
        - *rx.offset(2 as i32 as isize);
    let mut p0x: *mut f64 = 0 as *mut f64;
    let mut p0y: *mut f64 = 0 as *mut f64;
    let mut p0z: *mut f64 = 0 as *mut f64;
    let mut p1x: *mut f64 = 0 as *mut f64;
    let mut p1y: *mut f64 = 0 as *mut f64;
    let mut p1z: *mut f64 = 0 as *mut f64;
    let mut p2x: *mut f64 = 0 as *mut f64;
    let mut p2y: *mut f64 = 0 as *mut f64;
    let mut p2z: *mut f64 = 0 as *mut f64;
    let mut t2: *mut f64 = 0 as *mut f64;
    t2 = ((cache as uintptr_t).wrapping_add(63 as i32 as libc::c_ulong)
        & (64 as i32 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut f64;
    cache = t2.offset((104 as i32 * 4 as i32) as isize);
    let mut rirgx: *mut f64 = t2.offset(104 as i32 as isize);
    let mut rirgy: *mut f64 = rirgx.offset(104 as i32 as isize);
    let mut rirgz: *mut f64 = rirgy.offset(104 as i32 as isize);
    let mut aij2: f64 = 0.5f64 / aij;
    let mut tx: f64 = 0.;
    let mut ty: f64 = 0.;
    let mut tz: f64 = 0.;
    n = 0 as i32;
    while n < nroots {
        p0x = gx.offset((104 as i32 * n) as isize);
        p0y = gy.offset((104 as i32 * n) as isize);
        p0z = gz.offset((104 as i32 * n) as isize);
        p1x = p0x.offset(di as isize);
        p1y = p0y.offset(di as isize);
        p1z = p0z.offset(di as isize);
        ig = 0 as i32;
        while ig < bgrids {
            *rirgx
                .offset(
                    ig as isize,
                ) = rijrx[0 as i32 as usize]
                + *u.offset((ig + 104 as i32 * n) as isize)
                    * *rijrg
                        .offset((ig + 104 as i32 * 0 as i32) as isize);
            *rirgy
                .offset(
                    ig as isize,
                ) = rijrx[1 as i32 as usize]
                + *u.offset((ig + 104 as i32 * n) as isize)
                    * *rijrg
                        .offset((ig + 104 as i32 * 1 as i32) as isize);
            *rirgz
                .offset(
                    ig as isize,
                ) = rijrx[2 as i32 as usize]
                + *u.offset((ig + 104 as i32 * n) as isize)
                    * *rijrg
                        .offset((ig + 104 as i32 * 2 as i32) as isize);
            *p1x
                .offset(
                    ig as isize,
                ) = *rirgx.offset(ig as isize) * *p0x.offset(ig as isize);
            *p1y
                .offset(
                    ig as isize,
                ) = *rirgy.offset(ig as isize) * *p0y.offset(ig as isize);
            *p1z
                .offset(
                    ig as isize,
                ) = *rirgz.offset(ig as isize) * *p0z.offset(ig as isize);
            ig += 1;
        }
        if nmax > 0 as i32 {
            ig = 0 as i32;
            while ig < bgrids {
                *t2
                    .offset(
                        ig as isize,
                    ) = aij2
                    * (1 as i32 as f64
                        - *u.offset((ig + 104 as i32 * n) as isize));
                ig += 1;
            }
        }
        i = 1 as i32;
        while i < nmax {
            p0x = gx.offset((104 as i32 * n) as isize).offset((i * di) as isize);
            p0y = gy.offset((104 as i32 * n) as isize).offset((i * di) as isize);
            p0z = gz.offset((104 as i32 * n) as isize).offset((i * di) as isize);
            p1x = p0x.offset(di as isize);
            p1y = p0y.offset(di as isize);
            p1z = p0z.offset(di as isize);
            p2x = p0x.offset(-(di as isize));
            p2y = p0y.offset(-(di as isize));
            p2z = p0z.offset(-(di as isize));
            ig = 0 as i32;
            while ig < bgrids {
                *p1x
                    .offset(
                        ig as isize,
                    ) = i as f64 * *t2.offset(ig as isize)
                    * *p2x.offset(ig as isize)
                    + *rirgx.offset(ig as isize) * *p0x.offset(ig as isize);
                *p1y
                    .offset(
                        ig as isize,
                    ) = i as f64 * *t2.offset(ig as isize)
                    * *p2y.offset(ig as isize)
                    + *rirgy.offset(ig as isize) * *p0y.offset(ig as isize);
                *p1z
                    .offset(
                        ig as isize,
                    ) = i as f64 * *t2.offset(ig as isize)
                    * *p2z.offset(ig as isize)
                    + *rirgz.offset(ig as isize) * *p0z.offset(ig as isize);
                ig += 1;
            }
            i += 1;
        }
        n += 1;
    }
    j = 1 as i32;
    while j <= lj {
        i = 0 as i32;
        while i <= nmax - j {
            p0x = gx.offset((j * dj) as isize).offset((i * di) as isize);
            p0y = gy.offset((j * dj) as isize).offset((i * di) as isize);
            p0z = gz.offset((j * dj) as isize).offset((i * di) as isize);
            p1x = p0x.offset(-(dj as isize));
            p1y = p0y.offset(-(dj as isize));
            p1z = p0z.offset(-(dj as isize));
            p2x = p1x.offset(di as isize);
            p2y = p1y.offset(di as isize);
            p2z = p1z.offset(di as isize);
            n = 0 as i32;
            while n < nroots {
                ig = 0 as i32;
                while ig < bgrids {
                    *p0x
                        .offset(
                            (ig + 104 as i32 * n) as isize,
                        ) = *p2x.offset((ig + 104 as i32 * n) as isize)
                        + *rirj.offset(0 as i32 as isize)
                            * *p1x.offset((ig + 104 as i32 * n) as isize);
                    *p0y
                        .offset(
                            (ig + 104 as i32 * n) as isize,
                        ) = *p2y.offset((ig + 104 as i32 * n) as isize)
                        + *rirj.offset(1 as i32 as isize)
                            * *p1y.offset((ig + 104 as i32 * n) as isize);
                    *p0z
                        .offset(
                            (ig + 104 as i32 * n) as isize,
                        ) = *p2z.offset((ig + 104 as i32 * n) as isize)
                        + *rirj.offset(2 as i32 as isize)
                            * *p1z.offset((ig + 104 as i32 * n) as isize);
                    ig += 1;
                }
                n += 1;
            }
            i += 1;
        }
        j += 1;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_grids(
    mut gout: *mut f64,
    mut g: *mut f64,
    mut idx: *mut i32,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: i32,
) {
    let mut ngrids: i32 = (*envs).c2rust_unnamed_0.ngrids;
    let mut bgrids: i32 = if ngrids - (*envs).c2rust_unnamed.grids_offset
        < 104 as i32
    {
        ngrids - (*envs).c2rust_unnamed.grids_offset
    } else {
        104 as i32
    };
    let mut nroots: i32 = (*envs).nrys_roots;
    let mut nf: i32 = (*envs).nf;
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    let mut ig: i32 = 0;
    let mut gx: *mut f64 = 0 as *mut f64;
    let mut gy: *mut f64 = 0 as *mut f64;
    let mut gz: *mut f64 = 0 as *mut f64;
    let mut s: [f64; 104] = [0.; 104];
    if gout_empty != 0 {
        n = 0 as i32;
        while n < nf {
            gx = g.offset(*idx.offset(0 as i32 as isize) as isize);
            gy = g.offset(*idx.offset(1 as i32 as isize) as isize);
            gz = g.offset(*idx.offset(2 as i32 as isize) as isize);
            ig = 0 as i32;
            while ig < bgrids {
                s[ig as usize] = 0 as i32 as f64;
                ig += 1;
            }
            i = 0 as i32;
            while i < nroots {
                ig = 0 as i32;
                while ig < bgrids {
                    s[ig as usize]
                        += *gx.offset((ig + 104 as i32 * i) as isize)
                            * *gy.offset((ig + 104 as i32 * i) as isize)
                            * *gz.offset((ig + 104 as i32 * i) as isize);
                    ig += 1;
                }
                i += 1;
            }
            ig = 0 as i32;
            while ig < bgrids {
                *gout.offset((ig + bgrids * n) as isize) = s[ig as usize];
                ig += 1;
            }
            n += 1;
            idx = idx.offset(3 as i32 as isize);
        }
    } else {
        n = 0 as i32;
        while n < nf {
            gx = g.offset(*idx.offset(0 as i32 as isize) as isize);
            gy = g.offset(*idx.offset(1 as i32 as isize) as isize);
            gz = g.offset(*idx.offset(2 as i32 as isize) as isize);
            ig = 0 as i32;
            while ig < bgrids {
                s[ig as usize] = 0 as i32 as f64;
                ig += 1;
            }
            i = 0 as i32;
            while i < nroots {
                ig = 0 as i32;
                while ig < bgrids {
                    s[ig as usize]
                        += *gx.offset((ig + 104 as i32 * i) as isize)
                            * *gy.offset((ig + 104 as i32 * i) as isize)
                            * *gz.offset((ig + 104 as i32 * i) as isize);
                    ig += 1;
                }
                i += 1;
            }
            ig = 0 as i32;
            while ig < bgrids {
                *gout.offset((ig + bgrids * n) as isize) += s[ig as usize];
                ig += 1;
            }
            n += 1;
            idx = idx.offset(3 as i32 as isize);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1i_grids(
    mut f: *mut f64,
    mut g: *mut f64,
    mut li: i32,
    mut lj: i32,
    mut envs: *mut CINTEnvVars,
) {
    let mut ngrids: i32 = (*envs).c2rust_unnamed_0.ngrids;
    let mut bgrids: i32 = if ngrids - (*envs).c2rust_unnamed.grids_offset
        < 104 as i32
    {
        ngrids - (*envs).c2rust_unnamed.grids_offset
    } else {
        104 as i32
    };
    let mut nroots: i32 = (*envs).nrys_roots;
    let di: i32 = (*envs).g_stride_i;
    let dj: i32 = (*envs).g_stride_j;
    let ai2: f64 = -(2 as i32) as f64
        * (*envs).ai[0 as i32 as usize];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    let mut ig: i32 = 0;
    let mut ptr: i32 = 0;
    let mut gx: *const f64 = g;
    let mut gy: *const f64 = g.offset((*envs).g_size as isize);
    let mut gz: *const f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut fx: *mut f64 = f;
    let mut fy: *mut f64 = f.offset((*envs).g_size as isize);
    let mut fz: *mut f64 = f
        .offset(((*envs).g_size * 2 as i32) as isize);
    j = 0 as i32;
    while j <= lj {
        n = 0 as i32;
        while n < nroots {
            ptr = dj * j + n * 104 as i32;
            ig = ptr;
            while ig < ptr + bgrids {
                *fx.offset(ig as isize) = ai2 * *gx.offset((ig + di) as isize);
                *fy.offset(ig as isize) = ai2 * *gy.offset((ig + di) as isize);
                *fz.offset(ig as isize) = ai2 * *gz.offset((ig + di) as isize);
                ig += 1;
            }
            n += 1;
        }
        i = 1 as i32;
        while i <= li {
            n = 0 as i32;
            while n < nroots {
                ptr = dj * j + di * i + n * 104 as i32;
                ig = ptr;
                while ig < ptr + bgrids {
                    *fx
                        .offset(
                            ig as isize,
                        ) = i as f64 * *gx.offset((ig - di) as isize)
                        + ai2 * *gx.offset((ig + di) as isize);
                    *fy
                        .offset(
                            ig as isize,
                        ) = i as f64 * *gy.offset((ig - di) as isize)
                        + ai2 * *gy.offset((ig + di) as isize);
                    *fz
                        .offset(
                            ig as isize,
                        ) = i as f64 * *gz.offset((ig - di) as isize)
                        + ai2 * *gz.offset((ig + di) as isize);
                    ig += 1;
                }
                n += 1;
            }
            i += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1j_grids(
    mut f: *mut f64,
    mut g: *mut f64,
    mut li: i32,
    mut lj: i32,
    mut envs: *mut CINTEnvVars,
) {
    let mut ngrids: i32 = (*envs).c2rust_unnamed_0.ngrids;
    let mut bgrids: i32 = if ngrids - (*envs).c2rust_unnamed.grids_offset
        < 104 as i32
    {
        ngrids - (*envs).c2rust_unnamed.grids_offset
    } else {
        104 as i32
    };
    let mut nroots: i32 = (*envs).nrys_roots;
    let di: i32 = (*envs).g_stride_i;
    let dj: i32 = (*envs).g_stride_j;
    let aj2: f64 = -(2 as i32) as f64
        * (*envs).aj[0 as i32 as usize];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    let mut ig: i32 = 0;
    let mut ptr: i32 = 0;
    let mut gx: *const f64 = g;
    let mut gy: *const f64 = g.offset((*envs).g_size as isize);
    let mut gz: *const f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut fx: *mut f64 = f;
    let mut fy: *mut f64 = f.offset((*envs).g_size as isize);
    let mut fz: *mut f64 = f
        .offset(((*envs).g_size * 2 as i32) as isize);
    i = 0 as i32;
    while i <= li {
        n = 0 as i32;
        while n < nroots {
            ptr = di * i + n * 104 as i32;
            ig = ptr;
            while ig < ptr + bgrids {
                *fx.offset(ig as isize) = aj2 * *gx.offset((ig + dj) as isize);
                *fy.offset(ig as isize) = aj2 * *gy.offset((ig + dj) as isize);
                *fz.offset(ig as isize) = aj2 * *gz.offset((ig + dj) as isize);
                ig += 1;
            }
            n += 1;
        }
        i += 1;
    }
    j = 1 as i32;
    while j <= lj {
        i = 0 as i32;
        while i <= li {
            n = 0 as i32;
            while n < nroots {
                ptr = dj * j + di * i + n * 104 as i32;
                ig = ptr;
                while ig < ptr + bgrids {
                    *fx
                        .offset(
                            ig as isize,
                        ) = j as f64 * *gx.offset((ig - dj) as isize)
                        + aj2 * *gx.offset((ig + dj) as isize);
                    *fy
                        .offset(
                            ig as isize,
                        ) = j as f64 * *gy.offset((ig - dj) as isize)
                        + aj2 * *gy.offset((ig + dj) as isize);
                    *fz
                        .offset(
                            ig as isize,
                        ) = j as f64 * *gz.offset((ig - dj) as isize)
                        + aj2 * *gz.offset((ig + dj) as isize);
                    ig += 1;
                }
                n += 1;
            }
            i += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1i_grids(
    mut f: *mut f64,
    mut g: *mut f64,
    mut ri: *mut f64,
    mut li: i32,
    mut lj: i32,
    mut envs: *mut CINTEnvVars,
) {
    let mut ngrids: i32 = (*envs).c2rust_unnamed_0.ngrids;
    let mut bgrids: i32 = if ngrids - (*envs).c2rust_unnamed.grids_offset
        < 104 as i32
    {
        ngrids - (*envs).c2rust_unnamed.grids_offset
    } else {
        104 as i32
    };
    let mut nroots: i32 = (*envs).nrys_roots;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    let mut ig: i32 = 0;
    let mut ptr: i32 = 0;
    let di: i32 = (*envs).g_stride_i;
    let dj: i32 = (*envs).g_stride_j;
    let mut gx: *const f64 = g;
    let mut gy: *const f64 = g.offset((*envs).g_size as isize);
    let mut gz: *const f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut fx: *mut f64 = f;
    let mut fy: *mut f64 = f.offset((*envs).g_size as isize);
    let mut fz: *mut f64 = f
        .offset(((*envs).g_size * 2 as i32) as isize);
    j = 0 as i32;
    while j <= lj {
        i = 0 as i32;
        while i <= li {
            n = 0 as i32;
            while n < nroots {
                ptr = dj * j + di * i + n * 104 as i32;
                ig = ptr;
                while ig < ptr + bgrids {
                    *fx
                        .offset(
                            ig as isize,
                        ) = *gx.offset((ig + di) as isize)
                        + *ri.offset(0 as i32 as isize)
                            * *gx.offset(ig as isize);
                    *fy
                        .offset(
                            ig as isize,
                        ) = *gy.offset((ig + di) as isize)
                        + *ri.offset(1 as i32 as isize)
                            * *gy.offset(ig as isize);
                    *fz
                        .offset(
                            ig as isize,
                        ) = *gz.offset((ig + di) as isize)
                        + *ri.offset(2 as i32 as isize)
                            * *gz.offset(ig as isize);
                    ig += 1;
                }
                n += 1;
            }
            i += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1j_grids(
    mut f: *mut f64,
    mut g: *mut f64,
    mut rj: *mut f64,
    mut li: i32,
    mut lj: i32,
    mut envs: *mut CINTEnvVars,
) {
    let mut ngrids: i32 = (*envs).c2rust_unnamed_0.ngrids;
    let mut bgrids: i32 = if ngrids - (*envs).c2rust_unnamed.grids_offset
        < 104 as i32
    {
        ngrids - (*envs).c2rust_unnamed.grids_offset
    } else {
        104 as i32
    };
    let mut nroots: i32 = (*envs).nrys_roots;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    let mut ig: i32 = 0;
    let mut ptr: i32 = 0;
    let di: i32 = (*envs).g_stride_i;
    let dj: i32 = (*envs).g_stride_j;
    let mut gx: *const f64 = g;
    let mut gy: *const f64 = g.offset((*envs).g_size as isize);
    let mut gz: *const f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut fx: *mut f64 = f;
    let mut fy: *mut f64 = f.offset((*envs).g_size as isize);
    let mut fz: *mut f64 = f
        .offset(((*envs).g_size * 2 as i32) as isize);
    j = 0 as i32;
    while j <= lj {
        i = 0 as i32;
        while i <= li {
            n = 0 as i32;
            while n < nroots {
                ptr = dj * j + di * i + n * 104 as i32;
                ig = ptr;
                while ig < ptr + bgrids {
                    *fx
                        .offset(
                            ig as isize,
                        ) = *gx.offset((ig + dj) as isize)
                        + *rj.offset(0 as i32 as isize)
                            * *gx.offset(ig as isize);
                    *fy
                        .offset(
                            ig as isize,
                        ) = *gy.offset((ig + dj) as isize)
                        + *rj.offset(1 as i32 as isize)
                            * *gy.offset(ig as isize);
                    *fz
                        .offset(
                            ig as isize,
                        ) = *gz.offset((ig + dj) as isize)
                        + *rj.offset(2 as i32 as isize)
                            * *gz.offset(ig as isize);
                    ig += 1;
                }
                n += 1;
            }
            i += 1;
        }
        j += 1;
    }
}
