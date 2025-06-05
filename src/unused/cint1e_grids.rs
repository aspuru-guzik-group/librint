#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn CINTOpt_log_max_pgto_coeff(
        log_maxc: *mut libc::c_double,
        coeff: *mut libc::c_double,
        nprim: libc::c_int,
        nctr: libc::c_int,
    );
    fn CINTOpt_non0coeff_byshell(
        sortedidx: *mut libc::c_int,
        non0ctr: *mut libc::c_int,
        ci: *mut libc::c_double,
        iprim: libc::c_int,
        ictr: libc::c_int,
    );
    fn CINTset_pairdata(
        pairdata: *mut PairData,
        ai: *mut libc::c_double,
        aj: *mut libc::c_double,
        ri: *mut libc::c_double,
        rj: *mut libc::c_double,
        log_maxci: *mut libc::c_double,
        log_maxcj: *mut libc::c_double,
        li_ceil: libc::c_int,
        lj_ceil: libc::c_int,
        iprim: libc::c_int,
        jprim: libc::c_int,
        rr_ij: libc::c_double,
        expcutoff: libc::c_double,
        env: *mut libc::c_double,
    ) -> libc::c_int;
    fn CINTg1e_index_xyz(idx: *mut libc::c_int, envs: *mut CINTEnvVars);
    fn CINTprim_to_ctr_0(
        gc: *mut libc::c_double,
        gp: *mut libc::c_double,
        coeff: *mut libc::c_double,
        nf: size_t,
        nprim: libc::c_int,
        nctr: libc::c_int,
        non0ctr: libc::c_int,
        sortedidx: *mut libc::c_int,
    );
    fn CINTprim_to_ctr_1(
        gc: *mut libc::c_double,
        gp: *mut libc::c_double,
        coeff: *mut libc::c_double,
        nf: size_t,
        nprim: libc::c_int,
        nctr: libc::c_int,
        non0ctr: libc::c_int,
        sortedidx: *mut libc::c_int,
    );
    fn CINTinit_int1e_grids_EnvVars(
        envs: *mut CINTEnvVars,
        ng: *mut libc::c_int,
        shls: *mut libc::c_int,
        atm: *mut libc::c_int,
        natm: libc::c_int,
        bas: *mut libc::c_int,
        nbas: libc::c_int,
        env: *mut libc::c_double,
    );
    fn CINTg0_1e_grids(
        g: *mut libc::c_double,
        cutoff: libc::c_double,
        envs: *mut CINTEnvVars,
        cache: *mut libc::c_double,
        gridsT: *mut libc::c_double,
    ) -> libc::c_int;
    fn CINTgout1e_grids(
        gout: *mut libc::c_double,
        g: *mut libc::c_double,
        idx: *mut libc::c_int,
        envs: *mut CINTEnvVars,
        gout_empty: libc::c_int,
    );
    fn c2s_sph_1e_grids(
        out: *mut libc::c_double,
        gctr: *mut libc::c_double,
        dims: *mut libc::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut libc::c_double,
    );
    fn c2s_cart_1e_grids(
        out: *mut libc::c_double,
        gctr: *mut libc::c_double,
        dims: *mut libc::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut libc::c_double,
    );
    fn c2s_grids_dset0(
        out: *mut libc::c_double,
        dims: *mut libc::c_int,
        counts: *mut libc::c_int,
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
pub unsafe extern "C" fn CINT1e_grids_loop(
    mut gctr: *mut libc::c_double,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut libc::c_double,
) -> libc::c_int {
    let mut shls: *mut libc::c_int = (*envs).shls;
    let mut bas: *mut libc::c_int = (*envs).bas;
    let mut env: *mut libc::c_double = (*envs).env;
    let mut i_sh: libc::c_int = *shls.offset(0 as libc::c_int as isize);
    let mut j_sh: libc::c_int = *shls.offset(1 as libc::c_int as isize);
    let mut i_ctr: libc::c_int = (*envs).x_ctr[0 as libc::c_int as usize];
    let mut j_ctr: libc::c_int = (*envs).x_ctr[1 as libc::c_int as usize];
    let mut i_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * i_sh + 2 as libc::c_int) as isize);
    let mut j_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * j_sh + 2 as libc::c_int) as isize);
    let mut nf: libc::c_int = (*envs).nf;
    let mut n_comp: libc::c_int = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut ngrids: libc::c_int = (*envs).c2rust_unnamed_0.ngrids;
    let mut grids: *mut libc::c_double = (*envs).c2rust_unnamed_1.grids;
    let mut ai: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * i_sh + 5 as libc::c_int) as isize) as isize,
        );
    let mut aj: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * j_sh + 5 as libc::c_int) as isize) as isize,
        );
    let mut ci: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * i_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut cj: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * j_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut expcutoff: libc::c_double = (*envs).expcutoff;
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
        (*envs).rirj[0 as libc::c_int as usize] * (*envs).rirj[0 as libc::c_int as usize]
            + (*envs).rirj[1 as libc::c_int as usize]
                * (*envs).rirj[1 as libc::c_int as usize]
            + (*envs).rirj[2 as libc::c_int as usize]
                * (*envs).rirj[2 as libc::c_int as usize],
        expcutoff,
        env,
    ) != 0
    {
        return 0 as libc::c_int;
    }
    let mut fac1i: libc::c_double = 0.;
    let mut fac1j: libc::c_double = 0.;
    let mut expij: libc::c_double = 0.;
    let mut cutoff: libc::c_double = 0.;
    let mut rij: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut ip: libc::c_int = 0;
    let mut jp: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut grids_offset: libc::c_int = 0;
    let mut bgrids: libc::c_int = 0;
    let mut empty: [libc::c_int; 4] = [
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    let mut gempty: *mut libc::c_int = empty
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize);
    let mut iempty: *mut libc::c_int = empty
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize);
    let mut jempty: *mut libc::c_int = empty
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize);
    let mut all_empty: libc::c_int = 1 as libc::c_int;
    let mut idx: *mut libc::c_int = 0 as *mut libc::c_int;
    idx = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_int;
    cache = idx.offset((nf * 3 as libc::c_int) as isize) as *mut libc::c_double;
    CINTg1e_index_xyz(idx, envs);
    let mut non0ctri: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut non0ctrj: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut non0idxi: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut non0idxj: *mut libc::c_int = 0 as *mut libc::c_int;
    non0ctri = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_int;
    cache = non0ctri.offset((i_prim + j_prim + i_prim * i_ctr + j_prim * j_ctr) as isize)
        as *mut libc::c_double;
    non0ctrj = non0ctri.offset(i_prim as isize);
    non0idxi = non0ctrj.offset(j_prim as isize);
    non0idxj = non0idxi.offset((i_prim * i_ctr) as isize);
    CINTOpt_non0coeff_byshell(non0idxi, non0ctri, ci, i_prim, i_ctr);
    CINTOpt_non0coeff_byshell(non0idxj, non0ctrj, cj, j_prim, j_ctr);
    let nc: libc::c_int = i_ctr * j_ctr;
    let leng: libc::c_int = (*envs).g_size * 3 as libc::c_int
        * (((1 as libc::c_int) << (*envs).gbits) + 1 as libc::c_int);
    let lenj: libc::c_int = 104 as libc::c_int * nf * nc * n_comp;
    let leni: libc::c_int = 104 as libc::c_int * nf * i_ctr * n_comp;
    let len0: libc::c_int = 104 as libc::c_int * nf * n_comp;
    let len: libc::c_int = leng + lenj + leni + len0;
    let mut gridsT: *mut libc::c_double = 0 as *mut libc::c_double;
    gridsT = ((cache as uintptr_t).wrapping_add(63 as libc::c_int as libc::c_ulong)
        & (64 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_double;
    cache = gridsT.offset((len + 104 as libc::c_int * 3 as libc::c_int) as isize);
    let mut g: *mut libc::c_double = gridsT
        .offset((104 as libc::c_int * 3 as libc::c_int) as isize);
    let mut g1: *mut libc::c_double = g.offset(leng as isize);
    let mut gout: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut gctri: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut gctrj: *mut libc::c_double = 0 as *mut libc::c_double;
    if n_comp == 1 as libc::c_int {
        gctrj = gctr;
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
    }
    grids_offset = 0 as libc::c_int;
    while grids_offset < ngrids {
        (*envs).c2rust_unnamed.grids_offset = grids_offset;
        bgrids = if ngrids - grids_offset < 104 as libc::c_int {
            ngrids - grids_offset
        } else {
            104 as libc::c_int
        };
        i = 0 as libc::c_int;
        while i < bgrids {
            *gridsT
                .offset(
                    (i + 104 as libc::c_int * 0 as libc::c_int) as isize,
                ) = *grids
                .offset(
                    ((grids_offset + i) * 3 as libc::c_int + 0 as libc::c_int) as isize,
                );
            *gridsT
                .offset(
                    (i + 104 as libc::c_int * 1 as libc::c_int) as isize,
                ) = *grids
                .offset(
                    ((grids_offset + i) * 3 as libc::c_int + 1 as libc::c_int) as isize,
                );
            *gridsT
                .offset(
                    (i + 104 as libc::c_int * 2 as libc::c_int) as isize,
                ) = *grids
                .offset(
                    ((grids_offset + i) * 3 as libc::c_int + 2 as libc::c_int) as isize,
                );
            i += 1;
            i;
        }
        empty[0 as libc::c_int as usize] = 1 as libc::c_int;
        empty[1 as libc::c_int as usize] = 1 as libc::c_int;
        empty[2 as libc::c_int as usize] = 1 as libc::c_int;
        if n_comp == 1 as libc::c_int {
            gctrj = gctr.offset((grids_offset * nf * nc) as isize);
            if j_ctr == 1 as libc::c_int {
                gctri = gctrj;
            }
            if i_ctr == 1 as libc::c_int {
                gout = gctri;
            }
        }
        pdata_ij = pdata_base;
        jp = 0 as libc::c_int;
        while jp < j_prim {
            (*envs).aj[0 as libc::c_int as usize] = *aj.offset(jp as isize);
            if j_ctr == 1 as libc::c_int {
                fac1j = (*envs).common_factor * *cj.offset(jp as isize);
            } else {
                fac1j = (*envs).common_factor;
                *iempty = 1 as libc::c_int;
            }
            ip = 0 as libc::c_int;
            while ip < i_prim {
                if !((*pdata_ij).cceij > expcutoff) {
                    (*envs).ai[0 as libc::c_int as usize] = *ai.offset(ip as isize);
                    expij = (*pdata_ij).eij;
                    cutoff = expcutoff - (*pdata_ij).cceij;
                    rij = ((*pdata_ij).rij).as_mut_ptr();
                    (*envs)
                        .rij[0 as libc::c_int
                        as usize] = *rij.offset(0 as libc::c_int as isize);
                    (*envs)
                        .rij[1 as libc::c_int
                        as usize] = *rij.offset(1 as libc::c_int as isize);
                    (*envs)
                        .rij[2 as libc::c_int
                        as usize] = *rij.offset(2 as libc::c_int as isize);
                    if i_ctr == 1 as libc::c_int {
                        fac1i = fac1j * *ci.offset(ip as isize) * expij;
                    } else {
                        fac1i = fac1j * expij;
                    }
                    (*envs).fac[0 as libc::c_int as usize] = fac1i;
                    CINTg0_1e_grids(g, cutoff, envs, cache, gridsT);
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
                    *iempty = 0 as libc::c_int;
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
                *jempty = 0 as libc::c_int;
            }
            jp += 1;
            jp;
        }
        if n_comp > 1 as libc::c_int && *jempty == 0 {
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
        grids_offset += 104 as libc::c_int;
    }
    return (all_empty == 0) as libc::c_int;
}
unsafe extern "C" fn _transpose_comps(
    mut gctr: *mut libc::c_double,
    mut gctrj: *mut libc::c_double,
    mut bgrids: libc::c_int,
    mut dij: libc::c_int,
    mut ngrids: libc::c_int,
    mut n_comp: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    let mut ic: libc::c_int = 0;
    let mut ig: libc::c_int = 0;
    let mut pgctr: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut pgctrj: *mut libc::c_double = 0 as *mut libc::c_double;
    ic = 0 as libc::c_int;
    while ic < n_comp {
        pgctr = gctr.offset((ic * dij * ngrids) as isize);
        n = 0 as libc::c_int;
        while n < dij {
            pgctrj = gctrj.offset(((n * n_comp + ic) * bgrids) as isize);
            ig = 0 as libc::c_int;
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
    let mut bas: *mut libc::c_int = (*envs).bas;
    let mut shls: *mut libc::c_int = (*envs).shls;
    let mut x_ctr: *mut libc::c_int = ((*envs).x_ctr).as_mut_ptr();
    let mut ngrids: libc::c_int = (*envs).c2rust_unnamed_0.ngrids;
    let mut nroots: libc::c_int = (*envs).nrys_roots;
    let mut nf: libc::c_int = (*envs).nf;
    let mut nc: libc::c_int = ngrids * nf * *x_ctr.offset(0 as libc::c_int as isize)
        * *x_ctr.offset(1 as libc::c_int as isize);
    let mut n_comp: libc::c_int = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
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
    let mut pdata_size: libc::c_int = i_prim * j_prim * 5 as libc::c_int
        + i_prim * *x_ctr.offset(0 as libc::c_int as isize)
        + j_prim * *x_ctr.offset(1 as libc::c_int as isize)
        + (i_prim + j_prim) * 2 as libc::c_int + (*envs).nf * 3 as libc::c_int;
    let mut leng: size_t = ((*envs).g_size * 3 as libc::c_int
        * (((1 as libc::c_int) << (*envs).gbits) + 1 as libc::c_int)) as size_t;
    let mut len0: size_t = (104 as libc::c_int * nf * n_comp) as size_t;
    let mut leni: size_t = len0
        .wrapping_mul(*x_ctr.offset(0 as libc::c_int as isize) as libc::c_ulong);
    let mut lenj: size_t = leni
        .wrapping_mul(*x_ctr.offset(1 as libc::c_int as isize) as libc::c_ulong);
    let mut cache_size: size_t = if ((nc * n_comp) as libc::c_ulong)
        .wrapping_add(leng)
        .wrapping_add(len0)
        .wrapping_add(leni)
        .wrapping_add(lenj)
        .wrapping_add(pdata_size as libc::c_ulong)
        .wrapping_add(
            (104 as libc::c_int
                * (if n_comp > nroots + 10 as libc::c_int {
                    n_comp
                } else {
                    nroots + 10 as libc::c_int
                })) as libc::c_ulong,
        )
        > (nc * n_comp + 104 as libc::c_int * nf * 8 as libc::c_int * 2 as libc::c_int)
            as libc::c_ulong
    {
        ((nc * n_comp) as libc::c_ulong)
            .wrapping_add(leng)
            .wrapping_add(len0)
            .wrapping_add(leni)
            .wrapping_add(lenj)
            .wrapping_add(pdata_size as libc::c_ulong)
            .wrapping_add(
                (104 as libc::c_int
                    * (if n_comp > nroots + 10 as libc::c_int {
                        n_comp
                    } else {
                        nroots + 10 as libc::c_int
                    })) as libc::c_ulong,
            )
    } else {
        (nc * n_comp + 104 as libc::c_int * nf * 8 as libc::c_int * 2 as libc::c_int)
            as libc::c_ulong
    };
    return cache_size.wrapping_add(32 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn CINT1e_grids_drv(
    mut out: *mut libc::c_double,
    mut dims: *mut libc::c_int,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut libc::c_double,
    mut f_c2s: Option::<unsafe extern "C" fn() -> ()>,
) -> libc::c_int {
    if out.is_null() {
        return int1e_grids_cache_size(envs) as libc::c_int;
    }
    let mut x_ctr: *mut libc::c_int = ((*envs).x_ctr).as_mut_ptr();
    let mut ngrids_nf: libc::c_int = (*envs).c2rust_unnamed_0.ngrids * (*envs).nf;
    let mut nc: libc::c_int = ngrids_nf * *x_ctr.offset(0 as libc::c_int as isize)
        * *x_ctr.offset(1 as libc::c_int as isize);
    let mut n_comp: libc::c_int = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut stack: *mut libc::c_double = 0 as *mut libc::c_double;
    if cache.is_null() {
        let mut cache_size: size_t = int1e_grids_cache_size(envs);
        stack = malloc(
            (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(cache_size),
        ) as *mut libc::c_double;
        cache = stack;
    }
    let mut gctr: *mut libc::c_double = 0 as *mut libc::c_double;
    gctr = ((cache as uintptr_t).wrapping_add(63 as libc::c_int as libc::c_ulong)
        & (64 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_double;
    cache = gctr.offset((nc * n_comp) as isize);
    let mut has_value: libc::c_int = CINT1e_grids_loop(gctr, envs, cache);
    let mut counts: [libc::c_int; 4] = [0; 4];
    if dims.is_null() {
        dims = counts.as_mut_ptr();
    }
    if f_c2s
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
                c2s_sph_1e_grids
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
        counts[2 as libc::c_int as usize] = (*envs).c2rust_unnamed_0.ngrids;
        counts[3 as libc::c_int as usize] = 1 as libc::c_int;
    } else if f_c2s
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
                c2s_cart_1e_grids
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
            as usize] = (*envs).nfi * *x_ctr.offset(0 as libc::c_int as isize);
        counts[1 as libc::c_int
            as usize] = (*envs).nfj * *x_ctr.offset(1 as libc::c_int as isize);
        counts[2 as libc::c_int as usize] = (*envs).c2rust_unnamed_0.ngrids;
        counts[3 as libc::c_int as usize] = 1 as libc::c_int;
    }
    let mut nout: libc::c_int = *dims.offset(0 as libc::c_int as isize)
        * *dims.offset(1 as libc::c_int as isize)
        * *dims.offset(2 as libc::c_int as isize);
    let mut n: libc::c_int = 0;
    if has_value != 0 {
        n = 0 as libc::c_int;
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
        n = 0 as libc::c_int;
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
        0 as libc::c_int,
        1 as libc::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: 0 as *mut libc::c_int,
        bas: 0 as *mut libc::c_int,
        env: 0 as *mut libc::c_double,
        shls: 0 as *mut libc::c_int,
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: 0 as *mut libc::c_double,
        rx_in_rklrx: 0 as *mut libc::c_double,
        ri: 0 as *mut libc::c_double,
        rj: 0 as *mut libc::c_double,
        rk: 0 as *mut libc::c_double,
        c2rust_unnamed_1: C2RustUnnamed {
            rl: 0 as *mut libc::c_double,
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: 0 as *mut CINTOpt,
        idx: 0 as *mut libc::c_int,
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
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
            CINTgout1e_grids
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
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
                c2s_sph_1e_grids
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut libc::c_double,
                        *mut libc::c_int,
                        *mut CINTEnvVars,
                        *mut libc::c_double,
                    ) -> (),
            ),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_grids_optimizer(
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
pub unsafe extern "C" fn int1e_grids_cart(
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
        0 as libc::c_int,
        1 as libc::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: 0 as *mut libc::c_int,
        bas: 0 as *mut libc::c_int,
        env: 0 as *mut libc::c_double,
        shls: 0 as *mut libc::c_int,
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: 0 as *mut libc::c_double,
        rx_in_rklrx: 0 as *mut libc::c_double,
        ri: 0 as *mut libc::c_double,
        rj: 0 as *mut libc::c_double,
        rk: 0 as *mut libc::c_double,
        c2rust_unnamed_1: C2RustUnnamed {
            rl: 0 as *mut libc::c_double,
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: 0 as *mut CINTOpt,
        idx: 0 as *mut libc::c_int,
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
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
            CINTgout1e_grids
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
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
                c2s_cart_1e_grids
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut libc::c_double,
                        *mut libc::c_int,
                        *mut CINTEnvVars,
                        *mut libc::c_double,
                    ) -> (),
            ),
        ),
    );
}
