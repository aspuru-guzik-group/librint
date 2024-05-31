#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::rys_roots::CINTrys_roots;

extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn CINTcart_comp(
        nx: *mut libc::c_int,
        ny: *mut libc::c_int,
        nz: *mut libc::c_int,
        lmax: libc::c_int,
    );
    fn abs(_: libc::c_int) -> libc::c_int;
}


fn MAX<T: PartialOrd>(x: T, y: T) -> T {
    if x > y {
        x
    } else {
        y
    }
}

fn SQUARE(r: *mut f64) -> f64 {
    unsafe {
        (*r.add(0) * *r.add(0)) + (*r.add(1) * *r.add(1)) + (*r.add(2) * *r.add(2))
    }
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
#[no_mangle]
pub unsafe extern "C" fn CINTinit_int1e_EnvVars(
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
    let j_sh: libc::c_int = *shls.offset(1 as libc::c_int as isize);
    (*envs).i_l = *bas.offset((8 as libc::c_int * i_sh + 1 as libc::c_int) as isize);
    (*envs).j_l = *bas.offset((8 as libc::c_int * j_sh + 1 as libc::c_int) as isize);
    (*envs)
        .x_ctr[0 as libc::c_int
        as usize] = *bas.offset((8 as libc::c_int * i_sh + 3 as libc::c_int) as isize);
    (*envs)
        .x_ctr[1 as libc::c_int
        as usize] = *bas.offset((8 as libc::c_int * j_sh + 3 as libc::c_int) as isize);
    (*envs)
        .nfi = ((*envs).i_l + 1 as libc::c_int) * ((*envs).i_l + 2 as libc::c_int)
        / 2 as libc::c_int;
    (*envs)
        .nfj = ((*envs).j_l + 1 as libc::c_int) * ((*envs).j_l + 2 as libc::c_int)
        / 2 as libc::c_int;
    (*envs).nf = (*envs).nfi * (*envs).nfj;
    (*envs).common_factor = 1 as libc::c_int as libc::c_double;
    if *env.offset(0 as libc::c_int as isize) == 0 as libc::c_int as libc::c_double {
        (*envs).expcutoff = 60 as libc::c_int as libc::c_double;
    } else {
        (*envs)
            .expcutoff = MAX(40 as f64, *env.offset(0 as libc::c_int as isize))
            as libc::c_double;
    }
    (*envs).li_ceil = (*envs).i_l + *ng.offset(0 as libc::c_int as isize);
    (*envs).lj_ceil = (*envs).j_l + *ng.offset(1 as libc::c_int as isize);
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
        .rj = env
        .offset(
            *atm
                .offset(
                    (6 as libc::c_int
                        * *bas
                            .offset(
                                (8 as libc::c_int * j_sh + 0 as libc::c_int) as isize,
                            ) + 1 as libc::c_int) as isize,
                ) as isize,
        );
    (*envs).gbits = *ng.offset(4 as libc::c_int as isize);
    (*envs).ncomp_e1 = *ng.offset(5 as libc::c_int as isize);
    (*envs).ncomp_tensor = *ng.offset(7 as libc::c_int as isize);
    if *ng.offset(6 as libc::c_int as isize) > 0 as libc::c_int {
        (*envs).nrys_roots = *ng.offset(6 as libc::c_int as isize);
    } else {
        (*envs)
            .nrys_roots = ((*envs).li_ceil + (*envs).lj_ceil) / 2 as libc::c_int
            + 1 as libc::c_int;
    }
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
    (*envs).g_stride_i = (*envs).nrys_roots;
    (*envs).g_stride_j = (*envs).nrys_roots * dli;
    (*envs).g_size = (*envs).nrys_roots * dli * dlj;
    (*envs).g_stride_k = (*envs).g_size;
    (*envs).g_stride_l = (*envs).g_size;
}
#[no_mangle]
pub unsafe extern "C" fn CINTg1e_index_xyz(
    mut idx: *mut libc::c_int,
    mut envs: *mut CINTEnvVars,
) {
    let i_l: libc::c_int = (*envs).i_l;
    let j_l: libc::c_int = (*envs).j_l;
    let nfi: libc::c_int = (*envs).nfi;
    let nfj: libc::c_int = (*envs).nfj;
    let di: libc::c_int = (*envs).g_stride_i;
    let dj: libc::c_int = (*envs).g_stride_j;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut ofx: libc::c_int = 0;
    let mut ofjx: libc::c_int = 0;
    let mut ofy: libc::c_int = 0;
    let mut ofjy: libc::c_int = 0;
    let mut ofz: libc::c_int = 0;
    let mut ofjz: libc::c_int = 0;
    let mut i_nx: [libc::c_int; 136] = [0; 136];
    let mut i_ny: [libc::c_int; 136] = [0; 136];
    let mut i_nz: [libc::c_int; 136] = [0; 136];
    let mut j_nx: [libc::c_int; 136] = [0; 136];
    let mut j_ny: [libc::c_int; 136] = [0; 136];
    let mut j_nz: [libc::c_int; 136] = [0; 136];
    CINTcart_comp(i_nx.as_mut_ptr(), i_ny.as_mut_ptr(), i_nz.as_mut_ptr(), i_l);
    CINTcart_comp(j_nx.as_mut_ptr(), j_ny.as_mut_ptr(), j_nz.as_mut_ptr(), j_l);
    ofx = 0 as libc::c_int;
    ofy = (*envs).g_size;
    ofz = (*envs).g_size * 2 as libc::c_int;
    n = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while j < nfj {
        ofjx = ofx + dj * j_nx[j as usize];
        ofjy = ofy + dj * j_ny[j as usize];
        ofjz = ofz + dj * j_nz[j as usize];
        i = 0 as libc::c_int;
        while i < nfi {
            *idx.offset((n + 0 as libc::c_int) as isize) = ofjx + di * i_nx[i as usize];
            *idx.offset((n + 1 as libc::c_int) as isize) = ofjy + di * i_ny[i as usize];
            *idx.offset((n + 2 as libc::c_int) as isize) = ofjz + di * i_nz[i as usize];
            n += 3 as libc::c_int;
            i += 1;
            i;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTg1e_ovlp(
    mut g: *mut libc::c_double,
    mut envs: *mut CINTEnvVars,
) -> libc::c_int {
    let mut gx: *mut libc::c_double = g;
    let mut gy: *mut libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *mut libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut aij: libc::c_double = (*envs).ai[0 as libc::c_int as usize]
        + (*envs).aj[0 as libc::c_int as usize];
    *gx.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *gy.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *gz
        .offset(
            0 as libc::c_int as isize,
        ) = (*envs).fac[0 as libc::c_int as usize]
        * 1.7724538509055160272981674833411451f64 * 3.14159265358979323846f64
        / (aij * sqrt(aij));
    let mut nmax: libc::c_int = (*envs).li_ceil + (*envs).lj_ceil;
    if nmax == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    let mut rij: *mut libc::c_double = ((*envs).rij).as_mut_ptr();
    let mut rirj: *mut libc::c_double = ((*envs).rirj).as_mut_ptr();
    let mut lj: libc::c_int = 0;
    let mut di: libc::c_int = 0;
    let mut dj: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
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
    *gx
        .offset(
            di as isize,
        ) = rijrx[0 as libc::c_int as usize] * *gx.offset(0 as libc::c_int as isize);
    *gy
        .offset(
            di as isize,
        ) = rijrx[1 as libc::c_int as usize] * *gy.offset(0 as libc::c_int as isize);
    *gz
        .offset(
            di as isize,
        ) = rijrx[2 as libc::c_int as usize] * *gz.offset(0 as libc::c_int as isize);
    let mut aij2: libc::c_double = 0.5f64 / aij;
    i = 1 as libc::c_int;
    while i < nmax {
        *gx
            .offset(
                ((i + 1 as libc::c_int) * di) as isize,
            ) = i as libc::c_double * aij2
            * *gx.offset(((i - 1 as libc::c_int) * di) as isize)
            + rijrx[0 as libc::c_int as usize] * *gx.offset((i * di) as isize);
        *gy
            .offset(
                ((i + 1 as libc::c_int) * di) as isize,
            ) = i as libc::c_double * aij2
            * *gy.offset(((i - 1 as libc::c_int) * di) as isize)
            + rijrx[1 as libc::c_int as usize] * *gy.offset((i * di) as isize);
        *gz
            .offset(
                ((i + 1 as libc::c_int) * di) as isize,
            ) = i as libc::c_double * aij2
            * *gz.offset(((i - 1 as libc::c_int) * di) as isize)
            + rijrx[2 as libc::c_int as usize] * *gz.offset((i * di) as isize);
        i += 1;
        i;
    }
    j = 1 as libc::c_int;
    while j <= lj {
        ptr = dj * j;
        i = 0 as libc::c_int;
        n = ptr;
        while i <= nmax - j {
            *gx
                .offset(
                    n as isize,
                ) = *gx.offset((n + di - dj) as isize)
                + *rirj.offset(0 as libc::c_int as isize)
                    * *gx.offset((n - dj) as isize);
            *gy
                .offset(
                    n as isize,
                ) = *gy.offset((n + di - dj) as isize)
                + *rirj.offset(1 as libc::c_int as isize)
                    * *gy.offset((n - dj) as isize);
            *gz
                .offset(
                    n as isize,
                ) = *gz.offset((n + di - dj) as isize)
                + *rirj.offset(2 as libc::c_int as isize)
                    * *gz.offset((n - dj) as isize);
            i += 1;
            i;
            n += di;
        }
        j += 1;
        j;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINTnuc_mod(
    mut aij: libc::c_double,
    mut nuc_id: libc::c_int,
    mut atm: *mut libc::c_int,
    mut env: *mut libc::c_double,
) -> libc::c_double {
    let mut zeta: libc::c_double = 0.;
    if nuc_id < 0 as libc::c_int {
        zeta = *env.offset(7 as libc::c_int as isize);
    } else if *atm.offset((6 as libc::c_int * nuc_id + 2 as libc::c_int) as isize)
        == 2 as libc::c_int
    {
        zeta = *env
            .offset(
                *atm.offset((6 as libc::c_int * nuc_id + 3 as libc::c_int) as isize)
                    as isize,
            );
    } else {
        zeta = 0 as libc::c_int as libc::c_double;
    }
    if zeta > 0 as libc::c_int as libc::c_double {
        return sqrt(zeta / (aij + zeta))
    } else {
        return 1 as libc::c_int as libc::c_double
    };
}
#[no_mangle]
pub unsafe extern "C" fn CINTg1e_nuc(
    mut g: *mut libc::c_double,
    mut envs: *mut CINTEnvVars,
    mut nuc_id: libc::c_int,
) -> libc::c_int {
    let mut nrys_roots: libc::c_int = (*envs).nrys_roots;
    let mut atm: *mut libc::c_int = (*envs).atm;
    let mut env: *mut libc::c_double = (*envs).env;
    let mut rij: *mut libc::c_double = ((*envs).rij).as_mut_ptr();
    let mut gx: *mut libc::c_double = g;
    let mut gy: *mut libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *mut libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut u: [libc::c_double; 32] = [0.; 32];
    let mut w: *mut libc::c_double = gz;
    let mut cr: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut crij: [libc::c_double; 3] = [0.; 3];
    let mut x: libc::c_double = 0.;
    let mut fac1: libc::c_double = 0.;
    let mut aij: libc::c_double = (*envs).ai[0 as libc::c_int as usize]
        + (*envs).aj[0 as libc::c_int as usize];
    let mut tau: libc::c_double = CINTnuc_mod(aij, nuc_id, atm, env);
    if nuc_id < 0 as libc::c_int {
        fac1 = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
            * (*envs).fac[0 as libc::c_int as usize] * tau / aij;
        cr = env.offset(4 as libc::c_int as isize);
    } else if *atm.offset((6 as libc::c_int * nuc_id + 2 as libc::c_int) as isize)
        == 3 as libc::c_int
    {
        fac1 = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
            * -*env
                .offset(
                    *atm.offset((4 as libc::c_int + nuc_id * 6 as libc::c_int) as isize)
                        as isize,
                ) * (*envs).fac[0 as libc::c_int as usize] * tau / aij;
        cr = env
            .offset(
                *atm.offset((6 as libc::c_int * nuc_id + 1 as libc::c_int) as isize)
                    as isize,
            );
    } else {
        fac1 = 2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
            * -abs(*atm.offset((0 as libc::c_int + nuc_id * 6 as libc::c_int) as isize))
                as libc::c_double * (*envs).fac[0 as libc::c_int as usize] * tau / aij;
        cr = env
            .offset(
                *atm.offset((6 as libc::c_int * nuc_id + 1 as libc::c_int) as isize)
                    as isize,
            );
    }
    crij[0 as libc::c_int
        as usize] = *cr.offset(0 as libc::c_int as isize)
        - *rij.offset(0 as libc::c_int as isize);
    crij[1 as libc::c_int
        as usize] = *cr.offset(1 as libc::c_int as isize)
        - *rij.offset(1 as libc::c_int as isize);
    crij[2 as libc::c_int
        as usize] = *cr.offset(2 as libc::c_int as isize)
        - *rij.offset(2 as libc::c_int as isize);
    x = aij * tau * tau * SQUARE(crij.as_mut_ptr()) as libc::c_double;
    CINTrys_roots(nrys_roots, x, u.as_mut_ptr(), w);
    i = 0 as libc::c_int;
    while i < nrys_roots {
        *gx.offset(i as isize) = 1 as libc::c_int as libc::c_double;
        *gy.offset(i as isize) = 1 as libc::c_int as libc::c_double;
        *gz.offset(i as isize) *= fac1;
        i += 1;
        i;
    }
    let mut nmax: libc::c_int = (*envs).li_ceil + (*envs).lj_ceil;
    if nmax == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    let mut p0x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p0y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p0z: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p1x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p1y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p1z: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p2x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p2y: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut p2z: *mut libc::c_double = 0 as *mut libc::c_double;
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
    let mut rijrx: libc::c_double = *rij.offset(0 as libc::c_int as isize)
        - *rx.offset(0 as libc::c_int as isize);
    let mut rijry: libc::c_double = *rij.offset(1 as libc::c_int as isize)
        - *rx.offset(1 as libc::c_int as isize);
    let mut rijrz: libc::c_double = *rij.offset(2 as libc::c_int as isize)
        - *rx.offset(2 as libc::c_int as isize);
    let mut aij2: libc::c_double = 0.5f64 / aij;
    let mut ru: libc::c_double = 0.;
    let mut rt: libc::c_double = 0.;
    let mut r0: libc::c_double = 0.;
    let mut r1: libc::c_double = 0.;
    let mut r2: libc::c_double = 0.;
    p0x = gx.offset(di as isize);
    p0y = gy.offset(di as isize);
    p0z = gz.offset(di as isize);
    p1x = gx.offset(-(di as isize));
    p1y = gy.offset(-(di as isize));
    p1z = gz.offset(-(di as isize));
    n = 0 as libc::c_int;
    while n < nrys_roots {
        ru = tau * tau * u[n as usize]
            / (1 as libc::c_int as libc::c_double + u[n as usize]);
        rt = aij2 - aij2 * ru;
        r0 = rijrx + ru * crij[0 as libc::c_int as usize];
        r1 = rijry + ru * crij[1 as libc::c_int as usize];
        r2 = rijrz + ru * crij[2 as libc::c_int as usize];
        *p0x.offset(n as isize) = r0 * *gx.offset(n as isize);
        *p0y.offset(n as isize) = r1 * *gy.offset(n as isize);
        *p0z.offset(n as isize) = r2 * *gz.offset(n as isize);
        i = 1 as libc::c_int;
        while i < nmax {
            *p0x
                .offset(
                    (n + i * di) as isize,
                ) = i as libc::c_double * rt * *p1x.offset((n + i * di) as isize)
                + r0 * *gx.offset((n + i * di) as isize);
            *p0y
                .offset(
                    (n + i * di) as isize,
                ) = i as libc::c_double * rt * *p1y.offset((n + i * di) as isize)
                + r1 * *gy.offset((n + i * di) as isize);
            *p0z
                .offset(
                    (n + i * di) as isize,
                ) = i as libc::c_double * rt * *p1z.offset((n + i * di) as isize)
                + r2 * *gz.offset((n + i * di) as isize);
            i += 1;
            i;
        }
        n += 1;
        n;
    }
    let mut rirjx: libc::c_double = (*envs).rirj[0 as libc::c_int as usize];
    let mut rirjy: libc::c_double = (*envs).rirj[1 as libc::c_int as usize];
    let mut rirjz: libc::c_double = (*envs).rirj[2 as libc::c_int as usize];
    j = 1 as libc::c_int;
    while j <= lj {
        p0x = gx.offset((j * dj) as isize);
        p0y = gy.offset((j * dj) as isize);
        p0z = gz.offset((j * dj) as isize);
        p1x = p0x.offset(-(dj as isize));
        p1y = p0y.offset(-(dj as isize));
        p1z = p0z.offset(-(dj as isize));
        p2x = p1x.offset(di as isize);
        p2y = p1y.offset(di as isize);
        p2z = p1z.offset(di as isize);
        i = 0 as libc::c_int;
        while i <= nmax - j {
            n = 0 as libc::c_int;
            while n < nrys_roots {
                *p0x
                    .offset(
                        (n + i * di) as isize,
                    ) = *p2x.offset((n + i * di) as isize)
                    + rirjx * *p1x.offset((n + i * di) as isize);
                *p0y
                    .offset(
                        (n + i * di) as isize,
                    ) = *p2y.offset((n + i * di) as isize)
                    + rirjy * *p1y.offset((n + i * di) as isize);
                *p0z
                    .offset(
                        (n + i * di) as isize,
                    ) = *p2z.offset((n + i * di) as isize)
                    + rirjz * *p1z.offset((n + i * di) as isize);
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
pub unsafe extern "C" fn CINTnabla1i_1e(
    mut f: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut li: libc::c_int,
    mut lj: libc::c_int,
    mut lk: libc::c_int,
    mut envs: *mut CINTEnvVars,
) {
    let dj: libc::c_int = (*envs).g_stride_j;
    let dk: libc::c_int = (*envs).g_stride_k;
    let ai2: libc::c_double = -(2 as libc::c_int) as libc::c_double
        * (*envs).ai[0 as libc::c_int as usize];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut gx: *const libc::c_double = g;
    let mut gy: *const libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut fx: *mut libc::c_double = f;
    let mut fy: *mut libc::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut libc::c_double = f
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    k = 0 as libc::c_int;
    while k <= lk {
        j = 0 as libc::c_int;
        while j <= lj {
            ptr = dj * j + dk * k;
            *fx
                .offset(
                    ptr as isize,
                ) = ai2 * *gx.offset((ptr + 1 as libc::c_int) as isize);
            *fy
                .offset(
                    ptr as isize,
                ) = ai2 * *gy.offset((ptr + 1 as libc::c_int) as isize);
            *fz
                .offset(
                    ptr as isize,
                ) = ai2 * *gz.offset((ptr + 1 as libc::c_int) as isize);
            i = 1 as libc::c_int;
            while i <= li {
                *fx
                    .offset(
                        (ptr + i) as isize,
                    ) = i as libc::c_double
                    * *gx.offset((ptr + i - 1 as libc::c_int) as isize)
                    + ai2 * *gx.offset((ptr + i + 1 as libc::c_int) as isize);
                *fy
                    .offset(
                        (ptr + i) as isize,
                    ) = i as libc::c_double
                    * *gy.offset((ptr + i - 1 as libc::c_int) as isize)
                    + ai2 * *gy.offset((ptr + i + 1 as libc::c_int) as isize);
                *fz
                    .offset(
                        (ptr + i) as isize,
                    ) = i as libc::c_double
                    * *gz.offset((ptr + i - 1 as libc::c_int) as isize)
                    + ai2 * *gz.offset((ptr + i + 1 as libc::c_int) as isize);
                i += 1;
                i;
            }
            j += 1;
            j;
        }
        k += 1;
        k;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1j_1e(
    mut f: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut li: libc::c_int,
    mut lj: libc::c_int,
    mut lk: libc::c_int,
    mut envs: *mut CINTEnvVars,
) {
    let dj: libc::c_int = (*envs).g_stride_j;
    let dk: libc::c_int = (*envs).g_stride_k;
    let aj2: libc::c_double = -(2 as libc::c_int) as libc::c_double
        * (*envs).aj[0 as libc::c_int as usize];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut gx: *const libc::c_double = g;
    let mut gy: *const libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut fx: *mut libc::c_double = f;
    let mut fy: *mut libc::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut libc::c_double = f
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    k = 0 as libc::c_int;
    while k <= lk {
        ptr = dk * k;
        i = ptr;
        while i <= ptr + li {
            *fx.offset(i as isize) = aj2 * *gx.offset((i + dj) as isize);
            *fy.offset(i as isize) = aj2 * *gy.offset((i + dj) as isize);
            *fz.offset(i as isize) = aj2 * *gz.offset((i + dj) as isize);
            i += 1;
            i;
        }
        j = 1 as libc::c_int;
        while j <= lj {
            ptr = dj * j + dk * k;
            i = ptr;
            while i <= ptr + li {
                *fx
                    .offset(
                        i as isize,
                    ) = j as libc::c_double * *gx.offset((i - dj) as isize)
                    + aj2 * *gx.offset((i + dj) as isize);
                *fy
                    .offset(
                        i as isize,
                    ) = j as libc::c_double * *gy.offset((i - dj) as isize)
                    + aj2 * *gy.offset((i + dj) as isize);
                *fz
                    .offset(
                        i as isize,
                    ) = j as libc::c_double * *gz.offset((i - dj) as isize)
                    + aj2 * *gz.offset((i + dj) as isize);
                i += 1;
                i;
            }
            j += 1;
            j;
        }
        k += 1;
        k;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1k_1e(
    mut f: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut li: libc::c_int,
    mut lj: libc::c_int,
    mut lk: libc::c_int,
    mut envs: *mut CINTEnvVars,
) {
    let dj: libc::c_int = (*envs).g_stride_j;
    let dk: libc::c_int = (*envs).g_stride_k;
    let ak2: libc::c_double = -(2 as libc::c_int) as libc::c_double
        * (*envs).ak[0 as libc::c_int as usize];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
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
        ptr = dj * j;
        i = ptr;
        while i <= ptr + li {
            *fx.offset(i as isize) = ak2 * *gx.offset((i + dk) as isize);
            *fy.offset(i as isize) = ak2 * *gy.offset((i + dk) as isize);
            *fz.offset(i as isize) = ak2 * *gz.offset((i + dk) as isize);
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    k = 1 as libc::c_int;
    while k <= lk {
        j = 0 as libc::c_int;
        while j <= lj {
            ptr = dj * j + dk * k;
            i = ptr;
            while i <= ptr + li {
                *fx
                    .offset(
                        i as isize,
                    ) = k as libc::c_double * *gx.offset((i - dk) as isize)
                    + ak2 * *gx.offset((i + dk) as isize);
                *fy
                    .offset(
                        i as isize,
                    ) = k as libc::c_double * *gy.offset((i - dk) as isize)
                    + ak2 * *gy.offset((i + dk) as isize);
                *fz
                    .offset(
                        i as isize,
                    ) = k as libc::c_double * *gz.offset((i - dk) as isize)
                    + ak2 * *gz.offset((i + dk) as isize);
                i += 1;
                i;
            }
            j += 1;
            j;
        }
        k += 1;
        k;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1i_1e(
    mut f: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut ri: *mut libc::c_double,
    mut li: libc::c_int,
    mut lj: libc::c_int,
    mut lk: libc::c_int,
    mut envs: *mut CINTEnvVars,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let dj: libc::c_int = (*envs).g_stride_j;
    let dk: libc::c_int = (*envs).g_stride_k;
    let mut gx: *const libc::c_double = g;
    let mut gy: *const libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut fx: *mut libc::c_double = f;
    let mut fy: *mut libc::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut libc::c_double = f
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    k = 0 as libc::c_int;
    while k <= lk {
        j = 0 as libc::c_int;
        while j <= lj {
            ptr = dj * j + dk * k;
            i = ptr;
            while i <= ptr + li {
                *fx
                    .offset(
                        i as isize,
                    ) = *gx.offset((i + 1 as libc::c_int) as isize)
                    + *ri.offset(0 as libc::c_int as isize) * *gx.offset(i as isize);
                *fy
                    .offset(
                        i as isize,
                    ) = *gy.offset((i + 1 as libc::c_int) as isize)
                    + *ri.offset(1 as libc::c_int as isize) * *gy.offset(i as isize);
                *fz
                    .offset(
                        i as isize,
                    ) = *gz.offset((i + 1 as libc::c_int) as isize)
                    + *ri.offset(2 as libc::c_int as isize) * *gz.offset(i as isize);
                i += 1;
                i;
            }
            j += 1;
            j;
        }
        k += 1;
        k;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1j_1e(
    mut f: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut rj: *mut libc::c_double,
    mut li: libc::c_int,
    mut lj: libc::c_int,
    mut lk: libc::c_int,
    mut envs: *mut CINTEnvVars,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let dj: libc::c_int = (*envs).g_stride_j;
    let dk: libc::c_int = (*envs).g_stride_k;
    let mut gx: *const libc::c_double = g;
    let mut gy: *const libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut fx: *mut libc::c_double = f;
    let mut fy: *mut libc::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut libc::c_double = f
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    k = 0 as libc::c_int;
    while k <= lk {
        j = 0 as libc::c_int;
        while j <= lj {
            ptr = dj * j + dk * k;
            i = ptr;
            while i <= ptr + li {
                *fx
                    .offset(
                        i as isize,
                    ) = *gx.offset((i + dj) as isize)
                    + *rj.offset(0 as libc::c_int as isize) * *gx.offset(i as isize);
                *fy
                    .offset(
                        i as isize,
                    ) = *gy.offset((i + dj) as isize)
                    + *rj.offset(1 as libc::c_int as isize) * *gy.offset(i as isize);
                *fz
                    .offset(
                        i as isize,
                    ) = *gz.offset((i + dj) as isize)
                    + *rj.offset(2 as libc::c_int as isize) * *gz.offset(i as isize);
                i += 1;
                i;
            }
            j += 1;
            j;
        }
        k += 1;
        k;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1k_1e(
    mut f: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut rk: *mut libc::c_double,
    mut li: libc::c_int,
    mut lj: libc::c_int,
    mut lk: libc::c_int,
    mut envs: *mut CINTEnvVars,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let dj: libc::c_int = (*envs).g_stride_j;
    let dk: libc::c_int = (*envs).g_stride_k;
    let mut gx: *const libc::c_double = g;
    let mut gy: *const libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    let mut fx: *mut libc::c_double = f;
    let mut fy: *mut libc::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut libc::c_double = f
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    k = 0 as libc::c_int;
    while k <= lk {
        j = 0 as libc::c_int;
        while j <= lj {
            ptr = dj * j + dk * k;
            i = ptr;
            while i <= ptr + li {
                *fx
                    .offset(
                        i as isize,
                    ) = *gx.offset((i + dk) as isize)
                    + *rk.offset(0 as libc::c_int as isize) * *gx.offset(i as isize);
                *fy
                    .offset(
                        i as isize,
                    ) = *gy.offset((i + dk) as isize)
                    + *rk.offset(1 as libc::c_int as isize) * *gy.offset(i as isize);
                *fz
                    .offset(
                        i as isize,
                    ) = *gz.offset((i + dk) as isize)
                    + *rk.offset(2 as libc::c_int as isize) * *gz.offset(i as isize);
                i += 1;
                i;
            }
            j += 1;
            j;
        }
        k += 1;
        k;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTprim_to_ctr(
    mut gc: *mut libc::c_double,
    mut nf: libc::c_int,
    mut gp: *mut libc::c_double,
    mut inc: libc::c_int,
    mut nprim: libc::c_int,
    mut nctr: libc::c_int,
    mut coeff: *mut libc::c_double,
) {
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut pgc: *mut libc::c_double = gc;
    let mut c: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < inc {
        n = 0 as libc::c_int;
        while n < nctr {
            c = *coeff.offset((nprim * n) as isize);
            if c != 0 as libc::c_int as libc::c_double {
                k = 0 as libc::c_int;
                while k < nf {
                    *pgc.offset(k as isize) += c * *gp.offset((k * inc + i) as isize);
                    k += 1;
                    k;
                }
            }
            pgc = pgc.offset(nf as isize);
            n += 1;
            n;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTprim_to_ctr_0(
    mut gc: *mut libc::c_double,
    mut gp: *mut libc::c_double,
    mut coeff: *mut libc::c_double,
    mut nf: size_t,
    mut nprim: libc::c_int,
    mut nctr: libc::c_int,
    mut non0ctr: libc::c_int,
    mut sortedidx: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut n: size_t = 0;
    let mut c0: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < nctr {
        c0 = *coeff.offset((nprim * i) as isize);
        n = 0 as libc::c_int as size_t;
        while n < nf {
            *gc
                .offset(
                    nf.wrapping_mul(i as libc::c_ulong).wrapping_add(n) as isize,
                ) = c0 * *gp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTprim_to_ctr_1(
    mut gc: *mut libc::c_double,
    mut gp: *mut libc::c_double,
    mut coeff: *mut libc::c_double,
    mut nf: size_t,
    mut nprim: libc::c_int,
    mut nctr: libc::c_int,
    mut non0ctr: libc::c_int,
    mut sortedidx: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: size_t = 0;
    let mut c0: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < non0ctr {
        c0 = *coeff.offset((nprim * *sortedidx.offset(i as isize)) as isize);
        j = *sortedidx.offset(i as isize);
        n = 0 as libc::c_int as size_t;
        while n < nf {
            *gc.offset(nf.wrapping_mul(j as libc::c_ulong).wrapping_add(n) as isize)
                += c0 * *gp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTcommon_fac_sp(mut l: libc::c_int) -> libc::c_double {
    match l {
        0 => return 0.282094791773878143f64,
        1 => return 0.488602511902919921f64,
        _ => return 1 as libc::c_int as libc::c_double,
    };
}
