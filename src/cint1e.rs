#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::cint_bas::CINTcgto_spinor;
use crate::optimizer::CINTOpt_log_max_pgto_coeff;
use crate::optimizer::CINTOpt_non0coeff_byshell;
// use crate::optimizer::CINTset_pairdata;
// use crate::g1e::CINTinit_int1e_EnvVars;
// use crate::g1e::CINTg1e_index_xyz;
// use crate::g1e::CINTg1e_ovlp;
// use crate::g1e::CINTg1e_nuc;
// use crate::g1e::CINTcommon_fac_sp;
// use crate::g1e::CINTprim_to_ctr_0;
// use crate::g1e::CINTprim_to_ctr_1;
use crate::fblas::CINTdmat_transpose;
// use crate::cart2sph::c2s_sph_1e;
// use crate::cart2sph::c2s_cart_1e;
use crate::cart2sph::c2s_dset0;


extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    // fn CINTcgto_spinor(bas_id: libc::c_int, bas: *const libc::c_int) -> libc::c_int;
    // fn CINTOpt_log_max_pgto_coeff(
    //     log_maxc: *mut libc::c_double,
    //     coeff: *mut libc::c_double,
    //     nprim: libc::c_int,
    //     nctr: libc::c_int,
    // );
    // fn CINTOpt_non0coeff_byshell(
    //     sortedidx: *mut libc::c_int,
    //     non0ctr: *mut libc::c_int,
    //     ci: *mut libc::c_double,
    //     iprim: libc::c_int,
    //     ictr: libc::c_int,
    // );
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
    fn CINTg1e_index_xyz(idx: *mut libc::c_int, envs: *mut CINTEnvVars);
    fn CINTg1e_ovlp(g: *mut libc::c_double, envs: *mut CINTEnvVars) -> libc::c_int;
    fn CINTg1e_nuc(
        g: *mut libc::c_double,
        envs: *mut CINTEnvVars,
        nuc_id: libc::c_int,
    ) -> libc::c_int;
    fn CINTcommon_fac_sp(l: libc::c_int) -> libc::c_double;
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
    // fn CINTdmat_transpose(
    //     a_t: *mut libc::c_double,
    //     a: *mut libc::c_double,
    //     m: libc::c_int,
    //     n: libc::c_int,
    // );
    fn c2s_sph_1e(
        opij: *mut libc::c_double,
        gctr: *mut libc::c_double,
        dims: *mut libc::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut libc::c_double,
    );
    fn c2s_cart_1e(
        opij: *mut libc::c_double,
        gctr: *mut libc::c_double,
        dims: *mut libc::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut libc::c_double,
    );
    // fn c2s_dset0(
    //     out: *mut libc::c_double,
    //     dims: *mut libc::c_int,
    //     counts: *mut libc::c_int,
    // );
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
    pub f_g0_2e: Option::<unsafe fn() -> libc::c_int>,
    pub f_g0_2d4d: Option::<unsafe fn() -> ()>,
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

impl CINTEnvVars {
    fn new() -> Self {
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
    envs
    }
}

//typedef struct {
//        FINT *atm;
//        FINT *bas;
//        double *env;
//        FINT *shls;
//        FINT natm;
//        FINT nbas;
//
//        FINT i_l;
//        FINT j_l;
//        FINT k_l;
//        FINT l_l;
//        FINT nfi;  // number of cartesian components
//        FINT nfj;
//        // in int1e_grids, the grids_offset and the number of grids
//        union {FINT nfk; FINT grids_offset;};
//        union {FINT nfl; FINT ngrids;};
//        FINT nf;  // = nfi*nfj*nfk*nfl;
//        FINT rys_order; // = nrys_roots for regular ERIs. can be nrys_roots/2 for SR ERIs
//        FINT x_ctr[4];
//
//        FINT gbits;
//        FINT ncomp_e1; // = 1 if spin free, = 4 when spin included, it
//        FINT ncomp_e2; // corresponds to POSX,POSY,POSZ,POS1, see cint.h
//        FINT ncomp_tensor; // e.g. = 3 for gradients
//
//        /* values may diff based on the g0_2d4d algorithm */
//        FINT li_ceil; // power of x, == i_l if nabla is involved, otherwise == i_l
//        FINT lj_ceil;
//        FINT lk_ceil;
//        FINT ll_ceil;
//        FINT g_stride_i; // nrys_roots * shift of (i++,k,l,j)
//        FINT g_stride_k; // nrys_roots * shift of (i,k++,l,j)
//        FINT g_stride_l; // nrys_roots * shift of (i,k,l++,j)
//        FINT g_stride_j; // nrys_roots * shift of (i,k,l,j++)
//        FINT nrys_roots;
//        FINT g_size;  // ref to cint2e.c g = malloc(sizeof(double)*g_size)
//
//        FINT g2d_ijmax;
//        FINT g2d_klmax;
//        double common_factor;
//        double expcutoff;
//        double rirj[3]; // diff by sign in different g0_2d4d algorithm
//        double rkrl[3];
//        double *rx_in_rijrx;
//        double *rx_in_rklrx;
//
//        double *ri;
//        double *rj;
//        double *rk;
//        // in int2e or int3c2e, the coordinates of the fourth shell
//        // in int1e_grids, the pointer for the grids coordinates
//        union {double *rl; double *grids;};
//
//        FINT (*f_g0_2e)();
//        void (*f_g0_2d4d)();
//        void (*f_gout)();
//        CINTOpt *opt;
//
//        /* values are assigned during calculation */
//        int *idx;
//        double ai[1];
//        double aj[1];
//        double ak[1];
//        double al[1];
//        double fac[1];
//        double rij[3];
//        double rkl[3];
//} CINTEnvVars;
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
pub unsafe extern "C" fn CINT1e_loop(
    mut gctr: *mut libc::c_double,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut libc::c_double,
    mut int1e_type: libc::c_int,
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
    let mut n_comp: libc::c_int = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
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
    let mut ip: libc::c_int = 0;
    let mut jp: libc::c_int = 0;
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
    let mut rij: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut idx: *mut libc::c_int = 0 as *mut libc::c_int;
    idx = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_int;
    cache = idx.offset(((*envs).nf * 3 as libc::c_int) as isize) as *mut libc::c_double;
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
    let lenj: libc::c_int = (*envs).nf * nc * n_comp;
    let leni: libc::c_int = (*envs).nf * i_ctr * n_comp;
    let len0: libc::c_int = (*envs).nf * n_comp;
    let len: libc::c_int = leng + lenj + leni + len0;
    let mut g: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut gout: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut gctri: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut gctrj: *mut libc::c_double = 0 as *mut libc::c_double;
    g = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_double;
    cache = g.offset(len as isize);
    let mut g1: *mut libc::c_double = g.offset(leng as isize);
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
    let mut common_factor: libc::c_double = (*envs).common_factor
        * CINTcommon_fac_sp((*envs).i_l) * CINTcommon_fac_sp((*envs).j_l);
    pdata_ij = pdata_base;
    jp = 0 as libc::c_int;
    while jp < j_prim {
        (*envs).aj[0 as libc::c_int as usize] = *aj.offset(jp as isize);
        if j_ctr == 1 as libc::c_int {
            fac1j = common_factor * *cj.offset(jp as isize);
        } else {
            fac1j = common_factor;
            *iempty = 1 as libc::c_int;
        }
        ip = 0 as libc::c_int;
        while ip < i_prim {
            if !((*pdata_ij).cceij > expcutoff) {
                (*envs).ai[0 as libc::c_int as usize] = *ai.offset(ip as isize);
                expij = (*pdata_ij).eij;
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
                make_g1e_gout(gout, g, idx, envs, *gempty, int1e_type);
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
    if n_comp > 1 as libc::c_int && *jempty == 0 {
        CINTdmat_transpose(gctr, gctrj, (*envs).nf * nc, n_comp);
    }
    return (*jempty == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn int1e_cache_size(mut envs: *mut CINTEnvVars) -> libc::c_int {
    let mut shls: *mut libc::c_int = (*envs).shls;
    let mut bas: *mut libc::c_int = (*envs).bas;
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
    let mut x_ctr: *mut libc::c_int = ((*envs).x_ctr).as_mut_ptr();
    let mut nc: libc::c_int = (*envs).nf * *x_ctr.offset(0 as libc::c_int as isize)
        * *x_ctr.offset(1 as libc::c_int as isize);
    let mut n_comp: libc::c_int = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut leng: libc::c_int = (*envs).g_size * 3 as libc::c_int
        * (((1 as libc::c_int) << (*envs).gbits) + 1 as libc::c_int);
    let mut lenj: libc::c_int = (*envs).nf * nc * n_comp;
    let mut leni: libc::c_int = (*envs).nf * *x_ctr.offset(0 as libc::c_int as isize)
        * n_comp;
    let mut len0: libc::c_int = (*envs).nf * n_comp;
    let mut pdata_size: libc::c_int = i_prim * j_prim * 5 as libc::c_int
        + i_prim * *x_ctr.offset(0 as libc::c_int as isize)
        + j_prim * *x_ctr.offset(1 as libc::c_int as isize)
        + (i_prim + j_prim) * 2 as libc::c_int + (*envs).nf * 3 as libc::c_int;
    let mut cache_size: libc::c_int = if nc * n_comp + leng + lenj + leni + len0
        + pdata_size > nc * n_comp + (*envs).nf * 8 as libc::c_int * 2 as libc::c_int
    {
        nc * n_comp + leng + lenj + leni + len0 + pdata_size
    } else {
        nc * n_comp + (*envs).nf * 8 as libc::c_int * 2 as libc::c_int
    };
    return cache_size;
}
#[no_mangle]
pub unsafe extern "C" fn CINT1e_drv(
    mut out: *mut libc::c_double,
    mut dims: *mut libc::c_int,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut libc::c_double,
    mut f_c2s: Option::<unsafe extern "C" fn() -> ()>,
    mut int1e_type: libc::c_int,
) -> libc::c_int {
    if out.is_null() {
        return int1e_cache_size(envs);
    }
    let mut x_ctr: *mut libc::c_int = ((*envs).x_ctr).as_mut_ptr();
    let mut nc: libc::c_int = (*envs).nf * *x_ctr.offset(0 as libc::c_int as isize)
        * *x_ctr.offset(1 as libc::c_int as isize);
    let mut n_comp: libc::c_int = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut stack: *mut libc::c_double = 0 as *mut libc::c_double;
    if cache.is_null() {
        let mut cache_size: size_t = int1e_cache_size(envs) as size_t;
        stack = malloc(
            (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(cache_size),
        ) as *mut libc::c_double;
        cache = stack;
    }
    let mut gctr: *mut libc::c_double = 0 as *mut libc::c_double;
    gctr = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_double;
    cache = gctr.offset((nc * n_comp) as isize);
    let mut has_value: libc::c_int = CINT1e_loop(gctr, envs, cache, int1e_type);
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
                c2s_sph_1e
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
                c2s_cart_1e
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
    }
    counts[2 as libc::c_int as usize] = 1 as libc::c_int;
    counts[3 as libc::c_int as usize] = 1 as libc::c_int;
    let mut nout: libc::c_int = *dims.offset(0 as libc::c_int as isize)
        * *dims.offset(1 as libc::c_int as isize);
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
    mut gout: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut idx: *mut libc::c_int,
    mut envs: *mut CINTEnvVars,
    mut empty: libc::c_int,
    mut int1e_type: libc::c_int,
) {
    let mut ia: libc::c_int = 0;
    match int1e_type {
        0 => {
            CINTg1e_ovlp(g, envs);
            ::core::mem::transmute::<
                _,
                fn(_, _, _, _, _),
            >(
                (Some(((*envs).f_gout).expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )(gout, g, idx, envs, empty);
        }
        1 => {
            CINTg1e_nuc(g, envs, -(1 as libc::c_int));
            ::core::mem::transmute::<
                _,
                fn(_, _, _, _, _),
            >(
                (Some(((*envs).f_gout).expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )(gout, g, idx, envs, empty);
        }
        2 => {
            ia = 0 as libc::c_int;
            while ia < (*envs).natm {
                CINTg1e_nuc(g, envs, ia);
                ::core::mem::transmute::<
                    _,
                    fn(_, _, _, _, _),
                >(
                    (Some(((*envs).f_gout).expect("non-null function pointer")))
                        .expect("non-null function pointer"),
                )(
                    gout,
                    g,
                    idx,
                    envs,
                    (empty != 0 && ia == 0 as libc::c_int) as libc::c_int,
                );
                ia += 1;
                ia;
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e(
    mut gout: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut idx: *mut libc::c_int,
    mut envs: *mut CINTEnvVars,
    mut empty: libc::c_int,
) {
    let mut nf: libc::c_int = (*envs).nf;
    let mut n: libc::c_int = 0;
    let mut ix: libc::c_int = 0;
    let mut iy: libc::c_int = 0;
    let mut iz: libc::c_int = 0;
    if empty != 0 {
        n = 0 as libc::c_int;
        while n < nf {
            ix = *idx.offset((n * 3 as libc::c_int + 0 as libc::c_int) as isize);
            iy = *idx.offset((n * 3 as libc::c_int + 1 as libc::c_int) as isize);
            iz = *idx.offset((n * 3 as libc::c_int + 2 as libc::c_int) as isize);
            *gout
                .offset(
                    n as isize,
                ) = *g.offset(ix as isize) * *g.offset(iy as isize)
                * *g.offset(iz as isize);
            n += 1;
            n;
        }
    } else {
        n = 0 as libc::c_int;
        while n < nf {
            ix = *idx.offset((n * 3 as libc::c_int + 0 as libc::c_int) as isize);
            iy = *idx.offset((n * 3 as libc::c_int + 1 as libc::c_int) as isize);
            iz = *idx.offset((n * 3 as libc::c_int + 2 as libc::c_int) as isize);
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
    mut gout: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut idx: *mut libc::c_int,
    mut envs: *mut CINTEnvVars,
    mut empty: libc::c_int,
) {
    let mut nf: libc::c_int = (*envs).nf;
    let mut nrys_roots: libc::c_int = (*envs).nrys_roots;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut gx: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut gy: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut gz: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut s: libc::c_double = 0.;
    if empty != 0 {
        n = 0 as libc::c_int;
        while n < nf {
            gx = g
                .offset(
                    *idx.offset((n * 3 as libc::c_int + 0 as libc::c_int) as isize)
                        as isize,
                );
            gy = g
                .offset(
                    *idx.offset((n * 3 as libc::c_int + 1 as libc::c_int) as isize)
                        as isize,
                );
            gz = g
                .offset(
                    *idx.offset((n * 3 as libc::c_int + 2 as libc::c_int) as isize)
                        as isize,
                );
            s = 0 as libc::c_int as libc::c_double;
            i = 0 as libc::c_int;
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
        n = 0 as libc::c_int;
        while n < nf {
            gx = g
                .offset(
                    *idx.offset((n * 3 as libc::c_int + 0 as libc::c_int) as isize)
                        as isize,
                );
            gy = g
                .offset(
                    *idx.offset((n * 3 as libc::c_int + 1 as libc::c_int) as isize)
                        as isize,
                );
            gz = g
                .offset(
                    *idx.offset((n * 3 as libc::c_int + 2 as libc::c_int) as isize)
                        as isize,
                );
            s = 0 as libc::c_int as libc::c_double;
            i = 0 as libc::c_int;
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
    let mut ng = [0, 0, 0, 0, 0, 1, 1, 1];
    let mut envs = CINTEnvVars::new();
    CINTinit_int1e_EnvVars(&mut envs, ng.as_mut_ptr(), shls, atm, natm, bas, nbas, env);
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
    return CINT1e_drv(
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
                c2s_sph_1e
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
pub unsafe extern "C" fn int1e_ovlp_cart(
    out: &mut [f64],
    dims: &mut [i32],
    shls: &mut [i32],
    atm: &mut [i32],
    natm: i32,
    bas: &mut [i32],
    nbas: i32,
    env: &mut [f64],
    // TODO: verify if opt isn't a slice too
    //mut opt: *mut CINTOpt,
    opt: &mut CINTOpt,
    cache: &mut [f64],
) -> libc::c_int {
    let mut ng = [0, 0, 0, 0, 0, 1, 1, 1];
    let mut envs = CINTEnvVars::new();
    CINTinit_int1e_EnvVars(&mut envs as *mut CINTEnvVars, ng.as_mut_ptr(), shls.as_mut_ptr(), atm.as_mut_ptr(), natm, bas.as_mut_ptr(), nbas, env.as_mut_ptr());
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
    return CINT1e_drv(
        out.as_mut_ptr(),
        dims.as_mut_ptr(),
        &mut envs as *mut CINTEnvVars,
        cache.as_mut_ptr(),
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
                c2s_cart_1e
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
pub unsafe extern "C" fn int1e_ovlp_optimizer(
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
pub unsafe extern "C" fn int1e_nuc_sph(
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
    let mut ng = [0, 0, 0, 0, 0, 1, 0, 1];
    let mut envs = CINTEnvVars::new();
    CINTinit_int1e_EnvVars(&mut envs, ng.as_mut_ptr(), shls, atm, natm, bas, nbas, env);
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
            CINTgout1e_nuc
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
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
                c2s_sph_1e
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut libc::c_double,
                        *mut libc::c_int,
                        *mut CINTEnvVars,
                        *mut libc::c_double,
                    ) -> (),
            ),
        ),
        2 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_nuc_cart(
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
    let mut ng = [0, 0, 0, 0, 0, 1, 0, 1];
    let mut envs = CINTEnvVars::new();
    CINTinit_int1e_EnvVars(&mut envs, ng.as_mut_ptr(), shls, atm, natm, bas, nbas, env);
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
            CINTgout1e_nuc
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
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
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut libc::c_double,
                        *mut libc::c_int,
                        *mut CINTEnvVars,
                        *mut libc::c_double,
                    ) -> (),
            ),
        ),
        2 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_nuc_optimizer(
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
pub fn cint1e_ovlp_cart(
    out: &mut [f64],
    shls: &mut [i32],
    atm: &mut [i32],
    natm: i32,
    bas: &mut [i32],
    nbas: i32,
    env: &mut [f64],
    opt: &mut CINTOpt,
) -> i32 {
    let mut dims = [0;0];
    let mut cache = [0.0;0];
    unsafe { 
        return int1e_ovlp_cart(
        out,
        &mut dims,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        opt,
        &mut cache,
    );}
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_ovlp_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
) {
    int1e_ovlp_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_ovlp_sph(
    mut out: *mut libc::c_double,
    mut shls: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
    mut opt: *mut CINTOpt,
) -> libc::c_int {
    return int1e_ovlp_sph(
        out,
        0 as *mut libc::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        opt,
        0 as *mut libc::c_double,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_ovlp_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
) {
    int1e_ovlp_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_ovlp_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
) {
    int1e_ovlp_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_nuc_cart(
    mut out: *mut libc::c_double,
    mut shls: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
    mut opt: *mut CINTOpt,
) -> libc::c_int {
    return int1e_nuc_cart(
        out,
        0 as *mut libc::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        opt,
        0 as *mut libc::c_double,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_nuc_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
) {
    int1e_nuc_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_nuc_sph(
    mut out: *mut libc::c_double,
    mut shls: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
    mut opt: *mut CINTOpt,
) -> libc::c_int {
    return int1e_nuc_sph(
        out,
        0 as *mut libc::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        opt,
        0 as *mut libc::c_double,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_nuc_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
) {
    int1e_nuc_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_nuc_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
) {
    int1e_nuc_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_ovlp_sph_(
    mut out: *mut libc::c_double,
    mut shls: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: *mut libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: *mut libc::c_int,
    mut env: *mut libc::c_double,
    mut optptr_as_integer8: size_t,
) -> libc::c_int {
    let mut opt: *mut *mut CINTOpt = optptr_as_integer8 as *mut *mut CINTOpt;
    return int1e_ovlp_sph(
        out,
        0 as *mut libc::c_int,
        shls,
        atm,
        *natm,
        bas,
        *nbas,
        env,
        *opt,
        0 as *mut libc::c_double,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_ovlp_sph_optimizer_(
    mut optptr_as_integer8: size_t,
    mut atm: *mut libc::c_int,
    mut natm: *mut libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: *mut libc::c_int,
    mut env: *mut libc::c_double,
) {
    let mut opt: *mut *mut CINTOpt = optptr_as_integer8 as *mut *mut CINTOpt;
    int1e_ovlp_optimizer(opt, atm, *natm, bas, *nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_ovlp_cart_(
    mut out: *mut libc::c_double,
    mut shls: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: *mut libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: *mut libc::c_int,
    mut env: *mut libc::c_double,
    mut optptr_as_integer8: size_t,
) -> libc::c_int {
    let mut opt: *mut *mut CINTOpt = optptr_as_integer8 as *mut *mut CINTOpt;
    unimplemented!("why do we have the fn_name_ versions of fn_name functions?");
    //return int1e_ovlp_cart(
    //    out,
    //    0 as *mut libc::c_int,
    //    shls,
    //    atm,
    //    *natm,
    //    bas,
    //    *nbas,
    //    env,
    //    *opt,
    //    0 as *mut libc::c_double,
    //);
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_ovlp_cart_optimizer_(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: *mut libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: *mut libc::c_int,
    mut env: *mut libc::c_double,
) {
    int1e_ovlp_optimizer(opt, atm, *natm, bas, *nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_ovlp_optimizer_(
    mut optptr_as_integer8: size_t,
    mut atm: *mut libc::c_int,
    mut natm: *mut libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: *mut libc::c_int,
    mut env: *mut libc::c_double,
) {
    let mut opt: *mut *mut CINTOpt = optptr_as_integer8 as *mut *mut CINTOpt;
    int1e_ovlp_optimizer(opt, atm, *natm, bas, *nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_nuc_sph_(
    mut out: *mut libc::c_double,
    mut shls: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: *mut libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: *mut libc::c_int,
    mut env: *mut libc::c_double,
    mut optptr_as_integer8: size_t,
) -> libc::c_int {
    let mut opt: *mut *mut CINTOpt = optptr_as_integer8 as *mut *mut CINTOpt;
    return int1e_nuc_sph(
        out,
        0 as *mut libc::c_int,
        shls,
        atm,
        *natm,
        bas,
        *nbas,
        env,
        *opt,
        0 as *mut libc::c_double,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_nuc_sph_optimizer_(
    mut optptr_as_integer8: size_t,
    mut atm: *mut libc::c_int,
    mut natm: *mut libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: *mut libc::c_int,
    mut env: *mut libc::c_double,
) {
    let mut opt: *mut *mut CINTOpt = optptr_as_integer8 as *mut *mut CINTOpt;
    int1e_nuc_optimizer(opt, atm, *natm, bas, *nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_nuc_cart_(
    mut out: *mut libc::c_double,
    mut shls: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: *mut libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: *mut libc::c_int,
    mut env: *mut libc::c_double,
    mut optptr_as_integer8: size_t,
) -> libc::c_int {
    let mut opt: *mut *mut CINTOpt = optptr_as_integer8 as *mut *mut CINTOpt;
    return int1e_nuc_cart(
        out,
        0 as *mut libc::c_int,
        shls,
        atm,
        *natm,
        bas,
        *nbas,
        env,
        *opt,
        0 as *mut libc::c_double,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_nuc_cart_optimizer_(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: *mut libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: *mut libc::c_int,
    mut env: *mut libc::c_double,
) {
    int1e_nuc_optimizer(opt, atm, *natm, bas, *nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_nuc_optimizer_(
    mut optptr_as_integer8: size_t,
    mut atm: *mut libc::c_int,
    mut natm: *mut libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: *mut libc::c_int,
    mut env: *mut libc::c_double,
) {
    let mut opt: *mut *mut CINTOpt = optptr_as_integer8 as *mut *mut CINTOpt;
    int1e_nuc_optimizer(opt, atm, *natm, bas, *nbas, env);
}
