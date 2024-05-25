#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn CINTinit_int3c1e_EnvVars(
        envs: *mut CINTEnvVars,
        ng: *mut libc::c_int,
        shls: *mut libc::c_int,
        atm: *mut libc::c_int,
        natm: libc::c_int,
        bas: *mut libc::c_int,
        nbas: libc::c_int,
        env: *mut libc::c_double,
    );
    fn CINTnabla1i_1e(
        f: *mut libc::c_double,
        g: *mut libc::c_double,
        li: libc::c_int,
        lj: libc::c_int,
        lk: libc::c_int,
        envs: *mut CINTEnvVars,
    );
    fn c2s_sph_3c1e(
        fijkl: *mut libc::c_double,
        gctr: *mut libc::c_double,
        dims: *mut libc::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut libc::c_double,
    );
    fn c2s_cart_3c1e(
        fijkl: *mut libc::c_double,
        gctr: *mut libc::c_double,
        dims: *mut libc::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut libc::c_double,
    );
    fn CINTall_3c1e_optimizer(
        opt: *mut *mut CINTOpt,
        ng: *mut libc::c_int,
        atm: *mut libc::c_int,
        natm: libc::c_int,
        bas: *mut libc::c_int,
        nbas: libc::c_int,
        env: *mut libc::c_double,
    );
    fn CINT3c1e_drv(
        out: *mut libc::c_double,
        dims: *mut libc::c_int,
        envs: *mut CINTEnvVars,
        opt: *mut CINTOpt,
        cache: *mut libc::c_double,
        f_e1_c2s: Option::<unsafe extern "C" fn() -> ()>,
        int_type: libc::c_int,
        is_ssc: libc::c_int,
    ) -> libc::c_int;
}
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
unsafe extern "C" fn CINTgout1e_int3c1e_r2_origk(
    mut gout: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut idx: *mut libc::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: libc::c_int,
) {
    let mut nf: libc::c_int = (*envs).nf;
    let mut ix: libc::c_int = 0;
    let mut iy: libc::c_int = 0;
    let mut iz: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut g0: *mut libc::c_double = g;
    let mut g1: *mut libc::c_double = g0
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g2: *mut libc::c_double = g1
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g3: *mut libc::c_double = g2
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut s: libc::c_double = 0.;
    g1 = g0.offset((*envs).g_stride_k as isize);
    g3 = g1.offset((*envs).g_stride_k as isize);
    n = 0 as libc::c_int;
    while n < nf {
        ix = *idx.offset((0 as libc::c_int + n * 3 as libc::c_int) as isize);
        iy = *idx.offset((1 as libc::c_int + n * 3 as libc::c_int) as isize);
        iz = *idx.offset((2 as libc::c_int + n * 3 as libc::c_int) as isize);
        s = *g3.offset((ix + 0 as libc::c_int) as isize)
            * *g0.offset((iy + 0 as libc::c_int) as isize)
            * *g0.offset((iz + 0 as libc::c_int) as isize);
        s
            += *g0.offset((ix + 0 as libc::c_int) as isize)
                * *g3.offset((iy + 0 as libc::c_int) as isize)
                * *g0.offset((iz + 0 as libc::c_int) as isize);
        s
            += *g0.offset((ix + 0 as libc::c_int) as isize)
                * *g0.offset((iy + 0 as libc::c_int) as isize)
                * *g3.offset((iz + 0 as libc::c_int) as isize);
        if gout_empty != 0 {
            *gout.offset(n as isize) = s;
        } else {
            *gout.offset(n as isize) += s;
        }
        n += 1;
        n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_r2_origk_optimizer(
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
        2 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    CINTall_3c1e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_r2_origk_cart(
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
        2 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
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
    CINTinit_int3c1e_EnvVars(
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
            CINTgout1e_int3c1e_r2_origk
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
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
                c2s_cart_3c1e
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
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_r2_origk_sph(
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
        2 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
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
    CINTinit_int3c1e_EnvVars(
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
            CINTgout1e_int3c1e_r2_origk
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
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
                c2s_sph_3c1e
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
        0 as libc::c_int,
    );
}
unsafe extern "C" fn CINTgout1e_int3c1e_r4_origk(
    mut gout: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut idx: *mut libc::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: libc::c_int,
) {
    let mut nf: libc::c_int = (*envs).nf;
    let mut ix: libc::c_int = 0;
    let mut iy: libc::c_int = 0;
    let mut iz: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut g0: *mut libc::c_double = g;
    let mut g1: *mut libc::c_double = g0
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g2: *mut libc::c_double = g1
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g3: *mut libc::c_double = g2
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g4: *mut libc::c_double = g3
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g5: *mut libc::c_double = g4
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g6: *mut libc::c_double = g5
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g7: *mut libc::c_double = g6
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g8: *mut libc::c_double = g7
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g9: *mut libc::c_double = g8
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g10: *mut libc::c_double = g9
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g11: *mut libc::c_double = g10
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g12: *mut libc::c_double = g11
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g13: *mut libc::c_double = g12
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g14: *mut libc::c_double = g13
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g15: *mut libc::c_double = g14
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut s: libc::c_double = 0.;
    g1 = g0.offset((*envs).g_stride_k as isize);
    g3 = g1.offset((*envs).g_stride_k as isize);
    g4 = g0.offset((*envs).g_stride_k as isize);
    g7 = g3.offset((*envs).g_stride_k as isize);
    g12 = g4.offset((*envs).g_stride_k as isize);
    g15 = g7.offset((*envs).g_stride_k as isize);
    n = 0 as libc::c_int;
    while n < nf {
        ix = *idx.offset((0 as libc::c_int + n * 3 as libc::c_int) as isize);
        iy = *idx.offset((1 as libc::c_int + n * 3 as libc::c_int) as isize);
        iz = *idx.offset((2 as libc::c_int + n * 3 as libc::c_int) as isize);
        s = *g15.offset((ix + 0 as libc::c_int) as isize)
            * *g0.offset((iy + 0 as libc::c_int) as isize)
            * *g0.offset((iz + 0 as libc::c_int) as isize);
        s
            += *g12.offset((ix + 0 as libc::c_int) as isize)
                * *g3.offset((iy + 0 as libc::c_int) as isize)
                * *g0.offset((iz + 0 as libc::c_int) as isize)
                * 2 as libc::c_int as libc::c_double;
        s
            += *g12.offset((ix + 0 as libc::c_int) as isize)
                * *g0.offset((iy + 0 as libc::c_int) as isize)
                * *g3.offset((iz + 0 as libc::c_int) as isize)
                * 2 as libc::c_int as libc::c_double;
        s
            += *g0.offset((ix + 0 as libc::c_int) as isize)
                * *g15.offset((iy + 0 as libc::c_int) as isize)
                * *g0.offset((iz + 0 as libc::c_int) as isize);
        s
            += *g0.offset((ix + 0 as libc::c_int) as isize)
                * *g12.offset((iy + 0 as libc::c_int) as isize)
                * *g3.offset((iz + 0 as libc::c_int) as isize)
                * 2 as libc::c_int as libc::c_double;
        s
            += *g0.offset((ix + 0 as libc::c_int) as isize)
                * *g0.offset((iy + 0 as libc::c_int) as isize)
                * *g15.offset((iz + 0 as libc::c_int) as isize);
        if gout_empty != 0 {
            *gout.offset(n as isize) = s;
        } else {
            *gout.offset(n as isize) += s;
        }
        n += 1;
        n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_r4_origk_optimizer(
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
        4 as libc::c_int,
        0 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    CINTall_3c1e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_r4_origk_cart(
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
        4 as libc::c_int,
        0 as libc::c_int,
        4 as libc::c_int,
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
    CINTinit_int3c1e_EnvVars(
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
            CINTgout1e_int3c1e_r4_origk
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
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
                c2s_cart_3c1e
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
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_r4_origk_sph(
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
        4 as libc::c_int,
        0 as libc::c_int,
        4 as libc::c_int,
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
    CINTinit_int3c1e_EnvVars(
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
            CINTgout1e_int3c1e_r4_origk
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
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
                c2s_sph_3c1e
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
        0 as libc::c_int,
    );
}
unsafe extern "C" fn CINTgout1e_int3c1e_r6_origk(
    mut gout: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut idx: *mut libc::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: libc::c_int,
) {
    let mut nf: libc::c_int = (*envs).nf;
    let mut ix: libc::c_int = 0;
    let mut iy: libc::c_int = 0;
    let mut iz: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut g0: *mut libc::c_double = g;
    let mut g1: *mut libc::c_double = g0
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g2: *mut libc::c_double = g1
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g3: *mut libc::c_double = g2
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g4: *mut libc::c_double = g3
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g5: *mut libc::c_double = g4
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g6: *mut libc::c_double = g5
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g7: *mut libc::c_double = g6
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g8: *mut libc::c_double = g7
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g9: *mut libc::c_double = g8
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g10: *mut libc::c_double = g9
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g11: *mut libc::c_double = g10
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g12: *mut libc::c_double = g11
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g13: *mut libc::c_double = g12
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g14: *mut libc::c_double = g13
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g15: *mut libc::c_double = g14
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g16: *mut libc::c_double = g15
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g17: *mut libc::c_double = g16
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g18: *mut libc::c_double = g17
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g19: *mut libc::c_double = g18
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g20: *mut libc::c_double = g19
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g21: *mut libc::c_double = g20
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g22: *mut libc::c_double = g21
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g23: *mut libc::c_double = g22
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g24: *mut libc::c_double = g23
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g25: *mut libc::c_double = g24
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g26: *mut libc::c_double = g25
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g27: *mut libc::c_double = g26
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g28: *mut libc::c_double = g27
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g29: *mut libc::c_double = g28
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g30: *mut libc::c_double = g29
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g31: *mut libc::c_double = g30
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g32: *mut libc::c_double = g31
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g33: *mut libc::c_double = g32
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g34: *mut libc::c_double = g33
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g35: *mut libc::c_double = g34
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g36: *mut libc::c_double = g35
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g37: *mut libc::c_double = g36
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g38: *mut libc::c_double = g37
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g39: *mut libc::c_double = g38
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g40: *mut libc::c_double = g39
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g41: *mut libc::c_double = g40
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g42: *mut libc::c_double = g41
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g43: *mut libc::c_double = g42
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g44: *mut libc::c_double = g43
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g45: *mut libc::c_double = g44
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g46: *mut libc::c_double = g45
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g47: *mut libc::c_double = g46
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g48: *mut libc::c_double = g47
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g49: *mut libc::c_double = g48
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g50: *mut libc::c_double = g49
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g51: *mut libc::c_double = g50
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g52: *mut libc::c_double = g51
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g53: *mut libc::c_double = g52
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g54: *mut libc::c_double = g53
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g55: *mut libc::c_double = g54
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g56: *mut libc::c_double = g55
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g57: *mut libc::c_double = g56
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g58: *mut libc::c_double = g57
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g59: *mut libc::c_double = g58
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g60: *mut libc::c_double = g59
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g61: *mut libc::c_double = g60
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g62: *mut libc::c_double = g61
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g63: *mut libc::c_double = g62
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut s: libc::c_double = 0.;
    g1 = g0.offset((*envs).g_stride_k as isize);
    g3 = g1.offset((*envs).g_stride_k as isize);
    g4 = g0.offset((*envs).g_stride_k as isize);
    g7 = g3.offset((*envs).g_stride_k as isize);
    g12 = g4.offset((*envs).g_stride_k as isize);
    g15 = g7.offset((*envs).g_stride_k as isize);
    g16 = g0.offset((*envs).g_stride_k as isize);
    g28 = g12.offset((*envs).g_stride_k as isize);
    g31 = g15.offset((*envs).g_stride_k as isize);
    g48 = g16.offset((*envs).g_stride_k as isize);
    g60 = g28.offset((*envs).g_stride_k as isize);
    g63 = g31.offset((*envs).g_stride_k as isize);
    n = 0 as libc::c_int;
    while n < nf {
        ix = *idx.offset((0 as libc::c_int + n * 3 as libc::c_int) as isize);
        iy = *idx.offset((1 as libc::c_int + n * 3 as libc::c_int) as isize);
        iz = *idx.offset((2 as libc::c_int + n * 3 as libc::c_int) as isize);
        s = *g63.offset((ix + 0 as libc::c_int) as isize)
            * *g0.offset((iy + 0 as libc::c_int) as isize)
            * *g0.offset((iz + 0 as libc::c_int) as isize);
        s
            += *g60.offset((ix + 0 as libc::c_int) as isize)
                * *g3.offset((iy + 0 as libc::c_int) as isize)
                * *g0.offset((iz + 0 as libc::c_int) as isize)
                * 3 as libc::c_int as libc::c_double;
        s
            += *g60.offset((ix + 0 as libc::c_int) as isize)
                * *g0.offset((iy + 0 as libc::c_int) as isize)
                * *g3.offset((iz + 0 as libc::c_int) as isize)
                * 3 as libc::c_int as libc::c_double;
        s
            += *g48.offset((ix + 0 as libc::c_int) as isize)
                * *g15.offset((iy + 0 as libc::c_int) as isize)
                * *g0.offset((iz + 0 as libc::c_int) as isize)
                * 3 as libc::c_int as libc::c_double;
        s
            += *g48.offset((ix + 0 as libc::c_int) as isize)
                * *g12.offset((iy + 0 as libc::c_int) as isize)
                * *g3.offset((iz + 0 as libc::c_int) as isize)
                * 6 as libc::c_int as libc::c_double;
        s
            += *g48.offset((ix + 0 as libc::c_int) as isize)
                * *g0.offset((iy + 0 as libc::c_int) as isize)
                * *g15.offset((iz + 0 as libc::c_int) as isize)
                * 3 as libc::c_int as libc::c_double;
        s
            += *g0.offset((ix + 0 as libc::c_int) as isize)
                * *g63.offset((iy + 0 as libc::c_int) as isize)
                * *g0.offset((iz + 0 as libc::c_int) as isize);
        s
            += *g0.offset((ix + 0 as libc::c_int) as isize)
                * *g60.offset((iy + 0 as libc::c_int) as isize)
                * *g3.offset((iz + 0 as libc::c_int) as isize)
                * 3 as libc::c_int as libc::c_double;
        s
            += *g0.offset((ix + 0 as libc::c_int) as isize)
                * *g48.offset((iy + 0 as libc::c_int) as isize)
                * *g15.offset((iz + 0 as libc::c_int) as isize)
                * 3 as libc::c_int as libc::c_double;
        s
            += *g0.offset((ix + 0 as libc::c_int) as isize)
                * *g0.offset((iy + 0 as libc::c_int) as isize)
                * *g63.offset((iz + 0 as libc::c_int) as isize);
        if gout_empty != 0 {
            *gout.offset(n as isize) = s;
        } else {
            *gout.offset(n as isize) += s;
        }
        n += 1;
        n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_r6_origk_optimizer(
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
        6 as libc::c_int,
        0 as libc::c_int,
        6 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    CINTall_3c1e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_r6_origk_cart(
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
        6 as libc::c_int,
        0 as libc::c_int,
        6 as libc::c_int,
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
    CINTinit_int3c1e_EnvVars(
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
            CINTgout1e_int3c1e_r6_origk
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
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
                c2s_cart_3c1e
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
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_r6_origk_sph(
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
        6 as libc::c_int,
        0 as libc::c_int,
        6 as libc::c_int,
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
    CINTinit_int3c1e_EnvVars(
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
            CINTgout1e_int3c1e_r6_origk
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
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
                c2s_sph_3c1e
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
        0 as libc::c_int,
    );
}
unsafe extern "C" fn CINTgout1e_int3c1e_ip1_r2_origk(
    mut gout: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut idx: *mut libc::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: libc::c_int,
) {
    let mut nf: libc::c_int = (*envs).nf;
    let mut ix: libc::c_int = 0;
    let mut iy: libc::c_int = 0;
    let mut iz: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut g0: *mut libc::c_double = g;
    let mut g1: *mut libc::c_double = g0
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g2: *mut libc::c_double = g1
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g3: *mut libc::c_double = g2
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g4: *mut libc::c_double = g3
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g5: *mut libc::c_double = g4
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g6: *mut libc::c_double = g5
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g7: *mut libc::c_double = g6
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut s: [libc::c_double; 3] = [0.; 3];
    g1 = g0.offset((*envs).g_stride_k as isize);
    g3 = g1.offset((*envs).g_stride_k as isize);
    CINTnabla1i_1e(
        g4,
        g0,
        (*envs).i_l + 0 as libc::c_int,
        (*envs).j_l,
        (*envs).k_l,
        envs,
    );
    CINTnabla1i_1e(
        g7,
        g3,
        (*envs).i_l + 0 as libc::c_int,
        (*envs).j_l,
        (*envs).k_l,
        envs,
    );
    n = 0 as libc::c_int;
    while n < nf {
        ix = *idx.offset((0 as libc::c_int + n * 3 as libc::c_int) as isize);
        iy = *idx.offset((1 as libc::c_int + n * 3 as libc::c_int) as isize);
        iz = *idx.offset((2 as libc::c_int + n * 3 as libc::c_int) as isize);
        s[0 as libc::c_int
            as usize] = *g7.offset(ix as isize) * *g0.offset(iy as isize)
            * *g0.offset(iz as isize)
            + *g4.offset(ix as isize) * *g3.offset(iy as isize) * *g0.offset(iz as isize)
            + *g4.offset(ix as isize) * *g0.offset(iy as isize)
                * *g3.offset(iz as isize);
        s[1 as libc::c_int
            as usize] = *g3.offset(ix as isize) * *g4.offset(iy as isize)
            * *g0.offset(iz as isize)
            + *g0.offset(ix as isize) * *g7.offset(iy as isize) * *g0.offset(iz as isize)
            + *g0.offset(ix as isize) * *g4.offset(iy as isize)
                * *g3.offset(iz as isize);
        s[2 as libc::c_int
            as usize] = *g3.offset(ix as isize) * *g0.offset(iy as isize)
            * *g4.offset(iz as isize)
            + *g0.offset(ix as isize) * *g3.offset(iy as isize) * *g4.offset(iz as isize)
            + *g0.offset(ix as isize) * *g0.offset(iy as isize)
                * *g7.offset(iz as isize);
        if gout_empty != 0 {
            *gout.offset((n * 3 as libc::c_int) as isize) = s[0 as libc::c_int as usize];
            *gout
                .offset(
                    (n * 3 as libc::c_int + 1 as libc::c_int) as isize,
                ) = s[1 as libc::c_int as usize];
            *gout
                .offset(
                    (n * 3 as libc::c_int + 2 as libc::c_int) as isize,
                ) = s[2 as libc::c_int as usize];
        } else {
            *gout.offset((n * 3 as libc::c_int) as isize)
                += s[0 as libc::c_int as usize];
            *gout.offset((n * 3 as libc::c_int + 1 as libc::c_int) as isize)
                += s[1 as libc::c_int as usize];
            *gout.offset((n * 3 as libc::c_int + 2 as libc::c_int) as isize)
                += s[2 as libc::c_int as usize];
        }
        n += 1;
        n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_ip1_r2_origk_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
) {
    let mut ng: [libc::c_int; 8] = [
        1 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
    ];
    CINTall_3c1e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_ip1_r2_origk_cart(
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
        1 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
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
    CINTinit_int3c1e_EnvVars(
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
            CINTgout1e_int3c1e_ip1_r2_origk
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
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
                c2s_cart_3c1e
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
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_ip1_r2_origk_sph(
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
        1 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
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
    CINTinit_int3c1e_EnvVars(
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
            CINTgout1e_int3c1e_ip1_r2_origk
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
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
                c2s_sph_3c1e
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
        0 as libc::c_int,
    );
}
unsafe extern "C" fn CINTgout1e_int3c1e_ip1_r4_origk(
    mut gout: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut idx: *mut libc::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: libc::c_int,
) {
    let mut nf: libc::c_int = (*envs).nf;
    let mut ix: libc::c_int = 0;
    let mut iy: libc::c_int = 0;
    let mut iz: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut g0: *mut libc::c_double = g;
    let mut g1: *mut libc::c_double = g0
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g2: *mut libc::c_double = g1
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g3: *mut libc::c_double = g2
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g4: *mut libc::c_double = g3
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g5: *mut libc::c_double = g4
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g6: *mut libc::c_double = g5
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g7: *mut libc::c_double = g6
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g8: *mut libc::c_double = g7
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g9: *mut libc::c_double = g8
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g10: *mut libc::c_double = g9
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g11: *mut libc::c_double = g10
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g12: *mut libc::c_double = g11
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g13: *mut libc::c_double = g12
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g14: *mut libc::c_double = g13
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g15: *mut libc::c_double = g14
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g16: *mut libc::c_double = g15
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g17: *mut libc::c_double = g16
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g18: *mut libc::c_double = g17
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g19: *mut libc::c_double = g18
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g20: *mut libc::c_double = g19
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g21: *mut libc::c_double = g20
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g22: *mut libc::c_double = g21
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g23: *mut libc::c_double = g22
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g24: *mut libc::c_double = g23
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g25: *mut libc::c_double = g24
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g26: *mut libc::c_double = g25
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g27: *mut libc::c_double = g26
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g28: *mut libc::c_double = g27
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g29: *mut libc::c_double = g28
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g30: *mut libc::c_double = g29
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g31: *mut libc::c_double = g30
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut s: [libc::c_double; 3] = [0.; 3];
    g1 = g0.offset((*envs).g_stride_k as isize);
    g3 = g1.offset((*envs).g_stride_k as isize);
    g4 = g0.offset((*envs).g_stride_k as isize);
    g7 = g3.offset((*envs).g_stride_k as isize);
    g12 = g4.offset((*envs).g_stride_k as isize);
    g15 = g7.offset((*envs).g_stride_k as isize);
    CINTnabla1i_1e(
        g16,
        g0,
        (*envs).i_l + 0 as libc::c_int,
        (*envs).j_l,
        (*envs).k_l,
        envs,
    );
    CINTnabla1i_1e(
        g19,
        g3,
        (*envs).i_l + 0 as libc::c_int,
        (*envs).j_l,
        (*envs).k_l,
        envs,
    );
    CINTnabla1i_1e(
        g28,
        g12,
        (*envs).i_l + 0 as libc::c_int,
        (*envs).j_l,
        (*envs).k_l,
        envs,
    );
    CINTnabla1i_1e(
        g31,
        g15,
        (*envs).i_l + 0 as libc::c_int,
        (*envs).j_l,
        (*envs).k_l,
        envs,
    );
    n = 0 as libc::c_int;
    while n < nf {
        ix = *idx.offset((0 as libc::c_int + n * 3 as libc::c_int) as isize);
        iy = *idx.offset((1 as libc::c_int + n * 3 as libc::c_int) as isize);
        iz = *idx.offset((2 as libc::c_int + n * 3 as libc::c_int) as isize);
        s[0 as libc::c_int
            as usize] = *g31.offset(ix as isize) * *g0.offset(iy as isize)
            * *g0.offset(iz as isize)
            + 2 as libc::c_int as libc::c_double * *g28.offset(ix as isize)
                * *g3.offset(iy as isize) * *g0.offset(iz as isize)
            + 2 as libc::c_int as libc::c_double * *g28.offset(ix as isize)
                * *g0.offset(iy as isize) * *g3.offset(iz as isize)
            + *g16.offset(ix as isize) * *g15.offset(iy as isize)
                * *g0.offset(iz as isize)
            + 2 as libc::c_int as libc::c_double * *g16.offset(ix as isize)
                * *g12.offset(iy as isize) * *g3.offset(iz as isize)
            + *g16.offset(ix as isize) * *g0.offset(iy as isize)
                * *g15.offset(iz as isize);
        s[1 as libc::c_int
            as usize] = *g15.offset(ix as isize) * *g16.offset(iy as isize)
            * *g0.offset(iz as isize)
            + 2 as libc::c_int as libc::c_double * *g12.offset(ix as isize)
                * *g19.offset(iy as isize) * *g0.offset(iz as isize)
            + 2 as libc::c_int as libc::c_double * *g12.offset(ix as isize)
                * *g16.offset(iy as isize) * *g3.offset(iz as isize)
            + *g0.offset(ix as isize) * *g31.offset(iy as isize)
                * *g0.offset(iz as isize)
            + 2 as libc::c_int as libc::c_double * *g0.offset(ix as isize)
                * *g28.offset(iy as isize) * *g3.offset(iz as isize)
            + *g0.offset(ix as isize) * *g16.offset(iy as isize)
                * *g15.offset(iz as isize);
        s[2 as libc::c_int
            as usize] = *g15.offset(ix as isize) * *g0.offset(iy as isize)
            * *g16.offset(iz as isize)
            + 2 as libc::c_int as libc::c_double * *g12.offset(ix as isize)
                * *g3.offset(iy as isize) * *g16.offset(iz as isize)
            + 2 as libc::c_int as libc::c_double * *g12.offset(ix as isize)
                * *g0.offset(iy as isize) * *g19.offset(iz as isize)
            + *g0.offset(ix as isize) * *g15.offset(iy as isize)
                * *g16.offset(iz as isize)
            + 2 as libc::c_int as libc::c_double * *g0.offset(ix as isize)
                * *g12.offset(iy as isize) * *g19.offset(iz as isize)
            + *g0.offset(ix as isize) * *g0.offset(iy as isize)
                * *g31.offset(iz as isize);
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 3 as libc::c_int + 0 as libc::c_int) as isize,
                ) = s[0 as libc::c_int as usize];
            *gout
                .offset(
                    (n * 3 as libc::c_int + 1 as libc::c_int) as isize,
                ) = s[1 as libc::c_int as usize];
            *gout
                .offset(
                    (n * 3 as libc::c_int + 2 as libc::c_int) as isize,
                ) = s[2 as libc::c_int as usize];
        } else {
            *gout.offset((n * 3 as libc::c_int + 0 as libc::c_int) as isize)
                += s[0 as libc::c_int as usize];
            *gout.offset((n * 3 as libc::c_int + 1 as libc::c_int) as isize)
                += s[1 as libc::c_int as usize];
            *gout.offset((n * 3 as libc::c_int + 2 as libc::c_int) as isize)
                += s[2 as libc::c_int as usize];
        }
        n += 1;
        n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_ip1_r4_origk_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
) {
    let mut ng: [libc::c_int; 8] = [
        1 as libc::c_int,
        0 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
    ];
    CINTall_3c1e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_ip1_r4_origk_cart(
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
        1 as libc::c_int,
        0 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
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
    CINTinit_int3c1e_EnvVars(
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
            CINTgout1e_int3c1e_ip1_r4_origk
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
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
                c2s_cart_3c1e
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
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_ip1_r4_origk_sph(
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
        1 as libc::c_int,
        0 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
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
    CINTinit_int3c1e_EnvVars(
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
            CINTgout1e_int3c1e_ip1_r4_origk
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
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
                c2s_sph_3c1e
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
        0 as libc::c_int,
    );
}
unsafe extern "C" fn CINTgout1e_int3c1e_ip1_r6_origk(
    mut gout: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut idx: *mut libc::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: libc::c_int,
) {
    let mut nf: libc::c_int = (*envs).nf;
    let mut ix: libc::c_int = 0;
    let mut iy: libc::c_int = 0;
    let mut iz: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut g0: *mut libc::c_double = g;
    let mut g1: *mut libc::c_double = g0
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g2: *mut libc::c_double = g1
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g3: *mut libc::c_double = g2
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g4: *mut libc::c_double = g3
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g5: *mut libc::c_double = g4
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g6: *mut libc::c_double = g5
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g7: *mut libc::c_double = g6
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g8: *mut libc::c_double = g7
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g9: *mut libc::c_double = g8
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g10: *mut libc::c_double = g9
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g11: *mut libc::c_double = g10
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g12: *mut libc::c_double = g11
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g13: *mut libc::c_double = g12
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g14: *mut libc::c_double = g13
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g15: *mut libc::c_double = g14
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g16: *mut libc::c_double = g15
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g17: *mut libc::c_double = g16
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g18: *mut libc::c_double = g17
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g19: *mut libc::c_double = g18
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g20: *mut libc::c_double = g19
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g21: *mut libc::c_double = g20
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g22: *mut libc::c_double = g21
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g23: *mut libc::c_double = g22
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g24: *mut libc::c_double = g23
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g25: *mut libc::c_double = g24
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g26: *mut libc::c_double = g25
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g27: *mut libc::c_double = g26
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g28: *mut libc::c_double = g27
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g29: *mut libc::c_double = g28
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g30: *mut libc::c_double = g29
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g31: *mut libc::c_double = g30
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g32: *mut libc::c_double = g31
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g33: *mut libc::c_double = g32
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g34: *mut libc::c_double = g33
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g35: *mut libc::c_double = g34
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g36: *mut libc::c_double = g35
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g37: *mut libc::c_double = g36
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g38: *mut libc::c_double = g37
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g39: *mut libc::c_double = g38
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g40: *mut libc::c_double = g39
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g41: *mut libc::c_double = g40
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g42: *mut libc::c_double = g41
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g43: *mut libc::c_double = g42
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g44: *mut libc::c_double = g43
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g45: *mut libc::c_double = g44
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g46: *mut libc::c_double = g45
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g47: *mut libc::c_double = g46
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g48: *mut libc::c_double = g47
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g49: *mut libc::c_double = g48
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g50: *mut libc::c_double = g49
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g51: *mut libc::c_double = g50
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g52: *mut libc::c_double = g51
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g53: *mut libc::c_double = g52
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g54: *mut libc::c_double = g53
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g55: *mut libc::c_double = g54
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g56: *mut libc::c_double = g55
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g57: *mut libc::c_double = g56
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g58: *mut libc::c_double = g57
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g59: *mut libc::c_double = g58
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g60: *mut libc::c_double = g59
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g61: *mut libc::c_double = g60
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g62: *mut libc::c_double = g61
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g63: *mut libc::c_double = g62
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g64: *mut libc::c_double = g63
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g65: *mut libc::c_double = g64
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g66: *mut libc::c_double = g65
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g67: *mut libc::c_double = g66
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g68: *mut libc::c_double = g67
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g69: *mut libc::c_double = g68
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g70: *mut libc::c_double = g69
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g71: *mut libc::c_double = g70
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g72: *mut libc::c_double = g71
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g73: *mut libc::c_double = g72
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g74: *mut libc::c_double = g73
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g75: *mut libc::c_double = g74
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g76: *mut libc::c_double = g75
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g77: *mut libc::c_double = g76
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g78: *mut libc::c_double = g77
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g79: *mut libc::c_double = g78
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g80: *mut libc::c_double = g79
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g81: *mut libc::c_double = g80
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g82: *mut libc::c_double = g81
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g83: *mut libc::c_double = g82
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g84: *mut libc::c_double = g83
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g85: *mut libc::c_double = g84
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g86: *mut libc::c_double = g85
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g87: *mut libc::c_double = g86
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g88: *mut libc::c_double = g87
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g89: *mut libc::c_double = g88
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g90: *mut libc::c_double = g89
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g91: *mut libc::c_double = g90
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g92: *mut libc::c_double = g91
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g93: *mut libc::c_double = g92
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g94: *mut libc::c_double = g93
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g95: *mut libc::c_double = g94
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g96: *mut libc::c_double = g95
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g97: *mut libc::c_double = g96
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g98: *mut libc::c_double = g97
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g99: *mut libc::c_double = g98
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g100: *mut libc::c_double = g99
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g101: *mut libc::c_double = g100
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g102: *mut libc::c_double = g101
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g103: *mut libc::c_double = g102
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g104: *mut libc::c_double = g103
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g105: *mut libc::c_double = g104
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g106: *mut libc::c_double = g105
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g107: *mut libc::c_double = g106
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g108: *mut libc::c_double = g107
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g109: *mut libc::c_double = g108
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g110: *mut libc::c_double = g109
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g111: *mut libc::c_double = g110
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g112: *mut libc::c_double = g111
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g113: *mut libc::c_double = g112
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g114: *mut libc::c_double = g113
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g115: *mut libc::c_double = g114
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g116: *mut libc::c_double = g115
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g117: *mut libc::c_double = g116
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g118: *mut libc::c_double = g117
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g119: *mut libc::c_double = g118
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g120: *mut libc::c_double = g119
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g121: *mut libc::c_double = g120
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g122: *mut libc::c_double = g121
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g123: *mut libc::c_double = g122
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g124: *mut libc::c_double = g123
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g125: *mut libc::c_double = g124
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g126: *mut libc::c_double = g125
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut g127: *mut libc::c_double = g126
        .offset(((*envs).g_size * 3 as libc::c_int) as isize);
    let mut s: [libc::c_double; 3] = [0.; 3];
    g1 = g0.offset((*envs).g_stride_k as isize);
    g3 = g1.offset((*envs).g_stride_k as isize);
    g4 = g0.offset((*envs).g_stride_k as isize);
    g7 = g3.offset((*envs).g_stride_k as isize);
    g12 = g4.offset((*envs).g_stride_k as isize);
    g15 = g7.offset((*envs).g_stride_k as isize);
    g16 = g0.offset((*envs).g_stride_k as isize);
    g28 = g12.offset((*envs).g_stride_k as isize);
    g31 = g15.offset((*envs).g_stride_k as isize);
    g48 = g16.offset((*envs).g_stride_k as isize);
    g60 = g28.offset((*envs).g_stride_k as isize);
    g63 = g31.offset((*envs).g_stride_k as isize);
    CINTnabla1i_1e(
        g64,
        g0,
        (*envs).i_l + 0 as libc::c_int,
        (*envs).j_l,
        (*envs).k_l,
        envs,
    );
    CINTnabla1i_1e(
        g67,
        g3,
        (*envs).i_l + 0 as libc::c_int,
        (*envs).j_l,
        (*envs).k_l,
        envs,
    );
    CINTnabla1i_1e(
        g79,
        g15,
        (*envs).i_l + 0 as libc::c_int,
        (*envs).j_l,
        (*envs).k_l,
        envs,
    );
    CINTnabla1i_1e(
        g112,
        g48,
        (*envs).i_l + 0 as libc::c_int,
        (*envs).j_l,
        (*envs).k_l,
        envs,
    );
    CINTnabla1i_1e(
        g124,
        g60,
        (*envs).i_l + 0 as libc::c_int,
        (*envs).j_l,
        (*envs).k_l,
        envs,
    );
    CINTnabla1i_1e(
        g127,
        g63,
        (*envs).i_l + 0 as libc::c_int,
        (*envs).j_l,
        (*envs).k_l,
        envs,
    );
    n = 0 as libc::c_int;
    while n < nf {
        ix = *idx.offset((0 as libc::c_int + n * 3 as libc::c_int) as isize);
        iy = *idx.offset((1 as libc::c_int + n * 3 as libc::c_int) as isize);
        iz = *idx.offset((2 as libc::c_int + n * 3 as libc::c_int) as isize);
        s[0 as libc::c_int
            as usize] = *g127.offset(ix as isize) * *g0.offset(iy as isize)
            * *g0.offset(iz as isize)
            + 3 as libc::c_int as libc::c_double * *g124.offset(ix as isize)
                * *g3.offset(iy as isize) * *g0.offset(iz as isize)
            + 3 as libc::c_int as libc::c_double * *g124.offset(ix as isize)
                * *g0.offset(iy as isize) * *g3.offset(iz as isize)
            + 3 as libc::c_int as libc::c_double * *g112.offset(ix as isize)
                * *g15.offset(iy as isize) * *g0.offset(iz as isize)
            + 6 as libc::c_int as libc::c_double * *g112.offset(ix as isize)
                * *g12.offset(iy as isize) * *g3.offset(iz as isize)
            + 3 as libc::c_int as libc::c_double * *g112.offset(ix as isize)
                * *g0.offset(iy as isize) * *g15.offset(iz as isize)
            + *g64.offset(ix as isize) * *g63.offset(iy as isize)
                * *g0.offset(iz as isize)
            + 3 as libc::c_int as libc::c_double * *g64.offset(ix as isize)
                * *g60.offset(iy as isize) * *g3.offset(iz as isize)
            + 3 as libc::c_int as libc::c_double * *g64.offset(ix as isize)
                * *g48.offset(iy as isize) * *g15.offset(iz as isize)
            + *g64.offset(ix as isize) * *g0.offset(iy as isize)
                * *g63.offset(iz as isize);
        s[1 as libc::c_int
            as usize] = *g63.offset(ix as isize) * *g64.offset(iy as isize)
            * *g0.offset(iz as isize)
            + 3 as libc::c_int as libc::c_double * *g60.offset(ix as isize)
                * *g67.offset(iy as isize) * *g0.offset(iz as isize)
            + 3 as libc::c_int as libc::c_double * *g60.offset(ix as isize)
                * *g64.offset(iy as isize) * *g3.offset(iz as isize)
            + 3 as libc::c_int as libc::c_double * *g48.offset(ix as isize)
                * *g79.offset(iy as isize) * *g0.offset(iz as isize)
            + 6 as libc::c_int as libc::c_double * *g48.offset(ix as isize)
                * *g76.offset(iy as isize) * *g3.offset(iz as isize)
            + 3 as libc::c_int as libc::c_double * *g48.offset(ix as isize)
                * *g64.offset(iy as isize) * *g15.offset(iz as isize)
            + *g0.offset(ix as isize) * *g127.offset(iy as isize)
                * *g0.offset(iz as isize)
            + 3 as libc::c_int as libc::c_double * *g0.offset(ix as isize)
                * *g124.offset(iy as isize) * *g3.offset(iz as isize)
            + 3 as libc::c_int as libc::c_double * *g0.offset(ix as isize)
                * *g112.offset(iy as isize) * *g15.offset(iz as isize)
            + *g0.offset(ix as isize) * *g64.offset(iy as isize)
                * *g63.offset(iz as isize);
        s[2 as libc::c_int
            as usize] = *g63.offset(ix as isize) * *g0.offset(iy as isize)
            * *g64.offset(iz as isize)
            + 3 as libc::c_int as libc::c_double * *g60.offset(ix as isize)
                * *g3.offset(iy as isize) * *g64.offset(iz as isize)
            + 3 as libc::c_int as libc::c_double * *g60.offset(ix as isize)
                * *g0.offset(iy as isize) * *g67.offset(iz as isize)
            + 3 as libc::c_int as libc::c_double * *g48.offset(ix as isize)
                * *g15.offset(iy as isize) * *g64.offset(iz as isize)
            + 6 as libc::c_int as libc::c_double * *g48.offset(ix as isize)
                * *g12.offset(iy as isize) * *g67.offset(iz as isize)
            + 3 as libc::c_int as libc::c_double * *g48.offset(ix as isize)
                * *g0.offset(iy as isize) * *g79.offset(iz as isize)
            + *g0.offset(ix as isize) * *g63.offset(iy as isize)
                * *g64.offset(iz as isize)
            + 3 as libc::c_int as libc::c_double * *g0.offset(ix as isize)
                * *g60.offset(iy as isize) * *g67.offset(iz as isize)
            + 3 as libc::c_int as libc::c_double * *g0.offset(ix as isize)
                * *g48.offset(iy as isize) * *g79.offset(iz as isize)
            + *g0.offset(ix as isize) * *g0.offset(iy as isize)
                * *g127.offset(iz as isize);
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 3 as libc::c_int + 0 as libc::c_int) as isize,
                ) = s[0 as libc::c_int as usize];
            *gout
                .offset(
                    (n * 3 as libc::c_int + 1 as libc::c_int) as isize,
                ) = s[1 as libc::c_int as usize];
            *gout
                .offset(
                    (n * 3 as libc::c_int + 2 as libc::c_int) as isize,
                ) = s[2 as libc::c_int as usize];
        } else {
            *gout.offset((n * 3 as libc::c_int + 0 as libc::c_int) as isize)
                += s[0 as libc::c_int as usize];
            *gout.offset((n * 3 as libc::c_int + 1 as libc::c_int) as isize)
                += s[1 as libc::c_int as usize];
            *gout.offset((n * 3 as libc::c_int + 2 as libc::c_int) as isize)
                += s[2 as libc::c_int as usize];
        }
        n += 1;
        n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_ip1_r6_origk_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
) {
    let mut ng: [libc::c_int; 8] = [
        1 as libc::c_int,
        0 as libc::c_int,
        6 as libc::c_int,
        0 as libc::c_int,
        7 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
    ];
    CINTall_3c1e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_ip1_r6_origk_cart(
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
        1 as libc::c_int,
        0 as libc::c_int,
        6 as libc::c_int,
        0 as libc::c_int,
        7 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
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
    CINTinit_int3c1e_EnvVars(
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
            CINTgout1e_int3c1e_ip1_r6_origk
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
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
                c2s_cart_3c1e
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
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int3c1e_ip1_r6_origk_sph(
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
        1 as libc::c_int,
        0 as libc::c_int,
        6 as libc::c_int,
        0 as libc::c_int,
        7 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
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
    CINTinit_int3c1e_EnvVars(
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
            CINTgout1e_int3c1e_ip1_r6_origk
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_int,
                    *mut CINTEnvVars,
                    libc::c_int,
                ) -> (),
        ),
    );
    return CINT3c1e_drv(
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
                c2s_sph_3c1e
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
        0 as libc::c_int,
    );
}
