#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments
)]

use crate::cint_bas::CINTcart_comp;
use crate::rys_roots::CINTrys_roots;

use crate::cint::CINTEnvVars;

fn MAX<T: PartialOrd>(x: T, y: T) -> T {
    if x > y {
        x
    } else {
        y
    }
}

fn SQUARE(r: *mut f64) -> f64 {
    unsafe { (*r.add(0) * *r.add(0)) + (*r.add(1) * *r.add(1)) + (*r.add(2) * *r.add(2)) }
}

#[no_mangle]
pub unsafe extern "C" fn CINTinit_int1e_EnvVars(
    envs: *mut CINTEnvVars,
    ng: &[i32; 8],
    shls: *mut i32,
    atm: *mut i32,
    natm: i32,
    bas: *mut i32,
    nbas: i32,
    env: *mut f64,
) {
    (*envs).natm = natm;
    (*envs).nbas = nbas;
    (*envs).atm = atm;
    (*envs).bas = bas;
    (*envs).env = env;
    (*envs).shls = shls;
    let i_sh: i32 = *shls.offset(0_isize);
    let j_sh: i32 = *shls.offset(1_isize);
    (*envs).i_l = *bas.offset((8_i32 * i_sh + 1_i32) as isize);
    (*envs).j_l = *bas.offset((8_i32 * j_sh + 1_i32) as isize);
    (*envs).x_ctr[0_i32 as usize] = *bas.offset((8_i32 * i_sh + 3_i32) as isize);
    (*envs).x_ctr[1_i32 as usize] = *bas.offset((8_i32 * j_sh + 3_i32) as isize);
    (*envs).nfi = ((*envs).i_l + 1_i32) * ((*envs).i_l + 2_i32) / 2_i32;
    (*envs).nfj = ((*envs).j_l + 1_i32) * ((*envs).j_l + 2_i32) / 2_i32;
    (*envs).nf = (*envs).nfi * (*envs).nfj;
    (*envs).common_factor = 1_f64;
    if *env.offset(0_isize) == 0 as f64 {
        (*envs).expcutoff = 60_f64;
    } else {
        (*envs).expcutoff = MAX(40_f64, *env.offset(0_isize));
    }
    (*envs).li_ceil = (*envs).i_l + ng[0];
    (*envs).lj_ceil = (*envs).j_l + ng[1];
    (*envs).ri =
        env.offset(*atm.offset(
            (6_i32 * *bas.offset((8_i32 * i_sh + 0_i32) as isize) + 1_i32) as isize,
        ) as isize);
    (*envs).rj =
        env.offset(*atm.offset(
            (6_i32 * *bas.offset((8_i32 * j_sh + 0_i32) as isize) + 1_i32) as isize,
        ) as isize);
    (*envs).gbits = ng[4];
    (*envs).ncomp_e1 = ng[5];
    (*envs).ncomp_tensor = ng[7];
    if ng[6] > 0_i32 {
        (*envs).nrys_roots = ng[6];
    } else {
        (*envs).nrys_roots = ((*envs).li_ceil + (*envs).lj_ceil) / 2_i32 + 1_i32;
    }
    let mut dli: i32 = 0;
    let mut dlj: i32 = 0;
    let ibase: i32 = ((*envs).li_ceil > (*envs).lj_ceil) as i32;
    if ibase != 0 {
        dli = (*envs).li_ceil + (*envs).lj_ceil + 1_i32;
        dlj = (*envs).lj_ceil + 1_i32;
        (*envs).rirj[0_i32 as usize] =
            *((*envs).ri).offset(0_isize) - *((*envs).rj).offset(0_isize);
        (*envs).rirj[1_i32 as usize] =
            *((*envs).ri).offset(1_isize) - *((*envs).rj).offset(1_isize);
        (*envs).rirj[2_i32 as usize] =
            *((*envs).ri).offset(2_isize) - *((*envs).rj).offset(2_isize);
    } else {
        dli = (*envs).li_ceil + 1_i32;
        dlj = (*envs).li_ceil + (*envs).lj_ceil + 1_i32;
        (*envs).rirj[0_i32 as usize] =
            *((*envs).rj).offset(0_isize) - *((*envs).ri).offset(0_isize);
        (*envs).rirj[1_i32 as usize] =
            *((*envs).rj).offset(1_isize) - *((*envs).ri).offset(1_isize);
        (*envs).rirj[2_i32 as usize] =
            *((*envs).rj).offset(2_isize) - *((*envs).ri).offset(2_isize);
    }
    (*envs).g_stride_i = (*envs).nrys_roots;
    (*envs).g_stride_j = (*envs).nrys_roots * dli;
    (*envs).g_size = (*envs).nrys_roots * dli * dlj;
    (*envs).g_stride_k = (*envs).g_size;
    (*envs).g_stride_l = (*envs).g_size;
}
#[no_mangle]
pub unsafe extern "C" fn CINTg1e_index_xyz(idx: *mut i32, envs: *mut CINTEnvVars) {
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
    ofx = 0_i32;
    ofy = (*envs).g_size;
    ofz = (*envs).g_size * 2_i32;
    n = 0_i32;
    j = 0_i32;
    while j < nfj {
        ofjx = ofx + dj * j_nx[j as usize];
        ofjy = ofy + dj * j_ny[j as usize];
        ofjz = ofz + dj * j_nz[j as usize];
        i = 0_i32;
        while i < nfi {
            *idx.offset((n + 0_i32) as isize) = ofjx + di * i_nx[i as usize];
            *idx.offset((n + 1_i32) as isize) = ofjy + di * i_ny[i as usize];
            *idx.offset((n + 2_i32) as isize) = ofjz + di * i_nz[i as usize];
            n += 3_i32;
            i += 1;
            i;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTg1e_ovlp(g: *mut f64, envs: *mut CINTEnvVars) -> i32 {
    let gx: *mut f64 = g;
    let gy: *mut f64 = g.offset((*envs).g_size as isize);
    let gz: *mut f64 = g.offset(((*envs).g_size * 2_i32) as isize);
    let aij: f64 = (*envs).ai[0_usize] + (*envs).aj[0_usize];
    *gx.offset(0_isize) = 1_f64;
    *gy.offset(0_isize) = 1_f64;
    *gz.offset(0_isize) = (*envs).fac[0_usize]
        * 1.772_453_850_905_516_f64
        * 3.141_592_653_589_793_f64
        / (aij * (aij).sqrt());
    let nmax: i32 = (*envs).li_ceil + (*envs).lj_ceil;
    if nmax == 0_i32 {
        return 1_i32;
    }
    let rij: *mut f64 = ((*envs).rij).as_mut_ptr();
    let rirj: *mut f64 = ((*envs).rirj).as_mut_ptr();
    let mut lj: i32 = 0;
    let mut di: i32 = 0;
    let mut dj: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    let mut ptr: i32 = 0;
    let mut rx: *mut f64 = std::ptr::null_mut::<f64>();
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
    let mut rijrx: [f64; 3] = [0.; 3];
    rijrx[0_i32 as usize] = *rij.offset(0_isize) - *rx.offset(0_isize);
    rijrx[1_i32 as usize] = *rij.offset(1_isize) - *rx.offset(1_isize);
    rijrx[2_i32 as usize] = *rij.offset(2_isize) - *rx.offset(2_isize);
    *gx.offset(di as isize) = rijrx[0_usize] * *gx.offset(0_isize);
    *gy.offset(di as isize) = rijrx[1_usize] * *gy.offset(0_isize);
    *gz.offset(di as isize) = rijrx[2_usize] * *gz.offset(0_isize);
    let aij2: f64 = 0.5f64 / aij;
    i = 1_i32;
    while i < nmax {
        *gx.offset(((i + 1_i32) * di) as isize) =
            i as f64 * aij2 * *gx.offset(((i - 1_i32) * di) as isize)
                + rijrx[0_usize] * *gx.offset((i * di) as isize);
        *gy.offset(((i + 1_i32) * di) as isize) =
            i as f64 * aij2 * *gy.offset(((i - 1_i32) * di) as isize)
                + rijrx[1_usize] * *gy.offset((i * di) as isize);
        *gz.offset(((i + 1_i32) * di) as isize) =
            i as f64 * aij2 * *gz.offset(((i - 1_i32) * di) as isize)
                + rijrx[2_usize] * *gz.offset((i * di) as isize);
        i += 1;
        i;
    }
    j = 1_i32;
    while j <= lj {
        ptr = dj * j;
        i = 0_i32;
        n = ptr;
        while i <= nmax - j {
            *gx.offset(n as isize) = *gx.offset((n + di - dj) as isize)
                + *rirj.offset(0_isize) * *gx.offset((n - dj) as isize);
            *gy.offset(n as isize) = *gy.offset((n + di - dj) as isize)
                + *rirj.offset(1_isize) * *gy.offset((n - dj) as isize);
            *gz.offset(n as isize) = *gz.offset((n + di - dj) as isize)
                + *rirj.offset(2_isize) * *gz.offset((n - dj) as isize);
            i += 1;
            i;
            n += di;
        }
        j += 1;
        j;
    }
    1_i32
}
#[no_mangle]
pub unsafe extern "C" fn CINTnuc_mod(
    aij: f64,
    nuc_id: i32,
    atm: *mut i32,
    env: *mut f64,
) -> f64 {
    let mut zeta: f64 = 0.;
    if nuc_id < 0_i32 {
        zeta = *env.offset(7_isize);
    } else if *atm.offset((6_i32 * nuc_id + 2_i32) as isize) == 2_i32 {
        zeta = *env.offset(*atm.offset((6_i32 * nuc_id + 3_i32) as isize) as isize);
    } else {
        zeta = 0 as f64;
    }
    if zeta > 0 as f64 {
        (zeta / (aij + zeta)).sqrt()
    } else {
        1_f64
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTg1e_nuc(
    g: *mut f64,
    envs: *mut CINTEnvVars,
    nuc_id: i32,
) -> i32 {
    let nrys_roots: i32 = (*envs).nrys_roots;
    let atm: *mut i32 = (*envs).atm;
    let env: *mut f64 = (*envs).env;
    let rij: *mut f64 = ((*envs).rij).as_mut_ptr();
    let gx: *mut f64 = g;
    let gy: *mut f64 = g.offset((*envs).g_size as isize);
    let gz: *mut f64 = g.offset(((*envs).g_size * 2_i32) as isize);
    let mut u: [f64; 32] = [0.; 32];
    let w: *mut f64 = gz;
    let mut cr: *mut f64 = std::ptr::null_mut::<f64>();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    let mut crij: [f64; 3] = [0.; 3];
    let mut x: f64 = 0.;
    let mut fac1: f64 = 0.;
    let aij: f64 = (*envs).ai[0_usize] + (*envs).aj[0_usize];
    let tau: f64 = CINTnuc_mod(aij, nuc_id, atm, env);
    if nuc_id < 0_i32 {
        fac1 = 2_f64 * 3.141_592_653_589_793_f64 * (*envs).fac[0_usize] * tau / aij;
        cr = env.offset(4_isize);
    } else if *atm.offset((6_i32 * nuc_id + 2_i32) as isize) == 3_i32 {
        fac1 = 2_f64
            * 3.141_592_653_589_793_f64
            * -*env.offset(*atm.offset((4_i32 + nuc_id * 6_i32) as isize) as isize)
            * (*envs).fac[0_usize]
            * tau
            / aij;
        cr = env.offset(*atm.offset((6_i32 * nuc_id + 1_i32) as isize) as isize);
    } else {
        fac1 = 2_f64
            * 3.141_592_653_589_793_f64
            * -(*atm.offset((0_i32 + nuc_id * 6_i32) as isize)).abs() as f64
            * (*envs).fac[0_usize]
            * tau
            / aij;
        cr = env.offset(*atm.offset((6_i32 * nuc_id + 1_i32) as isize) as isize);
    }
    crij[0_i32 as usize] = *cr.offset(0_isize) - *rij.offset(0_isize);
    crij[1_i32 as usize] = *cr.offset(1_isize) - *rij.offset(1_isize);
    crij[2_i32 as usize] = *cr.offset(2_isize) - *rij.offset(2_isize);
    x = aij * tau * tau * SQUARE(crij.as_mut_ptr());
    CINTrys_roots(nrys_roots, x, u.as_mut_ptr(), w);
    i = 0_i32;
    while i < nrys_roots {
        *gx.offset(i as isize) = 1_f64;
        *gy.offset(i as isize) = 1_f64;
        *gz.offset(i as isize) *= fac1;
        i += 1;
        i;
    }
    let nmax: i32 = (*envs).li_ceil + (*envs).lj_ceil;
    if nmax == 0_i32 {
        return 1_i32;
    }
    let mut p0x: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p0y: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p0z: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p1x: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p1y: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p1z: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p2x: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p2y: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p2z: *mut f64 = std::ptr::null_mut::<f64>();
    let mut lj: i32 = 0;
    let mut di: i32 = 0;
    let mut dj: i32 = 0;
    let mut rx: *mut f64 = std::ptr::null_mut::<f64>();
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
    let rijrx: f64 = *rij.offset(0_isize) - *rx.offset(0_isize);
    let rijry: f64 = *rij.offset(1_isize) - *rx.offset(1_isize);
    let rijrz: f64 = *rij.offset(2_isize) - *rx.offset(2_isize);
    let aij2: f64 = 0.5f64 / aij;
    let mut ru: f64 = 0.;
    let mut rt: f64 = 0.;
    let mut r0: f64 = 0.;
    let mut r1: f64 = 0.;
    let mut r2: f64 = 0.;
    p0x = gx.offset(di as isize);
    p0y = gy.offset(di as isize);
    p0z = gz.offset(di as isize);
    p1x = gx.offset(-(di as isize));
    p1y = gy.offset(-(di as isize));
    p1z = gz.offset(-(di as isize));
    n = 0_i32;
    while n < nrys_roots {
        ru = tau * tau * u[n as usize] / (1_f64 + u[n as usize]);
        rt = aij2 - aij2 * ru;
        r0 = rijrx + ru * crij[0_usize];
        r1 = rijry + ru * crij[1_usize];
        r2 = rijrz + ru * crij[2_usize];
        *p0x.offset(n as isize) = r0 * *gx.offset(n as isize);
        *p0y.offset(n as isize) = r1 * *gy.offset(n as isize);
        *p0z.offset(n as isize) = r2 * *gz.offset(n as isize);
        i = 1_i32;
        while i < nmax {
            *p0x.offset((n + i * di) as isize) = i as f64 * rt * *p1x.offset((n + i * di) as isize)
                + r0 * *gx.offset((n + i * di) as isize);
            *p0y.offset((n + i * di) as isize) = i as f64 * rt * *p1y.offset((n + i * di) as isize)
                + r1 * *gy.offset((n + i * di) as isize);
            *p0z.offset((n + i * di) as isize) = i as f64 * rt * *p1z.offset((n + i * di) as isize)
                + r2 * *gz.offset((n + i * di) as isize);
            i += 1;
            i;
        }
        n += 1;
        n;
    }
    let rirjx: f64 = (*envs).rirj[0_usize];
    let rirjy: f64 = (*envs).rirj[1_usize];
    let rirjz: f64 = (*envs).rirj[2_usize];
    j = 1_i32;
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
        i = 0_i32;
        while i <= nmax - j {
            n = 0_i32;
            while n < nrys_roots {
                *p0x.offset((n + i * di) as isize) =
                    *p2x.offset((n + i * di) as isize) + rirjx * *p1x.offset((n + i * di) as isize);
                *p0y.offset((n + i * di) as isize) =
                    *p2y.offset((n + i * di) as isize) + rirjy * *p1y.offset((n + i * di) as isize);
                *p0z.offset((n + i * di) as isize) =
                    *p2z.offset((n + i * di) as isize) + rirjz * *p1z.offset((n + i * di) as isize);
                n += 1;
                n;
            }
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    1_i32
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1i_1e(
    f: *mut f64,
    g: *mut f64,
    li: i32,
    lj: i32,
    lk: i32,
    envs: *mut CINTEnvVars,
) {
    let dj: i32 = (*envs).g_stride_j;
    let dk: i32 = (*envs).g_stride_k;
    let ai2: f64 = -2_f64 * (*envs).ai[0_usize];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let gx: *const f64 = g;
    let gy: *const f64 = g.offset((*envs).g_size as isize);
    let gz: *const f64 = g.offset(((*envs).g_size * 2_i32) as isize);
    let fx: *mut f64 = f;
    let fy: *mut f64 = f.offset((*envs).g_size as isize);
    let fz: *mut f64 = f.offset(((*envs).g_size * 2_i32) as isize);
    k = 0_i32;
    while k <= lk {
        j = 0_i32;
        while j <= lj {
            ptr = dj * j + dk * k;
            *fx.offset(ptr as isize) = ai2 * *gx.offset((ptr + 1_i32) as isize);
            *fy.offset(ptr as isize) = ai2 * *gy.offset((ptr + 1_i32) as isize);
            *fz.offset(ptr as isize) = ai2 * *gz.offset((ptr + 1_i32) as isize);
            i = 1_i32;
            while i <= li {
                *fx.offset((ptr + i) as isize) = i as f64
                    * *gx.offset((ptr + i - 1_i32) as isize)
                    + ai2 * *gx.offset((ptr + i + 1_i32) as isize);
                *fy.offset((ptr + i) as isize) = i as f64
                    * *gy.offset((ptr + i - 1_i32) as isize)
                    + ai2 * *gy.offset((ptr + i + 1_i32) as isize);
                *fz.offset((ptr + i) as isize) = i as f64
                    * *gz.offset((ptr + i - 1_i32) as isize)
                    + ai2 * *gz.offset((ptr + i + 1_i32) as isize);
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
    f: *mut f64,
    g: *mut f64,
    li: i32,
    lj: i32,
    lk: i32,
    envs: *mut CINTEnvVars,
) {
    let dj: i32 = (*envs).g_stride_j;
    let dk: i32 = (*envs).g_stride_k;
    let aj2: f64 = -2_f64 * (*envs).aj[0_usize];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let gx: *const f64 = g;
    let gy: *const f64 = g.offset((*envs).g_size as isize);
    let gz: *const f64 = g.offset(((*envs).g_size * 2_i32) as isize);
    let fx: *mut f64 = f;
    let fy: *mut f64 = f.offset((*envs).g_size as isize);
    let fz: *mut f64 = f.offset(((*envs).g_size * 2_i32) as isize);
    k = 0_i32;
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
        j = 1_i32;
        while j <= lj {
            ptr = dj * j + dk * k;
            i = ptr;
            while i <= ptr + li {
                *fx.offset(i as isize) =
                    j as f64 * *gx.offset((i - dj) as isize) + aj2 * *gx.offset((i + dj) as isize);
                *fy.offset(i as isize) =
                    j as f64 * *gy.offset((i - dj) as isize) + aj2 * *gy.offset((i + dj) as isize);
                *fz.offset(i as isize) =
                    j as f64 * *gz.offset((i - dj) as isize) + aj2 * *gz.offset((i + dj) as isize);
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
    f: *mut f64,
    g: *mut f64,
    li: i32,
    lj: i32,
    lk: i32,
    envs: *mut CINTEnvVars,
) {
    let dj: i32 = (*envs).g_stride_j;
    let dk: i32 = (*envs).g_stride_k;
    let ak2: f64 = -2_f64 * (*envs).ak[0_usize];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let gx: *const f64 = g;
    let gy: *const f64 = g.offset((*envs).g_size as isize);
    let gz: *const f64 = g.offset(((*envs).g_size * 2_i32) as isize);
    let fx: *mut f64 = f;
    let fy: *mut f64 = f.offset((*envs).g_size as isize);
    let fz: *mut f64 = f.offset(((*envs).g_size * 2_i32) as isize);
    j = 0_i32;
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
    k = 1_i32;
    while k <= lk {
        j = 0_i32;
        while j <= lj {
            ptr = dj * j + dk * k;
            i = ptr;
            while i <= ptr + li {
                *fx.offset(i as isize) =
                    k as f64 * *gx.offset((i - dk) as isize) + ak2 * *gx.offset((i + dk) as isize);
                *fy.offset(i as isize) =
                    k as f64 * *gy.offset((i - dk) as isize) + ak2 * *gy.offset((i + dk) as isize);
                *fz.offset(i as isize) =
                    k as f64 * *gz.offset((i - dk) as isize) + ak2 * *gz.offset((i + dk) as isize);
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
    f: *mut f64,
    g: *mut f64,
    ri: *mut f64,
    li: i32,
    lj: i32,
    lk: i32,
    envs: *mut CINTEnvVars,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let dj: i32 = (*envs).g_stride_j;
    let dk: i32 = (*envs).g_stride_k;
    let gx: *const f64 = g;
    let gy: *const f64 = g.offset((*envs).g_size as isize);
    let gz: *const f64 = g.offset(((*envs).g_size * 2_i32) as isize);
    let fx: *mut f64 = f;
    let fy: *mut f64 = f.offset((*envs).g_size as isize);
    let fz: *mut f64 = f.offset(((*envs).g_size * 2_i32) as isize);
    k = 0_i32;
    while k <= lk {
        j = 0_i32;
        while j <= lj {
            ptr = dj * j + dk * k;
            i = ptr;
            while i <= ptr + li {
                *fx.offset(i as isize) = *gx.offset((i + 1_i32) as isize)
                    + *ri.offset(0_isize) * *gx.offset(i as isize);
                *fy.offset(i as isize) = *gy.offset((i + 1_i32) as isize)
                    + *ri.offset(1_isize) * *gy.offset(i as isize);
                *fz.offset(i as isize) = *gz.offset((i + 1_i32) as isize)
                    + *ri.offset(2_isize) * *gz.offset(i as isize);
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
    f: *mut f64,
    g: *mut f64,
    rj: *mut f64,
    li: i32,
    lj: i32,
    lk: i32,
    envs: *mut CINTEnvVars,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let dj: i32 = (*envs).g_stride_j;
    let dk: i32 = (*envs).g_stride_k;
    let gx: *const f64 = g;
    let gy: *const f64 = g.offset((*envs).g_size as isize);
    let gz: *const f64 = g.offset(((*envs).g_size * 2_i32) as isize);
    let fx: *mut f64 = f;
    let fy: *mut f64 = f.offset((*envs).g_size as isize);
    let fz: *mut f64 = f.offset(((*envs).g_size * 2_i32) as isize);
    k = 0_i32;
    while k <= lk {
        j = 0_i32;
        while j <= lj {
            ptr = dj * j + dk * k;
            i = ptr;
            while i <= ptr + li {
                *fx.offset(i as isize) =
                    *gx.offset((i + dj) as isize) + *rj.offset(0_isize) * *gx.offset(i as isize);
                *fy.offset(i as isize) =
                    *gy.offset((i + dj) as isize) + *rj.offset(1_isize) * *gy.offset(i as isize);
                *fz.offset(i as isize) =
                    *gz.offset((i + dj) as isize) + *rj.offset(2_isize) * *gz.offset(i as isize);
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
    f: *mut f64,
    g: *mut f64,
    rk: *mut f64,
    li: i32,
    lj: i32,
    lk: i32,
    envs: *mut CINTEnvVars,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let dj: i32 = (*envs).g_stride_j;
    let dk: i32 = (*envs).g_stride_k;
    let gx: *const f64 = g;
    let gy: *const f64 = g.offset((*envs).g_size as isize);
    let gz: *const f64 = g.offset(((*envs).g_size * 2_i32) as isize);
    let fx: *mut f64 = f;
    let fy: *mut f64 = f.offset((*envs).g_size as isize);
    let fz: *mut f64 = f.offset(((*envs).g_size * 2_i32) as isize);
    k = 0_i32;
    while k <= lk {
        j = 0_i32;
        while j <= lj {
            ptr = dj * j + dk * k;
            i = ptr;
            while i <= ptr + li {
                *fx.offset(i as isize) =
                    *gx.offset((i + dk) as isize) + *rk.offset(0_isize) * *gx.offset(i as isize);
                *fy.offset(i as isize) =
                    *gy.offset((i + dk) as isize) + *rk.offset(1_isize) * *gy.offset(i as isize);
                *fz.offset(i as isize) =
                    *gz.offset((i + dk) as isize) + *rk.offset(2_isize) * *gz.offset(i as isize);
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
    gc: *mut f64,
    nf: i32,
    gp: *mut f64,
    inc: i32,
    nprim: i32,
    nctr: i32,
    coeff: *mut f64,
) {
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut pgc: *mut f64 = gc;
    let mut c: f64 = 0.;
    i = 0_i32;
    while i < inc {
        n = 0_i32;
        while n < nctr {
            c = *coeff.offset((nprim * n) as isize);
            if c != 0 as f64 {
                k = 0_i32;
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
    gc: *mut f64,
    gp: *mut f64,
    coeff: *mut f64,
    nf: u64,
    nprim: i32,
    nctr: i32,
    _non0ctr: i32,
    _sortedidx: *mut i32,
) {
    let mut i: i32 = 0;
    let mut n: u64 = 0;
    let mut c0: f64 = 0.;
    i = 0_i32;
    while i < nctr {
        c0 = *coeff.offset((nprim * i) as isize);
        n = 0_u64;
        while n < nf {
            *gc.offset(nf.wrapping_mul(i as libc::c_ulong).wrapping_add(n) as isize) =
                c0 * *gp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTprim_to_ctr_1(
    gc: *mut f64,
    gp: *mut f64,
    coeff: *mut f64,
    nf: u64,
    nprim: i32,
    _nctr: i32,
    non0ctr: i32,
    sortedidx: *mut i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: u64 = 0;
    let mut c0: f64 = 0.;
    i = 0_i32;
    while i < non0ctr {
        c0 = *coeff.offset((nprim * *sortedidx.offset(i as isize)) as isize);
        j = *sortedidx.offset(i as isize);
        n = 0_u64;
        while n < nf {
            *gc.offset(nf.wrapping_mul(j as libc::c_ulong).wrapping_add(n) as isize) +=
                c0 * *gp.offset(n as isize);
            n = n.wrapping_add(1);
            n;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTcommon_fac_sp(l: i32) -> f64 {
    match l {
        0 => 0.282_094_791_773_878_14_f64,
        1 => 0.488_602_511_902_919_9_f64,
        _ => 1_f64,
    }
}
