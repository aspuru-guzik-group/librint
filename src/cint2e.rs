#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn CINTcgto_spinor(bas_id: libc::c_int, bas: *const libc::c_int) -> libc::c_int;
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
    fn CINTg2e_index_xyz(idx: *mut libc::c_int, envs: *const CINTEnvVars);
    fn CINTinit_int2e_EnvVars(
        envs: *mut CINTEnvVars,
        ng: *mut libc::c_int,
        shls: *mut libc::c_int,
        atm: *mut libc::c_int,
        natm: libc::c_int,
        bas: *mut libc::c_int,
        nbas: libc::c_int,
        env: *mut libc::c_double,
    );
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
    fn CINTall_2e_optimizer(
        opt: *mut *mut CINTOpt,
        ng: *mut libc::c_int,
        atm: *mut libc::c_int,
        natm: libc::c_int,
        bas: *mut libc::c_int,
        nbas: libc::c_int,
        env: *mut libc::c_double,
    );
    fn CINTdplus_transpose(
        a_t: *mut libc::c_double,
        a: *mut libc::c_double,
        m: libc::c_int,
        n: libc::c_int,
    );
    fn CINTdmat_transpose(
        a_t: *mut libc::c_double,
        a: *mut libc::c_double,
        m: libc::c_int,
        n: libc::c_int,
    );
    fn c2s_sph_2e1(
        fijkl: *mut libc::c_double,
        gctr: *mut libc::c_double,
        dims: *mut libc::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut libc::c_double,
    );
    fn c2s_cart_2e1(
        fijkl: *mut libc::c_double,
        gctr: *mut libc::c_double,
        dims: *mut libc::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut libc::c_double,
    );
    fn c2s_sf_2e1(
        opij: *mut libc::c_double,
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
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type uintptr_t = libc::c_ulong;
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
#[no_mangle]
pub unsafe extern "C" fn CINT2e_loop_nopt(
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
    let mut l_sh: libc::c_int = *shls.offset(3 as libc::c_int as isize);
    let mut i_ctr: libc::c_int = (*envs).x_ctr[0 as libc::c_int as usize];
    let mut j_ctr: libc::c_int = (*envs).x_ctr[1 as libc::c_int as usize];
    let mut k_ctr: libc::c_int = (*envs).x_ctr[2 as libc::c_int as usize];
    let mut l_ctr: libc::c_int = (*envs).x_ctr[3 as libc::c_int as usize];
    let mut i_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * i_sh + 2 as libc::c_int) as isize);
    let mut j_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * j_sh + 2 as libc::c_int) as isize);
    let mut k_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * k_sh + 2 as libc::c_int) as isize);
    let mut l_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * l_sh + 2 as libc::c_int) as isize);
    let mut rk: *mut libc::c_double = (*envs).rk;
    let mut rl: *mut libc::c_double = (*envs).c2rust_unnamed_1.rl;
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
    let mut al: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * l_sh + 5 as libc::c_int) as isize) as isize,
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
    let mut cl: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * l_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut expcutoff: libc::c_double = (*envs).expcutoff;
    let mut rr_ij: libc::c_double = (*envs).rirj[0 as libc::c_int as usize]
        * (*envs).rirj[0 as libc::c_int as usize]
        + (*envs).rirj[1 as libc::c_int as usize]
            * (*envs).rirj[1 as libc::c_int as usize]
        + (*envs).rirj[2 as libc::c_int as usize]
            * (*envs).rirj[2 as libc::c_int as usize];
    let mut rr_kl: libc::c_double = (*envs).rkrl[0 as libc::c_int as usize]
        * (*envs).rkrl[0 as libc::c_int as usize]
        + (*envs).rkrl[1 as libc::c_int as usize]
            * (*envs).rkrl[1 as libc::c_int as usize]
        + (*envs).rkrl[2 as libc::c_int as usize]
            * (*envs).rkrl[2 as libc::c_int as usize];
    let mut log_maxci: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut log_maxcj: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut log_maxck: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut log_maxcl: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut pdata_base: *mut PairData = 0 as *mut PairData;
    let mut pdata_ij: *mut PairData = 0 as *mut PairData;
    log_maxci = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_double;
    cache = log_maxci.offset((i_prim + j_prim + k_prim + l_prim) as isize);
    pdata_base = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut PairData;
    cache = pdata_base.offset((i_prim * j_prim) as isize) as *mut libc::c_double;
    log_maxcj = log_maxci.offset(i_prim as isize);
    log_maxck = log_maxcj.offset(j_prim as isize);
    log_maxcl = log_maxck.offset(k_prim as isize);
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
    CINTOpt_log_max_pgto_coeff(log_maxck, ck, k_prim, k_ctr);
    CINTOpt_log_max_pgto_coeff(log_maxcl, cl, l_prim, l_ctr);
    let mut n_comp: libc::c_int = (*envs).ncomp_e1 * (*envs).ncomp_e2
        * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: libc::c_double = 0.;
    let mut fac1j: libc::c_double = 0.;
    let mut fac1k: libc::c_double = 0.;
    let mut fac1l: libc::c_double = 0.;
    let mut ip: libc::c_int = 0;
    let mut jp: libc::c_int = 0;
    let mut kp: libc::c_int = 0;
    let mut lp: libc::c_int = 0;
    let mut _empty: [libc::c_int; 5] = [
        1 as libc::c_int,
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
    let mut lempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(3 as libc::c_int as isize);
    let mut gempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(4 as libc::c_int as isize);
    let mut lkl: libc::c_int = (*envs).lk_ceil + (*envs).ll_ceil;
    let mut akl: libc::c_double = 0.;
    let mut ekl: libc::c_double = 0.;
    let mut expijkl: libc::c_double = 0.;
    let mut ccekl: libc::c_double = 0.;
    let mut log_rr_kl: libc::c_double = 0.;
    let mut eijcutoff: libc::c_double = 0.;
    let mut cutoff: libc::c_double = 0.;
    let mut rkl: [libc::c_double; 3] = [0.; 3];
    let mut rij: *mut libc::c_double = 0 as *mut libc::c_double;
    akl = *ak.offset((k_prim - 1 as libc::c_int) as isize)
        + *al.offset((l_prim - 1 as libc::c_int) as isize);
    log_rr_kl = 1.7f64 - 1.5f64 * log(akl);
    let mut omega: libc::c_double = *env.offset(8 as libc::c_int as isize);
    if omega < 0 as libc::c_int as libc::c_double {
        if (*envs).rys_order > 1 as libc::c_int {
            let mut r_guess: libc::c_double = 8.0f64;
            let mut omega2: libc::c_double = omega * omega;
            let mut lij: libc::c_int = (*envs).li_ceil + (*envs).lj_ceil;
            if lij > 0 as libc::c_int {
                let mut aij: libc::c_double = *ai
                    .offset((i_prim - 1 as libc::c_int) as isize)
                    + *aj.offset((j_prim - 1 as libc::c_int) as isize);
                let mut dist_ij: libc::c_double = sqrt(rr_ij);
                let mut theta: libc::c_double = omega2 / (omega2 + aij);
                expcutoff
                    += lij as libc::c_double
                        * log((dist_ij + theta * r_guess + 1.0f64) / (dist_ij + 1.0f64));
            }
            if lkl > 0 as libc::c_int {
                let mut theta_0: libc::c_double = omega2 / (omega2 + akl);
                log_rr_kl
                    += lkl as libc::c_double
                        * log(sqrt(rr_kl) + theta_0 * r_guess + 1.0f64);
            }
        }
    } else if lkl > 0 as libc::c_int {
        log_rr_kl += lkl as libc::c_double * log(sqrt(rr_kl) + 1.0f64);
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
    let mut non0ctrl: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut non0idxi: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut non0idxj: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut non0idxk: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut non0idxl: *mut libc::c_int = 0 as *mut libc::c_int;
    non0ctri = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_int;
    cache = non0ctri
        .offset(
            (i_prim + j_prim + k_prim + l_prim + i_prim * i_ctr + j_prim * j_ctr
                + k_prim * k_ctr + l_prim * l_ctr) as isize,
        ) as *mut libc::c_double;
    non0ctrj = non0ctri.offset(i_prim as isize);
    non0ctrk = non0ctrj.offset(j_prim as isize);
    non0ctrl = non0ctrk.offset(k_prim as isize);
    non0idxi = non0ctrl.offset(l_prim as isize);
    non0idxj = non0idxi.offset((i_prim * i_ctr) as isize);
    non0idxk = non0idxj.offset((j_prim * j_ctr) as isize);
    non0idxl = non0idxk.offset((k_prim * k_ctr) as isize);
    CINTOpt_non0coeff_byshell(non0idxi, non0ctri, ci, i_prim, i_ctr);
    CINTOpt_non0coeff_byshell(non0idxj, non0ctrj, cj, j_prim, j_ctr);
    CINTOpt_non0coeff_byshell(non0idxk, non0ctrk, ck, k_prim, k_ctr);
    CINTOpt_non0coeff_byshell(non0idxl, non0ctrl, cl, l_prim, l_ctr);
    let mut nc: libc::c_int = i_ctr * j_ctr * k_ctr * l_ctr;
    let mut leng: size_t = ((*envs).g_size * 3 as libc::c_int
        * (((1 as libc::c_int) << (*envs).gbits) + 1 as libc::c_int)) as size_t;
    let mut lenl: size_t = nf
        .wrapping_mul(nc as libc::c_ulong)
        .wrapping_mul(n_comp as libc::c_ulong);
    let mut lenk: size_t = nf
        .wrapping_mul(i_ctr as libc::c_ulong)
        .wrapping_mul(j_ctr as libc::c_ulong)
        .wrapping_mul(k_ctr as libc::c_ulong)
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
        .wrapping_add(lenl)
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
    let mut gctrl: *mut libc::c_double = 0 as *mut libc::c_double;
    if n_comp == 1 as libc::c_int {
        gctrl = gctr;
        lempty = empty;
    } else {
        gctrl = g1;
        g1 = g1.offset(lenl as isize);
    }
    if l_ctr == 1 as libc::c_int {
        gctrk = gctrl;
        kempty = lempty;
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
    lp = 0 as libc::c_int;
    while lp < l_prim {
        (*envs).al[0 as libc::c_int as usize] = *al.offset(lp as isize);
        if l_ctr == 1 as libc::c_int {
            fac1l = (*envs).common_factor * *cl.offset(lp as isize);
        } else {
            fac1l = (*envs).common_factor;
            *kempty = 1 as libc::c_int;
        }
        kp = 0 as libc::c_int;
        while kp < k_prim {
            akl = *ak.offset(kp as isize) + *al.offset(lp as isize);
            ekl = rr_kl * *ak.offset(kp as isize) * *al.offset(lp as isize) / akl;
            ccekl = ekl - log_rr_kl - *log_maxck.offset(kp as isize)
                - *log_maxcl.offset(lp as isize);
            if !(ccekl > expcutoff) {
                (*envs).ak[0 as libc::c_int as usize] = *ak.offset(kp as isize);
                rkl[0 as libc::c_int
                    as usize] = (*ak.offset(kp as isize)
                    * *rk.offset(0 as libc::c_int as isize)
                    + *al.offset(lp as isize) * *rl.offset(0 as libc::c_int as isize))
                    / akl;
                rkl[1 as libc::c_int
                    as usize] = (*ak.offset(kp as isize)
                    * *rk.offset(1 as libc::c_int as isize)
                    + *al.offset(lp as isize) * *rl.offset(1 as libc::c_int as isize))
                    / akl;
                rkl[2 as libc::c_int
                    as usize] = (*ak.offset(kp as isize)
                    * *rk.offset(2 as libc::c_int as isize)
                    + *al.offset(lp as isize) * *rl.offset(2 as libc::c_int as isize))
                    / akl;
                eijcutoff = expcutoff - ccekl;
                ekl = exp(-ekl);
                if k_ctr == 1 as libc::c_int {
                    fac1k = fac1l * *ck.offset(kp as isize);
                } else {
                    fac1k = fac1l;
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
                        if !((*pdata_ij).cceij > eijcutoff) {
                            (*envs)
                                .ai[0 as libc::c_int as usize] = *ai.offset(ip as isize);
                            rij = ((*pdata_ij).rij).as_mut_ptr();
                            cutoff = eijcutoff - (*pdata_ij).cceij;
                            expijkl = (*pdata_ij).eij * ekl;
                            if i_ctr == 1 as libc::c_int {
                                fac1i = fac1j * *ci.offset(ip as isize) * expijkl;
                            } else {
                                fac1i = fac1j * expijkl;
                            }
                            (*envs).fac[0 as libc::c_int as usize] = fac1i;
                            if ::core::mem::transmute::<
                                _,
                                fn(_, _, _, _, _) -> libc::c_int,
                            >(
                                (Some(
                                    ((*envs).f_g0_2e).expect("non-null function pointer"),
                                ))
                                    .expect("non-null function pointer"),
                            )(g, rij, rkl.as_mut_ptr(), cutoff, envs) != 0
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
            }
            kp += 1;
            kp;
        }
        if *kempty == 0 {
            if l_ctr > 1 as libc::c_int {
                if *lempty != 0 {
                    CINTprim_to_ctr_0(
                        gctrl,
                        gctrk,
                        cl.offset(lp as isize),
                        lenk,
                        l_prim,
                        l_ctr,
                        *non0ctrl.offset(lp as isize),
                        non0idxl.offset((lp * l_ctr) as isize),
                    );
                } else {
                    CINTprim_to_ctr_1(
                        gctrl,
                        gctrk,
                        cl.offset(lp as isize),
                        lenk,
                        l_prim,
                        l_ctr,
                        *non0ctrl.offset(lp as isize),
                        non0idxl.offset((lp * l_ctr) as isize),
                    );
                }
            }
            *lempty = 0 as libc::c_int;
        }
        lp += 1;
        lp;
    }
    if n_comp > 1 as libc::c_int && *lempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gctrl,
                nf.wrapping_mul(nc as libc::c_ulong) as libc::c_int,
                n_comp,
            );
            *empty = 0 as libc::c_int;
        } else {
            CINTdplus_transpose(
                gctr,
                gctrl,
                nf.wrapping_mul(nc as libc::c_ulong) as libc::c_int,
                n_comp,
            );
        }
    }
    return (*empty == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINT2e_1111_loop(
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
    let mut l_sh: libc::c_int = *shls.offset(3 as libc::c_int as isize);
    let mut opt: *mut CINTOpt = (*envs).opt;
    if !((*opt).pairdata).is_null()
        && (*((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize)
            == 0xffffffffffffffff as libc::c_ulong as *mut libc::c_void as *mut PairData
            || *((*opt).pairdata).offset((k_sh * (*opt).nbas + l_sh) as isize)
                == 0xffffffffffffffff as libc::c_ulong as *mut libc::c_void
                    as *mut PairData)
    {
        return 0 as libc::c_int;
    }
    let mut i_ctr: libc::c_int = (*envs).x_ctr[0 as libc::c_int as usize];
    let mut j_ctr: libc::c_int = (*envs).x_ctr[1 as libc::c_int as usize];
    let mut k_ctr: libc::c_int = (*envs).x_ctr[2 as libc::c_int as usize];
    let mut l_ctr: libc::c_int = (*envs).x_ctr[3 as libc::c_int as usize];
    let mut i_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * i_sh + 2 as libc::c_int) as isize);
    let mut j_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * j_sh + 2 as libc::c_int) as isize);
    let mut k_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * k_sh + 2 as libc::c_int) as isize);
    let mut l_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * l_sh + 2 as libc::c_int) as isize);
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
    let mut al: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * l_sh + 5 as libc::c_int) as isize) as isize,
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
    let mut cl: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * l_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut expcutoff: libc::c_double = (*envs).expcutoff;
    let mut rr_ij: libc::c_double = (*envs).rirj[0 as libc::c_int as usize]
        * (*envs).rirj[0 as libc::c_int as usize]
        + (*envs).rirj[1 as libc::c_int as usize]
            * (*envs).rirj[1 as libc::c_int as usize]
        + (*envs).rirj[2 as libc::c_int as usize]
            * (*envs).rirj[2 as libc::c_int as usize];
    let mut rr_kl: libc::c_double = (*envs).rkrl[0 as libc::c_int as usize]
        * (*envs).rkrl[0 as libc::c_int as usize]
        + (*envs).rkrl[1 as libc::c_int as usize]
            * (*envs).rkrl[1 as libc::c_int as usize]
        + (*envs).rkrl[2 as libc::c_int as usize]
            * (*envs).rkrl[2 as libc::c_int as usize];
    let mut _pdata_ij: *mut PairData = 0 as *mut PairData;
    let mut _pdata_kl: *mut PairData = 0 as *mut PairData;
    let mut pdata_ij: *mut PairData = 0 as *mut PairData;
    let mut pdata_kl: *mut PairData = 0 as *mut PairData;
    if !((*opt).pairdata).is_null() {
        _pdata_ij = *((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize);
        _pdata_kl = *((*opt).pairdata).offset((k_sh * (*opt).nbas + l_sh) as isize);
    } else {
        let mut log_maxci: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(i_sh as isize);
        let mut log_maxcj: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(j_sh as isize);
        _pdata_ij = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
            & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut PairData;
        cache = _pdata_ij.offset((i_prim * j_prim + k_prim * l_prim) as isize)
            as *mut libc::c_double;
        if CINTset_pairdata(
            _pdata_ij,
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
        let mut log_maxck: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(k_sh as isize);
        let mut log_maxcl: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(l_sh as isize);
        _pdata_kl = _pdata_ij.offset((i_prim * j_prim) as isize);
        if CINTset_pairdata(
            _pdata_kl,
            ak,
            al,
            (*envs).rk,
            (*envs).c2rust_unnamed_1.rl,
            log_maxck,
            log_maxcl,
            (*envs).lk_ceil,
            (*envs).ll_ceil,
            k_prim,
            l_prim,
            rr_kl,
            expcutoff,
            env,
        ) != 0
        {
            return 0 as libc::c_int;
        }
    }
    let mut n_comp: libc::c_int = (*envs).ncomp_e1 * (*envs).ncomp_e2
        * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: libc::c_double = 0.;
    let mut fac1j: libc::c_double = 0.;
    let mut fac1k: libc::c_double = 0.;
    let mut fac1l: libc::c_double = 0.;
    let mut ip: libc::c_int = 0;
    let mut jp: libc::c_int = 0;
    let mut kp: libc::c_int = 0;
    let mut lp: libc::c_int = 0;
    let mut _empty: [libc::c_int; 5] = [
        1 as libc::c_int,
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
    let mut lempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(3 as libc::c_int as isize);
    let mut gempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(4 as libc::c_int as isize);
    let mut non0ctri: *mut libc::c_int = *((*opt).non0ctr).offset(i_sh as isize);
    let mut non0ctrj: *mut libc::c_int = *((*opt).non0ctr).offset(j_sh as isize);
    let mut non0ctrk: *mut libc::c_int = *((*opt).non0ctr).offset(k_sh as isize);
    let mut non0ctrl: *mut libc::c_int = *((*opt).non0ctr).offset(l_sh as isize);
    let mut non0idxi: *mut libc::c_int = *((*opt).sortedidx).offset(i_sh as isize);
    let mut non0idxj: *mut libc::c_int = *((*opt).sortedidx).offset(j_sh as isize);
    let mut non0idxk: *mut libc::c_int = *((*opt).sortedidx).offset(k_sh as isize);
    let mut non0idxl: *mut libc::c_int = *((*opt).sortedidx).offset(l_sh as isize);
    let mut expij: libc::c_double = 0.;
    let mut expkl: libc::c_double = 0.;
    let mut eijcutoff: libc::c_double = 0.;
    let mut eklcutoff: libc::c_double = 0.;
    let mut cutoff: libc::c_double = 0.;
    eklcutoff = expcutoff;
    let mut rij: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rkl: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut idx: *mut libc::c_int = *((*opt).index_xyz_array)
        .offset(
            ((*envs).i_l * 16 as libc::c_int * 16 as libc::c_int * 16 as libc::c_int
                + (*envs).j_l * 16 as libc::c_int * 16 as libc::c_int
                + (*envs).k_l * 16 as libc::c_int + (*envs).l_l) as isize,
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
        let mut lkl: libc::c_int = (*envs).lk_ceil + (*envs).ll_ceil;
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
        if lkl > 0 as libc::c_int {
            let mut dist_kl: libc::c_double = sqrt(rr_kl);
            let mut akl: libc::c_double = *ak
                .offset((k_prim - 1 as libc::c_int) as isize)
                + *al.offset((l_prim - 1 as libc::c_int) as isize);
            let mut theta_0: libc::c_double = omega2 / (omega2 + akl);
            expcutoff
                += lkl as libc::c_double
                    * log((dist_kl + theta_0 * r_guess + 1.0f64) / (dist_kl + 1.0f64));
        }
    }
    let mut nc: libc::c_int = 1 as libc::c_int;
    let mut leng: size_t = ((*envs).g_size * 3 as libc::c_int
        * (((1 as libc::c_int) << (*envs).gbits) + 1 as libc::c_int)) as size_t;
    let mut len0: size_t = nf.wrapping_mul(n_comp as libc::c_ulong);
    let mut len: size_t = leng.wrapping_add(len0);
    let mut gout: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut g: *mut libc::c_double = 0 as *mut libc::c_double;
    g = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_double;
    cache = g.offset(len as isize);
    if n_comp == 1 as libc::c_int {
        gout = gctr;
        gempty = empty;
    } else {
        gout = g.offset(leng as isize);
    }
    pdata_kl = _pdata_kl;
    lp = 0 as libc::c_int;
    while lp < l_prim {
        (*envs).al[0 as libc::c_int as usize] = *al.offset(lp as isize);
        fac1l = (*envs).common_factor * *cl.offset(lp as isize);
        kp = 0 as libc::c_int;
        while kp < k_prim {
            if !((*pdata_kl).cceij > eklcutoff) {
                (*envs).ak[0 as libc::c_int as usize] = *ak.offset(kp as isize);
                expkl = (*pdata_kl).eij;
                rkl = ((*pdata_kl).rij).as_mut_ptr();
                fac1k = fac1l * *ck.offset(kp as isize);
                eijcutoff = eklcutoff - (*pdata_kl).cceij;
                pdata_ij = _pdata_ij;
                jp = 0 as libc::c_int;
                while jp < j_prim {
                    (*envs).aj[0 as libc::c_int as usize] = *aj.offset(jp as isize);
                    fac1j = fac1k * *cj.offset(jp as isize);
                    ip = 0 as libc::c_int;
                    while ip < i_prim {
                        if !((*pdata_ij).cceij > eijcutoff) {
                            (*envs)
                                .ai[0 as libc::c_int as usize] = *ai.offset(ip as isize);
                            expij = (*pdata_ij).eij;
                            rij = ((*pdata_ij).rij).as_mut_ptr();
                            fac1i = fac1j * *ci.offset(ip as isize) * expij * expkl;
                            (*envs).fac[0 as libc::c_int as usize] = fac1i;
                            cutoff = eijcutoff - (*pdata_ij).cceij;
                            if ::core::mem::transmute::<
                                _,
                                fn(_, _, _, _, _) -> libc::c_int,
                            >(
                                (Some(
                                    ((*envs).f_g0_2e).expect("non-null function pointer"),
                                ))
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
            }
            kp += 1;
            kp;
            pdata_kl = pdata_kl.offset(1);
            pdata_kl;
        }
        lp += 1;
        lp;
    }
    if n_comp > 1 as libc::c_int && *gempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gout,
                nf.wrapping_mul(nc as libc::c_ulong) as libc::c_int,
                n_comp,
            );
            *empty = 0 as libc::c_int;
        } else {
            CINTdplus_transpose(
                gctr,
                gout,
                nf.wrapping_mul(nc as libc::c_ulong) as libc::c_int,
                n_comp,
            );
        }
    }
    return (*empty == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINT2e_n111_loop(
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
    let mut l_sh: libc::c_int = *shls.offset(3 as libc::c_int as isize);
    let mut opt: *mut CINTOpt = (*envs).opt;
    if !((*opt).pairdata).is_null()
        && (*((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize)
            == 0xffffffffffffffff as libc::c_ulong as *mut libc::c_void as *mut PairData
            || *((*opt).pairdata).offset((k_sh * (*opt).nbas + l_sh) as isize)
                == 0xffffffffffffffff as libc::c_ulong as *mut libc::c_void
                    as *mut PairData)
    {
        return 0 as libc::c_int;
    }
    let mut i_ctr: libc::c_int = (*envs).x_ctr[0 as libc::c_int as usize];
    let mut j_ctr: libc::c_int = (*envs).x_ctr[1 as libc::c_int as usize];
    let mut k_ctr: libc::c_int = (*envs).x_ctr[2 as libc::c_int as usize];
    let mut l_ctr: libc::c_int = (*envs).x_ctr[3 as libc::c_int as usize];
    let mut i_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * i_sh + 2 as libc::c_int) as isize);
    let mut j_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * j_sh + 2 as libc::c_int) as isize);
    let mut k_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * k_sh + 2 as libc::c_int) as isize);
    let mut l_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * l_sh + 2 as libc::c_int) as isize);
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
    let mut al: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * l_sh + 5 as libc::c_int) as isize) as isize,
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
    let mut cl: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * l_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut expcutoff: libc::c_double = (*envs).expcutoff;
    let mut rr_ij: libc::c_double = (*envs).rirj[0 as libc::c_int as usize]
        * (*envs).rirj[0 as libc::c_int as usize]
        + (*envs).rirj[1 as libc::c_int as usize]
            * (*envs).rirj[1 as libc::c_int as usize]
        + (*envs).rirj[2 as libc::c_int as usize]
            * (*envs).rirj[2 as libc::c_int as usize];
    let mut rr_kl: libc::c_double = (*envs).rkrl[0 as libc::c_int as usize]
        * (*envs).rkrl[0 as libc::c_int as usize]
        + (*envs).rkrl[1 as libc::c_int as usize]
            * (*envs).rkrl[1 as libc::c_int as usize]
        + (*envs).rkrl[2 as libc::c_int as usize]
            * (*envs).rkrl[2 as libc::c_int as usize];
    let mut _pdata_ij: *mut PairData = 0 as *mut PairData;
    let mut _pdata_kl: *mut PairData = 0 as *mut PairData;
    let mut pdata_ij: *mut PairData = 0 as *mut PairData;
    let mut pdata_kl: *mut PairData = 0 as *mut PairData;
    if !((*opt).pairdata).is_null() {
        _pdata_ij = *((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize);
        _pdata_kl = *((*opt).pairdata).offset((k_sh * (*opt).nbas + l_sh) as isize);
    } else {
        let mut log_maxci: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(i_sh as isize);
        let mut log_maxcj: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(j_sh as isize);
        _pdata_ij = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
            & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut PairData;
        cache = _pdata_ij.offset((i_prim * j_prim + k_prim * l_prim) as isize)
            as *mut libc::c_double;
        if CINTset_pairdata(
            _pdata_ij,
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
        let mut log_maxck: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(k_sh as isize);
        let mut log_maxcl: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(l_sh as isize);
        _pdata_kl = _pdata_ij.offset((i_prim * j_prim) as isize);
        if CINTset_pairdata(
            _pdata_kl,
            ak,
            al,
            (*envs).rk,
            (*envs).c2rust_unnamed_1.rl,
            log_maxck,
            log_maxcl,
            (*envs).lk_ceil,
            (*envs).ll_ceil,
            k_prim,
            l_prim,
            rr_kl,
            expcutoff,
            env,
        ) != 0
        {
            return 0 as libc::c_int;
        }
    }
    let mut n_comp: libc::c_int = (*envs).ncomp_e1 * (*envs).ncomp_e2
        * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: libc::c_double = 0.;
    let mut fac1j: libc::c_double = 0.;
    let mut fac1k: libc::c_double = 0.;
    let mut fac1l: libc::c_double = 0.;
    let mut ip: libc::c_int = 0;
    let mut jp: libc::c_int = 0;
    let mut kp: libc::c_int = 0;
    let mut lp: libc::c_int = 0;
    let mut _empty: [libc::c_int; 5] = [
        1 as libc::c_int,
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
    let mut lempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(3 as libc::c_int as isize);
    let mut gempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(4 as libc::c_int as isize);
    let mut non0ctri: *mut libc::c_int = *((*opt).non0ctr).offset(i_sh as isize);
    let mut non0ctrj: *mut libc::c_int = *((*opt).non0ctr).offset(j_sh as isize);
    let mut non0ctrk: *mut libc::c_int = *((*opt).non0ctr).offset(k_sh as isize);
    let mut non0ctrl: *mut libc::c_int = *((*opt).non0ctr).offset(l_sh as isize);
    let mut non0idxi: *mut libc::c_int = *((*opt).sortedidx).offset(i_sh as isize);
    let mut non0idxj: *mut libc::c_int = *((*opt).sortedidx).offset(j_sh as isize);
    let mut non0idxk: *mut libc::c_int = *((*opt).sortedidx).offset(k_sh as isize);
    let mut non0idxl: *mut libc::c_int = *((*opt).sortedidx).offset(l_sh as isize);
    let mut expij: libc::c_double = 0.;
    let mut expkl: libc::c_double = 0.;
    let mut eijcutoff: libc::c_double = 0.;
    let mut eklcutoff: libc::c_double = 0.;
    let mut cutoff: libc::c_double = 0.;
    eklcutoff = expcutoff;
    let mut rij: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rkl: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut idx: *mut libc::c_int = *((*opt).index_xyz_array)
        .offset(
            ((*envs).i_l * 16 as libc::c_int * 16 as libc::c_int * 16 as libc::c_int
                + (*envs).j_l * 16 as libc::c_int * 16 as libc::c_int
                + (*envs).k_l * 16 as libc::c_int + (*envs).l_l) as isize,
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
        let mut lkl: libc::c_int = (*envs).lk_ceil + (*envs).ll_ceil;
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
        if lkl > 0 as libc::c_int {
            let mut dist_kl: libc::c_double = sqrt(rr_kl);
            let mut akl: libc::c_double = *ak
                .offset((k_prim - 1 as libc::c_int) as isize)
                + *al.offset((l_prim - 1 as libc::c_int) as isize);
            let mut theta_0: libc::c_double = omega2 / (omega2 + akl);
            expcutoff
                += lkl as libc::c_double
                    * log((dist_kl + theta_0 * r_guess + 1.0f64) / (dist_kl + 1.0f64));
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
    pdata_kl = _pdata_kl;
    lp = 0 as libc::c_int;
    while lp < l_prim {
        (*envs).al[0 as libc::c_int as usize] = *al.offset(lp as isize);
        fac1l = (*envs).common_factor * *cl.offset(lp as isize);
        kp = 0 as libc::c_int;
        while kp < k_prim {
            if !((*pdata_kl).cceij > eklcutoff) {
                (*envs).ak[0 as libc::c_int as usize] = *ak.offset(kp as isize);
                expkl = (*pdata_kl).eij;
                rkl = ((*pdata_kl).rij).as_mut_ptr();
                fac1k = fac1l * *ck.offset(kp as isize);
                eijcutoff = eklcutoff - (*pdata_kl).cceij;
                pdata_ij = _pdata_ij;
                jp = 0 as libc::c_int;
                while jp < j_prim {
                    (*envs).aj[0 as libc::c_int as usize] = *aj.offset(jp as isize);
                    fac1j = fac1k * *cj.offset(jp as isize);
                    ip = 0 as libc::c_int;
                    while ip < i_prim {
                        if !((*pdata_ij).cceij > eijcutoff) {
                            if !((*pdata_ij).cceij > eijcutoff) {
                                (*envs)
                                    .ai[0 as libc::c_int as usize] = *ai.offset(ip as isize);
                                expij = (*pdata_ij).eij;
                                rij = ((*pdata_ij).rij).as_mut_ptr();
                                cutoff = eijcutoff - (*pdata_ij).cceij;
                                fac1i = fac1j * expij * expkl;
                                (*envs).fac[0 as libc::c_int as usize] = fac1i;
                                if ::core::mem::transmute::<
                                    _,
                                    fn(_, _, _, _, _) -> libc::c_int,
                                >(
                                    (Some(
                                        ((*envs).f_g0_2e).expect("non-null function pointer"),
                                    ))
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
                        }
                        ip += 1;
                        ip;
                        pdata_ij = pdata_ij.offset(1);
                        pdata_ij;
                    }
                    jp += 1;
                    jp;
                }
            }
            kp += 1;
            kp;
            pdata_kl = pdata_kl.offset(1);
            pdata_kl;
        }
        lp += 1;
        lp;
    }
    if n_comp > 1 as libc::c_int && *iempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gctri,
                nf.wrapping_mul(nc as libc::c_ulong) as libc::c_int,
                n_comp,
            );
            *empty = 0 as libc::c_int;
        } else {
            CINTdplus_transpose(
                gctr,
                gctri,
                nf.wrapping_mul(nc as libc::c_ulong) as libc::c_int,
                n_comp,
            );
        }
    }
    return (*empty == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINT2e_1n11_loop(
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
    let mut l_sh: libc::c_int = *shls.offset(3 as libc::c_int as isize);
    let mut opt: *mut CINTOpt = (*envs).opt;
    if !((*opt).pairdata).is_null()
        && (*((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize)
            == 0xffffffffffffffff as libc::c_ulong as *mut libc::c_void as *mut PairData
            || *((*opt).pairdata).offset((k_sh * (*opt).nbas + l_sh) as isize)
                == 0xffffffffffffffff as libc::c_ulong as *mut libc::c_void
                    as *mut PairData)
    {
        return 0 as libc::c_int;
    }
    let mut i_ctr: libc::c_int = (*envs).x_ctr[0 as libc::c_int as usize];
    let mut j_ctr: libc::c_int = (*envs).x_ctr[1 as libc::c_int as usize];
    let mut k_ctr: libc::c_int = (*envs).x_ctr[2 as libc::c_int as usize];
    let mut l_ctr: libc::c_int = (*envs).x_ctr[3 as libc::c_int as usize];
    let mut i_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * i_sh + 2 as libc::c_int) as isize);
    let mut j_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * j_sh + 2 as libc::c_int) as isize);
    let mut k_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * k_sh + 2 as libc::c_int) as isize);
    let mut l_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * l_sh + 2 as libc::c_int) as isize);
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
    let mut al: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * l_sh + 5 as libc::c_int) as isize) as isize,
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
    let mut cl: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * l_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut expcutoff: libc::c_double = (*envs).expcutoff;
    let mut rr_ij: libc::c_double = (*envs).rirj[0 as libc::c_int as usize]
        * (*envs).rirj[0 as libc::c_int as usize]
        + (*envs).rirj[1 as libc::c_int as usize]
            * (*envs).rirj[1 as libc::c_int as usize]
        + (*envs).rirj[2 as libc::c_int as usize]
            * (*envs).rirj[2 as libc::c_int as usize];
    let mut rr_kl: libc::c_double = (*envs).rkrl[0 as libc::c_int as usize]
        * (*envs).rkrl[0 as libc::c_int as usize]
        + (*envs).rkrl[1 as libc::c_int as usize]
            * (*envs).rkrl[1 as libc::c_int as usize]
        + (*envs).rkrl[2 as libc::c_int as usize]
            * (*envs).rkrl[2 as libc::c_int as usize];
    let mut _pdata_ij: *mut PairData = 0 as *mut PairData;
    let mut _pdata_kl: *mut PairData = 0 as *mut PairData;
    let mut pdata_ij: *mut PairData = 0 as *mut PairData;
    let mut pdata_kl: *mut PairData = 0 as *mut PairData;
    if !((*opt).pairdata).is_null() {
        _pdata_ij = *((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize);
        _pdata_kl = *((*opt).pairdata).offset((k_sh * (*opt).nbas + l_sh) as isize);
    } else {
        let mut log_maxci: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(i_sh as isize);
        let mut log_maxcj: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(j_sh as isize);
        _pdata_ij = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
            & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut PairData;
        cache = _pdata_ij.offset((i_prim * j_prim + k_prim * l_prim) as isize)
            as *mut libc::c_double;
        if CINTset_pairdata(
            _pdata_ij,
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
        let mut log_maxck: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(k_sh as isize);
        let mut log_maxcl: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(l_sh as isize);
        _pdata_kl = _pdata_ij.offset((i_prim * j_prim) as isize);
        if CINTset_pairdata(
            _pdata_kl,
            ak,
            al,
            (*envs).rk,
            (*envs).c2rust_unnamed_1.rl,
            log_maxck,
            log_maxcl,
            (*envs).lk_ceil,
            (*envs).ll_ceil,
            k_prim,
            l_prim,
            rr_kl,
            expcutoff,
            env,
        ) != 0
        {
            return 0 as libc::c_int;
        }
    }
    let mut n_comp: libc::c_int = (*envs).ncomp_e1 * (*envs).ncomp_e2
        * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: libc::c_double = 0.;
    let mut fac1j: libc::c_double = 0.;
    let mut fac1k: libc::c_double = 0.;
    let mut fac1l: libc::c_double = 0.;
    let mut ip: libc::c_int = 0;
    let mut jp: libc::c_int = 0;
    let mut kp: libc::c_int = 0;
    let mut lp: libc::c_int = 0;
    let mut _empty: [libc::c_int; 5] = [
        1 as libc::c_int,
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
    let mut lempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(3 as libc::c_int as isize);
    let mut gempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(4 as libc::c_int as isize);
    let mut non0ctri: *mut libc::c_int = *((*opt).non0ctr).offset(i_sh as isize);
    let mut non0ctrj: *mut libc::c_int = *((*opt).non0ctr).offset(j_sh as isize);
    let mut non0ctrk: *mut libc::c_int = *((*opt).non0ctr).offset(k_sh as isize);
    let mut non0ctrl: *mut libc::c_int = *((*opt).non0ctr).offset(l_sh as isize);
    let mut non0idxi: *mut libc::c_int = *((*opt).sortedidx).offset(i_sh as isize);
    let mut non0idxj: *mut libc::c_int = *((*opt).sortedidx).offset(j_sh as isize);
    let mut non0idxk: *mut libc::c_int = *((*opt).sortedidx).offset(k_sh as isize);
    let mut non0idxl: *mut libc::c_int = *((*opt).sortedidx).offset(l_sh as isize);
    let mut expij: libc::c_double = 0.;
    let mut expkl: libc::c_double = 0.;
    let mut eijcutoff: libc::c_double = 0.;
    let mut eklcutoff: libc::c_double = 0.;
    let mut cutoff: libc::c_double = 0.;
    eklcutoff = expcutoff;
    let mut rij: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rkl: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut idx: *mut libc::c_int = *((*opt).index_xyz_array)
        .offset(
            ((*envs).i_l * 16 as libc::c_int * 16 as libc::c_int * 16 as libc::c_int
                + (*envs).j_l * 16 as libc::c_int * 16 as libc::c_int
                + (*envs).k_l * 16 as libc::c_int + (*envs).l_l) as isize,
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
        let mut lkl: libc::c_int = (*envs).lk_ceil + (*envs).ll_ceil;
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
        if lkl > 0 as libc::c_int {
            let mut dist_kl: libc::c_double = sqrt(rr_kl);
            let mut akl: libc::c_double = *ak
                .offset((k_prim - 1 as libc::c_int) as isize)
                + *al.offset((l_prim - 1 as libc::c_int) as isize);
            let mut theta_0: libc::c_double = omega2 / (omega2 + akl);
            expcutoff
                += lkl as libc::c_double
                    * log((dist_kl + theta_0 * r_guess + 1.0f64) / (dist_kl + 1.0f64));
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
    pdata_kl = _pdata_kl;
    lp = 0 as libc::c_int;
    while lp < l_prim {
        (*envs).al[0 as libc::c_int as usize] = *al.offset(lp as isize);
        fac1l = (*envs).common_factor * *cl.offset(lp as isize);
        kp = 0 as libc::c_int;
        while kp < k_prim {
            if !((*pdata_kl).cceij > eklcutoff) {
                (*envs).ak[0 as libc::c_int as usize] = *ak.offset(kp as isize);
                expkl = (*pdata_kl).eij;
                rkl = ((*pdata_kl).rij).as_mut_ptr();
                fac1k = fac1l * *ck.offset(kp as isize);
                eijcutoff = eklcutoff - (*pdata_kl).cceij;
                pdata_ij = _pdata_ij;
                jp = 0 as libc::c_int;
                while jp < j_prim {
                    (*envs).aj[0 as libc::c_int as usize] = *aj.offset(jp as isize);
                    fac1j = fac1k;
                    *iempty = 1 as libc::c_int;
                    ip = 0 as libc::c_int;
                    while ip < i_prim {
                        if !((*pdata_ij).cceij > eijcutoff) {
                            (*envs)
                                .ai[0 as libc::c_int as usize] = *ai.offset(ip as isize);
                            expij = (*pdata_ij).eij;
                            rij = ((*pdata_ij).rij).as_mut_ptr();
                            cutoff = eijcutoff - (*pdata_ij).cceij;
                            fac1i = fac1j * *ci.offset(ip as isize) * expij * expkl;
                            (*envs).fac[0 as libc::c_int as usize] = fac1i;
                            if ::core::mem::transmute::<
                                _,
                                fn(_, _, _, _, _) -> libc::c_int,
                            >(
                                (Some(
                                    ((*envs).f_g0_2e).expect("non-null function pointer"),
                                ))
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
            }
            kp += 1;
            kp;
            pdata_kl = pdata_kl.offset(1);
            pdata_kl;
        }
        lp += 1;
        lp;
    }
    if n_comp > 1 as libc::c_int && *jempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gctrj,
                nf.wrapping_mul(nc as libc::c_ulong) as libc::c_int,
                n_comp,
            );
            *empty = 0 as libc::c_int;
        } else {
            CINTdplus_transpose(
                gctr,
                gctrj,
                nf.wrapping_mul(nc as libc::c_ulong) as libc::c_int,
                n_comp,
            );
        }
    }
    return (*empty == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINT2e_11n1_loop(
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
    let mut l_sh: libc::c_int = *shls.offset(3 as libc::c_int as isize);
    let mut opt: *mut CINTOpt = (*envs).opt;
    if !((*opt).pairdata).is_null()
        && (*((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize)
            == 0xffffffffffffffff as libc::c_ulong as *mut libc::c_void as *mut PairData
            || *((*opt).pairdata).offset((k_sh * (*opt).nbas + l_sh) as isize)
                == 0xffffffffffffffff as libc::c_ulong as *mut libc::c_void
                    as *mut PairData)
    {
        return 0 as libc::c_int;
    }
    let mut i_ctr: libc::c_int = (*envs).x_ctr[0 as libc::c_int as usize];
    let mut j_ctr: libc::c_int = (*envs).x_ctr[1 as libc::c_int as usize];
    let mut k_ctr: libc::c_int = (*envs).x_ctr[2 as libc::c_int as usize];
    let mut l_ctr: libc::c_int = (*envs).x_ctr[3 as libc::c_int as usize];
    let mut i_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * i_sh + 2 as libc::c_int) as isize);
    let mut j_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * j_sh + 2 as libc::c_int) as isize);
    let mut k_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * k_sh + 2 as libc::c_int) as isize);
    let mut l_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * l_sh + 2 as libc::c_int) as isize);
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
    let mut al: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * l_sh + 5 as libc::c_int) as isize) as isize,
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
    let mut cl: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * l_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut expcutoff: libc::c_double = (*envs).expcutoff;
    let mut rr_ij: libc::c_double = (*envs).rirj[0 as libc::c_int as usize]
        * (*envs).rirj[0 as libc::c_int as usize]
        + (*envs).rirj[1 as libc::c_int as usize]
            * (*envs).rirj[1 as libc::c_int as usize]
        + (*envs).rirj[2 as libc::c_int as usize]
            * (*envs).rirj[2 as libc::c_int as usize];
    let mut rr_kl: libc::c_double = (*envs).rkrl[0 as libc::c_int as usize]
        * (*envs).rkrl[0 as libc::c_int as usize]
        + (*envs).rkrl[1 as libc::c_int as usize]
            * (*envs).rkrl[1 as libc::c_int as usize]
        + (*envs).rkrl[2 as libc::c_int as usize]
            * (*envs).rkrl[2 as libc::c_int as usize];
    let mut _pdata_ij: *mut PairData = 0 as *mut PairData;
    let mut _pdata_kl: *mut PairData = 0 as *mut PairData;
    let mut pdata_ij: *mut PairData = 0 as *mut PairData;
    let mut pdata_kl: *mut PairData = 0 as *mut PairData;
    if !((*opt).pairdata).is_null() {
        _pdata_ij = *((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize);
        _pdata_kl = *((*opt).pairdata).offset((k_sh * (*opt).nbas + l_sh) as isize);
    } else {
        let mut log_maxci: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(i_sh as isize);
        let mut log_maxcj: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(j_sh as isize);
        _pdata_ij = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
            & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut PairData;
        cache = _pdata_ij.offset((i_prim * j_prim + k_prim * l_prim) as isize)
            as *mut libc::c_double;
        if CINTset_pairdata(
            _pdata_ij,
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
        let mut log_maxck: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(k_sh as isize);
        let mut log_maxcl: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(l_sh as isize);
        _pdata_kl = _pdata_ij.offset((i_prim * j_prim) as isize);
        if CINTset_pairdata(
            _pdata_kl,
            ak,
            al,
            (*envs).rk,
            (*envs).c2rust_unnamed_1.rl,
            log_maxck,
            log_maxcl,
            (*envs).lk_ceil,
            (*envs).ll_ceil,
            k_prim,
            l_prim,
            rr_kl,
            expcutoff,
            env,
        ) != 0
        {
            return 0 as libc::c_int;
        }
    }
    let mut n_comp: libc::c_int = (*envs).ncomp_e1 * (*envs).ncomp_e2
        * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: libc::c_double = 0.;
    let mut fac1j: libc::c_double = 0.;
    let mut fac1k: libc::c_double = 0.;
    let mut fac1l: libc::c_double = 0.;
    let mut ip: libc::c_int = 0;
    let mut jp: libc::c_int = 0;
    let mut kp: libc::c_int = 0;
    let mut lp: libc::c_int = 0;
    let mut _empty: [libc::c_int; 5] = [
        1 as libc::c_int,
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
    let mut lempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(3 as libc::c_int as isize);
    let mut gempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(4 as libc::c_int as isize);
    let mut non0ctri: *mut libc::c_int = *((*opt).non0ctr).offset(i_sh as isize);
    let mut non0ctrj: *mut libc::c_int = *((*opt).non0ctr).offset(j_sh as isize);
    let mut non0ctrk: *mut libc::c_int = *((*opt).non0ctr).offset(k_sh as isize);
    let mut non0ctrl: *mut libc::c_int = *((*opt).non0ctr).offset(l_sh as isize);
    let mut non0idxi: *mut libc::c_int = *((*opt).sortedidx).offset(i_sh as isize);
    let mut non0idxj: *mut libc::c_int = *((*opt).sortedidx).offset(j_sh as isize);
    let mut non0idxk: *mut libc::c_int = *((*opt).sortedidx).offset(k_sh as isize);
    let mut non0idxl: *mut libc::c_int = *((*opt).sortedidx).offset(l_sh as isize);
    let mut expij: libc::c_double = 0.;
    let mut expkl: libc::c_double = 0.;
    let mut eijcutoff: libc::c_double = 0.;
    let mut eklcutoff: libc::c_double = 0.;
    let mut cutoff: libc::c_double = 0.;
    eklcutoff = expcutoff;
    let mut rij: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rkl: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut idx: *mut libc::c_int = *((*opt).index_xyz_array)
        .offset(
            ((*envs).i_l * 16 as libc::c_int * 16 as libc::c_int * 16 as libc::c_int
                + (*envs).j_l * 16 as libc::c_int * 16 as libc::c_int
                + (*envs).k_l * 16 as libc::c_int + (*envs).l_l) as isize,
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
        let mut lkl: libc::c_int = (*envs).lk_ceil + (*envs).ll_ceil;
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
        if lkl > 0 as libc::c_int {
            let mut dist_kl: libc::c_double = sqrt(rr_kl);
            let mut akl: libc::c_double = *ak
                .offset((k_prim - 1 as libc::c_int) as isize)
                + *al.offset((l_prim - 1 as libc::c_int) as isize);
            let mut theta_0: libc::c_double = omega2 / (omega2 + akl);
            expcutoff
                += lkl as libc::c_double
                    * log((dist_kl + theta_0 * r_guess + 1.0f64) / (dist_kl + 1.0f64));
        }
    }
    let mut nc: libc::c_int = k_ctr;
    let mut leng: size_t = ((*envs).g_size * 3 as libc::c_int
        * (((1 as libc::c_int) << (*envs).gbits) + 1 as libc::c_int)) as size_t;
    let mut lenk: size_t = nf
        .wrapping_mul(k_ctr as libc::c_ulong)
        .wrapping_mul(n_comp as libc::c_ulong);
    let mut len0: size_t = nf.wrapping_mul(n_comp as libc::c_ulong);
    let mut len: size_t = leng.wrapping_add(lenk).wrapping_add(len0);
    let mut g: *mut libc::c_double = 0 as *mut libc::c_double;
    g = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_double;
    cache = g.offset(len as isize);
    let mut g1: *mut libc::c_double = g.offset(leng as isize);
    let mut gout: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut gctrk: *mut libc::c_double = 0 as *mut libc::c_double;
    if n_comp == 1 as libc::c_int {
        gctrk = gctr;
        kempty = empty;
    } else {
        gctrk = g1;
        g1 = g1.offset(lenk as isize);
    }
    gout = g1;
    pdata_kl = _pdata_kl;
    lp = 0 as libc::c_int;
    while lp < l_prim {
        (*envs).al[0 as libc::c_int as usize] = *al.offset(lp as isize);
        fac1l = (*envs).common_factor * *cl.offset(lp as isize);
        kp = 0 as libc::c_int;
        while kp < k_prim {
            if !((*pdata_kl).cceij > eklcutoff) {
                (*envs).ak[0 as libc::c_int as usize] = *ak.offset(kp as isize);
                expkl = (*pdata_kl).eij;
                rkl = ((*pdata_kl).rij).as_mut_ptr();
                fac1k = fac1l;
                eijcutoff = eklcutoff - (*pdata_kl).cceij;
                pdata_ij = _pdata_ij;
                *jempty = 1 as libc::c_int;
                jp = 0 as libc::c_int;
                while jp < j_prim {
                    (*envs).aj[0 as libc::c_int as usize] = *aj.offset(jp as isize);
                    fac1j = fac1k * *cj.offset(jp as isize);
                    ip = 0 as libc::c_int;
                    while ip < i_prim {
                        if !((*pdata_ij).cceij > eijcutoff) {
                            (*envs)
                                .ai[0 as libc::c_int as usize] = *ai.offset(ip as isize);
                            expij = (*pdata_ij).eij;
                            rij = ((*pdata_ij).rij).as_mut_ptr();
                            cutoff = eijcutoff - (*pdata_ij).cceij;
                            fac1i = fac1j * *ci.offset(ip as isize) * expij * expkl;
                            (*envs).fac[0 as libc::c_int as usize] = fac1i;
                            if ::core::mem::transmute::<
                                _,
                                fn(_, _, _, _, _) -> libc::c_int,
                            >(
                                (Some(
                                    ((*envs).f_g0_2e).expect("non-null function pointer"),
                                ))
                                    .expect("non-null function pointer"),
                            )(g, rij, rkl, cutoff, envs) != 0
                            {
                                ::core::mem::transmute::<
                                    _,
                                    fn(_, _, _, _, _),
                                >(
                                    (Some(((*envs).f_gout).expect("non-null function pointer")))
                                        .expect("non-null function pointer"),
                                )(gout, g, idx, envs, *jempty);
                                *jempty = 0 as libc::c_int;
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
                if *jempty == 0 {
                    if k_ctr > 1 as libc::c_int {
                        if *kempty != 0 {
                            CINTprim_to_ctr_0(
                                gctrk,
                                gout,
                                ck.offset(kp as isize),
                                len0,
                                k_prim,
                                k_ctr,
                                *non0ctrk.offset(kp as isize),
                                non0idxk.offset((kp * k_ctr) as isize),
                            );
                        } else {
                            CINTprim_to_ctr_1(
                                gctrk,
                                gout,
                                ck.offset(kp as isize),
                                len0,
                                k_prim,
                                k_ctr,
                                *non0ctrk.offset(kp as isize),
                                non0idxk.offset((kp * k_ctr) as isize),
                            );
                        }
                    }
                    *kempty = 0 as libc::c_int;
                }
            }
            kp += 1;
            kp;
            pdata_kl = pdata_kl.offset(1);
            pdata_kl;
        }
        lp += 1;
        lp;
    }
    if n_comp > 1 as libc::c_int && *kempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gctrk,
                nf.wrapping_mul(nc as libc::c_ulong) as libc::c_int,
                n_comp,
            );
            *empty = 0 as libc::c_int;
        } else {
            CINTdplus_transpose(
                gctr,
                gctrk,
                nf.wrapping_mul(nc as libc::c_ulong) as libc::c_int,
                n_comp,
            );
        }
    }
    return (*empty == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINT2e_111n_loop(
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
    let mut l_sh: libc::c_int = *shls.offset(3 as libc::c_int as isize);
    let mut opt: *mut CINTOpt = (*envs).opt;
    if !((*opt).pairdata).is_null()
        && (*((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize)
            == 0xffffffffffffffff as libc::c_ulong as *mut libc::c_void as *mut PairData
            || *((*opt).pairdata).offset((k_sh * (*opt).nbas + l_sh) as isize)
                == 0xffffffffffffffff as libc::c_ulong as *mut libc::c_void
                    as *mut PairData)
    {
        return 0 as libc::c_int;
    }
    let mut i_ctr: libc::c_int = (*envs).x_ctr[0 as libc::c_int as usize];
    let mut j_ctr: libc::c_int = (*envs).x_ctr[1 as libc::c_int as usize];
    let mut k_ctr: libc::c_int = (*envs).x_ctr[2 as libc::c_int as usize];
    let mut l_ctr: libc::c_int = (*envs).x_ctr[3 as libc::c_int as usize];
    let mut i_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * i_sh + 2 as libc::c_int) as isize);
    let mut j_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * j_sh + 2 as libc::c_int) as isize);
    let mut k_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * k_sh + 2 as libc::c_int) as isize);
    let mut l_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * l_sh + 2 as libc::c_int) as isize);
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
    let mut al: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * l_sh + 5 as libc::c_int) as isize) as isize,
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
    let mut cl: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * l_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut expcutoff: libc::c_double = (*envs).expcutoff;
    let mut rr_ij: libc::c_double = (*envs).rirj[0 as libc::c_int as usize]
        * (*envs).rirj[0 as libc::c_int as usize]
        + (*envs).rirj[1 as libc::c_int as usize]
            * (*envs).rirj[1 as libc::c_int as usize]
        + (*envs).rirj[2 as libc::c_int as usize]
            * (*envs).rirj[2 as libc::c_int as usize];
    let mut rr_kl: libc::c_double = (*envs).rkrl[0 as libc::c_int as usize]
        * (*envs).rkrl[0 as libc::c_int as usize]
        + (*envs).rkrl[1 as libc::c_int as usize]
            * (*envs).rkrl[1 as libc::c_int as usize]
        + (*envs).rkrl[2 as libc::c_int as usize]
            * (*envs).rkrl[2 as libc::c_int as usize];
    let mut _pdata_ij: *mut PairData = 0 as *mut PairData;
    let mut _pdata_kl: *mut PairData = 0 as *mut PairData;
    let mut pdata_ij: *mut PairData = 0 as *mut PairData;
    let mut pdata_kl: *mut PairData = 0 as *mut PairData;
    if !((*opt).pairdata).is_null() {
        _pdata_ij = *((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize);
        _pdata_kl = *((*opt).pairdata).offset((k_sh * (*opt).nbas + l_sh) as isize);
    } else {
        let mut log_maxci: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(i_sh as isize);
        let mut log_maxcj: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(j_sh as isize);
        _pdata_ij = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
            & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut PairData;
        cache = _pdata_ij.offset((i_prim * j_prim + k_prim * l_prim) as isize)
            as *mut libc::c_double;
        if CINTset_pairdata(
            _pdata_ij,
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
        let mut log_maxck: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(k_sh as isize);
        let mut log_maxcl: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(l_sh as isize);
        _pdata_kl = _pdata_ij.offset((i_prim * j_prim) as isize);
        if CINTset_pairdata(
            _pdata_kl,
            ak,
            al,
            (*envs).rk,
            (*envs).c2rust_unnamed_1.rl,
            log_maxck,
            log_maxcl,
            (*envs).lk_ceil,
            (*envs).ll_ceil,
            k_prim,
            l_prim,
            rr_kl,
            expcutoff,
            env,
        ) != 0
        {
            return 0 as libc::c_int;
        }
    }
    let mut n_comp: libc::c_int = (*envs).ncomp_e1 * (*envs).ncomp_e2
        * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: libc::c_double = 0.;
    let mut fac1j: libc::c_double = 0.;
    let mut fac1k: libc::c_double = 0.;
    let mut fac1l: libc::c_double = 0.;
    let mut ip: libc::c_int = 0;
    let mut jp: libc::c_int = 0;
    let mut kp: libc::c_int = 0;
    let mut lp: libc::c_int = 0;
    let mut _empty: [libc::c_int; 5] = [
        1 as libc::c_int,
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
    let mut lempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(3 as libc::c_int as isize);
    let mut gempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(4 as libc::c_int as isize);
    let mut non0ctri: *mut libc::c_int = *((*opt).non0ctr).offset(i_sh as isize);
    let mut non0ctrj: *mut libc::c_int = *((*opt).non0ctr).offset(j_sh as isize);
    let mut non0ctrk: *mut libc::c_int = *((*opt).non0ctr).offset(k_sh as isize);
    let mut non0ctrl: *mut libc::c_int = *((*opt).non0ctr).offset(l_sh as isize);
    let mut non0idxi: *mut libc::c_int = *((*opt).sortedidx).offset(i_sh as isize);
    let mut non0idxj: *mut libc::c_int = *((*opt).sortedidx).offset(j_sh as isize);
    let mut non0idxk: *mut libc::c_int = *((*opt).sortedidx).offset(k_sh as isize);
    let mut non0idxl: *mut libc::c_int = *((*opt).sortedidx).offset(l_sh as isize);
    let mut expij: libc::c_double = 0.;
    let mut expkl: libc::c_double = 0.;
    let mut eijcutoff: libc::c_double = 0.;
    let mut eklcutoff: libc::c_double = 0.;
    let mut cutoff: libc::c_double = 0.;
    eklcutoff = expcutoff;
    let mut rij: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rkl: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut idx: *mut libc::c_int = *((*opt).index_xyz_array)
        .offset(
            ((*envs).i_l * 16 as libc::c_int * 16 as libc::c_int * 16 as libc::c_int
                + (*envs).j_l * 16 as libc::c_int * 16 as libc::c_int
                + (*envs).k_l * 16 as libc::c_int + (*envs).l_l) as isize,
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
        let mut lkl: libc::c_int = (*envs).lk_ceil + (*envs).ll_ceil;
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
        if lkl > 0 as libc::c_int {
            let mut dist_kl: libc::c_double = sqrt(rr_kl);
            let mut akl: libc::c_double = *ak
                .offset((k_prim - 1 as libc::c_int) as isize)
                + *al.offset((l_prim - 1 as libc::c_int) as isize);
            let mut theta_0: libc::c_double = omega2 / (omega2 + akl);
            expcutoff
                += lkl as libc::c_double
                    * log((dist_kl + theta_0 * r_guess + 1.0f64) / (dist_kl + 1.0f64));
        }
    }
    let mut nc: libc::c_int = l_ctr;
    let mut leng: size_t = ((*envs).g_size * 3 as libc::c_int
        * (((1 as libc::c_int) << (*envs).gbits) + 1 as libc::c_int)) as size_t;
    let mut lenl: size_t = nf
        .wrapping_mul(l_ctr as libc::c_ulong)
        .wrapping_mul(n_comp as libc::c_ulong);
    let mut len0: size_t = nf.wrapping_mul(n_comp as libc::c_ulong);
    let mut len: size_t = leng.wrapping_add(lenl).wrapping_add(len0);
    let mut g: *mut libc::c_double = 0 as *mut libc::c_double;
    g = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
        & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut libc::c_double;
    cache = g.offset(len as isize);
    let mut g1: *mut libc::c_double = g.offset(leng as isize);
    let mut gout: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut gctrl: *mut libc::c_double = 0 as *mut libc::c_double;
    if n_comp == 1 as libc::c_int {
        gctrl = gctr;
        lempty = empty;
    } else {
        gctrl = g1;
        g1 = g1.offset(lenl as isize);
    }
    gout = g1;
    pdata_kl = _pdata_kl;
    lp = 0 as libc::c_int;
    while lp < l_prim {
        (*envs).al[0 as libc::c_int as usize] = *al.offset(lp as isize);
        fac1l = (*envs).common_factor;
        *kempty = 1 as libc::c_int;
        kp = 0 as libc::c_int;
        while kp < k_prim {
            if !((*pdata_kl).cceij > eklcutoff) {
                (*envs).ak[0 as libc::c_int as usize] = *ak.offset(kp as isize);
                expkl = (*pdata_kl).eij;
                rkl = ((*pdata_kl).rij).as_mut_ptr();
                fac1k = fac1l * *ck.offset(kp as isize);
                eijcutoff = eklcutoff - (*pdata_kl).cceij;
                pdata_ij = _pdata_ij;
                jp = 0 as libc::c_int;
                while jp < j_prim {
                    (*envs).aj[0 as libc::c_int as usize] = *aj.offset(jp as isize);
                    fac1j = fac1k * *cj.offset(jp as isize);
                    ip = 0 as libc::c_int;
                    while ip < i_prim {
                        if !((*pdata_ij).cceij > eijcutoff) {
                            (*envs)
                                .ai[0 as libc::c_int as usize] = *ai.offset(ip as isize);
                            expij = (*pdata_ij).eij;
                            rij = ((*pdata_ij).rij).as_mut_ptr();
                            cutoff = eijcutoff - (*pdata_ij).cceij;
                            fac1i = fac1j * *ci.offset(ip as isize) * expij * expkl;
                            (*envs).fac[0 as libc::c_int as usize] = fac1i;
                            if ::core::mem::transmute::<
                                _,
                                fn(_, _, _, _, _) -> libc::c_int,
                            >(
                                (Some(
                                    ((*envs).f_g0_2e).expect("non-null function pointer"),
                                ))
                                    .expect("non-null function pointer"),
                            )(g, rij, rkl, cutoff, envs) != 0
                            {
                                ::core::mem::transmute::<
                                    _,
                                    fn(_, _, _, _, _),
                                >(
                                    (Some(((*envs).f_gout).expect("non-null function pointer")))
                                        .expect("non-null function pointer"),
                                )(gout, g, idx, envs, *kempty);
                                *kempty = 0 as libc::c_int;
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
            }
            kp += 1;
            kp;
            pdata_kl = pdata_kl.offset(1);
            pdata_kl;
        }
        if *kempty == 0 {
            if l_ctr > 1 as libc::c_int {
                if *lempty != 0 {
                    CINTprim_to_ctr_0(
                        gctrl,
                        gout,
                        cl.offset(lp as isize),
                        len0,
                        l_prim,
                        l_ctr,
                        *non0ctrl.offset(lp as isize),
                        non0idxl.offset((lp * l_ctr) as isize),
                    );
                } else {
                    CINTprim_to_ctr_1(
                        gctrl,
                        gout,
                        cl.offset(lp as isize),
                        len0,
                        l_prim,
                        l_ctr,
                        *non0ctrl.offset(lp as isize),
                        non0idxl.offset((lp * l_ctr) as isize),
                    );
                }
            }
            *lempty = 0 as libc::c_int;
        }
        lp += 1;
        lp;
    }
    if n_comp > 1 as libc::c_int && *lempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gctrl,
                nf.wrapping_mul(nc as libc::c_ulong) as libc::c_int,
                n_comp,
            );
            *empty = 0 as libc::c_int;
        } else {
            CINTdplus_transpose(
                gctr,
                gctrl,
                nf.wrapping_mul(nc as libc::c_ulong) as libc::c_int,
                n_comp,
            );
        }
    }
    return (*empty == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINT2e_loop(
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
    let mut l_sh: libc::c_int = *shls.offset(3 as libc::c_int as isize);
    let mut opt: *mut CINTOpt = (*envs).opt;
    if !((*opt).pairdata).is_null()
        && (*((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize)
            == 0xffffffffffffffff as libc::c_ulong as *mut libc::c_void as *mut PairData
            || *((*opt).pairdata).offset((k_sh * (*opt).nbas + l_sh) as isize)
                == 0xffffffffffffffff as libc::c_ulong as *mut libc::c_void
                    as *mut PairData)
    {
        return 0 as libc::c_int;
    }
    let mut i_ctr: libc::c_int = (*envs).x_ctr[0 as libc::c_int as usize];
    let mut j_ctr: libc::c_int = (*envs).x_ctr[1 as libc::c_int as usize];
    let mut k_ctr: libc::c_int = (*envs).x_ctr[2 as libc::c_int as usize];
    let mut l_ctr: libc::c_int = (*envs).x_ctr[3 as libc::c_int as usize];
    let mut i_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * i_sh + 2 as libc::c_int) as isize);
    let mut j_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * j_sh + 2 as libc::c_int) as isize);
    let mut k_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * k_sh + 2 as libc::c_int) as isize);
    let mut l_prim: libc::c_int = *bas
        .offset((8 as libc::c_int * l_sh + 2 as libc::c_int) as isize);
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
    let mut al: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * l_sh + 5 as libc::c_int) as isize) as isize,
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
    let mut cl: *mut libc::c_double = env
        .offset(
            *bas.offset((8 as libc::c_int * l_sh + 6 as libc::c_int) as isize) as isize,
        );
    let mut expcutoff: libc::c_double = (*envs).expcutoff;
    let mut rr_ij: libc::c_double = (*envs).rirj[0 as libc::c_int as usize]
        * (*envs).rirj[0 as libc::c_int as usize]
        + (*envs).rirj[1 as libc::c_int as usize]
            * (*envs).rirj[1 as libc::c_int as usize]
        + (*envs).rirj[2 as libc::c_int as usize]
            * (*envs).rirj[2 as libc::c_int as usize];
    let mut rr_kl: libc::c_double = (*envs).rkrl[0 as libc::c_int as usize]
        * (*envs).rkrl[0 as libc::c_int as usize]
        + (*envs).rkrl[1 as libc::c_int as usize]
            * (*envs).rkrl[1 as libc::c_int as usize]
        + (*envs).rkrl[2 as libc::c_int as usize]
            * (*envs).rkrl[2 as libc::c_int as usize];
    let mut _pdata_ij: *mut PairData = 0 as *mut PairData;
    let mut _pdata_kl: *mut PairData = 0 as *mut PairData;
    let mut pdata_ij: *mut PairData = 0 as *mut PairData;
    let mut pdata_kl: *mut PairData = 0 as *mut PairData;
    if !((*opt).pairdata).is_null() {
        _pdata_ij = *((*opt).pairdata).offset((i_sh * (*opt).nbas + j_sh) as isize);
        _pdata_kl = *((*opt).pairdata).offset((k_sh * (*opt).nbas + l_sh) as isize);
    } else {
        let mut log_maxci: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(i_sh as isize);
        let mut log_maxcj: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(j_sh as isize);
        _pdata_ij = ((cache as uintptr_t).wrapping_add(7 as libc::c_int as libc::c_ulong)
            & (8 as libc::c_int as uintptr_t).wrapping_neg()) as *mut libc::c_void
            as *mut PairData;
        cache = _pdata_ij.offset((i_prim * j_prim + k_prim * l_prim) as isize)
            as *mut libc::c_double;
        if CINTset_pairdata(
            _pdata_ij,
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
        let mut log_maxck: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(k_sh as isize);
        let mut log_maxcl: *mut libc::c_double = *((*opt).log_max_coeff)
            .offset(l_sh as isize);
        _pdata_kl = _pdata_ij.offset((i_prim * j_prim) as isize);
        if CINTset_pairdata(
            _pdata_kl,
            ak,
            al,
            (*envs).rk,
            (*envs).c2rust_unnamed_1.rl,
            log_maxck,
            log_maxcl,
            (*envs).lk_ceil,
            (*envs).ll_ceil,
            k_prim,
            l_prim,
            rr_kl,
            expcutoff,
            env,
        ) != 0
        {
            return 0 as libc::c_int;
        }
    }
    let mut n_comp: libc::c_int = (*envs).ncomp_e1 * (*envs).ncomp_e2
        * (*envs).ncomp_tensor;
    let mut nf: size_t = (*envs).nf as size_t;
    let mut fac1i: libc::c_double = 0.;
    let mut fac1j: libc::c_double = 0.;
    let mut fac1k: libc::c_double = 0.;
    let mut fac1l: libc::c_double = 0.;
    let mut ip: libc::c_int = 0;
    let mut jp: libc::c_int = 0;
    let mut kp: libc::c_int = 0;
    let mut lp: libc::c_int = 0;
    let mut _empty: [libc::c_int; 5] = [
        1 as libc::c_int,
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
    let mut lempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(3 as libc::c_int as isize);
    let mut gempty: *mut libc::c_int = _empty
        .as_mut_ptr()
        .offset(4 as libc::c_int as isize);
    let mut non0ctri: *mut libc::c_int = *((*opt).non0ctr).offset(i_sh as isize);
    let mut non0ctrj: *mut libc::c_int = *((*opt).non0ctr).offset(j_sh as isize);
    let mut non0ctrk: *mut libc::c_int = *((*opt).non0ctr).offset(k_sh as isize);
    let mut non0ctrl: *mut libc::c_int = *((*opt).non0ctr).offset(l_sh as isize);
    let mut non0idxi: *mut libc::c_int = *((*opt).sortedidx).offset(i_sh as isize);
    let mut non0idxj: *mut libc::c_int = *((*opt).sortedidx).offset(j_sh as isize);
    let mut non0idxk: *mut libc::c_int = *((*opt).sortedidx).offset(k_sh as isize);
    let mut non0idxl: *mut libc::c_int = *((*opt).sortedidx).offset(l_sh as isize);
    let mut expij: libc::c_double = 0.;
    let mut expkl: libc::c_double = 0.;
    let mut eijcutoff: libc::c_double = 0.;
    let mut eklcutoff: libc::c_double = 0.;
    let mut cutoff: libc::c_double = 0.;
    eklcutoff = expcutoff;
    let mut rij: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rkl: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut idx: *mut libc::c_int = *((*opt).index_xyz_array)
        .offset(
            ((*envs).i_l * 16 as libc::c_int * 16 as libc::c_int * 16 as libc::c_int
                + (*envs).j_l * 16 as libc::c_int * 16 as libc::c_int
                + (*envs).k_l * 16 as libc::c_int + (*envs).l_l) as isize,
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
        let mut lkl: libc::c_int = (*envs).lk_ceil + (*envs).ll_ceil;
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
        if lkl > 0 as libc::c_int {
            let mut dist_kl: libc::c_double = sqrt(rr_kl);
            let mut akl: libc::c_double = *ak
                .offset((k_prim - 1 as libc::c_int) as isize)
                + *al.offset((l_prim - 1 as libc::c_int) as isize);
            let mut theta_0: libc::c_double = omega2 / (omega2 + akl);
            expcutoff
                += lkl as libc::c_double
                    * log((dist_kl + theta_0 * r_guess + 1.0f64) / (dist_kl + 1.0f64));
        }
    }
    let mut nc: libc::c_int = i_ctr * j_ctr * k_ctr * l_ctr;
    let mut leng: size_t = ((*envs).g_size * 3 as libc::c_int
        * (((1 as libc::c_int) << (*envs).gbits) + 1 as libc::c_int)) as size_t;
    let mut lenl: size_t = nf
        .wrapping_mul(nc as libc::c_ulong)
        .wrapping_mul(n_comp as libc::c_ulong);
    let mut lenk: size_t = nf
        .wrapping_mul(i_ctr as libc::c_ulong)
        .wrapping_mul(j_ctr as libc::c_ulong)
        .wrapping_mul(k_ctr as libc::c_ulong)
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
        .wrapping_add(lenl)
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
    let mut gctrl: *mut libc::c_double = 0 as *mut libc::c_double;
    if n_comp == 1 as libc::c_int {
        gctrl = gctr;
        lempty = empty;
    } else {
        gctrl = g1;
        g1 = g1.offset(lenl as isize);
    }
    if l_ctr == 1 as libc::c_int {
        gctrk = gctrl;
        kempty = lempty;
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
    pdata_kl = _pdata_kl;
    lp = 0 as libc::c_int;
    while lp < l_prim {
        (*envs).al[0 as libc::c_int as usize] = *al.offset(lp as isize);
        if l_ctr == 1 as libc::c_int {
            fac1l = (*envs).common_factor * *cl.offset(lp as isize);
        } else {
            fac1l = (*envs).common_factor;
            *kempty = 1 as libc::c_int;
        }
        kp = 0 as libc::c_int;
        while kp < k_prim {
            if !((*pdata_kl).cceij > eklcutoff) {
                (*envs).ak[0 as libc::c_int as usize] = *ak.offset(kp as isize);
                expkl = (*pdata_kl).eij;
                rkl = ((*pdata_kl).rij).as_mut_ptr();
                eijcutoff = eklcutoff - (*pdata_kl).cceij;
                if k_ctr == 1 as libc::c_int {
                    fac1k = fac1l * *ck.offset(kp as isize);
                } else {
                    fac1k = fac1l;
                    *jempty = 1 as libc::c_int;
                }
                pdata_ij = _pdata_ij;
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
                        if !((*pdata_ij).cceij > eijcutoff) {
                            (*envs)
                                .ai[0 as libc::c_int as usize] = *ai.offset(ip as isize);
                            expij = (*pdata_ij).eij;
                            rij = ((*pdata_ij).rij).as_mut_ptr();
                            cutoff = eijcutoff - (*pdata_ij).cceij;
                            if i_ctr == 1 as libc::c_int {
                                fac1i = fac1j * *ci.offset(ip as isize) * expij * expkl;
                            } else {
                                fac1i = fac1j * expij * expkl;
                            }
                            (*envs).fac[0 as libc::c_int as usize] = fac1i;
                            if ::core::mem::transmute::<
                                _,
                                fn(_, _, _, _, _) -> libc::c_int,
                            >(
                                (Some(
                                    ((*envs).f_g0_2e).expect("non-null function pointer"),
                                ))
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
            }
            kp += 1;
            kp;
            pdata_kl = pdata_kl.offset(1);
            pdata_kl;
        }
        if *kempty == 0 {
            if l_ctr > 1 as libc::c_int {
                if *lempty != 0 {
                    CINTprim_to_ctr_0(
                        gctrl,
                        gctrk,
                        cl.offset(lp as isize),
                        lenk,
                        l_prim,
                        l_ctr,
                        *non0ctrl.offset(lp as isize),
                        non0idxl.offset((lp * l_ctr) as isize),
                    );
                } else {
                    CINTprim_to_ctr_1(
                        gctrl,
                        gctrk,
                        cl.offset(lp as isize),
                        lenk,
                        l_prim,
                        l_ctr,
                        *non0ctrl.offset(lp as isize),
                        non0idxl.offset((lp * l_ctr) as isize),
                    );
                }
            }
            *lempty = 0 as libc::c_int;
        }
        lp += 1;
        lp;
    }
    if n_comp > 1 as libc::c_int && *lempty == 0 {
        if *empty != 0 {
            CINTdmat_transpose(
                gctr,
                gctrl,
                nf.wrapping_mul(nc as libc::c_ulong) as libc::c_int,
                n_comp,
            );
            *empty = 0 as libc::c_int;
        } else {
            CINTdplus_transpose(
                gctr,
                gctrl,
                nf.wrapping_mul(nc as libc::c_ulong) as libc::c_int,
                n_comp,
            );
        }
    }
    return (*empty == 0) as libc::c_int;
}
static mut CINTf_2e_loop: [Option::<
    unsafe extern "C" fn(
        *mut libc::c_double,
        *mut CINTEnvVars,
        *mut libc::c_double,
        *mut libc::c_int,
    ) -> libc::c_int,
>; 16] = unsafe {
    [
        Some(
            CINT2e_loop
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            CINT2e_loop
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            CINT2e_loop
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            CINT2e_loop
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            CINT2e_loop
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            CINT2e_loop
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            CINT2e_loop
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            CINT2e_n111_loop
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            CINT2e_loop
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            CINT2e_loop
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            CINT2e_loop
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            CINT2e_1n11_loop
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            CINT2e_loop
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            CINT2e_11n1_loop
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            CINT2e_111n_loop
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut CINTEnvVars,
                    *mut libc::c_double,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
        Some(
            CINT2e_1111_loop
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
pub unsafe extern "C" fn CINT2e_drv(
    mut out: *mut libc::c_double,
    mut dims: *mut libc::c_int,
    mut envs: *mut CINTEnvVars,
    mut opt: *mut CINTOpt,
    mut cache: *mut libc::c_double,
    mut f_c2s: Option::<unsafe extern "C" fn() -> ()>,
) -> libc::c_int {
    let mut x_ctr: *mut libc::c_int = ((*envs).x_ctr).as_mut_ptr();
    let mut nf: size_t = (*envs).nf as size_t;
    let mut nc: size_t = nf
        .wrapping_mul(*x_ctr.offset(0 as libc::c_int as isize) as libc::c_ulong)
        .wrapping_mul(*x_ctr.offset(1 as libc::c_int as isize) as libc::c_ulong)
        .wrapping_mul(*x_ctr.offset(2 as libc::c_int as isize) as libc::c_ulong)
        .wrapping_mul(*x_ctr.offset(3 as libc::c_int as isize) as libc::c_ulong);
    let mut n_comp: libc::c_int = (*envs).ncomp_e1 * (*envs).ncomp_e2
        * (*envs).ncomp_tensor;
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
        let mut l_prim: libc::c_int = *bas
            .offset(
                (8 as libc::c_int * *shls.offset(3 as libc::c_int as isize)
                    + 2 as libc::c_int) as isize,
            );
        let mut pdata_size: size_t = (((i_prim * j_prim + k_prim * l_prim)
            * 5 as libc::c_int + i_prim * *x_ctr.offset(0 as libc::c_int as isize)
            + j_prim * *x_ctr.offset(1 as libc::c_int as isize)
            + k_prim * *x_ctr.offset(2 as libc::c_int as isize)
            + l_prim * *x_ctr.offset(3 as libc::c_int as isize)
            + (i_prim + j_prim + k_prim + l_prim) * 2 as libc::c_int) as libc::c_ulong)
            .wrapping_add(nf.wrapping_mul(3 as libc::c_int as libc::c_ulong));
        let mut leng: size_t = ((*envs).g_size * 3 as libc::c_int
            * (((1 as libc::c_int) << (*envs).gbits) + 1 as libc::c_int)) as size_t;
        let mut len0: size_t = nf.wrapping_mul(n_comp as libc::c_ulong);
        let mut cache_size: size_t = if leng
            .wrapping_add(len0)
            .wrapping_add(
                nc
                    .wrapping_mul(n_comp as libc::c_ulong)
                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
            )
            .wrapping_add(pdata_size)
            > nc
                .wrapping_mul(n_comp as libc::c_ulong)
                .wrapping_add(nf.wrapping_mul(4 as libc::c_int as libc::c_ulong))
        {
            leng.wrapping_add(len0)
                .wrapping_add(
                    nc
                        .wrapping_mul(n_comp as libc::c_ulong)
                        .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(pdata_size)
        } else {
            nc.wrapping_mul(n_comp as libc::c_ulong)
                .wrapping_add(nf.wrapping_mul(4 as libc::c_int as libc::c_ulong))
        };
        if cache_size >= 2147483647 as libc::c_int as libc::c_ulong {
            fprintf(
                stderr,
                b"CINT2e_drv cache_size overflow: cache_size %zu > %d, nf %zu, nc %zu, n_comp %d\n\0"
                    as *const u8 as *const libc::c_char,
                cache_size,
                2147483647 as libc::c_int,
                nf,
                nc,
                n_comp,
            );
            cache_size = 0 as libc::c_int as size_t;
        }
        return cache_size as libc::c_int;
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
        let mut l_prim_0: libc::c_int = *bas_0
            .offset(
                (8 as libc::c_int * *shls_0.offset(3 as libc::c_int as isize)
                    + 2 as libc::c_int) as isize,
            );
        let mut pdata_size_0: size_t = (((i_prim_0 * j_prim_0 + k_prim_0 * l_prim_0)
            * 5 as libc::c_int + i_prim_0 * *x_ctr.offset(0 as libc::c_int as isize)
            + j_prim_0 * *x_ctr.offset(1 as libc::c_int as isize)
            + k_prim_0 * *x_ctr.offset(2 as libc::c_int as isize)
            + l_prim_0 * *x_ctr.offset(3 as libc::c_int as isize)
            + (i_prim_0 + j_prim_0 + k_prim_0 + l_prim_0) * 2 as libc::c_int)
            as libc::c_ulong)
            .wrapping_add(nf.wrapping_mul(3 as libc::c_int as libc::c_ulong));
        let mut leng_0: size_t = ((*envs).g_size * 3 as libc::c_int
            * (((1 as libc::c_int) << (*envs).gbits) + 1 as libc::c_int)) as size_t;
        let mut len0_0: size_t = nf.wrapping_mul(n_comp as libc::c_ulong);
        let mut cache_size_0: size_t = if leng_0
            .wrapping_add(len0_0)
            .wrapping_add(
                nc
                    .wrapping_mul(n_comp as libc::c_ulong)
                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
            )
            .wrapping_add(pdata_size_0)
            > nc
                .wrapping_mul(n_comp as libc::c_ulong)
                .wrapping_add(nf.wrapping_mul(4 as libc::c_int as libc::c_ulong))
        {
            leng_0
                .wrapping_add(len0_0)
                .wrapping_add(
                    nc
                        .wrapping_mul(n_comp as libc::c_ulong)
                        .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(pdata_size_0)
        } else {
            nc.wrapping_mul(n_comp as libc::c_ulong)
                .wrapping_add(nf.wrapping_mul(4 as libc::c_int as libc::c_ulong))
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
        n = (((*x_ctr.offset(0 as libc::c_int as isize) == 1 as libc::c_int)
            as libc::c_int) << 3 as libc::c_int)
            + (((*x_ctr.offset(1 as libc::c_int as isize) == 1 as libc::c_int)
                as libc::c_int) << 2 as libc::c_int)
            + (((*x_ctr.offset(2 as libc::c_int as isize) == 1 as libc::c_int)
                as libc::c_int) << 1 as libc::c_int)
            + (*x_ctr.offset(3 as libc::c_int as isize) == 1 as libc::c_int)
                as libc::c_int;
        (CINTf_2e_loop[n as usize])
            .expect("non-null function pointer")(gctr, envs, cache, &mut empty);
    } else {
        CINT2e_loop_nopt(gctr, envs, cache, &mut empty);
    }
    let mut counts: [libc::c_int; 4] = [0; 4];
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
                c2s_sph_2e1
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
        counts[2 as libc::c_int
            as usize] = ((*envs).k_l * 2 as libc::c_int + 1 as libc::c_int)
            * *x_ctr.offset(2 as libc::c_int as isize);
        counts[3 as libc::c_int
            as usize] = ((*envs).l_l * 2 as libc::c_int + 1 as libc::c_int)
            * *x_ctr.offset(3 as libc::c_int as isize);
    } else {
        counts[0 as libc::c_int
            as usize] = (*envs).nfi * *x_ctr.offset(0 as libc::c_int as isize);
        counts[1 as libc::c_int
            as usize] = (*envs).nfj * *x_ctr.offset(1 as libc::c_int as isize);
        counts[2 as libc::c_int
            as usize] = (*envs).c2rust_unnamed.nfk
            * *x_ctr.offset(2 as libc::c_int as isize);
        counts[3 as libc::c_int
            as usize] = (*envs).c2rust_unnamed_0.nfl
            * *x_ctr.offset(3 as libc::c_int as isize);
    }
    if dims.is_null() {
        dims = counts.as_mut_ptr();
    }
    let mut nout: libc::c_int = *dims.offset(0 as libc::c_int as isize)
        * *dims.offset(1 as libc::c_int as isize)
        * *dims.offset(2 as libc::c_int as isize)
        * *dims.offset(3 as libc::c_int as isize);
    if empty == 0 {
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
pub unsafe extern "C" fn CINTgout2e(
    mut gout: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut idx: *mut libc::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: libc::c_int,
) {
    let mut nf: libc::c_int = (*envs).nf;
    let mut i: libc::c_int = 0;
    let mut ix: libc::c_int = 0;
    let mut iy: libc::c_int = 0;
    let mut iz: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut s: libc::c_double = 0.;
    if gout_empty != 0 {
        match (*envs).nrys_roots {
            1 => {
                n = 0 as libc::c_int;
                while n < nf {
                    ix = *idx.offset(0 as libc::c_int as isize);
                    iy = *idx.offset(1 as libc::c_int as isize);
                    iz = *idx.offset(2 as libc::c_int as isize);
                    *gout
                        .offset(
                            n as isize,
                        ) = *g.offset(ix as isize) * *g.offset(iy as isize)
                        * *g.offset(iz as isize);
                    n += 1;
                    n;
                    idx = idx.offset(3 as libc::c_int as isize);
                }
            }
            2 => {
                n = 0 as libc::c_int;
                while n < nf {
                    ix = *idx.offset(0 as libc::c_int as isize);
                    iy = *idx.offset(1 as libc::c_int as isize);
                    iz = *idx.offset(2 as libc::c_int as isize);
                    *gout
                        .offset(
                            n as isize,
                        ) = *g.offset(ix as isize) * *g.offset(iy as isize)
                        * *g.offset(iz as isize)
                        + *g.offset((ix + 1 as libc::c_int) as isize)
                            * *g.offset((iy + 1 as libc::c_int) as isize)
                            * *g.offset((iz + 1 as libc::c_int) as isize);
                    n += 1;
                    n;
                    idx = idx.offset(3 as libc::c_int as isize);
                }
            }
            3 => {
                n = 0 as libc::c_int;
                while n < nf {
                    ix = *idx.offset(0 as libc::c_int as isize);
                    iy = *idx.offset(1 as libc::c_int as isize);
                    iz = *idx.offset(2 as libc::c_int as isize);
                    *gout
                        .offset(
                            n as isize,
                        ) = *g.offset(ix as isize) * *g.offset(iy as isize)
                        * *g.offset(iz as isize)
                        + *g.offset((ix + 1 as libc::c_int) as isize)
                            * *g.offset((iy + 1 as libc::c_int) as isize)
                            * *g.offset((iz + 1 as libc::c_int) as isize)
                        + *g.offset((ix + 2 as libc::c_int) as isize)
                            * *g.offset((iy + 2 as libc::c_int) as isize)
                            * *g.offset((iz + 2 as libc::c_int) as isize);
                    n += 1;
                    n;
                    idx = idx.offset(3 as libc::c_int as isize);
                }
            }
            4 => {
                n = 0 as libc::c_int;
                while n < nf {
                    ix = *idx.offset(0 as libc::c_int as isize);
                    iy = *idx.offset(1 as libc::c_int as isize);
                    iz = *idx.offset(2 as libc::c_int as isize);
                    *gout
                        .offset(
                            n as isize,
                        ) = *g.offset(ix as isize) * *g.offset(iy as isize)
                        * *g.offset(iz as isize)
                        + *g.offset((ix + 1 as libc::c_int) as isize)
                            * *g.offset((iy + 1 as libc::c_int) as isize)
                            * *g.offset((iz + 1 as libc::c_int) as isize)
                        + *g.offset((ix + 2 as libc::c_int) as isize)
                            * *g.offset((iy + 2 as libc::c_int) as isize)
                            * *g.offset((iz + 2 as libc::c_int) as isize)
                        + *g.offset((ix + 3 as libc::c_int) as isize)
                            * *g.offset((iy + 3 as libc::c_int) as isize)
                            * *g.offset((iz + 3 as libc::c_int) as isize);
                    n += 1;
                    n;
                    idx = idx.offset(3 as libc::c_int as isize);
                }
            }
            5 => {
                n = 0 as libc::c_int;
                while n < nf {
                    ix = *idx.offset(0 as libc::c_int as isize);
                    iy = *idx.offset(1 as libc::c_int as isize);
                    iz = *idx.offset(2 as libc::c_int as isize);
                    *gout
                        .offset(
                            n as isize,
                        ) = *g.offset(ix as isize) * *g.offset(iy as isize)
                        * *g.offset(iz as isize)
                        + *g.offset((ix + 1 as libc::c_int) as isize)
                            * *g.offset((iy + 1 as libc::c_int) as isize)
                            * *g.offset((iz + 1 as libc::c_int) as isize)
                        + *g.offset((ix + 2 as libc::c_int) as isize)
                            * *g.offset((iy + 2 as libc::c_int) as isize)
                            * *g.offset((iz + 2 as libc::c_int) as isize)
                        + *g.offset((ix + 3 as libc::c_int) as isize)
                            * *g.offset((iy + 3 as libc::c_int) as isize)
                            * *g.offset((iz + 3 as libc::c_int) as isize)
                        + *g.offset((ix + 4 as libc::c_int) as isize)
                            * *g.offset((iy + 4 as libc::c_int) as isize)
                            * *g.offset((iz + 4 as libc::c_int) as isize);
                    n += 1;
                    n;
                    idx = idx.offset(3 as libc::c_int as isize);
                }
            }
            6 => {
                n = 0 as libc::c_int;
                while n < nf {
                    ix = *idx.offset(0 as libc::c_int as isize);
                    iy = *idx.offset(1 as libc::c_int as isize);
                    iz = *idx.offset(2 as libc::c_int as isize);
                    *gout
                        .offset(
                            n as isize,
                        ) = *g.offset(ix as isize) * *g.offset(iy as isize)
                        * *g.offset(iz as isize)
                        + *g.offset((ix + 1 as libc::c_int) as isize)
                            * *g.offset((iy + 1 as libc::c_int) as isize)
                            * *g.offset((iz + 1 as libc::c_int) as isize)
                        + *g.offset((ix + 2 as libc::c_int) as isize)
                            * *g.offset((iy + 2 as libc::c_int) as isize)
                            * *g.offset((iz + 2 as libc::c_int) as isize)
                        + *g.offset((ix + 3 as libc::c_int) as isize)
                            * *g.offset((iy + 3 as libc::c_int) as isize)
                            * *g.offset((iz + 3 as libc::c_int) as isize)
                        + *g.offset((ix + 4 as libc::c_int) as isize)
                            * *g.offset((iy + 4 as libc::c_int) as isize)
                            * *g.offset((iz + 4 as libc::c_int) as isize)
                        + *g.offset((ix + 5 as libc::c_int) as isize)
                            * *g.offset((iy + 5 as libc::c_int) as isize)
                            * *g.offset((iz + 5 as libc::c_int) as isize);
                    n += 1;
                    n;
                    idx = idx.offset(3 as libc::c_int as isize);
                }
            }
            7 => {
                n = 0 as libc::c_int;
                while n < nf {
                    ix = *idx.offset(0 as libc::c_int as isize);
                    iy = *idx.offset(1 as libc::c_int as isize);
                    iz = *idx.offset(2 as libc::c_int as isize);
                    *gout
                        .offset(
                            n as isize,
                        ) = *g.offset(ix as isize) * *g.offset(iy as isize)
                        * *g.offset(iz as isize)
                        + *g.offset((ix + 1 as libc::c_int) as isize)
                            * *g.offset((iy + 1 as libc::c_int) as isize)
                            * *g.offset((iz + 1 as libc::c_int) as isize)
                        + *g.offset((ix + 2 as libc::c_int) as isize)
                            * *g.offset((iy + 2 as libc::c_int) as isize)
                            * *g.offset((iz + 2 as libc::c_int) as isize)
                        + *g.offset((ix + 3 as libc::c_int) as isize)
                            * *g.offset((iy + 3 as libc::c_int) as isize)
                            * *g.offset((iz + 3 as libc::c_int) as isize)
                        + *g.offset((ix + 4 as libc::c_int) as isize)
                            * *g.offset((iy + 4 as libc::c_int) as isize)
                            * *g.offset((iz + 4 as libc::c_int) as isize)
                        + *g.offset((ix + 5 as libc::c_int) as isize)
                            * *g.offset((iy + 5 as libc::c_int) as isize)
                            * *g.offset((iz + 5 as libc::c_int) as isize)
                        + *g.offset((ix + 6 as libc::c_int) as isize)
                            * *g.offset((iy + 6 as libc::c_int) as isize)
                            * *g.offset((iz + 6 as libc::c_int) as isize);
                    n += 1;
                    n;
                    idx = idx.offset(3 as libc::c_int as isize);
                }
            }
            8 => {
                n = 0 as libc::c_int;
                while n < nf {
                    ix = *idx.offset(0 as libc::c_int as isize);
                    iy = *idx.offset(1 as libc::c_int as isize);
                    iz = *idx.offset(2 as libc::c_int as isize);
                    *gout
                        .offset(
                            n as isize,
                        ) = *g.offset(ix as isize) * *g.offset(iy as isize)
                        * *g.offset(iz as isize)
                        + *g.offset((ix + 1 as libc::c_int) as isize)
                            * *g.offset((iy + 1 as libc::c_int) as isize)
                            * *g.offset((iz + 1 as libc::c_int) as isize)
                        + *g.offset((ix + 2 as libc::c_int) as isize)
                            * *g.offset((iy + 2 as libc::c_int) as isize)
                            * *g.offset((iz + 2 as libc::c_int) as isize)
                        + *g.offset((ix + 3 as libc::c_int) as isize)
                            * *g.offset((iy + 3 as libc::c_int) as isize)
                            * *g.offset((iz + 3 as libc::c_int) as isize)
                        + *g.offset((ix + 4 as libc::c_int) as isize)
                            * *g.offset((iy + 4 as libc::c_int) as isize)
                            * *g.offset((iz + 4 as libc::c_int) as isize)
                        + *g.offset((ix + 5 as libc::c_int) as isize)
                            * *g.offset((iy + 5 as libc::c_int) as isize)
                            * *g.offset((iz + 5 as libc::c_int) as isize)
                        + *g.offset((ix + 6 as libc::c_int) as isize)
                            * *g.offset((iy + 6 as libc::c_int) as isize)
                            * *g.offset((iz + 6 as libc::c_int) as isize)
                        + *g.offset((ix + 7 as libc::c_int) as isize)
                            * *g.offset((iy + 7 as libc::c_int) as isize)
                            * *g.offset((iz + 7 as libc::c_int) as isize);
                    n += 1;
                    n;
                    idx = idx.offset(3 as libc::c_int as isize);
                }
            }
            _ => {
                n = 0 as libc::c_int;
                while n < nf {
                    ix = *idx.offset(0 as libc::c_int as isize);
                    iy = *idx.offset(1 as libc::c_int as isize);
                    iz = *idx.offset(2 as libc::c_int as isize);
                    s = 0 as libc::c_int as libc::c_double;
                    i = 0 as libc::c_int;
                    while i < (*envs).nrys_roots {
                        s
                            += *g.offset((ix + i) as isize)
                                * *g.offset((iy + i) as isize)
                                * *g.offset((iz + i) as isize);
                        i += 1;
                        i;
                    }
                    *gout.offset(n as isize) = s;
                    n += 1;
                    n;
                    idx = idx.offset(3 as libc::c_int as isize);
                }
            }
        }
    } else {
        match (*envs).nrys_roots {
            1 => {
                n = 0 as libc::c_int;
                while n < nf {
                    ix = *idx.offset(0 as libc::c_int as isize);
                    iy = *idx.offset(1 as libc::c_int as isize);
                    iz = *idx.offset(2 as libc::c_int as isize);
                    *gout.offset(n as isize)
                        += *g.offset(ix as isize) * *g.offset(iy as isize)
                            * *g.offset(iz as isize);
                    n += 1;
                    n;
                    idx = idx.offset(3 as libc::c_int as isize);
                }
            }
            2 => {
                n = 0 as libc::c_int;
                while n < nf {
                    ix = *idx.offset(0 as libc::c_int as isize);
                    iy = *idx.offset(1 as libc::c_int as isize);
                    iz = *idx.offset(2 as libc::c_int as isize);
                    *gout.offset(n as isize)
                        += *g.offset(ix as isize) * *g.offset(iy as isize)
                            * *g.offset(iz as isize)
                            + *g.offset((ix + 1 as libc::c_int) as isize)
                                * *g.offset((iy + 1 as libc::c_int) as isize)
                                * *g.offset((iz + 1 as libc::c_int) as isize);
                    n += 1;
                    n;
                    idx = idx.offset(3 as libc::c_int as isize);
                }
            }
            3 => {
                n = 0 as libc::c_int;
                while n < nf {
                    ix = *idx.offset(0 as libc::c_int as isize);
                    iy = *idx.offset(1 as libc::c_int as isize);
                    iz = *idx.offset(2 as libc::c_int as isize);
                    *gout.offset(n as isize)
                        += *g.offset(ix as isize) * *g.offset(iy as isize)
                            * *g.offset(iz as isize)
                            + *g.offset((ix + 1 as libc::c_int) as isize)
                                * *g.offset((iy + 1 as libc::c_int) as isize)
                                * *g.offset((iz + 1 as libc::c_int) as isize)
                            + *g.offset((ix + 2 as libc::c_int) as isize)
                                * *g.offset((iy + 2 as libc::c_int) as isize)
                                * *g.offset((iz + 2 as libc::c_int) as isize);
                    n += 1;
                    n;
                    idx = idx.offset(3 as libc::c_int as isize);
                }
            }
            4 => {
                n = 0 as libc::c_int;
                while n < nf {
                    ix = *idx.offset(0 as libc::c_int as isize);
                    iy = *idx.offset(1 as libc::c_int as isize);
                    iz = *idx.offset(2 as libc::c_int as isize);
                    *gout.offset(n as isize)
                        += *g.offset(ix as isize) * *g.offset(iy as isize)
                            * *g.offset(iz as isize)
                            + *g.offset((ix + 1 as libc::c_int) as isize)
                                * *g.offset((iy + 1 as libc::c_int) as isize)
                                * *g.offset((iz + 1 as libc::c_int) as isize)
                            + *g.offset((ix + 2 as libc::c_int) as isize)
                                * *g.offset((iy + 2 as libc::c_int) as isize)
                                * *g.offset((iz + 2 as libc::c_int) as isize)
                            + *g.offset((ix + 3 as libc::c_int) as isize)
                                * *g.offset((iy + 3 as libc::c_int) as isize)
                                * *g.offset((iz + 3 as libc::c_int) as isize);
                    n += 1;
                    n;
                    idx = idx.offset(3 as libc::c_int as isize);
                }
            }
            5 => {
                n = 0 as libc::c_int;
                while n < nf {
                    ix = *idx.offset(0 as libc::c_int as isize);
                    iy = *idx.offset(1 as libc::c_int as isize);
                    iz = *idx.offset(2 as libc::c_int as isize);
                    *gout.offset(n as isize)
                        += *g.offset(ix as isize) * *g.offset(iy as isize)
                            * *g.offset(iz as isize)
                            + *g.offset((ix + 1 as libc::c_int) as isize)
                                * *g.offset((iy + 1 as libc::c_int) as isize)
                                * *g.offset((iz + 1 as libc::c_int) as isize)
                            + *g.offset((ix + 2 as libc::c_int) as isize)
                                * *g.offset((iy + 2 as libc::c_int) as isize)
                                * *g.offset((iz + 2 as libc::c_int) as isize)
                            + *g.offset((ix + 3 as libc::c_int) as isize)
                                * *g.offset((iy + 3 as libc::c_int) as isize)
                                * *g.offset((iz + 3 as libc::c_int) as isize)
                            + *g.offset((ix + 4 as libc::c_int) as isize)
                                * *g.offset((iy + 4 as libc::c_int) as isize)
                                * *g.offset((iz + 4 as libc::c_int) as isize);
                    n += 1;
                    n;
                    idx = idx.offset(3 as libc::c_int as isize);
                }
            }
            6 => {
                n = 0 as libc::c_int;
                while n < nf {
                    ix = *idx.offset(0 as libc::c_int as isize);
                    iy = *idx.offset(1 as libc::c_int as isize);
                    iz = *idx.offset(2 as libc::c_int as isize);
                    *gout.offset(n as isize)
                        += *g.offset(ix as isize) * *g.offset(iy as isize)
                            * *g.offset(iz as isize)
                            + *g.offset((ix + 1 as libc::c_int) as isize)
                                * *g.offset((iy + 1 as libc::c_int) as isize)
                                * *g.offset((iz + 1 as libc::c_int) as isize)
                            + *g.offset((ix + 2 as libc::c_int) as isize)
                                * *g.offset((iy + 2 as libc::c_int) as isize)
                                * *g.offset((iz + 2 as libc::c_int) as isize)
                            + *g.offset((ix + 3 as libc::c_int) as isize)
                                * *g.offset((iy + 3 as libc::c_int) as isize)
                                * *g.offset((iz + 3 as libc::c_int) as isize)
                            + *g.offset((ix + 4 as libc::c_int) as isize)
                                * *g.offset((iy + 4 as libc::c_int) as isize)
                                * *g.offset((iz + 4 as libc::c_int) as isize)
                            + *g.offset((ix + 5 as libc::c_int) as isize)
                                * *g.offset((iy + 5 as libc::c_int) as isize)
                                * *g.offset((iz + 5 as libc::c_int) as isize);
                    n += 1;
                    n;
                    idx = idx.offset(3 as libc::c_int as isize);
                }
            }
            7 => {
                n = 0 as libc::c_int;
                while n < nf {
                    ix = *idx.offset(0 as libc::c_int as isize);
                    iy = *idx.offset(1 as libc::c_int as isize);
                    iz = *idx.offset(2 as libc::c_int as isize);
                    *gout.offset(n as isize)
                        += *g.offset(ix as isize) * *g.offset(iy as isize)
                            * *g.offset(iz as isize)
                            + *g.offset((ix + 1 as libc::c_int) as isize)
                                * *g.offset((iy + 1 as libc::c_int) as isize)
                                * *g.offset((iz + 1 as libc::c_int) as isize)
                            + *g.offset((ix + 2 as libc::c_int) as isize)
                                * *g.offset((iy + 2 as libc::c_int) as isize)
                                * *g.offset((iz + 2 as libc::c_int) as isize)
                            + *g.offset((ix + 3 as libc::c_int) as isize)
                                * *g.offset((iy + 3 as libc::c_int) as isize)
                                * *g.offset((iz + 3 as libc::c_int) as isize)
                            + *g.offset((ix + 4 as libc::c_int) as isize)
                                * *g.offset((iy + 4 as libc::c_int) as isize)
                                * *g.offset((iz + 4 as libc::c_int) as isize)
                            + *g.offset((ix + 5 as libc::c_int) as isize)
                                * *g.offset((iy + 5 as libc::c_int) as isize)
                                * *g.offset((iz + 5 as libc::c_int) as isize)
                            + *g.offset((ix + 6 as libc::c_int) as isize)
                                * *g.offset((iy + 6 as libc::c_int) as isize)
                                * *g.offset((iz + 6 as libc::c_int) as isize);
                    n += 1;
                    n;
                    idx = idx.offset(3 as libc::c_int as isize);
                }
            }
            8 => {
                n = 0 as libc::c_int;
                while n < nf {
                    ix = *idx.offset(0 as libc::c_int as isize);
                    iy = *idx.offset(1 as libc::c_int as isize);
                    iz = *idx.offset(2 as libc::c_int as isize);
                    *gout.offset(n as isize)
                        += *g.offset(ix as isize) * *g.offset(iy as isize)
                            * *g.offset(iz as isize)
                            + *g.offset((ix + 1 as libc::c_int) as isize)
                                * *g.offset((iy + 1 as libc::c_int) as isize)
                                * *g.offset((iz + 1 as libc::c_int) as isize)
                            + *g.offset((ix + 2 as libc::c_int) as isize)
                                * *g.offset((iy + 2 as libc::c_int) as isize)
                                * *g.offset((iz + 2 as libc::c_int) as isize)
                            + *g.offset((ix + 3 as libc::c_int) as isize)
                                * *g.offset((iy + 3 as libc::c_int) as isize)
                                * *g.offset((iz + 3 as libc::c_int) as isize)
                            + *g.offset((ix + 4 as libc::c_int) as isize)
                                * *g.offset((iy + 4 as libc::c_int) as isize)
                                * *g.offset((iz + 4 as libc::c_int) as isize)
                            + *g.offset((ix + 5 as libc::c_int) as isize)
                                * *g.offset((iy + 5 as libc::c_int) as isize)
                                * *g.offset((iz + 5 as libc::c_int) as isize)
                            + *g.offset((ix + 6 as libc::c_int) as isize)
                                * *g.offset((iy + 6 as libc::c_int) as isize)
                                * *g.offset((iz + 6 as libc::c_int) as isize)
                            + *g.offset((ix + 7 as libc::c_int) as isize)
                                * *g.offset((iy + 7 as libc::c_int) as isize)
                                * *g.offset((iz + 7 as libc::c_int) as isize);
                    n += 1;
                    n;
                    idx = idx.offset(3 as libc::c_int as isize);
                }
            }
            _ => {
                n = 0 as libc::c_int;
                while n < nf {
                    ix = *idx.offset(0 as libc::c_int as isize);
                    iy = *idx.offset(1 as libc::c_int as isize);
                    iz = *idx.offset(2 as libc::c_int as isize);
                    s = 0 as libc::c_int as libc::c_double;
                    i = 0 as libc::c_int;
                    while i < (*envs).nrys_roots {
                        s
                            += *g.offset((ix + i) as isize)
                                * *g.offset((iy + i) as isize)
                                * *g.offset((iz + i) as isize);
                        i += 1;
                        i;
                    }
                    *gout.offset(n as isize) += s;
                    n += 1;
                    n;
                    idx = idx.offset(3 as libc::c_int as isize);
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn int2e_sph(
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
    CINTinit_int2e_EnvVars(&mut envs, ng.as_mut_ptr(), shls, atm, natm, bas, nbas, env);
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
    return CINT2e_drv(
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
                c2s_sph_2e1
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
pub unsafe extern "C" fn int2e_optimizer(
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
    CINTall_2e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int2e_cart(
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
    CINTinit_int2e_EnvVars(&mut envs, ng.as_mut_ptr(), shls, atm, natm, bas, nbas, env);
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
    return CINT2e_drv(
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
                c2s_cart_2e1
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
pub unsafe extern "C" fn cint2e_sph(
    mut out: *mut libc::c_double,
    mut shls: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
    mut opt: *mut CINTOpt,
) -> libc::c_int {
    return int2e_sph(
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
pub unsafe extern "C" fn cint2e_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
) {
    int2e_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
) {
    int2e_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
) {
    int2e_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_cart(
    mut out: *mut libc::c_double,
    mut shls: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
    mut opt: *mut CINTOpt,
) -> libc::c_int {
    return int2e_cart(
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
pub unsafe extern "C" fn cint2e_sph_(
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
    return int2e_sph(
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
pub unsafe extern "C" fn cint2e_sph_optimizer_(
    mut optptr_as_integer8: size_t,
    mut atm: *mut libc::c_int,
    mut natm: *mut libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: *mut libc::c_int,
    mut env: *mut libc::c_double,
) {
    let mut opt: *mut *mut CINTOpt = optptr_as_integer8 as *mut *mut CINTOpt;
    int2e_optimizer(opt, atm, *natm, bas, *nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_cart_(
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
    return int2e_cart(
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
pub unsafe extern "C" fn cint2e_cart_optimizer_(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: *mut libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: *mut libc::c_int,
    mut env: *mut libc::c_double,
) {
    int2e_optimizer(opt, atm, *natm, bas, *nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_optimizer_(
    mut optptr_as_integer8: size_t,
    mut atm: *mut libc::c_int,
    mut natm: *mut libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: *mut libc::c_int,
    mut env: *mut libc::c_double,
) {
    let mut opt: *mut *mut CINTOpt = optptr_as_integer8 as *mut *mut CINTOpt;
    int2e_optimizer(opt, atm, *natm, bas, *nbas, env);
}
