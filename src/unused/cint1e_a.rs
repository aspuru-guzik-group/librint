#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
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
    fn CINTnabla1j_1e(
        f: *mut libc::c_double,
        g: *mut libc::c_double,
        li: libc::c_int,
        lj: libc::c_int,
        lk: libc::c_int,
        envs: *mut CINTEnvVars,
    );
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
    fn CINTall_1e_optimizer(
        opt: *mut *mut CINTOpt,
        ng: *mut libc::c_int,
        atm: *mut libc::c_int,
        natm: libc::c_int,
        bas: *mut libc::c_int,
        nbas: libc::c_int,
        env: *mut libc::c_double,
    );
    fn CINT1e_drv(
        out: *mut libc::c_double,
        dims: *mut libc::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut libc::c_double,
        f_c2s: Option::<unsafe extern "C" fn() -> ()>,
        int1e_type: libc::c_int,
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
unsafe extern "C" fn CINTgout1e_int1e_r2_origi(
    mut gout: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut idx: *mut libc::c_int,
    mut envs: *mut CINTEnvVars,
    mut empty: libc::c_int,
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
    g1 = g0.offset((*envs).g_stride_i as isize);
    g3 = g1.offset((*envs).g_stride_i as isize);
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
        if empty != 0 {
            *gout.offset(n as isize) = s;
        } else {
            *gout.offset(n as isize) += s;
        }
        n += 1;
        n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_r2_origi_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
) {
    let mut ng: [libc::c_int; 8] = [
        2 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    CINTall_1e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int1e_r2_origi_cart(
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
        2 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
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
            CINTgout1e_int1e_r2_origi
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
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_r2_origi_sph(
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
        2 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
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
            CINTgout1e_int1e_r2_origi
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
unsafe extern "C" fn CINTgout1e_int1e_r4_origi(
    mut gout: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut idx: *mut libc::c_int,
    mut envs: *mut CINTEnvVars,
    mut empty: libc::c_int,
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
    g1 = g0.offset((*envs).g_stride_i as isize);
    g3 = g1.offset((*envs).g_stride_i as isize);
    g4 = g0.offset((*envs).g_stride_i as isize);
    g7 = g3.offset((*envs).g_stride_i as isize);
    g12 = g4.offset((*envs).g_stride_i as isize);
    g15 = g7.offset((*envs).g_stride_i as isize);
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
        if empty != 0 {
            *gout.offset(n as isize) = s;
        } else {
            *gout.offset(n as isize) += s;
        }
        n += 1;
        n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_r4_origi_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
) {
    let mut ng: [libc::c_int; 8] = [
        4 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ];
    CINTall_1e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int1e_r4_origi_cart(
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
        4 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
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
            CINTgout1e_int1e_r4_origi
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
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_r4_origi_sph(
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
        4 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
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
            CINTgout1e_int1e_r4_origi
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
pub unsafe extern "C" fn CINTgout1e_int1e_r2_origi_ip2(
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
    CINTnabla1j_1e(
        g1,
        g0,
        (*envs).i_l + 2 as libc::c_int,
        (*envs).j_l + 0 as libc::c_int,
        0 as libc::c_int,
        envs,
    );
    g2 = g0.offset((*envs).g_stride_i as isize);
    g3 = g1.offset((*envs).g_stride_i as isize);
    g6 = g2.offset((*envs).g_stride_i as isize);
    g7 = g3.offset((*envs).g_stride_i as isize);
    n = 0 as libc::c_int;
    while n < nf {
        ix = *idx.offset((0 as libc::c_int + n * 3 as libc::c_int) as isize);
        iy = *idx.offset((1 as libc::c_int + n * 3 as libc::c_int) as isize);
        iz = *idx.offset((2 as libc::c_int + n * 3 as libc::c_int) as isize);
        s[0 as libc::c_int
            as usize] = *g7.offset((ix + 0 as libc::c_int) as isize)
            * *g0.offset((iy + 0 as libc::c_int) as isize)
            * *g0.offset((iz + 0 as libc::c_int) as isize)
            + *g1.offset((ix + 0 as libc::c_int) as isize)
                * *g6.offset((iy + 0 as libc::c_int) as isize)
                * *g0.offset((iz + 0 as libc::c_int) as isize)
            + *g1.offset((ix + 0 as libc::c_int) as isize)
                * *g0.offset((iy + 0 as libc::c_int) as isize)
                * *g6.offset((iz + 0 as libc::c_int) as isize);
        s[1 as libc::c_int
            as usize] = *g6.offset((ix + 0 as libc::c_int) as isize)
            * *g1.offset((iy + 0 as libc::c_int) as isize)
            * *g0.offset((iz + 0 as libc::c_int) as isize)
            + *g0.offset((ix + 0 as libc::c_int) as isize)
                * *g7.offset((iy + 0 as libc::c_int) as isize)
                * *g0.offset((iz + 0 as libc::c_int) as isize)
            + *g0.offset((ix + 0 as libc::c_int) as isize)
                * *g1.offset((iy + 0 as libc::c_int) as isize)
                * *g6.offset((iz + 0 as libc::c_int) as isize);
        s[2 as libc::c_int
            as usize] = *g6.offset((ix + 0 as libc::c_int) as isize)
            * *g0.offset((iy + 0 as libc::c_int) as isize)
            * *g1.offset((iz + 0 as libc::c_int) as isize)
            + *g0.offset((ix + 0 as libc::c_int) as isize)
                * *g6.offset((iy + 0 as libc::c_int) as isize)
                * *g1.offset((iz + 0 as libc::c_int) as isize)
            + *g0.offset((ix + 0 as libc::c_int) as isize)
                * *g0.offset((iy + 0 as libc::c_int) as isize)
                * *g7.offset((iz + 0 as libc::c_int) as isize);
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
pub unsafe extern "C" fn int1e_r2_origi_ip2_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
) {
    let mut ng: [libc::c_int; 8] = [
        2 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
    ];
    CINTall_1e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int1e_r2_origi_ip2_cart(
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
        2 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
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
            CINTgout1e_int1e_r2_origi_ip2
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
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_r2_origi_ip2_sph(
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
        2 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
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
            CINTgout1e_int1e_r2_origi_ip2
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
pub unsafe extern "C" fn CINTgout1e_int1e_r4_origi_ip2(
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
    let mut s: [libc::c_double; 243] = [0.; 243];
    CINTnabla1j_1e(
        g1,
        g0,
        (*envs).i_l + 4 as libc::c_int,
        (*envs).j_l + 0 as libc::c_int,
        0 as libc::c_int,
        envs,
    );
    g2 = g0.offset((*envs).g_stride_i as isize);
    g3 = g1.offset((*envs).g_stride_i as isize);
    g6 = g2.offset((*envs).g_stride_i as isize);
    g7 = g3.offset((*envs).g_stride_i as isize);
    g8 = g0.offset((*envs).g_stride_i as isize);
    g9 = g1.offset((*envs).g_stride_i as isize);
    g14 = g6.offset((*envs).g_stride_i as isize);
    g15 = g7.offset((*envs).g_stride_i as isize);
    g24 = g8.offset((*envs).g_stride_i as isize);
    g25 = g9.offset((*envs).g_stride_i as isize);
    g30 = g14.offset((*envs).g_stride_i as isize);
    g31 = g15.offset((*envs).g_stride_i as isize);
    n = 0 as libc::c_int;
    while n < nf {
        ix = *idx.offset((0 as libc::c_int + n * 3 as libc::c_int) as isize);
        iy = *idx.offset((1 as libc::c_int + n * 3 as libc::c_int) as isize);
        iz = *idx.offset((2 as libc::c_int + n * 3 as libc::c_int) as isize);
        s[0 as libc::c_int
            as usize] = *g31.offset((ix + 0 as libc::c_int) as isize)
            * *g0.offset((iy + 0 as libc::c_int) as isize)
            * *g0.offset((iz + 0 as libc::c_int) as isize)
            + 2 as libc::c_int as libc::c_double
                * *g25.offset((ix + 0 as libc::c_int) as isize)
                * *g6.offset((iy + 0 as libc::c_int) as isize)
                * *g0.offset((iz + 0 as libc::c_int) as isize)
            + 2 as libc::c_int as libc::c_double
                * *g25.offset((ix + 0 as libc::c_int) as isize)
                * *g0.offset((iy + 0 as libc::c_int) as isize)
                * *g6.offset((iz + 0 as libc::c_int) as isize)
            + *g1.offset((ix + 0 as libc::c_int) as isize)
                * *g30.offset((iy + 0 as libc::c_int) as isize)
                * *g0.offset((iz + 0 as libc::c_int) as isize)
            + 2 as libc::c_int as libc::c_double
                * *g1.offset((ix + 0 as libc::c_int) as isize)
                * *g24.offset((iy + 0 as libc::c_int) as isize)
                * *g6.offset((iz + 0 as libc::c_int) as isize)
            + *g1.offset((ix + 0 as libc::c_int) as isize)
                * *g0.offset((iy + 0 as libc::c_int) as isize)
                * *g30.offset((iz + 0 as libc::c_int) as isize);
        s[1 as libc::c_int
            as usize] = *g30.offset((ix + 0 as libc::c_int) as isize)
            * *g1.offset((iy + 0 as libc::c_int) as isize)
            * *g0.offset((iz + 0 as libc::c_int) as isize)
            + 2 as libc::c_int as libc::c_double
                * *g24.offset((ix + 0 as libc::c_int) as isize)
                * *g7.offset((iy + 0 as libc::c_int) as isize)
                * *g0.offset((iz + 0 as libc::c_int) as isize)
            + 2 as libc::c_int as libc::c_double
                * *g24.offset((ix + 0 as libc::c_int) as isize)
                * *g1.offset((iy + 0 as libc::c_int) as isize)
                * *g6.offset((iz + 0 as libc::c_int) as isize)
            + *g0.offset((ix + 0 as libc::c_int) as isize)
                * *g31.offset((iy + 0 as libc::c_int) as isize)
                * *g0.offset((iz + 0 as libc::c_int) as isize)
            + 2 as libc::c_int as libc::c_double
                * *g0.offset((ix + 0 as libc::c_int) as isize)
                * *g25.offset((iy + 0 as libc::c_int) as isize)
                * *g6.offset((iz + 0 as libc::c_int) as isize)
            + *g0.offset((ix + 0 as libc::c_int) as isize)
                * *g1.offset((iy + 0 as libc::c_int) as isize)
                * *g30.offset((iz + 0 as libc::c_int) as isize);
        s[2 as libc::c_int
            as usize] = *g30.offset((ix + 0 as libc::c_int) as isize)
            * *g0.offset((iy + 0 as libc::c_int) as isize)
            * *g1.offset((iz + 0 as libc::c_int) as isize)
            + 2 as libc::c_int as libc::c_double
                * *g24.offset((ix + 0 as libc::c_int) as isize)
                * *g6.offset((iy + 0 as libc::c_int) as isize)
                * *g1.offset((iz + 0 as libc::c_int) as isize)
            + 2 as libc::c_int as libc::c_double
                * *g24.offset((ix + 0 as libc::c_int) as isize)
                * *g0.offset((iy + 0 as libc::c_int) as isize)
                * *g7.offset((iz + 0 as libc::c_int) as isize)
            + *g0.offset((ix + 0 as libc::c_int) as isize)
                * *g30.offset((iy + 0 as libc::c_int) as isize)
                * *g1.offset((iz + 0 as libc::c_int) as isize)
            + 2 as libc::c_int as libc::c_double
                * *g0.offset((ix + 0 as libc::c_int) as isize)
                * *g24.offset((iy + 0 as libc::c_int) as isize)
                * *g7.offset((iz + 0 as libc::c_int) as isize)
            + *g0.offset((ix + 0 as libc::c_int) as isize)
                * *g0.offset((iy + 0 as libc::c_int) as isize)
                * *g31.offset((iz + 0 as libc::c_int) as isize);
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
pub unsafe extern "C" fn int1e_r4_origi_ip2_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
) {
    let mut ng: [libc::c_int; 8] = [
        4 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
    ];
    CINTall_1e_optimizer(opt, ng.as_mut_ptr(), atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn int1e_r4_origi_ip2_cart(
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
        4 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
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
            CINTgout1e_int1e_r4_origi_ip2
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
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_r4_origi_ip2_sph(
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
        4 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
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
            CINTgout1e_int1e_r4_origi_ip2
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
