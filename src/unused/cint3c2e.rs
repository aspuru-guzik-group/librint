#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
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
    fn CINTall_3c2e_optimizer(
        opt: *mut *mut CINTOpt,
        ng: *mut libc::c_int,
        atm: *mut libc::c_int,
        natm: libc::c_int,
        bas: *mut libc::c_int,
        nbas: libc::c_int,
        env: *mut libc::c_double,
    );
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
    fn CINTinit_int3c2e_EnvVars(
        envs: *mut CINTEnvVars,
        ng: *mut libc::c_int,
        shls: *mut libc::c_int,
        atm: *mut libc::c_int,
        natm: libc::c_int,
        bas: *mut libc::c_int,
        nbas: libc::c_int,
        env: *mut libc::c_double,
    );
    fn CINTg2e_index_xyz(idx: *mut libc::c_int, envs: *const CINTEnvVars);
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
    fn CINTgout2e(
        g: *mut libc::c_double,
        gout: *mut libc::c_double,
        idx: *mut libc::c_int,
        envs: *mut CINTEnvVars,
        gout_empty: libc::c_int,
    );
    fn CINTdmat_transpose(
        a_t: *mut libc::c_double,
        a: *mut libc::c_double,
        m: libc::c_int,
        n: libc::c_int,
    );
    fn CINTdplus_transpose(
        a_t: *mut libc::c_double,
        a: *mut libc::c_double,
        m: libc::c_int,
        n: libc::c_int,
    );
    fn c2s_sph_3c2e1(
        fijkl: *mut libc::c_double,
        gctr: *mut libc::c_double,
        dims: *mut libc::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut libc::c_double,
    );
    fn c2s_cart_3c2e1(
        fijkl: *mut libc::c_double,
        gctr: *mut libc::c_double,
        dims: *mut libc::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut libc::c_double,
    );
    fn c2s_sph_3c2e1_ssc(
        fijkl: *mut libc::c_double,
        gctr: *mut libc::c_double,
        dims: *mut libc::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut libc::c_double,
    );
    fn c2s_dset0(
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
