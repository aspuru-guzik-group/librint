#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn CINTg0_2e(
        g: *mut libc::c_double,
        rij: *mut libc::c_double,
        rkl: *mut libc::c_double,
        cutoff: libc::c_double,
        envs: *mut CINTEnvVars,
    ) -> libc::c_int;
    fn CINTg0_2e_2d(g: *mut libc::c_double, bc: *mut Rys2eT, envs: *mut CINTEnvVars);
    fn CINTg0_2e_2d4d_unrolled(
        g: *mut libc::c_double,
        bc: *mut Rys2eT,
        envs: *mut CINTEnvVars,
    );
    fn CINTsrg0_2e_2d4d_unrolled(
        g: *mut libc::c_double,
        bc: *mut Rys2eT,
        envs: *mut CINTEnvVars,
    );
    fn CINTcommon_fac_sp(l: libc::c_int) -> libc::c_double;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Rys2eT {
    pub c00x: [libc::c_double; 32],
    pub c00y: [libc::c_double; 32],
    pub c00z: [libc::c_double; 32],
    pub c0px: [libc::c_double; 32],
    pub c0py: [libc::c_double; 32],
    pub c0pz: [libc::c_double; 32],
    pub b01: [libc::c_double; 32],
    pub b00: [libc::c_double; 32],
    pub b10: [libc::c_double; 32],
}
#[no_mangle]
pub unsafe extern "C" fn CINTinit_int2c2e_EnvVars(
    mut envs: *mut CINTEnvVars,
    mut ng: *mut libc::c_int,
    mut shls: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut libc::c_double,
) {
    (*envs).natm = natm;
    (*envs).nbas = nbas;
    (*envs).atm = atm;
    (*envs).bas = bas;
    (*envs).env = env;
    (*envs).shls = shls;
    let i_sh: libc::c_int = *shls.offset(0 as libc::c_int as isize);
    let k_sh: libc::c_int = *shls.offset(1 as libc::c_int as isize);
    (*envs).i_l = *bas.offset((8 as libc::c_int * i_sh + 1 as libc::c_int) as isize);
    (*envs).j_l = 0 as libc::c_int;
    (*envs).k_l = *bas.offset((8 as libc::c_int * k_sh + 1 as libc::c_int) as isize);
    (*envs).l_l = 0 as libc::c_int;
    (*envs)
        .x_ctr[0 as libc::c_int
        as usize] = *bas.offset((8 as libc::c_int * i_sh + 3 as libc::c_int) as isize);
    (*envs)
        .x_ctr[1 as libc::c_int
        as usize] = *bas.offset((8 as libc::c_int * k_sh + 3 as libc::c_int) as isize);
    (*envs).x_ctr[2 as libc::c_int as usize] = 1 as libc::c_int;
    (*envs).x_ctr[3 as libc::c_int as usize] = 1 as libc::c_int;
    (*envs)
        .nfi = ((*envs).i_l + 1 as libc::c_int) * ((*envs).i_l + 2 as libc::c_int)
        / 2 as libc::c_int;
    (*envs).nfj = 1 as libc::c_int;
    (*envs)
        .c2rust_unnamed
        .nfk = ((*envs).k_l + 1 as libc::c_int) * ((*envs).k_l + 2 as libc::c_int)
        / 2 as libc::c_int;
    (*envs).c2rust_unnamed_0.nfl = 1 as libc::c_int;
    (*envs).nf = (*envs).nfi * (*envs).c2rust_unnamed.nfk;
    (*envs)
        .ri = env
        .offset(
            *atm
                .offset(
                    (6 as libc::c_int
                        * *bas
                            .offset(
                                (8 as libc::c_int * i_sh + 0 as libc::c_int) as isize,
                            ) + 1 as libc::c_int) as isize,
                ) as isize,
        );
    (*envs)
        .rk = env
        .offset(
            *atm
                .offset(
                    (6 as libc::c_int
                        * *bas
                            .offset(
                                (8 as libc::c_int * k_sh + 0 as libc::c_int) as isize,
                            ) + 1 as libc::c_int) as isize,
                ) as isize,
        );
    (*envs)
        .common_factor = 3.14159265358979323846f64 * 3.14159265358979323846f64
        * 3.14159265358979323846f64 * 2 as libc::c_int as libc::c_double
        / 1.7724538509055160272981674833411451f64 * CINTcommon_fac_sp((*envs).i_l)
        * CINTcommon_fac_sp((*envs).k_l);
    if *env.offset(0 as libc::c_int as isize) == 0 as libc::c_int as libc::c_double {
        (*envs).expcutoff = 60 as libc::c_int as libc::c_double;
    } else {
        (*envs)
            .expcutoff = if 40 as libc::c_int as libc::c_double
            > *env.offset(0 as libc::c_int as isize)
        {
            40 as libc::c_int as libc::c_double
        } else {
            *env.offset(0 as libc::c_int as isize)
        };
    }
    (*envs).gbits = *ng.offset(4 as libc::c_int as isize);
    (*envs).ncomp_e1 = *ng.offset(5 as libc::c_int as isize);
    (*envs).ncomp_e2 = *ng.offset(6 as libc::c_int as isize);
    (*envs).ncomp_tensor = *ng.offset(7 as libc::c_int as isize);
    (*envs).li_ceil = (*envs).i_l + *ng.offset(0 as libc::c_int as isize);
    (*envs).lj_ceil = 0 as libc::c_int;
    (*envs).lk_ceil = (*envs).k_l + *ng.offset(2 as libc::c_int as isize);
    (*envs).ll_ceil = 0 as libc::c_int;
    let mut rys_order: libc::c_int = ((*envs).li_ceil + (*envs).lk_ceil)
        / 2 as libc::c_int + 1 as libc::c_int;
    let mut nrys_roots: libc::c_int = rys_order;
    let mut omega: libc::c_double = *env.offset(8 as libc::c_int as isize);
    if omega < 0 as libc::c_int as libc::c_double && rys_order <= 3 as libc::c_int {
        nrys_roots *= 2 as libc::c_int;
    }
    (*envs).rys_order = rys_order;
    (*envs).nrys_roots = nrys_roots;
    let mut dli: libc::c_int = (*envs).li_ceil + 1 as libc::c_int;
    let mut dlk: libc::c_int = (*envs).lk_ceil + 1 as libc::c_int;
    (*envs).g_stride_i = nrys_roots;
    (*envs).g_stride_k = nrys_roots * dli;
    (*envs).g_stride_l = (*envs).g_stride_k;
    (*envs).g_size = nrys_roots * dli * dlk;
    (*envs).aj[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
    (*envs).al[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
    (*envs)
        .rij[0 as libc::c_int
        as usize] = *((*envs).ri).offset(0 as libc::c_int as isize);
    (*envs)
        .rij[1 as libc::c_int
        as usize] = *((*envs).ri).offset(1 as libc::c_int as isize);
    (*envs)
        .rij[2 as libc::c_int
        as usize] = *((*envs).ri).offset(2 as libc::c_int as isize);
    (*envs)
        .rkl[0 as libc::c_int
        as usize] = *((*envs).rk).offset(0 as libc::c_int as isize);
    (*envs)
        .rkl[1 as libc::c_int
        as usize] = *((*envs).rk).offset(1 as libc::c_int as isize);
    (*envs)
        .rkl[2 as libc::c_int
        as usize] = *((*envs).rk).offset(2 as libc::c_int as isize);
    (*envs).g2d_ijmax = (*envs).g_stride_i;
    (*envs).g2d_klmax = (*envs).g_stride_k;
    (*envs)
        .rkrl[0 as libc::c_int
        as usize] = *((*envs).rk).offset(0 as libc::c_int as isize);
    (*envs)
        .rkrl[1 as libc::c_int
        as usize] = *((*envs).rk).offset(1 as libc::c_int as isize);
    (*envs)
        .rkrl[2 as libc::c_int
        as usize] = *((*envs).rk).offset(2 as libc::c_int as isize);
    (*envs)
        .rirj[0 as libc::c_int
        as usize] = *((*envs).ri).offset(0 as libc::c_int as isize);
    (*envs)
        .rirj[1 as libc::c_int
        as usize] = *((*envs).ri).offset(1 as libc::c_int as isize);
    (*envs)
        .rirj[2 as libc::c_int
        as usize] = *((*envs).ri).offset(2 as libc::c_int as isize);
    (*envs).rx_in_rklrx = (*envs).rk;
    (*envs).rx_in_rijrx = (*envs).ri;
    if rys_order <= 2 as libc::c_int {
        (*envs)
            .f_g0_2d4d = ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut Rys2eT,
                    *mut CINTEnvVars,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg0_2e_2d4d_unrolled
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut Rys2eT,
                        *mut CINTEnvVars,
                    ) -> (),
            ),
        );
        if rys_order != nrys_roots {
            (*envs)
                .f_g0_2d4d = ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut Rys2eT,
                        *mut CINTEnvVars,
                    ) -> (),
                >,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    CINTsrg0_2e_2d4d_unrolled
                        as unsafe extern "C" fn(
                            *mut libc::c_double,
                            *mut Rys2eT,
                            *mut CINTEnvVars,
                        ) -> (),
                ),
            );
        }
    } else {
        (*envs)
            .f_g0_2d4d = ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut Rys2eT,
                    *mut CINTEnvVars,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg0_2e_2d
                    as unsafe extern "C" fn(
                        *mut libc::c_double,
                        *mut Rys2eT,
                        *mut CINTEnvVars,
                    ) -> (),
            ),
        );
    }
    (*envs)
        .f_g0_2e = ::core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut libc::c_double,
                *mut libc::c_double,
                *mut libc::c_double,
                libc::c_double,
                *mut CINTEnvVars,
            ) -> libc::c_int,
        >,
        Option::<unsafe extern "C" fn() -> libc::c_int>,
    >(
        Some(
            CINTg0_2e
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                    *mut libc::c_double,
                    libc::c_double,
                    *mut CINTEnvVars,
                ) -> libc::c_int,
        ),
    );
    (*envs).j_l = (*envs).k_l;
    (*envs).nfj = (*envs).c2rust_unnamed.nfk;
    (*envs).g_stride_j = (*envs).g_stride_k;
}
