#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments
)]

/// Which `gout` contraction kernel this integral uses -- the step that reduces
/// the g-buffer to the output block for one primitive shell pair/quartet.
///
/// This replaces a c2rust function pointer whose parameter list was erased to
/// `fn() -> ()`. It matters more than the other two: `f_gout` is called in the
/// *innermost* primitive loop (once per primitive quartet in `CINT2e_loop`), and
/// as a transmuted `unsafe extern "C"` pointer it was an opaque C-ABI call that
/// LLVM could neither inline nor vectorise across. Each enum arm is a direct
/// call, so fat LTO can inline the contraction into the loop body.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Gout {
    /// `CINTgout2e` -- the two-electron contraction.
    E2,
    /// `CINTgout1e` -- plain one-electron (overlap).
    E1,
    /// `CINTgout1e_nuc` -- nuclear attraction, accumulated over atoms.
    E1Nuc,
    /// `CINTgout1e_int1e_kin` -- kinetic energy.
    E1Kin,
}

impl Gout {
    /// Dispatch to the contraction kernel this integral was configured with.
    ///
    /// # Safety
    ///
    /// Same requirements as the function pointer this replaces: `gout`, `g` and
    /// `idx` must be valid for the block sizes implied by `envs`. `envs` itself
    /// is a reference, so only its *initialisation* is still a caller
    /// obligation -- it must be the `CINTEnvVars` already filled in for this
    /// shell pair/quartet, since the block sizes above are read out of it.
    #[inline]
    pub unsafe fn call(
        self,
        gout: *mut f64,
        g: *mut f64,
        idx: *mut i32,
        envs: &mut CINTEnvVars,
        gempty: i32,
    ) {
        match self {
            Gout::E2 => crate::cint2e::CINTgout2e(gout, g, idx, envs, gempty),
            Gout::E1 => crate::cint1e::CINTgout1e(gout, g, idx, envs, gempty),
            Gout::E1Nuc => crate::cint1e::CINTgout1e_nuc(gout, g, idx, envs, gempty),
            Gout::E1Kin => crate::intor1::CINTgout1e_int1e_kin(gout, g, idx, envs, gempty),
        }
    }
}

/// The six 2D-to-4D g-buffer transforms `CINTg0_2e` can dispatch to. Replaces a
/// c2rust function pointer whose parameter list was erased; the variants map
/// one-to-one onto the `CINT*2d4d` functions in `g2e.rs`, which is where the
/// dispatch lives.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum G0_2d4d {
    /// `CINTg0_2e_2d4d_unrolled` — `rys_order <= 2`.
    Unrolled,
    /// `CINTsrg0_2e_2d4d_unrolled` — as above but range-separated
    /// (`rys_order != nrys_roots`).
    SrUnrolled,
    /// `CINTg0_2e_ik2d4d` — `kbase` and `ibase`.
    Ik,
    /// `CINTg0_2e_kj2d4d` — `kbase`, no `ibase`.
    Kj,
    /// `CINTg0_2e_il2d4d` — no `kbase`, `ibase`.
    Il,
    /// `CINTg0_2e_lj2d4d` — neither; the general fallback.
    Lj,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PairData {
    pub rij: [f64; 3],
    pub eij: f64,
    pub cceij: f64,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CINTOpt {
    pub index_xyz_array: *mut *mut i32,
    pub non0ctr: *mut *mut i32,
    pub sortedidx: *mut *mut i32,
    pub nbas: i32,
    pub log_max_coeff: *mut *mut f64,
    pub pairdata: *mut *mut PairData,
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
    pub f_g0_2e: Option<unsafe extern "C" fn() -> i32>,
    /// Which 2D-to-4D g-buffer transform this quartet uses. libcint chooses one
    /// of six at setup time from `(rys_order, ibase, kbase)`; c2rust stored the
    /// choice as a function pointer with its signature erased to `fn() -> ()`,
    /// so the single call site in `g2e.rs` had to `transmute` the real signature
    /// back before calling. `None` means not yet configured by
    /// `CINTinit_int2e_EnvVars`. See `impl G0_2d4d` in `g2e.rs`.
    pub f_g0_2d4d: Option<G0_2d4d>,
    /// Which `gout` contraction kernel to call in the innermost primitive loop.
    /// `None` means not yet configured by the `cint1e_*`/`cint2e_*` entry point.
    pub f_gout: Option<Gout>,
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
        let envs: CINTEnvVars = CINTEnvVars {
            atm: std::ptr::null_mut::<i32>(),
            bas: std::ptr::null_mut::<i32>(),
            env: std::ptr::null_mut::<f64>(),
            shls: std::ptr::null_mut::<i32>(),
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
            rx_in_rijrx: std::ptr::null_mut::<f64>(),
            rx_in_rklrx: std::ptr::null_mut::<f64>(),
            ri: std::ptr::null_mut::<f64>(),
            rj: std::ptr::null_mut::<f64>(),
            rk: std::ptr::null_mut::<f64>(),
            c2rust_unnamed_1: C2RustUnnamed {
                rl: std::ptr::null_mut::<f64>(),
            },
            f_g0_2e: None,
            f_g0_2d4d: None,
            f_gout: None,
            opt: std::ptr::null_mut::<CINTOpt>(),
            idx: std::ptr::null_mut::<i32>(),
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
