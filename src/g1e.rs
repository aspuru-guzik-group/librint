#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::rys_roots::CINTrys_roots;
use crate::cint_bas::CINTcart_comp;

use crate::cint::CINTEnvVars;

pub type size_t = libc::c_ulong;

extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn abs(_: i32) -> i32;
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

#[no_mangle]
pub unsafe extern "C" fn CINTinit_int1e_EnvVars(
    mut envs: *mut CINTEnvVars,
    mut ng: *mut i32,
    mut shls: *mut i32,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut libc::c_double,
) {
    (*envs).natm = natm;
    (*envs).nbas = nbas;
    (*envs).atm = atm;
    (*envs).bas = bas;
    (*envs).env = env;
    (*envs).shls = shls;
    let i_sh: i32 = *shls.offset(0 as isize);
    let j_sh: i32 = *shls.offset(1 as isize);
    (*envs).i_l = *bas.offset((8 as i32 * i_sh + 1 as i32) as isize);
    (*envs).j_l = *bas.offset((8 as i32 * j_sh + 1 as i32) as isize);
    (*envs)
        .x_ctr[0 as i32
        as usize] = *bas.offset((8 as i32 * i_sh + 3 as i32) as isize);
    (*envs)
        .x_ctr[1 as i32
        as usize] = *bas.offset((8 as i32 * j_sh + 3 as i32) as isize);
    (*envs)
        .nfi = ((*envs).i_l + 1 as i32) * ((*envs).i_l + 2 as i32)
        / 2 as i32;
    (*envs)
        .nfj = ((*envs).j_l + 1 as i32) * ((*envs).j_l + 2 as i32)
        / 2 as i32;
    (*envs).nf = (*envs).nfi * (*envs).nfj;
    (*envs).common_factor = 1 as libc::c_double;
    if *env.offset(0 as isize) == 0 as libc::c_double {
        (*envs).expcutoff = 60 as libc::c_double;
    } else {
        (*envs)
            .expcutoff = MAX(40 as f64, *env.offset(0 as isize))
            as libc::c_double;
    }
    (*envs).li_ceil = (*envs).i_l + *ng.offset(0 as isize);
    (*envs).lj_ceil = (*envs).j_l + *ng.offset(1 as isize);
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
    (*envs).gbits = *ng.offset(4 as isize);
    (*envs).ncomp_e1 = *ng.offset(5 as isize);
    (*envs).ncomp_tensor = *ng.offset(7 as isize);
    if *ng.offset(6 as isize) > 0 as i32 {
        (*envs).nrys_roots = *ng.offset(6 as isize);
    } else {
        (*envs)
            .nrys_roots = ((*envs).li_ceil + (*envs).lj_ceil) / 2 as i32
            + 1 as i32;
    }
    let mut dli: i32 = 0;
    let mut dlj: i32 = 0;
    let mut ibase: i32 = ((*envs).li_ceil > (*envs).lj_ceil) as i32;
    if ibase != 0 {
        dli = (*envs).li_ceil + (*envs).lj_ceil + 1 as i32;
        dlj = (*envs).lj_ceil + 1 as i32;
        (*envs)
            .rirj[0 as i32
            as usize] = *((*envs).ri).offset(0 as isize)
            - *((*envs).rj).offset(0 as isize);
        (*envs)
            .rirj[1 as i32
            as usize] = *((*envs).ri).offset(1 as isize)
            - *((*envs).rj).offset(1 as isize);
        (*envs)
            .rirj[2 as i32
            as usize] = *((*envs).ri).offset(2 as isize)
            - *((*envs).rj).offset(2 as isize);
    } else {
        dli = (*envs).li_ceil + 1 as i32;
        dlj = (*envs).li_ceil + (*envs).lj_ceil + 1 as i32;
        (*envs)
            .rirj[0 as i32
            as usize] = *((*envs).rj).offset(0 as isize)
            - *((*envs).ri).offset(0 as isize);
        (*envs)
            .rirj[1 as i32
            as usize] = *((*envs).rj).offset(1 as isize)
            - *((*envs).ri).offset(1 as isize);
        (*envs)
            .rirj[2 as i32
            as usize] = *((*envs).rj).offset(2 as isize)
            - *((*envs).ri).offset(2 as isize);
    }
    (*envs).g_stride_i = (*envs).nrys_roots;
    (*envs).g_stride_j = (*envs).nrys_roots * dli;
    (*envs).g_size = (*envs).nrys_roots * dli * dlj;
    (*envs).g_stride_k = (*envs).g_size;
    (*envs).g_stride_l = (*envs).g_size;
}
#[no_mangle]
pub unsafe extern "C" fn CINTg1e_index_xyz(
    mut idx: *mut i32,
    mut envs: *mut CINTEnvVars,
) {
    let i_l: i32 = (*envs).i_l;
    let j_l: i32 = (*envs).j_l;
    let nfi: i32 = (*envs).nfi;
    let nfj: i32 = (*envs).nfj;
    let di: i32 = (*envs).g_stride_i;
    let dj: i32 = (*envs).g_stride_j;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    let mut ofx: i32 = 0;
    let mut ofjx: i32 = 0;
    let mut ofy: i32 = 0;
    let mut ofjy: i32 = 0;
    let mut ofz: i32 = 0;
    let mut ofjz: i32 = 0;
    let mut i_nx: [i32; 136] = [0; 136];
    let mut i_ny: [i32; 136] = [0; 136];
    let mut i_nz: [i32; 136] = [0; 136];
    let mut j_nx: [i32; 136] = [0; 136];
    let mut j_ny: [i32; 136] = [0; 136];
    let mut j_nz: [i32; 136] = [0; 136];
    CINTcart_comp(i_nx.as_mut_ptr(), i_ny.as_mut_ptr(), i_nz.as_mut_ptr(), i_l);
    CINTcart_comp(j_nx.as_mut_ptr(), j_ny.as_mut_ptr(), j_nz.as_mut_ptr(), j_l);
    ofx = 0 as i32;
    ofy = (*envs).g_size;
    ofz = (*envs).g_size * 2 as i32;
    n = 0 as i32;
    j = 0 as i32;
    while j < nfj {
        ofjx = ofx + dj * j_nx[j as usize];
        ofjy = ofy + dj * j_ny[j as usize];
        ofjz = ofz + dj * j_nz[j as usize];
        i = 0 as i32;
        while i < nfi {
            *idx.offset((n + 0 as i32) as isize) = ofjx + di * i_nx[i as usize];
            *idx.offset((n + 1 as i32) as isize) = ofjy + di * i_ny[i as usize];
            *idx.offset((n + 2 as i32) as isize) = ofjz + di * i_nz[i as usize];
            n += 3 as i32;
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
) -> i32 {
    let mut gx: *mut libc::c_double = g;
    let mut gy: *mut libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *mut libc::c_double = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut aij: libc::c_double = (*envs).ai[0 as usize]
        + (*envs).aj[0 as usize];
    *gx.offset(0 as isize) = 1 as libc::c_double;
    *gy.offset(0 as isize) = 1 as libc::c_double;
    *gz
        .offset(
            0 as isize,
        ) = (*envs).fac[0 as usize]
        * 1.7724538509055160272981674833411451f64 * 3.14159265358979323846f64
        / (aij * sqrt(aij));
    let mut nmax: i32 = (*envs).li_ceil + (*envs).lj_ceil;
    if nmax == 0 as i32 {
        return 1 as i32;
    }
    let mut rij: *mut libc::c_double = ((*envs).rij).as_mut_ptr();
    let mut rirj: *mut libc::c_double = ((*envs).rirj).as_mut_ptr();
    let mut lj: i32 = 0;
    let mut di: i32 = 0;
    let mut dj: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    let mut ptr: i32 = 0;
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
    rijrx[0 as i32
        as usize] = *rij.offset(0 as isize)
        - *rx.offset(0 as isize);
    rijrx[1 as i32
        as usize] = *rij.offset(1 as isize)
        - *rx.offset(1 as isize);
    rijrx[2 as i32
        as usize] = *rij.offset(2 as isize)
        - *rx.offset(2 as isize);
    *gx
        .offset(
            di as isize,
        ) = rijrx[0 as usize] * *gx.offset(0 as isize);
    *gy
        .offset(
            di as isize,
        ) = rijrx[1 as usize] * *gy.offset(0 as isize);
    *gz
        .offset(
            di as isize,
        ) = rijrx[2 as usize] * *gz.offset(0 as isize);
    let mut aij2: libc::c_double = 0.5f64 / aij;
    i = 1 as i32;
    while i < nmax {
        *gx
            .offset(
                ((i + 1 as i32) * di) as isize,
            ) = i as libc::c_double * aij2
            * *gx.offset(((i - 1 as i32) * di) as isize)
            + rijrx[0 as usize] * *gx.offset((i * di) as isize);
        *gy
            .offset(
                ((i + 1 as i32) * di) as isize,
            ) = i as libc::c_double * aij2
            * *gy.offset(((i - 1 as i32) * di) as isize)
            + rijrx[1 as usize] * *gy.offset((i * di) as isize);
        *gz
            .offset(
                ((i + 1 as i32) * di) as isize,
            ) = i as libc::c_double * aij2
            * *gz.offset(((i - 1 as i32) * di) as isize)
            + rijrx[2 as usize] * *gz.offset((i * di) as isize);
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= lj {
        ptr = dj * j;
        i = 0 as i32;
        n = ptr;
        while i <= nmax - j {
            *gx
                .offset(
                    n as isize,
                ) = *gx.offset((n + di - dj) as isize)
                + *rirj.offset(0 as isize)
                    * *gx.offset((n - dj) as isize);
            *gy
                .offset(
                    n as isize,
                ) = *gy.offset((n + di - dj) as isize)
                + *rirj.offset(1 as isize)
                    * *gy.offset((n - dj) as isize);
            *gz
                .offset(
                    n as isize,
                ) = *gz.offset((n + di - dj) as isize)
                + *rirj.offset(2 as isize)
                    * *gz.offset((n - dj) as isize);
            i += 1;
            i;
            n += di;
        }
        j += 1;
        j;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn CINTnuc_mod(
    mut aij: libc::c_double,
    mut nuc_id: i32,
    mut atm: *mut i32,
    mut env: *mut libc::c_double,
) -> libc::c_double {
    let mut zeta: libc::c_double = 0.;
    if nuc_id < 0 as i32 {
        zeta = *env.offset(7 as isize);
    } else if *atm.offset((6 as i32 * nuc_id + 2 as i32) as isize)
        == 2 as i32
    {
        zeta = *env
            .offset(
                *atm.offset((6 as i32 * nuc_id + 3 as i32) as isize)
                    as isize,
            );
    } else {
        zeta = 0 as libc::c_double;
    }
    if zeta > 0 as libc::c_double {
        return sqrt(zeta / (aij + zeta))
    } else {
        return 1 as libc::c_double
    };
}
#[no_mangle]
pub unsafe extern "C" fn CINTg1e_nuc(
    mut g: *mut libc::c_double,
    mut envs: *mut CINTEnvVars,
    mut nuc_id: i32,
) -> i32 {
    let mut nrys_roots: i32 = (*envs).nrys_roots;
    let mut atm: *mut i32 = (*envs).atm;
    let mut env: *mut libc::c_double = (*envs).env;
    let mut rij: *mut libc::c_double = ((*envs).rij).as_mut_ptr();
    let mut gx: *mut libc::c_double = g;
    let mut gy: *mut libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *mut libc::c_double = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut u: [libc::c_double; 32] = [0.; 32];
    let mut w: *mut libc::c_double = gz;
    let mut cr: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    let mut crij: [libc::c_double; 3] = [0.; 3];
    let mut x: libc::c_double = 0.;
    let mut fac1: libc::c_double = 0.;
    let mut aij: libc::c_double = (*envs).ai[0 as usize]
        + (*envs).aj[0 as usize];
    let mut tau: libc::c_double = CINTnuc_mod(aij, nuc_id, atm, env);
    if nuc_id < 0 as i32 {
        fac1 = 2 as libc::c_double * 3.14159265358979323846f64
            * (*envs).fac[0 as usize] * tau / aij;
        cr = env.offset(4 as isize);
    } else if *atm.offset((6 as i32 * nuc_id + 2 as i32) as isize)
        == 3 as i32
    {
        fac1 = 2 as libc::c_double * 3.14159265358979323846f64
            * -*env
                .offset(
                    *atm.offset((4 as i32 + nuc_id * 6 as i32) as isize)
                        as isize,
                ) * (*envs).fac[0 as usize] * tau / aij;
        cr = env
            .offset(
                *atm.offset((6 as i32 * nuc_id + 1 as i32) as isize)
                    as isize,
            );
    } else {
        fac1 = 2 as libc::c_double * 3.14159265358979323846f64
            * -abs(*atm.offset((0 as i32 + nuc_id * 6 as i32) as isize))
                as libc::c_double * (*envs).fac[0 as usize] * tau / aij;
        cr = env
            .offset(
                *atm.offset((6 as i32 * nuc_id + 1 as i32) as isize)
                    as isize,
            );
    }
    crij[0 as i32
        as usize] = *cr.offset(0 as isize)
        - *rij.offset(0 as isize);
    crij[1 as i32
        as usize] = *cr.offset(1 as isize)
        - *rij.offset(1 as isize);
    crij[2 as i32
        as usize] = *cr.offset(2 as isize)
        - *rij.offset(2 as isize);
    x = aij * tau * tau * SQUARE(crij.as_mut_ptr()) as libc::c_double;
    CINTrys_roots(nrys_roots, x, u.as_mut_ptr(), w);
    i = 0 as i32;
    while i < nrys_roots {
        *gx.offset(i as isize) = 1 as libc::c_double;
        *gy.offset(i as isize) = 1 as libc::c_double;
        *gz.offset(i as isize) *= fac1;
        i += 1;
        i;
    }
    let mut nmax: i32 = (*envs).li_ceil + (*envs).lj_ceil;
    if nmax == 0 as i32 {
        return 1 as i32;
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
    let mut lj: i32 = 0;
    let mut di: i32 = 0;
    let mut dj: i32 = 0;
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
    let mut rijrx: libc::c_double = *rij.offset(0 as isize)
        - *rx.offset(0 as isize);
    let mut rijry: libc::c_double = *rij.offset(1 as isize)
        - *rx.offset(1 as isize);
    let mut rijrz: libc::c_double = *rij.offset(2 as isize)
        - *rx.offset(2 as isize);
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
    n = 0 as i32;
    while n < nrys_roots {
        ru = tau * tau * u[n as usize]
            / (1 as libc::c_double + u[n as usize]);
        rt = aij2 - aij2 * ru;
        r0 = rijrx + ru * crij[0 as usize];
        r1 = rijry + ru * crij[1 as usize];
        r2 = rijrz + ru * crij[2 as usize];
        *p0x.offset(n as isize) = r0 * *gx.offset(n as isize);
        *p0y.offset(n as isize) = r1 * *gy.offset(n as isize);
        *p0z.offset(n as isize) = r2 * *gz.offset(n as isize);
        i = 1 as i32;
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
    let mut rirjx: libc::c_double = (*envs).rirj[0 as usize];
    let mut rirjy: libc::c_double = (*envs).rirj[1 as usize];
    let mut rirjz: libc::c_double = (*envs).rirj[2 as usize];
    j = 1 as i32;
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
        i = 0 as i32;
        while i <= nmax - j {
            n = 0 as i32;
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
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1i_1e(
    mut f: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut li: i32,
    mut lj: i32,
    mut lk: i32,
    mut envs: *mut CINTEnvVars,
) {
    let dj: i32 = (*envs).g_stride_j;
    let dk: i32 = (*envs).g_stride_k;
    let ai2: libc::c_double = -(2 as i32) as libc::c_double
        * (*envs).ai[0 as usize];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let mut gx: *const libc::c_double = g;
    let mut gy: *const libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const libc::c_double = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut fx: *mut libc::c_double = f;
    let mut fy: *mut libc::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut libc::c_double = f
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
                    ) = i as libc::c_double
                    * *gx.offset((ptr + i - 1 as i32) as isize)
                    + ai2 * *gx.offset((ptr + i + 1 as i32) as isize);
                *fy
                    .offset(
                        (ptr + i) as isize,
                    ) = i as libc::c_double
                    * *gy.offset((ptr + i - 1 as i32) as isize)
                    + ai2 * *gy.offset((ptr + i + 1 as i32) as isize);
                *fz
                    .offset(
                        (ptr + i) as isize,
                    ) = i as libc::c_double
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
pub unsafe extern "C" fn CINTnabla1j_1e(
    mut f: *mut libc::c_double,
    mut g: *mut libc::c_double,
    mut li: i32,
    mut lj: i32,
    mut lk: i32,
    mut envs: *mut CINTEnvVars,
) {
    let dj: i32 = (*envs).g_stride_j;
    let dk: i32 = (*envs).g_stride_k;
    let aj2: libc::c_double = -(2 as i32) as libc::c_double
        * (*envs).aj[0 as usize];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let mut gx: *const libc::c_double = g;
    let mut gy: *const libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const libc::c_double = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut fx: *mut libc::c_double = f;
    let mut fy: *mut libc::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut libc::c_double = f
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
    mut li: i32,
    mut lj: i32,
    mut lk: i32,
    mut envs: *mut CINTEnvVars,
) {
    let dj: i32 = (*envs).g_stride_j;
    let dk: i32 = (*envs).g_stride_k;
    let ak2: libc::c_double = -(2 as i32) as libc::c_double
        * (*envs).ak[0 as usize];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let mut gx: *const libc::c_double = g;
    let mut gy: *const libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const libc::c_double = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut fx: *mut libc::c_double = f;
    let mut fy: *mut libc::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut libc::c_double = f
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
    mut li: i32,
    mut lj: i32,
    mut lk: i32,
    mut envs: *mut CINTEnvVars,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let dj: i32 = (*envs).g_stride_j;
    let dk: i32 = (*envs).g_stride_k;
    let mut gx: *const libc::c_double = g;
    let mut gy: *const libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const libc::c_double = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut fx: *mut libc::c_double = f;
    let mut fy: *mut libc::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut libc::c_double = f
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
                    + *ri.offset(0 as isize) * *gx.offset(i as isize);
                *fy
                    .offset(
                        i as isize,
                    ) = *gy.offset((i + 1 as i32) as isize)
                    + *ri.offset(1 as isize) * *gy.offset(i as isize);
                *fz
                    .offset(
                        i as isize,
                    ) = *gz.offset((i + 1 as i32) as isize)
                    + *ri.offset(2 as isize) * *gz.offset(i as isize);
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
    mut li: i32,
    mut lj: i32,
    mut lk: i32,
    mut envs: *mut CINTEnvVars,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let dj: i32 = (*envs).g_stride_j;
    let dk: i32 = (*envs).g_stride_k;
    let mut gx: *const libc::c_double = g;
    let mut gy: *const libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const libc::c_double = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut fx: *mut libc::c_double = f;
    let mut fy: *mut libc::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut libc::c_double = f
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
                    + *rj.offset(0 as isize) * *gx.offset(i as isize);
                *fy
                    .offset(
                        i as isize,
                    ) = *gy.offset((i + dj) as isize)
                    + *rj.offset(1 as isize) * *gy.offset(i as isize);
                *fz
                    .offset(
                        i as isize,
                    ) = *gz.offset((i + dj) as isize)
                    + *rj.offset(2 as isize) * *gz.offset(i as isize);
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
    mut li: i32,
    mut lj: i32,
    mut lk: i32,
    mut envs: *mut CINTEnvVars,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let dj: i32 = (*envs).g_stride_j;
    let dk: i32 = (*envs).g_stride_k;
    let mut gx: *const libc::c_double = g;
    let mut gy: *const libc::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const libc::c_double = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut fx: *mut libc::c_double = f;
    let mut fy: *mut libc::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut libc::c_double = f
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
                    + *rk.offset(0 as isize) * *gx.offset(i as isize);
                *fy
                    .offset(
                        i as isize,
                    ) = *gy.offset((i + dk) as isize)
                    + *rk.offset(1 as isize) * *gy.offset(i as isize);
                *fz
                    .offset(
                        i as isize,
                    ) = *gz.offset((i + dk) as isize)
                    + *rk.offset(2 as isize) * *gz.offset(i as isize);
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
    mut nf: i32,
    mut gp: *mut libc::c_double,
    mut inc: i32,
    mut nprim: i32,
    mut nctr: i32,
    mut coeff: *mut libc::c_double,
) {
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut pgc: *mut libc::c_double = gc;
    let mut c: libc::c_double = 0.;
    i = 0 as i32;
    while i < inc {
        n = 0 as i32;
        while n < nctr {
            c = *coeff.offset((nprim * n) as isize);
            if c != 0 as libc::c_double {
                k = 0 as i32;
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
    mut nprim: i32,
    mut nctr: i32,
    mut non0ctr: i32,
    mut sortedidx: *mut i32,
) {
    let mut i: i32 = 0;
    let mut n: size_t = 0;
    let mut c0: libc::c_double = 0.;
    i = 0 as i32;
    while i < nctr {
        c0 = *coeff.offset((nprim * i) as isize);
        n = 0 as size_t;
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
    mut nprim: i32,
    mut nctr: i32,
    mut non0ctr: i32,
    mut sortedidx: *mut i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: size_t = 0;
    let mut c0: libc::c_double = 0.;
    i = 0 as i32;
    while i < non0ctr {
        c0 = *coeff.offset((nprim * *sortedidx.offset(i as isize)) as isize);
        j = *sortedidx.offset(i as isize);
        n = 0 as size_t;
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
pub unsafe extern "C" fn CINTcommon_fac_sp(mut l: i32) -> libc::c_double {
    match l {
        0 => return 0.282094791773878143f64,
        1 => return 0.488602511902919921f64,
        _ => return 1 as libc::c_double,
    };
}
