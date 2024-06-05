#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::cint_bas::CINTcart_comp;
use crate::g1e::CINTcommon_fac_sp;

use crate::cint::CINTEnvVars;

#[no_mangle]
pub unsafe extern "C" fn CINTinit_int3c1e_EnvVars(
    mut envs: *mut CINTEnvVars,
    mut ng: *mut i32,
    mut shls: *mut i32,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
) {
    (*envs).natm = natm;
    (*envs).nbas = nbas;
    (*envs).atm = atm;
    (*envs).bas = bas;
    (*envs).env = env;
    (*envs).shls = shls;
    let i_sh: i32 = *shls.offset(0 as i32 as isize);
    let j_sh: i32 = *shls.offset(1 as i32 as isize);
    let k_sh: i32 = *shls.offset(2 as i32 as isize);
    (*envs).i_l = *bas.offset((8 as i32 * i_sh + 1 as i32) as isize);
    (*envs).j_l = *bas.offset((8 as i32 * j_sh + 1 as i32) as isize);
    (*envs).k_l = *bas.offset((8 as i32 * k_sh + 1 as i32) as isize);
    (*envs).l_l = 0 as i32;
    (*envs)
        .x_ctr[0 as i32
        as usize] = *bas.offset((8 as i32 * i_sh + 3 as i32) as isize);
    (*envs)
        .x_ctr[1 as i32
        as usize] = *bas.offset((8 as i32 * j_sh + 3 as i32) as isize);
    (*envs)
        .x_ctr[2 as i32
        as usize] = *bas.offset((8 as i32 * k_sh + 3 as i32) as isize);
    (*envs).x_ctr[3 as i32 as usize] = 1 as i32;
    (*envs)
        .nfi = ((*envs).i_l + 1 as i32) * ((*envs).i_l + 2 as i32)
        / 2 as i32;
    (*envs)
        .nfj = ((*envs).j_l + 1 as i32) * ((*envs).j_l + 2 as i32)
        / 2 as i32;
    (*envs)
        .c2rust_unnamed
        .nfk = ((*envs).k_l + 1 as i32) * ((*envs).k_l + 2 as i32)
        / 2 as i32;
    (*envs).c2rust_unnamed_0.nfl = 1 as i32;
    (*envs).nf = (*envs).nfi * (*envs).nfj * (*envs).c2rust_unnamed.nfk;
    (*envs)
        .ri = env
        .offset(
            *atm
                .offset(
                    (6 as i32
                        * *bas
                            .offset(
                                (8 as i32 * i_sh + 0 as i32) as isize,
                            ) + 1 as i32) as isize,
                ) as isize,
        );
    (*envs)
        .rj = env
        .offset(
            *atm
                .offset(
                    (6 as i32
                        * *bas
                            .offset(
                                (8 as i32 * j_sh + 0 as i32) as isize,
                            ) + 1 as i32) as isize,
                ) as isize,
        );
    (*envs)
        .rk = env
        .offset(
            *atm
                .offset(
                    (6 as i32
                        * *bas
                            .offset(
                                (8 as i32 * k_sh + 0 as i32) as isize,
                            ) + 1 as i32) as isize,
                ) as isize,
        );
    (*envs).gbits = *ng.offset(4 as i32 as isize);
    (*envs).ncomp_e1 = *ng.offset(5 as i32 as isize);
    (*envs).ncomp_e2 = 0 as i32;
    (*envs).ncomp_tensor = *ng.offset(7 as i32 as isize);
    (*envs).li_ceil = (*envs).i_l + *ng.offset(0 as i32 as isize);
    (*envs).lj_ceil = (*envs).j_l + *ng.offset(1 as i32 as isize);
    (*envs).lk_ceil = (*envs).k_l + *ng.offset(2 as i32 as isize);
    (*envs).ll_ceil = 0 as i32;
    (*envs)
        .nrys_roots = ((*envs).li_ceil + (*envs).lj_ceil + (*envs).lk_ceil)
        / 2 as i32 + 1 as i32;
    (*envs)
        .common_factor = 1.7724538509055160272981674833411451f64
        * 3.14159265358979323846f64 * CINTcommon_fac_sp((*envs).i_l)
        * CINTcommon_fac_sp((*envs).j_l) * CINTcommon_fac_sp((*envs).k_l);
    if *env.offset(0 as i32 as isize) == 0 as i32 as f64 {
        (*envs).expcutoff = 60 as i32 as f64;
    } else {
        (*envs)
            .expcutoff = if 40 as i32 as f64
            > *env.offset(0 as i32 as isize)
        {
            40 as i32 as f64
        } else {
            *env.offset(0 as i32 as isize)
        };
    }
    let mut dli: i32 = (*envs).li_ceil + 1 as i32;
    let mut dlj: i32 = (*envs).lj_ceil + (*envs).lk_ceil + 1 as i32;
    let mut dlk: i32 = (*envs).lk_ceil + 1 as i32;
    (*envs).g_stride_i = 1 as i32;
    (*envs).g_stride_j = dli;
    (*envs).g_stride_k = dli * dlj;
    (*envs).g_stride_l = (*envs).g_stride_k;
    let mut nmax: i32 = (*envs).li_ceil + dlj;
    (*envs)
        .g_size = if dli * dlj * dlk > dli * nmax {
        dli * dlj * dlk
    } else {
        dli * nmax
    };
    (*envs)
        .rirj[0 as i32
        as usize] = *((*envs).ri).offset(0 as i32 as isize)
        - *((*envs).rj).offset(0 as i32 as isize);
    (*envs)
        .rirj[1 as i32
        as usize] = *((*envs).ri).offset(1 as i32 as isize)
        - *((*envs).rj).offset(1 as i32 as isize);
    (*envs)
        .rirj[2 as i32
        as usize] = *((*envs).ri).offset(2 as i32 as isize)
        - *((*envs).rj).offset(2 as i32 as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CINTg3c1e_index_xyz(
    mut idx: *mut i32,
    mut envs: *const CINTEnvVars,
) {
    let i_l: i32 = (*envs).i_l;
    let j_l: i32 = (*envs).j_l;
    let k_l: i32 = (*envs).k_l;
    let nfi: i32 = (*envs).nfi;
    let nfj: i32 = (*envs).nfj;
    let nfk: i32 = (*envs).c2rust_unnamed.nfk;
    let dj: i32 = (*envs).g_stride_j;
    let dk: i32 = (*envs).g_stride_k;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut n: i32 = 0;
    let mut ofx: i32 = 0;
    let mut ofjx: i32 = 0;
    let mut ofkx: i32 = 0;
    let mut ofy: i32 = 0;
    let mut ofjy: i32 = 0;
    let mut ofky: i32 = 0;
    let mut ofz: i32 = 0;
    let mut ofjz: i32 = 0;
    let mut ofkz: i32 = 0;
    let mut i_nx: [i32; 136] = [0; 136];
    let mut i_ny: [i32; 136] = [0; 136];
    let mut i_nz: [i32; 136] = [0; 136];
    let mut j_nx: [i32; 136] = [0; 136];
    let mut j_ny: [i32; 136] = [0; 136];
    let mut j_nz: [i32; 136] = [0; 136];
    let mut k_nx: [i32; 136] = [0; 136];
    let mut k_ny: [i32; 136] = [0; 136];
    let mut k_nz: [i32; 136] = [0; 136];
    CINTcart_comp(i_nx.as_mut_ptr(), i_ny.as_mut_ptr(), i_nz.as_mut_ptr(), i_l);
    CINTcart_comp(j_nx.as_mut_ptr(), j_ny.as_mut_ptr(), j_nz.as_mut_ptr(), j_l);
    CINTcart_comp(k_nx.as_mut_ptr(), k_ny.as_mut_ptr(), k_nz.as_mut_ptr(), k_l);
    ofx = 0 as i32;
    ofy = (*envs).g_size;
    ofz = (*envs).g_size * 2 as i32;
    n = 0 as i32;
    k = 0 as i32;
    while k < nfk {
        ofkx = ofx + dk * k_nx[k as usize];
        ofky = ofy + dk * k_ny[k as usize];
        ofkz = ofz + dk * k_nz[k as usize];
        j = 0 as i32;
        while j < nfj {
            ofjx = ofkx + dj * j_nx[j as usize];
            ofjy = ofky + dj * j_ny[j as usize];
            ofjz = ofkz + dj * j_nz[j as usize];
            i = 0 as i32;
            while i < nfi {
                *idx.offset((n + 0 as i32) as isize) = ofjx + i_nx[i as usize];
                *idx.offset((n + 1 as i32) as isize) = ofjy + i_ny[i as usize];
                *idx.offset((n + 2 as i32) as isize) = ofjz + i_nz[i as usize];
                n += 3 as i32;
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
    mut g: *mut f64,
    mut ai: f64,
    mut aj: f64,
    mut ak: f64,
    mut envs: *const CINTEnvVars,
) {
    let li: i32 = (*envs).li_ceil;
    let lj: i32 = (*envs).lj_ceil;
    let lk: i32 = (*envs).lk_ceil;
    let nmax: i32 = li + lj + lk;
    let mmax: i32 = lj + lk;
    let mut gx: *mut f64 = g;
    let mut gy: *mut f64 = g.offset((*envs).g_size as isize);
    let mut gz: *mut f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    *gx.offset(0 as i32 as isize) = 1 as i32 as f64;
    *gy.offset(0 as i32 as isize) = 1 as i32 as f64;
    *gz.offset(0 as i32 as isize) = (*envs).fac[0 as i32 as usize];
    if nmax == 0 as i32 {
        return;
    }
    let mut dj: i32 = li + 1 as i32;
    let dk: i32 = (*envs).g_stride_k;
    let aijk: f64 = ai + aj + ak;
    let aijk1: f64 = 0.5f64 / aijk;
    let mut ri: *const f64 = (*envs).ri;
    let mut rj: *const f64 = (*envs).rj;
    let mut rk: *const f64 = (*envs).rk;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut off: i32 = 0;
    let mut rirj: *const f64 = ((*envs).rirj).as_ptr();
    let mut rjrk: [f64; 3] = [0.; 3];
    let mut rjrijk: [f64; 3] = [0.; 3];
    rjrk[0 as i32
        as usize] = *rj.offset(0 as i32 as isize)
        - *rk.offset(0 as i32 as isize);
    rjrk[1 as i32
        as usize] = *rj.offset(1 as i32 as isize)
        - *rk.offset(1 as i32 as isize);
    rjrk[2 as i32
        as usize] = *rj.offset(2 as i32 as isize)
        - *rk.offset(2 as i32 as isize);
    rjrijk[0 as i32
        as usize] = *rj.offset(0 as i32 as isize)
        - (ai * *ri.offset(0 as i32 as isize)
            + aj * *rj.offset(0 as i32 as isize)
            + ak * *rk.offset(0 as i32 as isize)) / aijk;
    rjrijk[1 as i32
        as usize] = *rj.offset(1 as i32 as isize)
        - (ai * *ri.offset(1 as i32 as isize)
            + aj * *rj.offset(1 as i32 as isize)
            + ak * *rk.offset(1 as i32 as isize)) / aijk;
    rjrijk[2 as i32
        as usize] = *rj.offset(2 as i32 as isize)
        - (ai * *ri.offset(2 as i32 as isize)
            + aj * *rj.offset(2 as i32 as isize)
            + ak * *rk.offset(2 as i32 as isize)) / aijk;
    *gx
        .offset(
            dj as isize,
        ) = -rjrijk[0 as i32 as usize] * *gx.offset(0 as i32 as isize);
    *gy
        .offset(
            dj as isize,
        ) = -rjrijk[1 as i32 as usize] * *gy.offset(0 as i32 as isize);
    *gz
        .offset(
            dj as isize,
        ) = -rjrijk[2 as i32 as usize] * *gz.offset(0 as i32 as isize);
    j = 1 as i32;
    while j < nmax {
        *gx
            .offset(
                ((j + 1 as i32) * dj) as isize,
            ) = aijk1 * j as f64
            * *gx.offset(((j - 1 as i32) * dj) as isize)
            - rjrijk[0 as i32 as usize] * *gx.offset((j * dj) as isize);
        *gy
            .offset(
                ((j + 1 as i32) * dj) as isize,
            ) = aijk1 * j as f64
            * *gy.offset(((j - 1 as i32) * dj) as isize)
            - rjrijk[1 as i32 as usize] * *gy.offset((j * dj) as isize);
        *gz
            .offset(
                ((j + 1 as i32) * dj) as isize,
            ) = aijk1 * j as f64
            * *gz.offset(((j - 1 as i32) * dj) as isize)
            - rjrijk[2 as i32 as usize] * *gz.offset((j * dj) as isize);
        j += 1;
        j;
    }
    i = 1 as i32;
    while i <= li {
        j = 0 as i32;
        while j <= nmax - i {
            *gx
                .offset(
                    (i + j * dj) as isize,
                ) = *gx
                .offset((i - 1 as i32 + (j + 1 as i32) * dj) as isize)
                - *rirj.offset(0 as i32 as isize)
                    * *gx.offset((i - 1 as i32 + j * dj) as isize);
            *gy
                .offset(
                    (i + j * dj) as isize,
                ) = *gy
                .offset((i - 1 as i32 + (j + 1 as i32) * dj) as isize)
                - *rirj.offset(1 as i32 as isize)
                    * *gy.offset((i - 1 as i32 + j * dj) as isize);
            *gz
                .offset(
                    (i + j * dj) as isize,
                ) = *gz
                .offset((i - 1 as i32 + (j + 1 as i32) * dj) as isize)
                - *rirj.offset(2 as i32 as isize)
                    * *gz.offset((i - 1 as i32 + j * dj) as isize);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    dj = (*envs).g_stride_j;
    k = 1 as i32;
    while k <= lk {
        j = 0 as i32;
        while j <= mmax - k {
            off = k * dk + j * dj;
            i = off;
            while i <= off + li {
                *gx
                    .offset(
                        i as isize,
                    ) = *gx.offset((i + dj - dk) as isize)
                    + rjrk[0 as i32 as usize] * *gx.offset((i - dk) as isize);
                *gy
                    .offset(
                        i as isize,
                    ) = *gy.offset((i + dj - dk) as isize)
                    + rjrk[1 as i32 as usize] * *gy.offset((i - dk) as isize);
                *gz
                    .offset(
                        i as isize,
                    ) = *gz.offset((i + dj - dk) as isize)
                    + rjrk[2 as i32 as usize] * *gz.offset((i - dk) as isize);
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
    mut g: *mut f64,
    mut ai: f64,
    mut aj: f64,
    mut ak: f64,
    mut rijk: *mut f64,
    mut cr: *mut f64,
    mut t2: f64,
    mut envs: *mut CINTEnvVars,
) {
    let li: i32 = (*envs).li_ceil;
    let lj: i32 = (*envs).lj_ceil;
    let lk: i32 = (*envs).lk_ceil;
    let nmax: i32 = li + lj + lk;
    let mmax: i32 = lj + lk;
    let mut gx: *mut f64 = g;
    let mut gy: *mut f64 = g.offset((*envs).g_size as isize);
    let mut gz: *mut f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    *gx.offset(0 as i32 as isize) = 1 as i32 as f64;
    *gy.offset(0 as i32 as isize) = 1 as i32 as f64;
    *gz
        .offset(
            0 as i32 as isize,
        ) = 2 as i32 as f64 / 1.7724538509055160272981674833411451f64
        * (*envs).fac[0 as i32 as usize];
    if nmax == 0 as i32 {
        return;
    }
    let mut dj: i32 = li + 1 as i32;
    let dk: i32 = (*envs).g_stride_k;
    let aijk: f64 = ai + aj + ak;
    let mut rj: *const f64 = (*envs).rj;
    let mut rk: *const f64 = (*envs).rk;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut off: i32 = 0;
    let mut rirj: *const f64 = ((*envs).rirj).as_mut_ptr();
    let mut rjrk: [f64; 3] = [0.; 3];
    let mut rjr0: [f64; 3] = [0.; 3];
    rjrk[0 as i32
        as usize] = *rj.offset(0 as i32 as isize)
        - *rk.offset(0 as i32 as isize);
    rjrk[1 as i32
        as usize] = *rj.offset(1 as i32 as isize)
        - *rk.offset(1 as i32 as isize);
    rjrk[2 as i32
        as usize] = *rj.offset(2 as i32 as isize)
        - *rk.offset(2 as i32 as isize);
    rjr0[0 as i32
        as usize] = *rj.offset(0 as i32 as isize)
        - (*rijk.offset(0 as i32 as isize)
            + t2
                * (*cr.offset(0 as i32 as isize)
                    - *rijk.offset(0 as i32 as isize)));
    rjr0[1 as i32
        as usize] = *rj.offset(1 as i32 as isize)
        - (*rijk.offset(1 as i32 as isize)
            + t2
                * (*cr.offset(1 as i32 as isize)
                    - *rijk.offset(1 as i32 as isize)));
    rjr0[2 as i32
        as usize] = *rj.offset(2 as i32 as isize)
        - (*rijk.offset(2 as i32 as isize)
            + t2
                * (*cr.offset(2 as i32 as isize)
                    - *rijk.offset(2 as i32 as isize)));
    *gx
        .offset(
            dj as isize,
        ) = -rjr0[0 as i32 as usize] * *gx.offset(0 as i32 as isize);
    *gy
        .offset(
            dj as isize,
        ) = -rjr0[1 as i32 as usize] * *gy.offset(0 as i32 as isize);
    *gz
        .offset(
            dj as isize,
        ) = -rjr0[2 as i32 as usize] * *gz.offset(0 as i32 as isize);
    let aijk1: f64 = 0.5f64 * (1 as i32 as f64 - t2)
        / aijk;
    j = 1 as i32;
    while j < nmax {
        *gx
            .offset(
                ((j + 1 as i32) * dj) as isize,
            ) = aijk1 * j as f64
            * *gx.offset(((j - 1 as i32) * dj) as isize)
            - rjr0[0 as i32 as usize] * *gx.offset((j * dj) as isize);
        *gy
            .offset(
                ((j + 1 as i32) * dj) as isize,
            ) = aijk1 * j as f64
            * *gy.offset(((j - 1 as i32) * dj) as isize)
            - rjr0[1 as i32 as usize] * *gy.offset((j * dj) as isize);
        *gz
            .offset(
                ((j + 1 as i32) * dj) as isize,
            ) = aijk1 * j as f64
            * *gz.offset(((j - 1 as i32) * dj) as isize)
            - rjr0[2 as i32 as usize] * *gz.offset((j * dj) as isize);
        j += 1;
        j;
    }
    i = 1 as i32;
    while i <= li {
        j = 0 as i32;
        while j <= nmax - i {
            *gx
                .offset(
                    (i + j * dj) as isize,
                ) = *gx
                .offset((i - 1 as i32 + (j + 1 as i32) * dj) as isize)
                - *rirj.offset(0 as i32 as isize)
                    * *gx.offset((i - 1 as i32 + j * dj) as isize);
            *gy
                .offset(
                    (i + j * dj) as isize,
                ) = *gy
                .offset((i - 1 as i32 + (j + 1 as i32) * dj) as isize)
                - *rirj.offset(1 as i32 as isize)
                    * *gy.offset((i - 1 as i32 + j * dj) as isize);
            *gz
                .offset(
                    (i + j * dj) as isize,
                ) = *gz
                .offset((i - 1 as i32 + (j + 1 as i32) * dj) as isize)
                - *rirj.offset(2 as i32 as isize)
                    * *gz.offset((i - 1 as i32 + j * dj) as isize);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    dj = (*envs).g_stride_j;
    k = 1 as i32;
    while k <= lk {
        j = 0 as i32;
        while j <= mmax - k {
            off = k * dk + j * dj;
            i = off;
            while i <= off + li {
                *gx
                    .offset(
                        i as isize,
                    ) = *gx.offset((i + dj - dk) as isize)
                    + rjrk[0 as i32 as usize] * *gx.offset((i - dk) as isize);
                *gy
                    .offset(
                        i as isize,
                    ) = *gy.offset((i + dj - dk) as isize)
                    + rjrk[1 as i32 as usize] * *gy.offset((i - dk) as isize);
                *gz
                    .offset(
                        i as isize,
                    ) = *gz.offset((i + dj - dk) as isize)
                    + rjrk[2 as i32 as usize] * *gz.offset((i - dk) as isize);
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
    mut f: *mut f64,
    mut g: *const f64,
    li: i32,
    lj: i32,
    lk: i32,
    mut envs: *const CINTEnvVars,
) {
    let dj: i32 = (*envs).g_stride_j;
    let dk: i32 = (*envs).g_stride_k;
    let ai2: f64 = -(2 as i32) as f64
        * (*envs).ai[0 as i32 as usize];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let mut gx: *const f64 = g;
    let mut gy: *const f64 = g.offset((*envs).g_size as isize);
    let mut gz: *const f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut fx: *mut f64 = f;
    let mut fy: *mut f64 = f.offset((*envs).g_size as isize);
    let mut fz: *mut f64 = f
        .offset(((*envs).g_size * 2 as i32) as isize);
    k = 0 as i32;
    while k <= lk {
        j = 0 as i32;
        while j <= lj {
            ptr = dj * j + dk * k;
            *fx
                .offset(
                    ptr as isize,
                ) = ai2 * *gx.offset((ptr + 1 as i32) as isize);
            *fy
                .offset(
                    ptr as isize,
                ) = ai2 * *gy.offset((ptr + 1 as i32) as isize);
            *fz
                .offset(
                    ptr as isize,
                ) = ai2 * *gz.offset((ptr + 1 as i32) as isize);
            i = 1 as i32;
            while i <= li {
                *fx
                    .offset(
                        (ptr + i) as isize,
                    ) = i as f64
                    * *gx.offset((ptr + i - 1 as i32) as isize)
                    + ai2 * *gx.offset((ptr + i + 1 as i32) as isize);
                *fy
                    .offset(
                        (ptr + i) as isize,
                    ) = i as f64
                    * *gy.offset((ptr + i - 1 as i32) as isize)
                    + ai2 * *gy.offset((ptr + i + 1 as i32) as isize);
                *fz
                    .offset(
                        (ptr + i) as isize,
                    ) = i as f64
                    * *gz.offset((ptr + i - 1 as i32) as isize)
                    + ai2 * *gz.offset((ptr + i + 1 as i32) as isize);
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
    mut f: *mut f64,
    mut g: *const f64,
    li: i32,
    lj: i32,
    lk: i32,
    mut envs: *const CINTEnvVars,
) {
    let dj: i32 = (*envs).g_stride_j;
    let dk: i32 = (*envs).g_stride_k;
    let aj2: f64 = -(2 as i32) as f64
        * (*envs).aj[0 as i32 as usize];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let mut gx: *const f64 = g;
    let mut gy: *const f64 = g.offset((*envs).g_size as isize);
    let mut gz: *const f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut fx: *mut f64 = f;
    let mut fy: *mut f64 = f.offset((*envs).g_size as isize);
    let mut fz: *mut f64 = f
        .offset(((*envs).g_size * 2 as i32) as isize);
    k = 0 as i32;
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
        j = 1 as i32;
        while j <= lj {
            ptr = dj * j + dk * k;
            i = ptr;
            while i <= ptr + li {
                *fx
                    .offset(
                        i as isize,
                    ) = j as f64 * *gx.offset((i - dj) as isize)
                    + aj2 * *gx.offset((i + dj) as isize);
                *fy
                    .offset(
                        i as isize,
                    ) = j as f64 * *gy.offset((i - dj) as isize)
                    + aj2 * *gy.offset((i + dj) as isize);
                *fz
                    .offset(
                        i as isize,
                    ) = j as f64 * *gz.offset((i - dj) as isize)
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
    mut f: *mut f64,
    mut g: *const f64,
    li: i32,
    lj: i32,
    lk: i32,
    mut envs: *const CINTEnvVars,
) {
    let dj: i32 = (*envs).g_stride_j;
    let dk: i32 = (*envs).g_stride_k;
    let ak2: f64 = -(2 as i32) as f64
        * (*envs).ak[0 as i32 as usize];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let mut gx: *const f64 = g;
    let mut gy: *const f64 = g.offset((*envs).g_size as isize);
    let mut gz: *const f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut fx: *mut f64 = f;
    let mut fy: *mut f64 = f.offset((*envs).g_size as isize);
    let mut fz: *mut f64 = f
        .offset(((*envs).g_size * 2 as i32) as isize);
    j = 0 as i32;
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
    k = 1 as i32;
    while k <= lk {
        j = 0 as i32;
        while j <= lj {
            ptr = dj * j + dk * k;
            i = ptr;
            while i <= ptr + li {
                *fx
                    .offset(
                        i as isize,
                    ) = k as f64 * *gx.offset((i - dk) as isize)
                    + ak2 * *gx.offset((i + dk) as isize);
                *fy
                    .offset(
                        i as isize,
                    ) = k as f64 * *gy.offset((i - dk) as isize)
                    + ak2 * *gy.offset((i + dk) as isize);
                *fz
                    .offset(
                        i as isize,
                    ) = k as f64 * *gz.offset((i - dk) as isize)
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
    mut f: *mut f64,
    mut g: *const f64,
    mut ri: *const f64,
    li: i32,
    lj: i32,
    lk: i32,
    mut envs: *const CINTEnvVars,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let dj: i32 = (*envs).g_stride_j;
    let dk: i32 = (*envs).g_stride_k;
    let mut gx: *const f64 = g;
    let mut gy: *const f64 = g.offset((*envs).g_size as isize);
    let mut gz: *const f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut fx: *mut f64 = f;
    let mut fy: *mut f64 = f.offset((*envs).g_size as isize);
    let mut fz: *mut f64 = f
        .offset(((*envs).g_size * 2 as i32) as isize);
    k = 0 as i32;
    while k <= lk {
        j = 0 as i32;
        while j <= lj {
            ptr = dj * j + dk * k;
            i = ptr;
            while i <= ptr + li {
                *fx
                    .offset(
                        i as isize,
                    ) = *gx.offset((i + 1 as i32) as isize)
                    + *ri.offset(0 as i32 as isize) * *gx.offset(i as isize);
                *fy
                    .offset(
                        i as isize,
                    ) = *gy.offset((i + 1 as i32) as isize)
                    + *ri.offset(1 as i32 as isize) * *gy.offset(i as isize);
                *fz
                    .offset(
                        i as isize,
                    ) = *gz.offset((i + 1 as i32) as isize)
                    + *ri.offset(2 as i32 as isize) * *gz.offset(i as isize);
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
    mut f: *mut f64,
    mut g: *const f64,
    mut rj: *const f64,
    li: i32,
    lj: i32,
    lk: i32,
    mut envs: *const CINTEnvVars,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let dj: i32 = (*envs).g_stride_j;
    let dk: i32 = (*envs).g_stride_k;
    let mut gx: *const f64 = g;
    let mut gy: *const f64 = g.offset((*envs).g_size as isize);
    let mut gz: *const f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut fx: *mut f64 = f;
    let mut fy: *mut f64 = f.offset((*envs).g_size as isize);
    let mut fz: *mut f64 = f
        .offset(((*envs).g_size * 2 as i32) as isize);
    k = 0 as i32;
    while k <= lk {
        j = 0 as i32;
        while j <= lj {
            ptr = dj * j + dk * k;
            i = ptr;
            while i <= ptr + li {
                *fx
                    .offset(
                        i as isize,
                    ) = *gx.offset((i + dj) as isize)
                    + *rj.offset(0 as i32 as isize) * *gx.offset(i as isize);
                *fy
                    .offset(
                        i as isize,
                    ) = *gy.offset((i + dj) as isize)
                    + *rj.offset(1 as i32 as isize) * *gy.offset(i as isize);
                *fz
                    .offset(
                        i as isize,
                    ) = *gz.offset((i + dj) as isize)
                    + *rj.offset(2 as i32 as isize) * *gz.offset(i as isize);
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
    mut f: *mut f64,
    mut g: *const f64,
    mut rk: *const f64,
    li: i32,
    lj: i32,
    lk: i32,
    mut envs: *const CINTEnvVars,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let dj: i32 = (*envs).g_stride_j;
    let dk: i32 = (*envs).g_stride_k;
    let mut gx: *const f64 = g;
    let mut gy: *const f64 = g.offset((*envs).g_size as isize);
    let mut gz: *const f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut fx: *mut f64 = f;
    let mut fy: *mut f64 = f.offset((*envs).g_size as isize);
    let mut fz: *mut f64 = f
        .offset(((*envs).g_size * 2 as i32) as isize);
    k = 0 as i32;
    while k <= lk {
        j = 0 as i32;
        while j <= lj {
            ptr = dj * j + dk * k;
            i = ptr;
            while i <= ptr + li {
                *fx
                    .offset(
                        i as isize,
                    ) = *gx.offset((i + dk) as isize)
                    + *rk.offset(0 as i32 as isize) * *gx.offset(i as isize);
                *fy
                    .offset(
                        i as isize,
                    ) = *gy.offset((i + dk) as isize)
                    + *rk.offset(1 as i32 as isize) * *gy.offset(i as isize);
                *fz
                    .offset(
                        i as isize,
                    ) = *gz.offset((i + dk) as isize)
                    + *rk.offset(2 as i32 as isize) * *gz.offset(i as isize);
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
