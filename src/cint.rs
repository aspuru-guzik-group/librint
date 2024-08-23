#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PairData {
    pub rij: [f64; 3],
    pub eij: f64,
    pub cceij: f64,
}

// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct CINTOpt {
//     pub index_xyz_array: *mut *mut i32,
//     pub non0ctr: *mut *mut i32,
//     pub sortedidx: *mut *mut i32,
//     pub nbas: i32,
//     pub log_max_coeff: *mut *mut f64,
//     pub pairdata: *mut *mut PairData,
// }

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
    // pub opt: *mut CINTOpt,
    pub idx: *mut i32,
    pub ai: [f64; 1],
    pub aj: [f64; 1],
    pub ak: [f64; 1],
    pub al: [f64; 1],
    pub fac: [f64; 1],
    pub rij: [f64; 3],
    pub rkl: [f64; 3],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub rl: *mut f64,
    pub grids: *mut f64,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub nfl: i32,
    pub ngrids: i32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub nfk: i32,
    pub grids_offset: i32,
}

impl CINTEnvVars {
    pub fn new() -> Self {
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: 0 as *mut i32,
        bas: 0 as *mut i32,
        env: 0 as *mut f64,
        shls: 0 as *mut i32,
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
        rx_in_rijrx: 0 as *mut f64,
        rx_in_rklrx: 0 as *mut f64,
        ri: 0 as *mut f64,
        rj: 0 as *mut f64,
        rk: 0 as *mut f64,
        c2rust_unnamed_1: C2RustUnnamed {
            rl: 0 as *mut f64,
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        // opt: 0 as *mut CINTOpt,
        idx: 0 as *mut i32,
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