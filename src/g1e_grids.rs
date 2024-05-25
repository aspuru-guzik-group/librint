#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn CINTinit_int1e_EnvVars(
        envs: *mut CINTEnvVars,
        ng: *mut libc::c_int,
        shls: *mut libc::c_int,
        atm: *mut libc::c_int,
        natm: libc::c_int,
        bas: *mut libc::c_int,
        nbas: libc::c_int,
        env: *mut libc::c_double,
    );
    fn CINTcommon_fac_sp(l: libc::c_int) -> libc::c_double;
    fn CINTsr_rys_roots(
        nroots: libc::c_int,
        x: libc::c_double,
        lower: libc::c_double,
        u: *mut libc::c_double,
        w: *mut libc::c_double,
    );
    fn CINTrys_roots(
        nroots: libc::c_int,
        x: libc::c_double,
        u: *mut libc::c_double,
        w: *mut libc::c_double,
    );
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PairData {
    pub rij: [libc::c_double; 3],
    pub eij: libc::c_double,
    pub cceij: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CINTOpt {
    pub index_xyz_array: *mut *mut libc::c_int,
    pub non0ctr: *mut *mut libc::c_int,
    pub sortedidx: *mut *mut libc::c_int,
    pub nbas: libc::c_int,
    pub log_max_coeff: *mut *mut libc::c_double,
    pub pairdata: *mut *mut PairData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CINTEnvVars {
    pub atm: *mut libc::c_int,
    pub bas: *mut libc::c_int,
    pub env: *mut libc::c_double,
    pub shls: *mut libc::c_int,
    pub natm: libc::c_int,
    pub nbas: libc::c_int,
    pub i_l: libc::c_int,
    pub j_l: libc::c_int,
    pub k_l: libc::c_int,
    pub l_l: libc::c_int,
    pub nfi: libc::c_int,
    pub nfj: libc::c_int,
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed_0,
    pub nf: libc::c_int,
    pub rys_order: libc::c_int,
    pub x_ctr: [libc::c_int; 4],
    pub gbits: libc::c_int,
    pub ncomp_e1: libc::c_int,
    pub ncomp_e2: libc::c_int,
    pub ncomp_tensor: libc::c_int,
    pub li_ceil: libc::c_int,
    pub lj_ceil: libc::c_int,
    pub lk_ceil: libc::c_int,
    pub ll_ceil: libc::c_int,
    pub g_stride_i: libc::c_int,
    pub g_stride_k: libc::c_int,
    pub g_stride_l: libc::c_int,
    pub g_stride_j: libc::c_int,
    pub nrys_roots: libc::c_int,
    pub g_size: libc::c_int,
    pub g2d_ijmax: libc::c_int,
    pub g2d_klmax: libc::c_int,
    pub common_factor: libc::c_double,
    pub expcutoff: libc::c_double,
    pub rirj: [libc::c_double; 3],
    pub rkrl: [libc::c_double; 3],
    pub rx_in_rijrx: *mut libc::c_double,
    pub rx_in_rklrx: *mut libc::c_double,
    pub ri: *mut libc::c_double,
    pub rj: *mut libc::c_double,
    pub rk: *mut libc::c_double,
    pub c2rust_unnamed_1: C2RustUnnamed,
    pub f_g0_2e: Option::<unsafe extern "C" fn() -> libc::c_int>,
    pub f_g0_2d4d: Option::<unsafe extern "C" fn() -> ()>,
    pub f_gout: Option::<unsafe extern "C" fn() -> ()>,
    pub opt: *mut CINTOpt,
    pub idx: *mut libc::c_int,
    pub ai: [libc::c_double; 1],
    pub aj: [libc::c_double; 1],
    pub ak: [libc::c_double; 1],
    pub al: [libc::c_double; 1],
    pub fac: [libc::c_double; 1],
    pub rij: [libc::c_double; 3],
    pub rkl: [libc::c_double; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub rl: *mut libc::c_double,
    pub grids: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub nfl: libc::c_int,
    pub ngrids: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub nfk: libc::c_int,
    pub grids_offset: libc::c_int,
}
pub type uintptr_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn CINTinit_int1e_grids_EnvVars(
    mut envs: *mut CINTEnvVars,
    mut ng: *mut libc::c_int,
    mut shls: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
) {
    CINTinit_int1e_EnvVars(envs, ng, shls, atm, natm, bas, nbas, env);
    let mut ngrids: libc::c_int = *shls.offset(3 as libc::c_int as isize)
        - *shls.offset(2 as libc::c_int as isize);
    let mut grids: *mut libc::c_double = env
        .offset(*env.offset(12 as libc::c_int as isize) as size_t as isize)
        .offset((*shls.offset(2 as libc::c_int as isize) * 3 as libc::c_int) as isize);
    (*envs).c2rust_unnamed_0.ngrids = ngrids;
    (*envs).c2rust_unnamed_1.grids = grids;
    (*envs)
        .common_factor = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
        * CINTcommon_fac_sp((*envs).i_l) * CINTcommon_fac_sp((*envs).j_l);
    let mut rys_order: libc::c_int = (*envs).nrys_roots;
    let mut nroots: libc::c_int = rys_order;
    let mut omega: libc::c_double = *env.offset(8 as libc::c_int as isize);
    if omega < 0 as libc::c_int as libc::c_double && rys_order <= 3 as libc::c_int {
        nroots *= 2 as libc::c_int;
    }
    (*envs).rys_order = rys_order;
    (*envs).nrys_roots = nroots;
    let mut dli: libc::c_int = 0;
    let mut dlj: libc::c_int = 0;
    let mut ibase: libc::c_int = ((*envs).li_ceil > (*envs).lj_ceil) as libc::c_int;
    if ibase != 0 {
        dli = (*envs).li_ceil + (*envs).lj_ceil + 1 as libc::c_int;
        dlj = (*envs).lj_ceil + 1 as libc::c_int;
        (*envs)
            .rirj[0 as libc::c_int
            as usize] = *((*envs).ri).offset(0 as libc::c_int as isize)
            - *((*envs).rj).offset(0 as libc::c_int as isize);
        (*envs)
            .rirj[1 as libc::c_int
            as usize] = *((*envs).ri).offset(1 as libc::c_int as isize)
            - *((*envs).rj).offset(1 as libc::c_int as isize);
        (*envs)
            .rirj[2 as libc::c_int
            as usize] = *((*envs).ri).offset(2 as libc::c_int as isize)
            - *((*envs).rj).offset(2 as libc::c_int as isize);
    } else {
        dli = (*envs).li_ceil + 1 as libc::c_int;
        dlj = (*envs).li_ceil + (*envs).lj_ceil + 1 as libc::c_int;
        (*envs)
            .rirj[0 as libc::c_int
            as usize] = *((*envs).rj).offset(0 as libc::c_int as isize)
            - *((*envs).ri).offset(0 as libc::c_int as isize);
        (*envs)
            .rirj[1 as libc::c_int
            as usize] = *((*envs).rj).offset(1 as libc::c_int as isize)
            - *((*envs).ri).offset(1 as libc::c_int as isize);
        (*envs)
            .rirj[2 as libc::c_int
            as usize] = *((*envs).rj).offset(2 as libc::c_int as isize)
            - *((*envs).ri).offset(2 as libc::c_int as isize);
    }
    (*envs).g_stride_i = 104 as libc::c_int * nroots;
    (*envs).g_stride_j = 104 as libc::c_int * nroots * dli;
    (*envs).g_size = 104 as libc::c_int * nroots * dli * dlj;
    (*envs).g_stride_k = (*envs).g_size;
    (*envs).g_stride_l = (*envs).g_size;
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_1e_grids(
    mut g: *mut libc::c_double,
    mut cutoff: libc::c_double,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut libc::c_double,
    mut gridsT: *mut libc::c_double,
) -> libc::c_int {
    let mut ngrids: libc::c_int = (*envs).c2rust_unnamed_0.ngrids;
    let mut bgrids: libc::c_int = if ngrids - (*envs).c2rust_unnamed.grids_offset
        < 104 as libc::c_int
    {
        ngrids - (*envs).c2rust_unnamed.grids_offset
    } else {
        104 as libc::c_int
    };
    let mut nroots: libc::c_int = (*envs).nrys_roots;
    let mut gx: *mut libc::c_double = g;
    let mut gy: *mut libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *mut libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut w: *mut libc::c_double = gz;
    let mut rij: *mut libc::c_double = ((*envs).rij).as_mut_ptr();
    let mut ubuf: [libc::c_double; 32] = [0.; 32];
    let mut wbuf: [libc::c_double; 32] = [0.; 32];
    let mut u: *mut libc::c_double = 0 as *mut libc::c_double;
    u = ((cache as uintptr_t).wrapping_add(63 as libc::c_int as libc::c_ulong)
        & (64 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_double;
    cache = u.offset((104 as libc::c_int * nroots) as isize);
    let mut rijrg: *mut libc::c_double = 0 as *mut libc::c_double;
    rijrg = ((cache as uintptr_t).wrapping_add(63 as libc::c_int as libc::c_ulong)
        & (64 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_double;
    cache = rijrg.offset((104 as libc::c_int * 3 as libc::c_int) as isize);
    let mut aij: libc::c_double = (*envs).ai[0 as libc::c_int as usize]
        + (*envs).aj[0 as libc::c_int as usize];
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ig: libc::c_int = 0;
    let mut x: libc::c_double = 0.;
    let mut fac1: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < nroots {
        ig = 0 as libc::c_int;
        while ig < bgrids {
            *gx
                .offset(
                    (ig + 104 as libc::c_int * i) as isize,
                ) = 1 as libc::c_int as libc::c_double;
            *gy
                .offset(
                    (ig + 104 as libc::c_int * i) as isize,
                ) = 1 as libc::c_int as libc::c_double;
            ig += 1;
            ig;
        }
        i += 1;
        i;
    }
    ig = 0 as libc::c_int;
    while ig < bgrids {
        *rijrg
            .offset(
                (ig + 104 as libc::c_int * 0 as libc::c_int) as isize,
            ) = *gridsT.offset((ig + 104 as libc::c_int * 0 as libc::c_int) as isize)
            - *rij.offset(0 as libc::c_int as isize);
        *rijrg
            .offset(
                (ig + 104 as libc::c_int * 1 as libc::c_int) as isize,
            ) = *gridsT.offset((ig + 104 as libc::c_int * 1 as libc::c_int) as isize)
            - *rij.offset(1 as libc::c_int as isize);
        *rijrg
            .offset(
                (ig + 104 as libc::c_int * 2 as libc::c_int) as isize,
            ) = *gridsT.offset((ig + 104 as libc::c_int * 2 as libc::c_int) as isize)
            - *rij.offset(2 as libc::c_int as isize);
        ig += 1;
        ig;
    }
    let mut omega: libc::c_double = *((*envs).env).offset(8 as libc::c_int as isize);
    let mut zeta: libc::c_double = *((*envs).env).offset(7 as libc::c_int as isize);
    let mut omega2: libc::c_double = 0.;
    let mut theta: libc::c_double = 0.;
    let mut sqrt_theta: libc::c_double = 0.;
    let mut a0: libc::c_double = 0.;
    let mut tau2: libc::c_double = 0.;
    if omega == 0.0f64 && zeta == 0.0f64 {
        fac1 = (*envs).fac[0 as libc::c_int as usize] / aij;
        ig = 0 as libc::c_int;
        while ig < bgrids {
            x = aij
                * (*rijrg.offset((ig + 104 as libc::c_int * 0 as libc::c_int) as isize)
                    * *rijrg
                        .offset((ig + 104 as libc::c_int * 0 as libc::c_int) as isize)
                    + *rijrg
                        .offset((ig + 104 as libc::c_int * 1 as libc::c_int) as isize)
                        * *rijrg
                            .offset(
                                (ig + 104 as libc::c_int * 1 as libc::c_int) as isize,
                            )
                    + *rijrg
                        .offset((ig + 104 as libc::c_int * 2 as libc::c_int) as isize)
                        * *rijrg
                            .offset(
                                (ig + 104 as libc::c_int * 2 as libc::c_int) as isize,
                            ));
            CINTrys_roots(nroots, x, ubuf.as_mut_ptr(), wbuf.as_mut_ptr());
            i = 0 as libc::c_int;
            while i < nroots {
                *u
                    .offset(
                        (ig + 104 as libc::c_int * i) as isize,
                    ) = ubuf[i as usize]
                    / (ubuf[i as usize] + 1 as libc::c_int as libc::c_double);
                *w
                    .offset(
                        (ig + 104 as libc::c_int * i) as isize,
                    ) = wbuf[i as usize] * fac1;
                i += 1;
                i;
            }
            ig += 1;
            ig;
        }
    } else if omega < 0.0f64 {
        a0 = aij;
        fac1 = (*envs).fac[0 as libc::c_int as usize] / aij;
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
        let mut temp_cutoff: libc::c_double = if cutoff
            < 40 as libc::c_int as libc::c_double
        {
            cutoff
        } else {
            40 as libc::c_int as libc::c_double
        };
        let mut rorder: libc::c_int = (*envs).rys_order;
        let mut tau_theta: libc::c_double = 0.;
        let mut fac_theta: libc::c_double = 0.;
        ig = 0 as libc::c_int;
        while ig < bgrids {
            x = a0
                * (*rijrg.offset((ig + 104 as libc::c_int * 0 as libc::c_int) as isize)
                    * *rijrg
                        .offset((ig + 104 as libc::c_int * 0 as libc::c_int) as isize)
                    + *rijrg
                        .offset((ig + 104 as libc::c_int * 1 as libc::c_int) as isize)
                        * *rijrg
                            .offset(
                                (ig + 104 as libc::c_int * 1 as libc::c_int) as isize,
                            )
                    + *rijrg
                        .offset((ig + 104 as libc::c_int * 2 as libc::c_int) as isize)
                        * *rijrg
                            .offset(
                                (ig + 104 as libc::c_int * 2 as libc::c_int) as isize,
                            ));
            if theta * x > temp_cutoff {
                i = 0 as libc::c_int;
                while i < nroots {
                    *u
                        .offset(
                            (ig + 104 as libc::c_int * i) as isize,
                        ) = 0 as libc::c_int as libc::c_double;
                    *w
                        .offset(
                            (ig + 104 as libc::c_int * i) as isize,
                        ) = 0 as libc::c_int as libc::c_double;
                    i += 1;
                    i;
                }
            } else if rorder == nroots {
                CINTsr_rys_roots(
                    nroots,
                    x,
                    sqrt_theta,
                    ubuf.as_mut_ptr(),
                    wbuf.as_mut_ptr(),
                );
                i = 0 as libc::c_int;
                while i < nroots {
                    *u
                        .offset(
                            (ig + 104 as libc::c_int * i) as isize,
                        ) = ubuf[i as usize]
                        / (ubuf[i as usize] + 1 as libc::c_int as libc::c_double) * tau2;
                    *w
                        .offset(
                            (ig + 104 as libc::c_int * i) as isize,
                        ) = wbuf[i as usize] * fac1;
                    i += 1;
                    i;
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
                i = 0 as libc::c_int;
                while i < rorder {
                    *u
                        .offset(
                            (ig + 104 as libc::c_int * i) as isize,
                        ) = ubuf[i as usize]
                        / (ubuf[i as usize] + 1 as libc::c_int as libc::c_double) * tau2;
                    *w
                        .offset(
                            (ig + 104 as libc::c_int * i) as isize,
                        ) = wbuf[i as usize] * fac1;
                    *u
                        .offset(
                            (ig + 104 as libc::c_int * (i + rorder)) as isize,
                        ) = ubuf[(i + rorder) as usize]
                        / (ubuf[(i + rorder) as usize]
                            + 1 as libc::c_int as libc::c_double) * tau_theta;
                    *w
                        .offset(
                            (ig + 104 as libc::c_int * (i + rorder)) as isize,
                        ) = wbuf[(i + rorder) as usize] * fac_theta;
                    i += 1;
                    i;
                }
            }
            ig += 1;
            ig;
        }
    } else {
        a0 = aij;
        fac1 = (*envs).fac[0 as libc::c_int as usize] / aij;
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
        ig = 0 as libc::c_int;
        while ig < bgrids {
            x = a0
                * (*rijrg.offset((ig + 104 as libc::c_int * 0 as libc::c_int) as isize)
                    * *rijrg
                        .offset((ig + 104 as libc::c_int * 0 as libc::c_int) as isize)
                    + *rijrg
                        .offset((ig + 104 as libc::c_int * 1 as libc::c_int) as isize)
                        * *rijrg
                            .offset(
                                (ig + 104 as libc::c_int * 1 as libc::c_int) as isize,
                            )
                    + *rijrg
                        .offset((ig + 104 as libc::c_int * 2 as libc::c_int) as isize)
                        * *rijrg
                            .offset(
                                (ig + 104 as libc::c_int * 2 as libc::c_int) as isize,
                            ));
            CINTrys_roots(nroots, x, ubuf.as_mut_ptr(), wbuf.as_mut_ptr());
            i = 0 as libc::c_int;
            while i < nroots {
                *u
                    .offset(
                        (ig + 104 as libc::c_int * i) as isize,
                    ) = ubuf[i as usize]
                    / (ubuf[i as usize] + 1 as libc::c_int as libc::c_double) * theta;
                *w
                    .offset(
                        (ig + 104 as libc::c_int * i) as isize,
                    ) = wbuf[i as usize] * fac1;
                i += 1;
                i;
            }
            ig += 1;
            ig;
        }
    }
    let mut nmax: libc::c_int = (*envs).li_ceil + (*envs).lj_ceil;
    if nmax == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    let mut rirj: *mut libc::c_double = ((*envs).rirj).as_mut_ptr();
    let mut lj: libc::c_int = 0;
    let mut di: libc::c_int = 0;
    let mut dj: libc::c_int = 0;
    let mut rx: *mut libc::c_double = 0 as *mut libc::c_double;
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
    let mut rijrx: [libc::c_double; 3] = [0.; 3];
    rijrx[0 as libc::c_int
        as usize] = *rij.offset(0 as libc::c_int as isize)
        - *rx.offset(0 as libc::c_int as isize);
    rijrx[1 as libc::c_int
        as usize] = *rij.offset(1 as libc::c_int as isize)
        - *rx.offset(1 as libc::c_int as isize);
    rijrx[2 as libc::c_int
        as usize] = *rij.offset(2 as libc::c_int as isize)
        - *rx.offset(2 as libc::c_int as isize);
    let mut p0x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p0y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p0z: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p1x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p1y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p1z: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p2x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p2y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p2z: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut t2: *mut libc::c_double = 0 as *mut libc::c_double;
    t2 = ((cache as uintptr_t).wrapping_add(63 as libc::c_int as libc::c_ulong)
        & (64 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_double;
    cache = t2.offset((104 as libc::c_int * 4 as libc::c_int) as isize);
    let mut rirgx: *mut libc::c_double = t2.offset(104 as libc::c_int as isize);
    let mut rirgy: *mut libc::c_double = rirgx.offset(104 as libc::c_int as isize);
    let mut rirgz: *mut libc::c_double = rirgy.offset(104 as libc::c_int as isize);
    let mut aij2: libc::c_double = 0.5f64 / aij;
    let mut tx: libc::c_double = 0.;
    let mut ty: libc::c_double = 0.;
    let mut tz: libc::c_double = 0.;
    n = 0 as libc::c_int;
    while n < nroots {
        p0x = gx.offset((104 as libc::c_int * n) as isize);
        p0y = gy.offset((104 as libc::c_int * n) as isize);
        p0z = gz.offset((104 as libc::c_int * n) as isize);
        p1x = p0x.offset(di as isize);
        p1y = p0y.offset(di as isize);
        p1z = p0z.offset(di as isize);
        ig = 0 as libc::c_int;
        while ig < bgrids {
            *rirgx
                .offset(
                    ig as isize,
                ) = rijrx[0 as libc::c_int as usize]
                + *u.offset((ig + 104 as libc::c_int * n) as isize)
                    * *rijrg
                        .offset((ig + 104 as libc::c_int * 0 as libc::c_int) as isize);
            *rirgy
                .offset(
                    ig as isize,
                ) = rijrx[1 as libc::c_int as usize]
                + *u.offset((ig + 104 as libc::c_int * n) as isize)
                    * *rijrg
                        .offset((ig + 104 as libc::c_int * 1 as libc::c_int) as isize);
            *rirgz
                .offset(
                    ig as isize,
                ) = rijrx[2 as libc::c_int as usize]
                + *u.offset((ig + 104 as libc::c_int * n) as isize)
                    * *rijrg
                        .offset((ig + 104 as libc::c_int * 2 as libc::c_int) as isize);
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
            ig;
        }
        if nmax > 0 as libc::c_int {
            ig = 0 as libc::c_int;
            while ig < bgrids {
                *t2
                    .offset(
                        ig as isize,
                    ) = aij2
                    * (1 as libc::c_int as libc::c_double
                        - *u.offset((ig + 104 as libc::c_int * n) as isize));
                ig += 1;
                ig;
            }
        }
        i = 1 as libc::c_int;
        while i < nmax {
            p0x = gx.offset((104 as libc::c_int * n) as isize).offset((i * di) as isize);
            p0y = gy.offset((104 as libc::c_int * n) as isize).offset((i * di) as isize);
            p0z = gz.offset((104 as libc::c_int * n) as isize).offset((i * di) as isize);
            p1x = p0x.offset(di as isize);
            p1y = p0y.offset(di as isize);
            p1z = p0z.offset(di as isize);
            p2x = p0x.offset(-(di as isize));
            p2y = p0y.offset(-(di as isize));
            p2z = p0z.offset(-(di as isize));
            ig = 0 as libc::c_int;
            while ig < bgrids {
                *p1x
                    .offset(
                        ig as isize,
                    ) = i as libc::c_double * *t2.offset(ig as isize)
                    * *p2x.offset(ig as isize)
                    + *rirgx.offset(ig as isize) * *p0x.offset(ig as isize);
                *p1y
                    .offset(
                        ig as isize,
                    ) = i as libc::c_double * *t2.offset(ig as isize)
                    * *p2y.offset(ig as isize)
                    + *rirgy.offset(ig as isize) * *p0y.offset(ig as isize);
                *p1z
                    .offset(
                        ig as isize,
                    ) = i as libc::c_double * *t2.offset(ig as isize)
                    * *p2z.offset(ig as isize)
                    + *rirgz.offset(ig as isize) * *p0z.offset(ig as isize);
                ig += 1;
                ig;
            }
            i += 1;
            i;
        }
        n += 1;
        n;
    }
    j = 1 as libc::c_int;
    while j <= lj {
        i = 0 as libc::c_int;
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
            n = 0 as libc::c_int;
            while n < nroots {
                ig = 0 as libc::c_int;
                while ig < bgrids {
                    *p0x
                        .offset(
                            (ig + 104 as libc::c_int * n) as isize,
                        ) = *p2x.offset((ig + 104 as libc::c_int * n) as isize)
                        + *rirj.offset(0 as libc::c_int as isize)
                            * *p1x.offset((ig + 104 as libc::c_int * n) as isize);
                    *p0y
                        .offset(
                            (ig + 104 as libc::c_int * n) as isize,
                        ) = *p2y.offset((ig + 104 as libc::c_int * n) as isize)
                        + *rirj.offset(1 as libc::c_int as isize)
                            * *p1y.offset((ig + 104 as libc::c_int * n) as isize);
                    *p0z
                        .offset(
                            (ig + 104 as libc::c_int * n) as isize,
                        ) = *p2z.offset((ig + 104 as libc::c_int * n) as isize)
                        + *rirj.offset(2 as libc::c_int as isize)
                            * *p1z.offset((ig + 104 as libc::c_int * n) as isize);
                    ig += 1;
                    ig;
                }
                n += 1;
                n;
            }
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_grids(
    mut gout: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut idx: *mut libc::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: libc::c_int,
) {
    let mut ngrids: libc::c_int = (*envs).c2rust_unnamed_0.ngrids;
    let mut bgrids: libc::c_int = if ngrids - (*envs).c2rust_unnamed.grids_offset
        < 104 as libc::c_int
    {
        ngrids - (*envs).c2rust_unnamed.grids_offset
    } else {
        104 as libc::c_int
    };
    let mut nroots: libc::c_int = (*envs).nrys_roots;
    let mut nf: libc::c_int = (*envs).nf;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut ig: libc::c_int = 0;
    let mut gx: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut gy: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut gz: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut s: [libc::c_double; 104] = [0.; 104];
    if gout_empty != 0 {
        n = 0 as libc::c_int;
        while n < nf {
            gx = g.offset(*idx.offset(0 as libc::c_int as isize) as isize);
            gy = g.offset(*idx.offset(1 as libc::c_int as isize) as isize);
            gz = g.offset(*idx.offset(2 as libc::c_int as isize) as isize);
            ig = 0 as libc::c_int;
            while ig < bgrids {
                s[ig as usize] = 0 as libc::c_int as libc::c_double;
                ig += 1;
                ig;
            }
            i = 0 as libc::c_int;
            while i < nroots {
                ig = 0 as libc::c_int;
                while ig < bgrids {
                    s[ig as usize]
                        += *gx.offset((ig + 104 as libc::c_int * i) as isize)
                            * *gy.offset((ig + 104 as libc::c_int * i) as isize)
                            * *gz.offset((ig + 104 as libc::c_int * i) as isize);
                    ig += 1;
                    ig;
                }
                i += 1;
                i;
            }
            ig = 0 as libc::c_int;
            while ig < bgrids {
                *gout.offset((ig + bgrids * n) as isize) = s[ig as usize];
                ig += 1;
                ig;
            }
            n += 1;
            n;
            idx = idx.offset(3 as libc::c_int as isize);
        }
    } else {
        n = 0 as libc::c_int;
        while n < nf {
            gx = g.offset(*idx.offset(0 as libc::c_int as isize) as isize);
            gy = g.offset(*idx.offset(1 as libc::c_int as isize) as isize);
            gz = g.offset(*idx.offset(2 as libc::c_int as isize) as isize);
            ig = 0 as libc::c_int;
            while ig < bgrids {
                s[ig as usize] = 0 as libc::c_int as libc::c_double;
                ig += 1;
                ig;
            }
            i = 0 as libc::c_int;
            while i < nroots {
                ig = 0 as libc::c_int;
                while ig < bgrids {
                    s[ig as usize]
                        += *gx.offset((ig + 104 as libc::c_int * i) as isize)
                            * *gy.offset((ig + 104 as libc::c_int * i) as isize)
                            * *gz.offset((ig + 104 as libc::c_int * i) as isize);
                    ig += 1;
                    ig;
                }
                i += 1;
                i;
            }
            ig = 0 as libc::c_int;
            while ig < bgrids {
                *gout.offset((ig + bgrids * n) as isize) += s[ig as usize];
                ig += 1;
                ig;
            }
            n += 1;
            n;
            idx = idx.offset(3 as libc::c_int as isize);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1i_grids(
    mut f: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut li: libc::c_int,
    mut lj: libc::c_int,
    mut envs: *mut CINTEnvVars,
) {
    let mut ngrids: libc::c_int = (*envs).c2rust_unnamed_0.ngrids;
    let mut bgrids: libc::c_int = if ngrids - (*envs).c2rust_unnamed.grids_offset
        < 104 as libc::c_int
    {
        ngrids - (*envs).c2rust_unnamed.grids_offset
    } else {
        104 as libc::c_int
    };
    let mut nroots: libc::c_int = (*envs).nrys_roots;
    let di: libc::c_int = (*envs).g_stride_i;
    let dj: libc::c_int = (*envs).g_stride_j;
    let ai2: libc::c_double = -(2 as libc::c_int) as libc::c_double
        * (*envs).ai[0 as libc::c_int as usize];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut ig: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut gx: *const libc::c_double = g;
    let mut gy: *const libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut fx: *mut libc::c_double = f;
    let mut fy: *mut libc::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut libc::c_double = f
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    j = 0 as libc::c_int;
    while j <= lj {
        n = 0 as libc::c_int;
        while n < nroots {
            ptr = dj * j + n * 104 as libc::c_int;
            ig = ptr;
            while ig < ptr + bgrids {
                *fx.offset(ig as isize) = ai2 * *gx.offset((ig + di) as isize);
                *fy.offset(ig as isize) = ai2 * *gy.offset((ig + di) as isize);
                *fz.offset(ig as isize) = ai2 * *gz.offset((ig + di) as isize);
                ig += 1;
                ig;
            }
            n += 1;
            n;
        }
        i = 1 as libc::c_int;
        while i <= li {
            n = 0 as libc::c_int;
            while n < nroots {
                ptr = dj * j + di * i + n * 104 as libc::c_int;
                ig = ptr;
                while ig < ptr + bgrids {
                    *fx
                        .offset(
                            ig as isize,
                        ) = i as libc::c_double * *gx.offset((ig - di) as isize)
                        + ai2 * *gx.offset((ig + di) as isize);
                    *fy
                        .offset(
                            ig as isize,
                        ) = i as libc::c_double * *gy.offset((ig - di) as isize)
                        + ai2 * *gy.offset((ig + di) as isize);
                    *fz
                        .offset(
                            ig as isize,
                        ) = i as libc::c_double * *gz.offset((ig - di) as isize)
                        + ai2 * *gz.offset((ig + di) as isize);
                    ig += 1;
                    ig;
                }
                n += 1;
                n;
            }
            i += 1;
            i;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1j_grids(
    mut f: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut li: libc::c_int,
    mut lj: libc::c_int,
    mut envs: *mut CINTEnvVars,
) {
    let mut ngrids: libc::c_int = (*envs).c2rust_unnamed_0.ngrids;
    let mut bgrids: libc::c_int = if ngrids - (*envs).c2rust_unnamed.grids_offset
        < 104 as libc::c_int
    {
        ngrids - (*envs).c2rust_unnamed.grids_offset
    } else {
        104 as libc::c_int
    };
    let mut nroots: libc::c_int = (*envs).nrys_roots;
    let di: libc::c_int = (*envs).g_stride_i;
    let dj: libc::c_int = (*envs).g_stride_j;
    let aj2: libc::c_double = -(2 as libc::c_int) as libc::c_double
        * (*envs).aj[0 as libc::c_int as usize];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut ig: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut gx: *const libc::c_double = g;
    let mut gy: *const libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut fx: *mut libc::c_double = f;
    let mut fy: *mut libc::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut libc::c_double = f
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    i = 0 as libc::c_int;
    while i <= li {
        n = 0 as libc::c_int;
        while n < nroots {
            ptr = di * i + n * 104 as libc::c_int;
            ig = ptr;
            while ig < ptr + bgrids {
                *fx.offset(ig as isize) = aj2 * *gx.offset((ig + dj) as isize);
                *fy.offset(ig as isize) = aj2 * *gy.offset((ig + dj) as isize);
                *fz.offset(ig as isize) = aj2 * *gz.offset((ig + dj) as isize);
                ig += 1;
                ig;
            }
            n += 1;
            n;
        }
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= lj {
        i = 0 as libc::c_int;
        while i <= li {
            n = 0 as libc::c_int;
            while n < nroots {
                ptr = dj * j + di * i + n * 104 as libc::c_int;
                ig = ptr;
                while ig < ptr + bgrids {
                    *fx
                        .offset(
                            ig as isize,
                        ) = j as libc::c_double * *gx.offset((ig - dj) as isize)
                        + aj2 * *gx.offset((ig + dj) as isize);
                    *fy
                        .offset(
                            ig as isize,
                        ) = j as libc::c_double * *gy.offset((ig - dj) as isize)
                        + aj2 * *gy.offset((ig + dj) as isize);
                    *fz
                        .offset(
                            ig as isize,
                        ) = j as libc::c_double * *gz.offset((ig - dj) as isize)
                        + aj2 * *gz.offset((ig + dj) as isize);
                    ig += 1;
                    ig;
                }
                n += 1;
                n;
            }
            i += 1;
            i;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1i_grids(
    mut f: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut ri: *mut libc::c_double,
    mut li: libc::c_int,
    mut lj: libc::c_int,
    mut envs: *mut CINTEnvVars,
) {
    let mut ngrids: libc::c_int = (*envs).c2rust_unnamed_0.ngrids;
    let mut bgrids: libc::c_int = if ngrids - (*envs).c2rust_unnamed.grids_offset
        < 104 as libc::c_int
    {
        ngrids - (*envs).c2rust_unnamed.grids_offset
    } else {
        104 as libc::c_int
    };
    let mut nroots: libc::c_int = (*envs).nrys_roots;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut ig: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let di: libc::c_int = (*envs).g_stride_i;
    let dj: libc::c_int = (*envs).g_stride_j;
    let mut gx: *const libc::c_double = g;
    let mut gy: *const libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut fx: *mut libc::c_double = f;
    let mut fy: *mut libc::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut libc::c_double = f
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    j = 0 as libc::c_int;
    while j <= lj {
        i = 0 as libc::c_int;
        while i <= li {
            n = 0 as libc::c_int;
            while n < nroots {
                ptr = dj * j + di * i + n * 104 as libc::c_int;
                ig = ptr;
                while ig < ptr + bgrids {
                    *fx
                        .offset(
                            ig as isize,
                        ) = *gx.offset((ig + di) as isize)
                        + *ri.offset(0 as libc::c_int as isize)
                            * *gx.offset(ig as isize);
                    *fy
                        .offset(
                            ig as isize,
                        ) = *gy.offset((ig + di) as isize)
                        + *ri.offset(1 as libc::c_int as isize)
                            * *gy.offset(ig as isize);
                    *fz
                        .offset(
                            ig as isize,
                        ) = *gz.offset((ig + di) as isize)
                        + *ri.offset(2 as libc::c_int as isize)
                            * *gz.offset(ig as isize);
                    ig += 1;
                    ig;
                }
                n += 1;
                n;
            }
            i += 1;
            i;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1j_grids(
    mut f: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut rj: *mut libc::c_double,
    mut li: libc::c_int,
    mut lj: libc::c_int,
    mut envs: *mut CINTEnvVars,
) {
    let mut ngrids: libc::c_int = (*envs).c2rust_unnamed_0.ngrids;
    let mut bgrids: libc::c_int = if ngrids - (*envs).c2rust_unnamed.grids_offset
        < 104 as libc::c_int
    {
        ngrids - (*envs).c2rust_unnamed.grids_offset
    } else {
        104 as libc::c_int
    };
    let mut nroots: libc::c_int = (*envs).nrys_roots;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut ig: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let di: libc::c_int = (*envs).g_stride_i;
    let dj: libc::c_int = (*envs).g_stride_j;
    let mut gx: *const libc::c_double = g;
    let mut gy: *const libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut fx: *mut libc::c_double = f;
    let mut fy: *mut libc::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut libc::c_double = f
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    j = 0 as libc::c_int;
    while j <= lj {
        i = 0 as libc::c_int;
        while i <= li {
            n = 0 as libc::c_int;
            while n < nroots {
                ptr = dj * j + di * i + n * 104 as libc::c_int;
                ig = ptr;
                while ig < ptr + bgrids {
                    *fx
                        .offset(
                            ig as isize,
                        ) = *gx.offset((ig + dj) as isize)
                        + *rj.offset(0 as libc::c_int as isize)
                            * *gx.offset(ig as isize);
                    *fy
                        .offset(
                            ig as isize,
                        ) = *gy.offset((ig + dj) as isize)
                        + *rj.offset(1 as libc::c_int as isize)
                            * *gy.offset(ig as isize);
                    *fz
                        .offset(
                            ig as isize,
                        ) = *gz.offset((ig + dj) as isize)
                        + *rj.offset(2 as libc::c_int as isize)
                            * *gz.offset(ig as isize);
                    ig += 1;
                    ig;
                }
                n += 1;
                n;
            }
            i += 1;
            i;
        }
        j += 1;
        j;
    }
}
