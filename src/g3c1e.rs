#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn CINTcart_comp(
        nx: *mut libc::c_int,
        ny: *mut libc::c_int,
        nz: *mut libc::c_int,
        lmax: libc::c_int,
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
#[no_mangle]
pub unsafe extern "C" fn CINTinit_int3c1e_EnvVars(
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
    let k_sh: libc::c_int = *shls.offset(2 as libc::c_int as isize);
    (*envs).i_l = *bas.offset((8 as libc::c_int * i_sh + 1 as libc::c_int) as isize);
    (*envs).j_l = *bas.offset((8 as libc::c_int * j_sh + 1 as libc::c_int) as isize);
    (*envs).k_l = *bas.offset((8 as libc::c_int * k_sh + 1 as libc::c_int) as isize);
    (*envs).l_l = 0 as libc::c_int;
    (*envs)
        .x_ctr[0 as libc::c_int
        as usize] = *bas.offset((8 as libc::c_int * i_sh + 3 as libc::c_int) as isize);
    (*envs)
        .x_ctr[1 as libc::c_int
        as usize] = *bas.offset((8 as libc::c_int * j_sh + 3 as libc::c_int) as isize);
    (*envs)
        .x_ctr[2 as libc::c_int
        as usize] = *bas.offset((8 as libc::c_int * k_sh + 3 as libc::c_int) as isize);
    (*envs).x_ctr[3 as libc::c_int as usize] = 1 as libc::c_int;
    (*envs)
        .nfi = ((*envs).i_l + 1 as libc::c_int) * ((*envs).i_l + 2 as libc::c_int)
        / 2 as libc::c_int;
    (*envs)
        .nfj = ((*envs).j_l + 1 as libc::c_int) * ((*envs).j_l + 2 as libc::c_int)
        / 2 as libc::c_int;
    (*envs)
        .c2rust_unnamed
        .nfk = ((*envs).k_l + 1 as libc::c_int) * ((*envs).k_l + 2 as libc::c_int)
        / 2 as libc::c_int;
    (*envs).c2rust_unnamed_0.nfl = 1 as libc::c_int;
    (*envs).nf = (*envs).nfi * (*envs).nfj * (*envs).c2rust_unnamed.nfk;
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
    (*envs).gbits = *ng.offset(4 as libc::c_int as isize);
    (*envs).ncomp_e1 = *ng.offset(5 as libc::c_int as isize);
    (*envs).ncomp_e2 = 0 as libc::c_int;
    (*envs).ncomp_tensor = *ng.offset(7 as libc::c_int as isize);
    (*envs).li_ceil = (*envs).i_l + *ng.offset(0 as libc::c_int as isize);
    (*envs).lj_ceil = (*envs).j_l + *ng.offset(1 as libc::c_int as isize);
    (*envs).lk_ceil = (*envs).k_l + *ng.offset(2 as libc::c_int as isize);
    (*envs).ll_ceil = 0 as libc::c_int;
    (*envs)
        .nrys_roots = ((*envs).li_ceil + (*envs).lj_ceil + (*envs).lk_ceil)
        / 2 as libc::c_int + 1 as libc::c_int;
    (*envs)
        .common_factor = 1.7724538509055160272981674833411451f64
        * 3.14159265358979323846f64 * CINTcommon_fac_sp((*envs).i_l)
        * CINTcommon_fac_sp((*envs).j_l) * CINTcommon_fac_sp((*envs).k_l);
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
    let mut dli: libc::c_int = (*envs).li_ceil + 1 as libc::c_int;
    let mut dlj: libc::c_int = (*envs).lj_ceil + (*envs).lk_ceil + 1 as libc::c_int;
    let mut dlk: libc::c_int = (*envs).lk_ceil + 1 as libc::c_int;
    (*envs).g_stride_i = 1 as libc::c_int;
    (*envs).g_stride_j = dli;
    (*envs).g_stride_k = dli * dlj;
    (*envs).g_stride_l = (*envs).g_stride_k;
    let mut nmax: libc::c_int = (*envs).li_ceil + dlj;
    (*envs)
        .g_size = if dli * dlj * dlk > dli * nmax {
        dli * dlj * dlk
    } else {
        dli * nmax
    };
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
}
#[no_mangle]
pub unsafe extern "C" fn CINTg3c1e_index_xyz(
    mut idx: *mut libc::c_int,
    mut envs: *const CINTEnvVars,
) {
    let i_l: libc::c_int = (*envs).i_l;
    let j_l: libc::c_int = (*envs).j_l;
    let k_l: libc::c_int = (*envs).k_l;
    let nfi: libc::c_int = (*envs).nfi;
    let nfj: libc::c_int = (*envs).nfj;
    let nfk: libc::c_int = (*envs).c2rust_unnamed.nfk;
    let dj: libc::c_int = (*envs).g_stride_j;
    let dk: libc::c_int = (*envs).g_stride_k;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut ofx: libc::c_int = 0;
    let mut ofjx: libc::c_int = 0;
    let mut ofkx: libc::c_int = 0;
    let mut ofy: libc::c_int = 0;
    let mut ofjy: libc::c_int = 0;
    let mut ofky: libc::c_int = 0;
    let mut ofz: libc::c_int = 0;
    let mut ofjz: libc::c_int = 0;
    let mut ofkz: libc::c_int = 0;
    let mut i_nx: [libc::c_int; 136] = [0; 136];
    let mut i_ny: [libc::c_int; 136] = [0; 136];
    let mut i_nz: [libc::c_int; 136] = [0; 136];
    let mut j_nx: [libc::c_int; 136] = [0; 136];
    let mut j_ny: [libc::c_int; 136] = [0; 136];
    let mut j_nz: [libc::c_int; 136] = [0; 136];
    let mut k_nx: [libc::c_int; 136] = [0; 136];
    let mut k_ny: [libc::c_int; 136] = [0; 136];
    let mut k_nz: [libc::c_int; 136] = [0; 136];
    CINTcart_comp(i_nx.as_mut_ptr(), i_ny.as_mut_ptr(), i_nz.as_mut_ptr(), i_l);
    CINTcart_comp(j_nx.as_mut_ptr(), j_ny.as_mut_ptr(), j_nz.as_mut_ptr(), j_l);
    CINTcart_comp(k_nx.as_mut_ptr(), k_ny.as_mut_ptr(), k_nz.as_mut_ptr(), k_l);
    ofx = 0 as libc::c_int;
    ofy = (*envs).g_size;
    ofz = (*envs).g_size * 2 as libc::c_int;
    n = 0 as libc::c_int;
    k = 0 as libc::c_int;
    while k < nfk {
        ofkx = ofx + dk * k_nx[k as usize];
        ofky = ofy + dk * k_ny[k as usize];
        ofkz = ofz + dk * k_nz[k as usize];
        j = 0 as libc::c_int;
        while j < nfj {
            ofjx = ofkx + dj * j_nx[j as usize];
            ofjy = ofky + dj * j_ny[j as usize];
            ofjz = ofkz + dj * j_nz[j as usize];
            i = 0 as libc::c_int;
            while i < nfi {
                *idx.offset((n + 0 as libc::c_int) as isize) = ofjx + i_nx[i as usize];
                *idx.offset((n + 1 as libc::c_int) as isize) = ofjy + i_ny[i as usize];
                *idx.offset((n + 2 as libc::c_int) as isize) = ofjz + i_nz[i as usize];
                n += 3 as libc::c_int;
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
pub unsafe extern "C" fn CINTg3c1e_ovlp(
    mut g: *mut libc::c_double,
    mut ai: libc::c_double,
    mut aj: libc::c_double,
    mut ak: libc::c_double,
    mut envs: *const CINTEnvVars,
) {
    let li: libc::c_int = (*envs).li_ceil;
    let lj: libc::c_int = (*envs).lj_ceil;
    let lk: libc::c_int = (*envs).lk_ceil;
    let nmax: libc::c_int = li + lj + lk;
    let mmax: libc::c_int = lj + lk;
    let mut gx: *mut libc::c_double = g;
    let mut gy: *mut libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *mut libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    *gx.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *gy.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *gz.offset(0 as libc::c_int as isize) = (*envs).fac[0 as libc::c_int as usize];
    if nmax == 0 as libc::c_int {
        return;
    }
    let mut dj: libc::c_int = li + 1 as libc::c_int;
    let dk: libc::c_int = (*envs).g_stride_k;
    let aijk: libc::c_double = ai + aj + ak;
    let aijk1: libc::c_double = 0.5f64 / aijk;
    let mut ri: *const libc::c_double = (*envs).ri;
    let mut rj: *const libc::c_double = (*envs).rj;
    let mut rk: *const libc::c_double = (*envs).rk;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut off: libc::c_int = 0;
    let mut rirj: *const libc::c_double = ((*envs).rirj).as_ptr();
    let mut rjrk: [libc::c_double; 3] = [0.; 3];
    let mut rjrijk: [libc::c_double; 3] = [0.; 3];
    rjrk[0 as libc::c_int
        as usize] = *rj.offset(0 as libc::c_int as isize)
        - *rk.offset(0 as libc::c_int as isize);
    rjrk[1 as libc::c_int
        as usize] = *rj.offset(1 as libc::c_int as isize)
        - *rk.offset(1 as libc::c_int as isize);
    rjrk[2 as libc::c_int
        as usize] = *rj.offset(2 as libc::c_int as isize)
        - *rk.offset(2 as libc::c_int as isize);
    rjrijk[0 as libc::c_int
        as usize] = *rj.offset(0 as libc::c_int as isize)
        - (ai * *ri.offset(0 as libc::c_int as isize)
            + aj * *rj.offset(0 as libc::c_int as isize)
            + ak * *rk.offset(0 as libc::c_int as isize)) / aijk;
    rjrijk[1 as libc::c_int
        as usize] = *rj.offset(1 as libc::c_int as isize)
        - (ai * *ri.offset(1 as libc::c_int as isize)
            + aj * *rj.offset(1 as libc::c_int as isize)
            + ak * *rk.offset(1 as libc::c_int as isize)) / aijk;
    rjrijk[2 as libc::c_int
        as usize] = *rj.offset(2 as libc::c_int as isize)
        - (ai * *ri.offset(2 as libc::c_int as isize)
            + aj * *rj.offset(2 as libc::c_int as isize)
            + ak * *rk.offset(2 as libc::c_int as isize)) / aijk;
    *gx
        .offset(
            dj as isize,
        ) = -rjrijk[0 as libc::c_int as usize] * *gx.offset(0 as libc::c_int as isize);
    *gy
        .offset(
            dj as isize,
        ) = -rjrijk[1 as libc::c_int as usize] * *gy.offset(0 as libc::c_int as isize);
    *gz
        .offset(
            dj as isize,
        ) = -rjrijk[2 as libc::c_int as usize] * *gz.offset(0 as libc::c_int as isize);
    j = 1 as libc::c_int;
    while j < nmax {
        *gx
            .offset(
                ((j + 1 as libc::c_int) * dj) as isize,
            ) = aijk1 * j as libc::c_double
            * *gx.offset(((j - 1 as libc::c_int) * dj) as isize)
            - rjrijk[0 as libc::c_int as usize] * *gx.offset((j * dj) as isize);
        *gy
            .offset(
                ((j + 1 as libc::c_int) * dj) as isize,
            ) = aijk1 * j as libc::c_double
            * *gy.offset(((j - 1 as libc::c_int) * dj) as isize)
            - rjrijk[1 as libc::c_int as usize] * *gy.offset((j * dj) as isize);
        *gz
            .offset(
                ((j + 1 as libc::c_int) * dj) as isize,
            ) = aijk1 * j as libc::c_double
            * *gz.offset(((j - 1 as libc::c_int) * dj) as isize)
            - rjrijk[2 as libc::c_int as usize] * *gz.offset((j * dj) as isize);
        j += 1;
        j;
    }
    i = 1 as libc::c_int;
    while i <= li {
        j = 0 as libc::c_int;
        while j <= nmax - i {
            *gx
                .offset(
                    (i + j * dj) as isize,
                ) = *gx
                .offset((i - 1 as libc::c_int + (j + 1 as libc::c_int) * dj) as isize)
                - *rirj.offset(0 as libc::c_int as isize)
                    * *gx.offset((i - 1 as libc::c_int + j * dj) as isize);
            *gy
                .offset(
                    (i + j * dj) as isize,
                ) = *gy
                .offset((i - 1 as libc::c_int + (j + 1 as libc::c_int) * dj) as isize)
                - *rirj.offset(1 as libc::c_int as isize)
                    * *gy.offset((i - 1 as libc::c_int + j * dj) as isize);
            *gz
                .offset(
                    (i + j * dj) as isize,
                ) = *gz
                .offset((i - 1 as libc::c_int + (j + 1 as libc::c_int) * dj) as isize)
                - *rirj.offset(2 as libc::c_int as isize)
                    * *gz.offset((i - 1 as libc::c_int + j * dj) as isize);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    dj = (*envs).g_stride_j;
    k = 1 as libc::c_int;
    while k <= lk {
        j = 0 as libc::c_int;
        while j <= mmax - k {
            off = k * dk + j * dj;
            i = off;
            while i <= off + li {
                *gx
                    .offset(
                        i as isize,
                    ) = *gx.offset((i + dj - dk) as isize)
                    + rjrk[0 as libc::c_int as usize] * *gx.offset((i - dk) as isize);
                *gy
                    .offset(
                        i as isize,
                    ) = *gy.offset((i + dj - dk) as isize)
                    + rjrk[1 as libc::c_int as usize] * *gy.offset((i - dk) as isize);
                *gz
                    .offset(
                        i as isize,
                    ) = *gz.offset((i + dj - dk) as isize)
                    + rjrk[2 as libc::c_int as usize] * *gz.offset((i - dk) as isize);
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
pub unsafe extern "C" fn CINTg3c1e_nuc(
    mut g: *mut libc::c_double,
    mut ai: libc::c_double,
    mut aj: libc::c_double,
    mut ak: libc::c_double,
    mut rijk: *mut libc::c_double,
    mut cr: *mut libc::c_double,
    mut t2: libc::c_double,
    mut envs: *mut CINTEnvVars,
) {
    let li: libc::c_int = (*envs).li_ceil;
    let lj: libc::c_int = (*envs).lj_ceil;
    let lk: libc::c_int = (*envs).lk_ceil;
    let nmax: libc::c_int = li + lj + lk;
    let mmax: libc::c_int = lj + lk;
    let mut gx: *mut libc::c_double = g;
    let mut gy: *mut libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *mut libc::c_double = g
        .offset(((*envs).g_size * 2 as libc::c_int) as isize);
    *gx.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *gy.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *gz
        .offset(
            0 as libc::c_int as isize,
        ) = 2 as libc::c_int as libc::c_double / 1.7724538509055160272981674833411451f64
        * (*envs).fac[0 as libc::c_int as usize];
    if nmax == 0 as libc::c_int {
        return;
    }
    let mut dj: libc::c_int = li + 1 as libc::c_int;
    let dk: libc::c_int = (*envs).g_stride_k;
    let aijk: libc::c_double = ai + aj + ak;
    let mut rj: *const libc::c_double = (*envs).rj;
    let mut rk: *const libc::c_double = (*envs).rk;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut off: libc::c_int = 0;
    let mut rirj: *const libc::c_double = ((*envs).rirj).as_mut_ptr();
    let mut rjrk: [libc::c_double; 3] = [0.; 3];
    let mut rjr0: [libc::c_double; 3] = [0.; 3];
    rjrk[0 as libc::c_int
        as usize] = *rj.offset(0 as libc::c_int as isize)
        - *rk.offset(0 as libc::c_int as isize);
    rjrk[1 as libc::c_int
        as usize] = *rj.offset(1 as libc::c_int as isize)
        - *rk.offset(1 as libc::c_int as isize);
    rjrk[2 as libc::c_int
        as usize] = *rj.offset(2 as libc::c_int as isize)
        - *rk.offset(2 as libc::c_int as isize);
    rjr0[0 as libc::c_int
        as usize] = *rj.offset(0 as libc::c_int as isize)
        - (*rijk.offset(0 as libc::c_int as isize)
            + t2
                * (*cr.offset(0 as libc::c_int as isize)
                    - *rijk.offset(0 as libc::c_int as isize)));
    rjr0[1 as libc::c_int
        as usize] = *rj.offset(1 as libc::c_int as isize)
        - (*rijk.offset(1 as libc::c_int as isize)
            + t2
                * (*cr.offset(1 as libc::c_int as isize)
                    - *rijk.offset(1 as libc::c_int as isize)));
    rjr0[2 as libc::c_int
        as usize] = *rj.offset(2 as libc::c_int as isize)
        - (*rijk.offset(2 as libc::c_int as isize)
            + t2
                * (*cr.offset(2 as libc::c_int as isize)
                    - *rijk.offset(2 as libc::c_int as isize)));
    *gx
        .offset(
            dj as isize,
        ) = -rjr0[0 as libc::c_int as usize] * *gx.offset(0 as libc::c_int as isize);
    *gy
        .offset(
            dj as isize,
        ) = -rjr0[1 as libc::c_int as usize] * *gy.offset(0 as libc::c_int as isize);
    *gz
        .offset(
            dj as isize,
        ) = -rjr0[2 as libc::c_int as usize] * *gz.offset(0 as libc::c_int as isize);
    let aijk1: libc::c_double = 0.5f64 * (1 as libc::c_int as libc::c_double - t2)
        / aijk;
    j = 1 as libc::c_int;
    while j < nmax {
        *gx
            .offset(
                ((j + 1 as libc::c_int) * dj) as isize,
            ) = aijk1 * j as libc::c_double
            * *gx.offset(((j - 1 as libc::c_int) * dj) as isize)
            - rjr0[0 as libc::c_int as usize] * *gx.offset((j * dj) as isize);
        *gy
            .offset(
                ((j + 1 as libc::c_int) * dj) as isize,
            ) = aijk1 * j as libc::c_double
            * *gy.offset(((j - 1 as libc::c_int) * dj) as isize)
            - rjr0[1 as libc::c_int as usize] * *gy.offset((j * dj) as isize);
        *gz
            .offset(
                ((j + 1 as libc::c_int) * dj) as isize,
            ) = aijk1 * j as libc::c_double
            * *gz.offset(((j - 1 as libc::c_int) * dj) as isize)
            - rjr0[2 as libc::c_int as usize] * *gz.offset((j * dj) as isize);
        j += 1;
        j;
    }
    i = 1 as libc::c_int;
    while i <= li {
        j = 0 as libc::c_int;
        while j <= nmax - i {
            *gx
                .offset(
                    (i + j * dj) as isize,
                ) = *gx
                .offset((i - 1 as libc::c_int + (j + 1 as libc::c_int) * dj) as isize)
                - *rirj.offset(0 as libc::c_int as isize)
                    * *gx.offset((i - 1 as libc::c_int + j * dj) as isize);
            *gy
                .offset(
                    (i + j * dj) as isize,
                ) = *gy
                .offset((i - 1 as libc::c_int + (j + 1 as libc::c_int) * dj) as isize)
                - *rirj.offset(1 as libc::c_int as isize)
                    * *gy.offset((i - 1 as libc::c_int + j * dj) as isize);
            *gz
                .offset(
                    (i + j * dj) as isize,
                ) = *gz
                .offset((i - 1 as libc::c_int + (j + 1 as libc::c_int) * dj) as isize)
                - *rirj.offset(2 as libc::c_int as isize)
                    * *gz.offset((i - 1 as libc::c_int + j * dj) as isize);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    dj = (*envs).g_stride_j;
    k = 1 as libc::c_int;
    while k <= lk {
        j = 0 as libc::c_int;
        while j <= mmax - k {
            off = k * dk + j * dj;
            i = off;
            while i <= off + li {
                *gx
                    .offset(
                        i as isize,
                    ) = *gx.offset((i + dj - dk) as isize)
                    + rjrk[0 as libc::c_int as usize] * *gx.offset((i - dk) as isize);
                *gy
                    .offset(
                        i as isize,
                    ) = *gy.offset((i + dj - dk) as isize)
                    + rjrk[1 as libc::c_int as usize] * *gy.offset((i - dk) as isize);
                *gz
                    .offset(
                        i as isize,
                    ) = *gz.offset((i + dj - dk) as isize)
                    + rjrk[2 as libc::c_int as usize] * *gz.offset((i - dk) as isize);
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
pub unsafe extern "C" fn CINTnabla1i_3c1e(
    mut f: *mut libc::c_double,
    mut g: *const libc::c_double,
    li: libc::c_int,
    lj: libc::c_int,
    lk: libc::c_int,
    mut envs: *const CINTEnvVars,
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
pub unsafe extern "C" fn CINTnabla1j_3c1e(
    mut f: *mut libc::c_double,
    mut g: *const libc::c_double,
    li: libc::c_int,
    lj: libc::c_int,
    lk: libc::c_int,
    mut envs: *const CINTEnvVars,
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
pub unsafe extern "C" fn CINTnabla1k_3c1e(
    mut f: *mut libc::c_double,
    mut g: *const libc::c_double,
    li: libc::c_int,
    lj: libc::c_int,
    lk: libc::c_int,
    mut envs: *const CINTEnvVars,
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
pub unsafe extern "C" fn CINTx1i_3c1e(
    mut f: *mut libc::c_double,
    mut g: *const libc::c_double,
    mut ri: *const libc::c_double,
    li: libc::c_int,
    lj: libc::c_int,
    lk: libc::c_int,
    mut envs: *const CINTEnvVars,
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
pub unsafe extern "C" fn CINTx1j_3c1e(
    mut f: *mut libc::c_double,
    mut g: *const libc::c_double,
    mut rj: *const libc::c_double,
    li: libc::c_int,
    lj: libc::c_int,
    lk: libc::c_int,
    mut envs: *const CINTEnvVars,
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
pub unsafe extern "C" fn CINTx1k_3c1e(
    mut f: *mut libc::c_double,
    mut g: *const libc::c_double,
    mut rk: *const libc::c_double,
    li: libc::c_int,
    lj: libc::c_int,
    lk: libc::c_int,
    mut envs: *const CINTEnvVars,
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
