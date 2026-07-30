#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments
)]

use crate::cint_bas::CINTcart_comp;
use crate::g1e::CINTcommon_fac_sp;
use crate::rys_roots::CINTrys_roots;
use crate::rys_roots::CINTsr_rys_roots;

use crate::cint::CINTEnvVars;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Rys2eT {
    pub c00x: [f64; 32],
    pub c00y: [f64; 32],
    pub c00z: [f64; 32],
    pub c0px: [f64; 32],
    pub c0py: [f64; 32],
    pub c0pz: [f64; 32],
    pub b01: [f64; 32],
    pub b00: [f64; 32],
    pub b10: [f64; 32],
}
#[no_mangle]
pub unsafe extern "C" fn CINTinit_int2e_EnvVars(
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
    let k_sh: i32 = *shls.offset(2_isize);
    let l_sh: i32 = *shls.offset(3_isize);
    (*envs).i_l = *bas.offset((8_i32 * i_sh + 1_i32) as isize);
    (*envs).j_l = *bas.offset((8_i32 * j_sh + 1_i32) as isize);
    (*envs).k_l = *bas.offset((8_i32 * k_sh + 1_i32) as isize);
    (*envs).l_l = *bas.offset((8_i32 * l_sh + 1_i32) as isize);
    (*envs).x_ctr[0_i32 as usize] = *bas.offset((8_i32 * i_sh + 3_i32) as isize);
    (*envs).x_ctr[1_i32 as usize] = *bas.offset((8_i32 * j_sh + 3_i32) as isize);
    (*envs).x_ctr[2_i32 as usize] = *bas.offset((8_i32 * k_sh + 3_i32) as isize);
    (*envs).x_ctr[3_i32 as usize] = *bas.offset((8_i32 * l_sh + 3_i32) as isize);
    (*envs).nfi = ((*envs).i_l + 1_i32) * ((*envs).i_l + 2_i32) / 2_i32;
    (*envs).nfj = ((*envs).j_l + 1_i32) * ((*envs).j_l + 2_i32) / 2_i32;
    (*envs).c2rust_unnamed.nfk = ((*envs).k_l + 1_i32) * ((*envs).k_l + 2_i32) / 2_i32;
    (*envs).c2rust_unnamed_0.nfl = ((*envs).l_l + 1_i32) * ((*envs).l_l + 2_i32) / 2_i32;
    (*envs).nf =
        (*envs).nfi * (*envs).c2rust_unnamed.nfk * (*envs).c2rust_unnamed_0.nfl * (*envs).nfj;
    (*envs).ri =
        env.offset(*atm.offset(
            (6_i32 * *bas.offset((8_i32 * i_sh + 0_i32) as isize) + 1_i32) as isize,
        ) as isize);
    (*envs).rj =
        env.offset(*atm.offset(
            (6_i32 * *bas.offset((8_i32 * j_sh + 0_i32) as isize) + 1_i32) as isize,
        ) as isize);
    (*envs).rk =
        env.offset(*atm.offset(
            (6_i32 * *bas.offset((8_i32 * k_sh + 0_i32) as isize) + 1_i32) as isize,
        ) as isize);
    (*envs).c2rust_unnamed_1.rl =
        env.offset(*atm.offset(
            (6_i32 * *bas.offset((8_i32 * l_sh + 0_i32) as isize) + 1_i32) as isize,
        ) as isize);
    (*envs).common_factor = 3.141_592_653_589_793_f64
        * 3.141_592_653_589_793_f64
        * 3.141_592_653_589_793_f64
        * 2_f64
        / 1.772_453_850_905_516_f64
        * CINTcommon_fac_sp((*envs).i_l)
        * CINTcommon_fac_sp((*envs).j_l)
        * CINTcommon_fac_sp((*envs).k_l)
        * CINTcommon_fac_sp((*envs).l_l);
    if *env.offset(0_isize) == 0 as f64 {
        (*envs).expcutoff = 60_f64;
    } else {
        (*envs).expcutoff = (if 40_f64 > *env.offset(0_isize) {
            40_f64
        } else {
            *env.offset(0_isize)
        }) + 1_f64;
    }
    (*envs).gbits = ng[4];
    (*envs).ncomp_e1 = ng[5];
    (*envs).ncomp_e2 = ng[6];
    (*envs).ncomp_tensor = ng[7];
    (*envs).li_ceil = (*envs).i_l + ng[0];
    (*envs).lj_ceil = (*envs).j_l + ng[1];
    (*envs).lk_ceil = (*envs).k_l + ng[2];
    (*envs).ll_ceil = (*envs).l_l + ng[3];
    let rys_order: i32 =
        ((*envs).li_ceil + (*envs).lj_ceil + (*envs).lk_ceil + (*envs).ll_ceil) / 2_i32
            + 1_i32;
    let mut nrys_roots: i32 = rys_order;
    let omega: f64 = *env.offset(8_isize);
    if omega < 0 as f64 && rys_order <= 3_i32 {
        nrys_roots *= 2_i32;
    }
    (*envs).rys_order = rys_order;
    (*envs).nrys_roots = nrys_roots;
    let mut dli: i32 = 0;
    let mut dlj: i32 = 0;
    let mut dlk: i32 = 0;
    let mut dll: i32 = 0;
    let ibase: i32 = ((*envs).li_ceil > (*envs).lj_ceil) as i32;
    let kbase: i32 = ((*envs).lk_ceil > (*envs).ll_ceil) as i32;
    if kbase != 0 {
        dlk = (*envs).lk_ceil + (*envs).ll_ceil + 1_i32;
        dll = (*envs).ll_ceil + 1_i32;
    } else {
        dlk = (*envs).lk_ceil + 1_i32;
        dll = (*envs).lk_ceil + (*envs).ll_ceil + 1_i32;
    }
    if ibase != 0 {
        dli = (*envs).li_ceil + (*envs).lj_ceil + 1_i32;
        dlj = (*envs).lj_ceil + 1_i32;
    } else {
        dli = (*envs).li_ceil + 1_i32;
        dlj = (*envs).li_ceil + (*envs).lj_ceil + 1_i32;
    }
    (*envs).g_stride_i = nrys_roots;
    (*envs).g_stride_k = nrys_roots * dli;
    (*envs).g_stride_l = nrys_roots * dli * dlk;
    (*envs).g_stride_j = nrys_roots * dli * dlk * dll;
    (*envs).g_size = nrys_roots * dli * dlk * dll * dlj;
    if kbase != 0 {
        (*envs).g2d_klmax = (*envs).g_stride_k;
        (*envs).rx_in_rklrx = (*envs).rk;
        (*envs).rkrl[0_i32 as usize] =
            *((*envs).rk).offset(0_isize) - *((*envs).c2rust_unnamed_1.rl).offset(0_isize);
        (*envs).rkrl[1_i32 as usize] =
            *((*envs).rk).offset(1_isize) - *((*envs).c2rust_unnamed_1.rl).offset(1_isize);
        (*envs).rkrl[2_i32 as usize] =
            *((*envs).rk).offset(2_isize) - *((*envs).c2rust_unnamed_1.rl).offset(2_isize);
    } else {
        (*envs).g2d_klmax = (*envs).g_stride_l;
        (*envs).rx_in_rklrx = (*envs).c2rust_unnamed_1.rl;
        (*envs).rkrl[0_i32 as usize] =
            *((*envs).c2rust_unnamed_1.rl).offset(0_isize) - *((*envs).rk).offset(0_isize);
        (*envs).rkrl[1_i32 as usize] =
            *((*envs).c2rust_unnamed_1.rl).offset(1_isize) - *((*envs).rk).offset(1_isize);
        (*envs).rkrl[2_i32 as usize] =
            *((*envs).c2rust_unnamed_1.rl).offset(2_isize) - *((*envs).rk).offset(2_isize);
    }
    if ibase != 0 {
        (*envs).g2d_ijmax = (*envs).g_stride_i;
        (*envs).rx_in_rijrx = (*envs).ri;
        (*envs).rirj[0_i32 as usize] =
            *((*envs).ri).offset(0_isize) - *((*envs).rj).offset(0_isize);
        (*envs).rirj[1_i32 as usize] =
            *((*envs).ri).offset(1_isize) - *((*envs).rj).offset(1_isize);
        (*envs).rirj[2_i32 as usize] =
            *((*envs).ri).offset(2_isize) - *((*envs).rj).offset(2_isize);
    } else {
        (*envs).g2d_ijmax = (*envs).g_stride_j;
        (*envs).rx_in_rijrx = (*envs).rj;
        (*envs).rirj[0_i32 as usize] =
            *((*envs).rj).offset(0_isize) - *((*envs).ri).offset(0_isize);
        (*envs).rirj[1_i32 as usize] =
            *((*envs).rj).offset(1_isize) - *((*envs).ri).offset(1_isize);
        (*envs).rirj[2_i32 as usize] =
            *((*envs).rj).offset(2_isize) - *((*envs).ri).offset(2_isize);
    }
    if rys_order <= 2_i32 {
        (*envs).f_g0_2d4d = ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut f64, *mut Rys2eT, *mut CINTEnvVars) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            CINTg0_2e_2d4d_unrolled
                as unsafe extern "C" fn(*mut f64, *mut Rys2eT, *mut CINTEnvVars) -> (),
        ));
        if rys_order != nrys_roots {
            (*envs).f_g0_2d4d = ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut f64, *mut Rys2eT, *mut CINTEnvVars) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                CINTsrg0_2e_2d4d_unrolled
                    as unsafe extern "C" fn(*mut f64, *mut Rys2eT, *mut CINTEnvVars) -> (),
            ));
        }
    } else if kbase != 0 {
        if ibase != 0 {
            (*envs).f_g0_2d4d = ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut f64, *mut Rys2eT, *mut CINTEnvVars) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                CINTg0_2e_ik2d4d
                    as unsafe extern "C" fn(*mut f64, *mut Rys2eT, *mut CINTEnvVars) -> (),
            ));
        } else {
            (*envs).f_g0_2d4d = ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut f64, *mut Rys2eT, *mut CINTEnvVars) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                CINTg0_2e_kj2d4d
                    as unsafe extern "C" fn(*mut f64, *mut Rys2eT, *mut CINTEnvVars) -> (),
            ));
        }
    } else if ibase != 0 {
        (*envs).f_g0_2d4d = ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut f64, *mut Rys2eT, *mut CINTEnvVars) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            CINTg0_2e_il2d4d as unsafe extern "C" fn(*mut f64, *mut Rys2eT, *mut CINTEnvVars) -> (),
        ));
    } else {
        (*envs).f_g0_2d4d = ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut f64, *mut Rys2eT, *mut CINTEnvVars) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            CINTg0_2e_lj2d4d as unsafe extern "C" fn(*mut f64, *mut Rys2eT, *mut CINTEnvVars) -> (),
        ));
    }
    (*envs).f_g0_2e = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut f64, *mut f64, *mut f64, f64, *mut CINTEnvVars) -> i32>,
        Option<unsafe extern "C" fn() -> i32>,
    >(Some(
        CINTg0_2e
            as unsafe extern "C" fn(*mut f64, *mut f64, *mut f64, f64, *mut CINTEnvVars) -> i32,
    ));
}
#[no_mangle]
pub unsafe extern "C" fn CINTg2e_index_xyz(idx: *mut i32, envs: *const CINTEnvVars) {
    let i_l: i32 = (*envs).i_l;
    let j_l: i32 = (*envs).j_l;
    let k_l: i32 = (*envs).k_l;
    let l_l: i32 = (*envs).l_l;
    let nfi: i32 = (*envs).nfi;
    let nfj: i32 = (*envs).nfj;
    let nfk: i32 = (*envs).c2rust_unnamed.nfk;
    let nfl: i32 = (*envs).c2rust_unnamed_0.nfl;
    let di: i32 = (*envs).g_stride_i;
    let dk: i32 = (*envs).g_stride_k;
    let dl: i32 = (*envs).g_stride_l;
    let dj: i32 = (*envs).g_stride_j;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut n: i32 = 0;
    let mut ofx: i32 = 0;
    let mut ofkx: i32 = 0;
    let mut oflx: i32 = 0;
    let mut ofy: i32 = 0;
    let mut ofky: i32 = 0;
    let mut ofly: i32 = 0;
    let mut ofz: i32 = 0;
    let mut ofkz: i32 = 0;
    let mut oflz: i32 = 0;
    let mut i_nx: [i32; 136] = [0; 136];
    let mut i_ny: [i32; 136] = [0; 136];
    let mut i_nz: [i32; 136] = [0; 136];
    let mut j_nx: [i32; 136] = [0; 136];
    let mut j_ny: [i32; 136] = [0; 136];
    let mut j_nz: [i32; 136] = [0; 136];
    let mut k_nx: [i32; 136] = [0; 136];
    let mut k_ny: [i32; 136] = [0; 136];
    let mut k_nz: [i32; 136] = [0; 136];
    let mut l_nx: [i32; 136] = [0; 136];
    let mut l_ny: [i32; 136] = [0; 136];
    let mut l_nz: [i32; 136] = [0; 136];
    CINTcart_comp(i_nx.as_mut_ptr(), i_ny.as_mut_ptr(), i_nz.as_mut_ptr(), i_l);
    CINTcart_comp(j_nx.as_mut_ptr(), j_ny.as_mut_ptr(), j_nz.as_mut_ptr(), j_l);
    CINTcart_comp(k_nx.as_mut_ptr(), k_ny.as_mut_ptr(), k_nz.as_mut_ptr(), k_l);
    CINTcart_comp(l_nx.as_mut_ptr(), l_ny.as_mut_ptr(), l_nz.as_mut_ptr(), l_l);
    ofx = 0_i32;
    ofy = (*envs).g_size;
    ofz = (*envs).g_size * 2_i32;
    n = 0_i32;
    j = 0_i32;
    while j < nfj {
        l = 0_i32;
        while l < nfl {
            oflx = ofx + dj * j_nx[j as usize] + dl * l_nx[l as usize];
            ofly = ofy + dj * j_ny[j as usize] + dl * l_ny[l as usize];
            oflz = ofz + dj * j_nz[j as usize] + dl * l_nz[l as usize];
            k = 0_i32;
            while k < nfk {
                ofkx = oflx + dk * k_nx[k as usize];
                ofky = ofly + dk * k_ny[k as usize];
                ofkz = oflz + dk * k_nz[k as usize];
                match i_l {
                    0 => {
                        *idx.offset((n + 0_i32) as isize) = ofkx;
                        *idx.offset((n + 1_i32) as isize) = ofky;
                        *idx.offset((n + 2_i32) as isize) = ofkz;
                        n += 3_i32;
                    }
                    1 => {
                        *idx.offset((n + 0_i32) as isize) = ofkx + di;
                        *idx.offset((n + 1_i32) as isize) = ofky;
                        *idx.offset((n + 2_i32) as isize) = ofkz;
                        *idx.offset((n + 3_i32) as isize) = ofkx;
                        *idx.offset((n + 4_i32) as isize) = ofky + di;
                        *idx.offset((n + 5_i32) as isize) = ofkz;
                        *idx.offset((n + 6_i32) as isize) = ofkx;
                        *idx.offset((n + 7_i32) as isize) = ofky;
                        *idx.offset((n + 8_i32) as isize) = ofkz + di;
                        n += 9_i32;
                    }
                    2 => {
                        *idx.offset((n + 0_i32) as isize) = ofkx + di * 2_i32;
                        *idx.offset((n + 1_i32) as isize) = ofky;
                        *idx.offset((n + 2_i32) as isize) = ofkz;
                        *idx.offset((n + 3_i32) as isize) = ofkx + di;
                        *idx.offset((n + 4_i32) as isize) = ofky + di;
                        *idx.offset((n + 5_i32) as isize) = ofkz;
                        *idx.offset((n + 6_i32) as isize) = ofkx + di;
                        *idx.offset((n + 7_i32) as isize) = ofky;
                        *idx.offset((n + 8_i32) as isize) = ofkz + di;
                        *idx.offset((n + 9_i32) as isize) = ofkx;
                        *idx.offset((n + 10_i32) as isize) = ofky + di * 2_i32;
                        *idx.offset((n + 11_i32) as isize) = ofkz;
                        *idx.offset((n + 12_i32) as isize) = ofkx;
                        *idx.offset((n + 13_i32) as isize) = ofky + di;
                        *idx.offset((n + 14_i32) as isize) = ofkz + di;
                        *idx.offset((n + 15_i32) as isize) = ofkx;
                        *idx.offset((n + 16_i32) as isize) = ofky;
                        *idx.offset((n + 17_i32) as isize) = ofkz + di * 2_i32;
                        n += 18_i32;
                    }
                    _ => {
                        i = 0_i32;
                        while i < nfi {
                            *idx.offset((n + 0_i32) as isize) = ofkx + di * i_nx[i as usize];
                            *idx.offset((n + 1_i32) as isize) = ofky + di * i_ny[i as usize];
                            *idx.offset((n + 2_i32) as isize) = ofkz + di * i_nz[i as usize];
                            n += 3_i32;
                            i += 1;
                            i;
                        }
                    }
                }
                k += 1;
                k;
            }
            l += 1;
            l;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e_2d(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    let nroots: i32 = (*envs).nrys_roots;
    let nmax: i32 = (*envs).li_ceil + (*envs).lj_ceil;
    let mmax: i32 = (*envs).lk_ceil + (*envs).ll_ceil;
    let dm: i32 = (*envs).g2d_klmax;
    let dn: i32 = (*envs).g2d_ijmax;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    let mut off: i32 = 0;
    let gx: *mut f64 = g;
    let gy: *mut f64 = g.offset((*envs).g_size as isize);
    let gz: *mut f64 = g.offset(((*envs).g_size * 2_i32) as isize);
    let _p0x: *mut f64 = std::ptr::null_mut::<f64>();
    let _p0y: *mut f64 = std::ptr::null_mut::<f64>();
    let _p0z: *mut f64 = std::ptr::null_mut::<f64>();
    let _p1x: *mut f64 = std::ptr::null_mut::<f64>();
    let _p1y: *mut f64 = std::ptr::null_mut::<f64>();
    let _p1z: *mut f64 = std::ptr::null_mut::<f64>();
    let _nb1: f64 = 0.;
    let _mb0: f64 = 0.;
    i = 0_i32;
    while i < nroots {
        *gx.offset(i as isize) = 1_f64;
        *gy.offset(i as isize) = 1_f64;
        i += 1;
        i;
    }
    let mut s0x: f64 = 0.;
    let mut s1x: f64 = 0.;
    let mut s2x: f64 = 0.;
    let _t0x: f64 = 0.;
    let _t1x: f64 = 0.;
    let mut s0y: f64 = 0.;
    let mut s1y: f64 = 0.;
    let mut s2y: f64 = 0.;
    let _t0y: f64 = 0.;
    let _t1y: f64 = 0.;
    let mut s0z: f64 = 0.;
    let mut s1z: f64 = 0.;
    let mut s2z: f64 = 0.;
    let _t0z: f64 = 0.;
    let _t1z: f64 = 0.;
    let mut c00x: f64 = 0.;
    let mut c00y: f64 = 0.;
    let mut c00z: f64 = 0.;
    let mut c0px: f64 = 0.;
    let mut c0py: f64 = 0.;
    let mut c0pz: f64 = 0.;
    let mut b10: f64 = 0.;
    let mut b01: f64 = 0.;
    let mut b00: f64 = 0.;
    i = 0_i32;
    while i < nroots {
        c00x = (*bc).c00x[i as usize];
        c00y = (*bc).c00y[i as usize];
        c00z = (*bc).c00z[i as usize];
        c0px = (*bc).c0px[i as usize];
        c0py = (*bc).c0py[i as usize];
        c0pz = (*bc).c0pz[i as usize];
        b10 = (*bc).b10[i as usize];
        b01 = (*bc).b01[i as usize];
        b00 = (*bc).b00[i as usize];
        if nmax > 0_i32 {
            s0x = *gx.offset(i as isize);
            s0y = *gy.offset(i as isize);
            s0z = *gz.offset(i as isize);
            s1x = c00x * s0x;
            s1y = c00y * s0y;
            s1z = c00z * s0z;
            *gx.offset((i + dn) as isize) = s1x;
            *gy.offset((i + dn) as isize) = s1y;
            *gz.offset((i + dn) as isize) = s1z;
            n = 1_i32;
            while n < nmax {
                s2x = c00x * s1x + n as f64 * b10 * s0x;
                s2y = c00y * s1y + n as f64 * b10 * s0y;
                s2z = c00z * s1z + n as f64 * b10 * s0z;
                *gx.offset((i + (n + 1_i32) * dn) as isize) = s2x;
                *gy.offset((i + (n + 1_i32) * dn) as isize) = s2y;
                *gz.offset((i + (n + 1_i32) * dn) as isize) = s2z;
                s0x = s1x;
                s0y = s1y;
                s0z = s1z;
                s1x = s2x;
                s1y = s2y;
                s1z = s2z;
                n += 1;
                n;
            }
        }
        if mmax > 0_i32 {
            s0x = *gx.offset(i as isize);
            s0y = *gy.offset(i as isize);
            s0z = *gz.offset(i as isize);
            s1x = c0px * s0x;
            s1y = c0py * s0y;
            s1z = c0pz * s0z;
            *gx.offset((i + dm) as isize) = s1x;
            *gy.offset((i + dm) as isize) = s1y;
            *gz.offset((i + dm) as isize) = s1z;
            m = 1_i32;
            while m < mmax {
                s2x = c0px * s1x + m as f64 * b01 * s0x;
                s2y = c0py * s1y + m as f64 * b01 * s0y;
                s2z = c0pz * s1z + m as f64 * b01 * s0z;
                *gx.offset((i + (m + 1_i32) * dm) as isize) = s2x;
                *gy.offset((i + (m + 1_i32) * dm) as isize) = s2y;
                *gz.offset((i + (m + 1_i32) * dm) as isize) = s2z;
                s0x = s1x;
                s0y = s1y;
                s0z = s1z;
                s1x = s2x;
                s1y = s2y;
                s1z = s2z;
                m += 1;
                m;
            }
            if nmax > 0_i32 {
                s0x = *gx.offset((i + dn) as isize);
                s0y = *gy.offset((i + dn) as isize);
                s0z = *gz.offset((i + dn) as isize);
                s1x = c0px * s0x + b00 * *gx.offset(i as isize);
                s1y = c0py * s0y + b00 * *gy.offset(i as isize);
                s1z = c0pz * s0z + b00 * *gz.offset(i as isize);
                *gx.offset((i + dn + dm) as isize) = s1x;
                *gy.offset((i + dn + dm) as isize) = s1y;
                *gz.offset((i + dn + dm) as isize) = s1z;
                m = 1_i32;
                while m < mmax {
                    s2x =
                        c0px * s1x + m as f64 * b01 * s0x + b00 * *gx.offset((i + m * dm) as isize);
                    s2y =
                        c0py * s1y + m as f64 * b01 * s0y + b00 * *gy.offset((i + m * dm) as isize);
                    s2z =
                        c0pz * s1z + m as f64 * b01 * s0z + b00 * *gz.offset((i + m * dm) as isize);
                    *gx.offset((i + dn + (m + 1_i32) * dm) as isize) = s2x;
                    *gy.offset((i + dn + (m + 1_i32) * dm) as isize) = s2y;
                    *gz.offset((i + dn + (m + 1_i32) * dm) as isize) = s2z;
                    s0x = s1x;
                    s0y = s1y;
                    s0z = s1z;
                    s1x = s2x;
                    s1y = s2y;
                    s1z = s2z;
                    m += 1;
                    m;
                }
            }
        }
        m = 1_i32;
        while m <= mmax {
            off = m * dm;
            j = off + i;
            s0x = *gx.offset(j as isize);
            s0y = *gy.offset(j as isize);
            s0z = *gz.offset(j as isize);
            s1x = *gx.offset((j + dn) as isize);
            s1y = *gy.offset((j + dn) as isize);
            s1z = *gz.offset((j + dn) as isize);
            n = 1_i32;
            while n < nmax {
                s2x = c00x * s1x
                    + n as f64 * b10 * s0x
                    + m as f64 * b00 * *gx.offset((j + n * dn - dm) as isize);
                s2y = c00y * s1y
                    + n as f64 * b10 * s0y
                    + m as f64 * b00 * *gy.offset((j + n * dn - dm) as isize);
                s2z = c00z * s1z
                    + n as f64 * b10 * s0z
                    + m as f64 * b00 * *gz.offset((j + n * dn - dm) as isize);
                *gx.offset((j + (n + 1_i32) * dn) as isize) = s2x;
                *gy.offset((j + (n + 1_i32) * dn) as isize) = s2y;
                *gz.offset((j + (n + 1_i32) * dn) as isize) = s2z;
                s0x = s1x;
                s0y = s1y;
                s0z = s1z;
                s1x = s2x;
                s1y = s2y;
                s1z = s2z;
                n += 1;
                n;
            }
            m += 1;
            m;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_lj2d_4d(g: *mut f64, envs: *mut CINTEnvVars) {
    let li: i32 = (*envs).li_ceil;
    let lk: i32 = (*envs).lk_ceil;
    if li == 0_i32 && lk == 0_i32 {
        return;
    }
    let nmax: i32 = (*envs).li_ceil + (*envs).lj_ceil;
    let mmax: i32 = (*envs).lk_ceil + (*envs).ll_ceil;
    let lj: i32 = (*envs).lj_ceil;
    let nroots: i32 = (*envs).nrys_roots;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut ptr: i32 = 0;
    let mut n: i32 = 0;
    let di: i32 = (*envs).g_stride_i;
    let dk: i32 = (*envs).g_stride_k;
    let dl: i32 = (*envs).g_stride_l;
    let dj: i32 = (*envs).g_stride_j;
    let rirj: *mut f64 = ((*envs).rirj).as_mut_ptr();
    let rkrl: *mut f64 = ((*envs).rkrl).as_mut_ptr();
    let gx: *mut f64 = g;
    let gy: *mut f64 = g.offset((*envs).g_size as isize);
    let gz: *mut f64 = g.offset(((*envs).g_size * 2_i32) as isize);
    let mut p1x: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p1y: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p1z: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p2x: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p2y: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p2z: *mut f64 = std::ptr::null_mut::<f64>();
    let mut rx: f64 = 0.;
    let mut ry: f64 = 0.;
    let mut rz: f64 = 0.;
    rx = *rirj.offset(0_isize);
    ry = *rirj.offset(1_isize);
    rz = *rirj.offset(2_isize);
    p1x = gx.offset(-(di as isize));
    p1y = gy.offset(-(di as isize));
    p1z = gz.offset(-(di as isize));
    p2x = gx.offset(-(di as isize)).offset(dj as isize);
    p2y = gy.offset(-(di as isize)).offset(dj as isize);
    p2z = gz.offset(-(di as isize)).offset(dj as isize);
    i = 1_i32;
    while i <= li {
        j = 0_i32;
        while j <= nmax - i {
            l = 0_i32;
            while l <= mmax {
                ptr = j * dj + l * dl + i * di;
                n = ptr;
                while n < ptr + nroots {
                    *gx.offset(n as isize) = rx * *p1x.offset(n as isize) + *p2x.offset(n as isize);
                    *gy.offset(n as isize) = ry * *p1y.offset(n as isize) + *p2y.offset(n as isize);
                    *gz.offset(n as isize) = rz * *p1z.offset(n as isize) + *p2z.offset(n as isize);
                    n += 1;
                    n;
                }
                l += 1;
                l;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    rx = *rkrl.offset(0_isize);
    ry = *rkrl.offset(1_isize);
    rz = *rkrl.offset(2_isize);
    p1x = gx.offset(-(dk as isize));
    p1y = gy.offset(-(dk as isize));
    p1z = gz.offset(-(dk as isize));
    p2x = gx.offset(-(dk as isize)).offset(dl as isize);
    p2y = gy.offset(-(dk as isize)).offset(dl as isize);
    p2z = gz.offset(-(dk as isize)).offset(dl as isize);
    j = 0_i32;
    while j <= lj {
        k = 1_i32;
        while k <= lk {
            l = 0_i32;
            while l <= mmax - k {
                ptr = j * dj + l * dl + k * dk;
                n = ptr;
                while n < ptr + dk {
                    *gx.offset(n as isize) = rx * *p1x.offset(n as isize) + *p2x.offset(n as isize);
                    *gy.offset(n as isize) = ry * *p1y.offset(n as isize) + *p2y.offset(n as isize);
                    *gz.offset(n as isize) = rz * *p1z.offset(n as isize) + *p2z.offset(n as isize);
                    n += 1;
                    n;
                }
                l += 1;
                l;
            }
            k += 1;
            k;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_kj2d_4d(g: *mut f64, envs: *mut CINTEnvVars) {
    let li: i32 = (*envs).li_ceil;
    let ll: i32 = (*envs).ll_ceil;
    if li == 0_i32 && ll == 0_i32 {
        return;
    }
    let nmax: i32 = (*envs).li_ceil + (*envs).lj_ceil;
    let mmax: i32 = (*envs).lk_ceil + (*envs).ll_ceil;
    let lj: i32 = (*envs).lj_ceil;
    let nroots: i32 = (*envs).nrys_roots;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut ptr: i32 = 0;
    let mut n: i32 = 0;
    let di: i32 = (*envs).g_stride_i;
    let dk: i32 = (*envs).g_stride_k;
    let dl: i32 = (*envs).g_stride_l;
    let dj: i32 = (*envs).g_stride_j;
    let rirj: *mut f64 = ((*envs).rirj).as_mut_ptr();
    let rkrl: *mut f64 = ((*envs).rkrl).as_mut_ptr();
    let gx: *mut f64 = g;
    let gy: *mut f64 = g.offset((*envs).g_size as isize);
    let gz: *mut f64 = g.offset(((*envs).g_size * 2_i32) as isize);
    let mut p1x: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p1y: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p1z: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p2x: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p2y: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p2z: *mut f64 = std::ptr::null_mut::<f64>();
    let mut rx: f64 = 0.;
    let mut ry: f64 = 0.;
    let mut rz: f64 = 0.;
    rx = *rirj.offset(0_isize);
    ry = *rirj.offset(1_isize);
    rz = *rirj.offset(2_isize);
    p1x = gx.offset(-(di as isize));
    p1y = gy.offset(-(di as isize));
    p1z = gz.offset(-(di as isize));
    p2x = gx.offset(-(di as isize)).offset(dj as isize);
    p2y = gy.offset(-(di as isize)).offset(dj as isize);
    p2z = gz.offset(-(di as isize)).offset(dj as isize);
    i = 1_i32;
    while i <= li {
        j = 0_i32;
        while j <= nmax - i {
            k = 0_i32;
            while k <= mmax {
                ptr = j * dj + k * dk + i * di;
                n = ptr;
                while n < ptr + nroots {
                    *gx.offset(n as isize) = rx * *p1x.offset(n as isize) + *p2x.offset(n as isize);
                    *gy.offset(n as isize) = ry * *p1y.offset(n as isize) + *p2y.offset(n as isize);
                    *gz.offset(n as isize) = rz * *p1z.offset(n as isize) + *p2z.offset(n as isize);
                    n += 1;
                    n;
                }
                k += 1;
                k;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    rx = *rkrl.offset(0_isize);
    ry = *rkrl.offset(1_isize);
    rz = *rkrl.offset(2_isize);
    p1x = gx.offset(-(dl as isize));
    p1y = gy.offset(-(dl as isize));
    p1z = gz.offset(-(dl as isize));
    p2x = gx.offset(-(dl as isize)).offset(dk as isize);
    p2y = gy.offset(-(dl as isize)).offset(dk as isize);
    p2z = gz.offset(-(dl as isize)).offset(dk as isize);
    j = 0_i32;
    while j <= lj {
        l = 1_i32;
        while l <= ll {
            k = 0_i32;
            while k <= mmax - l {
                ptr = j * dj + l * dl + k * dk;
                n = ptr;
                while n < ptr + dk {
                    *gx.offset(n as isize) = rx * *p1x.offset(n as isize) + *p2x.offset(n as isize);
                    *gy.offset(n as isize) = ry * *p1y.offset(n as isize) + *p2y.offset(n as isize);
                    *gz.offset(n as isize) = rz * *p1z.offset(n as isize) + *p2z.offset(n as isize);
                    n += 1;
                    n;
                }
                k += 1;
                k;
            }
            l += 1;
            l;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_il2d_4d(g: *mut f64, envs: *mut CINTEnvVars) {
    let lk: i32 = (*envs).lk_ceil;
    let lj: i32 = (*envs).lj_ceil;
    if lj == 0_i32 && lk == 0_i32 {
        return;
    }
    let nmax: i32 = (*envs).li_ceil + (*envs).lj_ceil;
    let mmax: i32 = (*envs).lk_ceil + (*envs).ll_ceil;
    let ll: i32 = (*envs).ll_ceil;
    let nroots: i32 = (*envs).nrys_roots;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut ptr: i32 = 0;
    let mut n: i32 = 0;
    let di: i32 = (*envs).g_stride_i;
    let dk: i32 = (*envs).g_stride_k;
    let dl: i32 = (*envs).g_stride_l;
    let dj: i32 = (*envs).g_stride_j;
    let rirj: *mut f64 = ((*envs).rirj).as_mut_ptr();
    let rkrl: *mut f64 = ((*envs).rkrl).as_mut_ptr();
    let gx: *mut f64 = g;
    let gy: *mut f64 = g.offset((*envs).g_size as isize);
    let gz: *mut f64 = g.offset(((*envs).g_size * 2_i32) as isize);
    let mut p1x: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p1y: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p1z: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p2x: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p2y: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p2z: *mut f64 = std::ptr::null_mut::<f64>();
    let mut rx: f64 = 0.;
    let mut ry: f64 = 0.;
    let mut rz: f64 = 0.;
    rx = *rkrl.offset(0_isize);
    ry = *rkrl.offset(1_isize);
    rz = *rkrl.offset(2_isize);
    p1x = gx.offset(-(dk as isize));
    p1y = gy.offset(-(dk as isize));
    p1z = gz.offset(-(dk as isize));
    p2x = gx.offset(-(dk as isize)).offset(dl as isize);
    p2y = gy.offset(-(dk as isize)).offset(dl as isize);
    p2z = gz.offset(-(dk as isize)).offset(dl as isize);
    k = 1_i32;
    while k <= lk {
        l = 0_i32;
        while l <= mmax - k {
            i = 0_i32;
            while i <= nmax {
                ptr = l * dl + k * dk + i * di;
                n = ptr;
                while n < ptr + nroots {
                    *gx.offset(n as isize) = rx * *p1x.offset(n as isize) + *p2x.offset(n as isize);
                    *gy.offset(n as isize) = ry * *p1y.offset(n as isize) + *p2y.offset(n as isize);
                    *gz.offset(n as isize) = rz * *p1z.offset(n as isize) + *p2z.offset(n as isize);
                    n += 1;
                    n;
                }
                i += 1;
                i;
            }
            l += 1;
            l;
        }
        k += 1;
        k;
    }
    rx = *rirj.offset(0_isize);
    ry = *rirj.offset(1_isize);
    rz = *rirj.offset(2_isize);
    p1x = gx.offset(-(dj as isize));
    p1y = gy.offset(-(dj as isize));
    p1z = gz.offset(-(dj as isize));
    p2x = gx.offset(-(dj as isize)).offset(di as isize);
    p2y = gy.offset(-(dj as isize)).offset(di as isize);
    p2z = gz.offset(-(dj as isize)).offset(di as isize);
    j = 1_i32;
    while j <= lj {
        l = 0_i32;
        while l <= ll {
            k = 0_i32;
            while k <= lk {
                ptr = j * dj + l * dl + k * dk;
                n = ptr;
                while n < ptr + dk - di * j {
                    *gx.offset(n as isize) = rx * *p1x.offset(n as isize) + *p2x.offset(n as isize);
                    *gy.offset(n as isize) = ry * *p1y.offset(n as isize) + *p2y.offset(n as isize);
                    *gz.offset(n as isize) = rz * *p1z.offset(n as isize) + *p2z.offset(n as isize);
                    n += 1;
                    n;
                }
                k += 1;
                k;
            }
            l += 1;
            l;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_ik2d_4d(g: *mut f64, envs: *mut CINTEnvVars) {
    let lj: i32 = (*envs).lj_ceil;
    let ll: i32 = (*envs).ll_ceil;
    if lj == 0_i32 && ll == 0_i32 {
        return;
    }
    let nmax: i32 = (*envs).li_ceil + (*envs).lj_ceil;
    let mmax: i32 = (*envs).lk_ceil + (*envs).ll_ceil;
    let lk: i32 = (*envs).lk_ceil;
    let nroots: i32 = (*envs).nrys_roots;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut ptr: i32 = 0;
    let mut n: i32 = 0;
    let di: i32 = (*envs).g_stride_i;
    let dk: i32 = (*envs).g_stride_k;
    let dl: i32 = (*envs).g_stride_l;
    let dj: i32 = (*envs).g_stride_j;
    let rirj: *mut f64 = ((*envs).rirj).as_mut_ptr();
    let rkrl: *mut f64 = ((*envs).rkrl).as_mut_ptr();
    let gx: *mut f64 = g;
    let gy: *mut f64 = g.offset((*envs).g_size as isize);
    let gz: *mut f64 = g.offset(((*envs).g_size * 2_i32) as isize);
    let mut p1x: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p1y: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p1z: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p2x: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p2y: *mut f64 = std::ptr::null_mut::<f64>();
    let mut p2z: *mut f64 = std::ptr::null_mut::<f64>();
    let mut rx: f64 = 0.;
    let mut ry: f64 = 0.;
    let mut rz: f64 = 0.;
    rx = *rkrl.offset(0_isize);
    ry = *rkrl.offset(1_isize);
    rz = *rkrl.offset(2_isize);
    p1x = gx.offset(-(dl as isize));
    p1y = gy.offset(-(dl as isize));
    p1z = gz.offset(-(dl as isize));
    p2x = gx.offset(-(dl as isize)).offset(dk as isize);
    p2y = gy.offset(-(dl as isize)).offset(dk as isize);
    p2z = gz.offset(-(dl as isize)).offset(dk as isize);
    l = 1_i32;
    while l <= ll {
        k = 0_i32;
        while k <= mmax - l {
            i = 0_i32;
            while i <= nmax {
                ptr = l * dl + k * dk + i * di;
                n = ptr;
                while n < ptr + nroots {
                    *gx.offset(n as isize) = rx * *p1x.offset(n as isize) + *p2x.offset(n as isize);
                    *gy.offset(n as isize) = ry * *p1y.offset(n as isize) + *p2y.offset(n as isize);
                    *gz.offset(n as isize) = rz * *p1z.offset(n as isize) + *p2z.offset(n as isize);
                    n += 1;
                    n;
                }
                i += 1;
                i;
            }
            k += 1;
            k;
        }
        l += 1;
        l;
    }
    rx = *rirj.offset(0_isize);
    ry = *rirj.offset(1_isize);
    rz = *rirj.offset(2_isize);
    p1x = gx.offset(-(dj as isize));
    p1y = gy.offset(-(dj as isize));
    p1z = gz.offset(-(dj as isize));
    p2x = gx.offset(-(dj as isize)).offset(di as isize);
    p2y = gy.offset(-(dj as isize)).offset(di as isize);
    p2z = gz.offset(-(dj as isize)).offset(di as isize);
    j = 1_i32;
    while j <= lj {
        l = 0_i32;
        while l <= ll {
            k = 0_i32;
            while k <= lk {
                ptr = j * dj + l * dl + k * dk;
                n = ptr;
                while n < ptr + dk - di * j {
                    *gx.offset(n as isize) = rx * *p1x.offset(n as isize) + *p2x.offset(n as isize);
                    *gy.offset(n as isize) = ry * *p1y.offset(n as isize) + *p2y.offset(n as isize);
                    *gz.offset(n as isize) = rz * *p1z.offset(n as isize) + *p2z.offset(n as isize);
                    n += 1;
                    n;
                }
                k += 1;
                k;
            }
            l += 1;
            l;
        }
        j += 1;
        j;
    }
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0000(
    g: *mut f64,
    _bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0001(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = *cpx.offset(0_isize);
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = *cpy.offset(0_isize);
    *g.offset(5_isize) = *cpz.offset(0_isize) * *g.offset(4_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0002(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *cpx.offset(0_isize);
    *g.offset(3_isize) = *cpx.offset(1_isize);
    *g.offset(4_isize) =
        *cpx.offset(0_isize) * *cpx.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(5_isize) =
        *cpx.offset(1_isize) * *cpx.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(6_isize) = 1_f64;
    *g.offset(7_isize) = 1_f64;
    *g.offset(8_isize) = *cpy.offset(0_isize);
    *g.offset(9_isize) = *cpy.offset(1_isize);
    *g.offset(10_isize) =
        *cpy.offset(0_isize) * *cpy.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(11_isize) =
        *cpy.offset(1_isize) * *cpy.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(14_isize) = *cpz.offset(0_isize) * *g.offset(12_isize);
    *g.offset(15_isize) = *cpz.offset(1_isize) * *g.offset(13_isize);
    *g.offset(16_isize) = *cpz.offset(0_isize) * *g.offset(14_isize)
        + *b01.offset(0_isize) * *g.offset(12_isize);
    *g.offset(17_isize) = *cpz.offset(1_isize) * *g.offset(15_isize)
        + *b01.offset(1_isize) * *g.offset(13_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0003(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *cpx.offset(0_isize);
    *g.offset(3_isize) = *cpx.offset(1_isize);
    *g.offset(4_isize) =
        *cpx.offset(0_isize) * *cpx.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(5_isize) =
        *cpx.offset(1_isize) * *cpx.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(6_isize) =
        *cpx.offset(0_isize) * (*g.offset(4_isize) + 2_f64 * *b01.offset(0_isize));
    *g.offset(7_isize) =
        *cpx.offset(1_isize) * (*g.offset(5_isize) + 2_f64 * *b01.offset(1_isize));
    *g.offset(8_isize) = 1_f64;
    *g.offset(9_isize) = 1_f64;
    *g.offset(10_isize) = *cpy.offset(0_isize);
    *g.offset(11_isize) = *cpy.offset(1_isize);
    *g.offset(12_isize) =
        *cpy.offset(0_isize) * *cpy.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(13_isize) =
        *cpy.offset(1_isize) * *cpy.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(14_isize) =
        *cpy.offset(0_isize) * (*g.offset(12_isize) + 2_f64 * *b01.offset(0_isize));
    *g.offset(15_isize) =
        *cpy.offset(1_isize) * (*g.offset(13_isize) + 2_f64 * *b01.offset(1_isize));
    *g.offset(18_isize) = *cpz.offset(0_isize) * *g.offset(16_isize);
    *g.offset(19_isize) = *cpz.offset(1_isize) * *g.offset(17_isize);
    *g.offset(20_isize) = *cpz.offset(0_isize) * *g.offset(18_isize)
        + *b01.offset(0_isize) * *g.offset(16_isize);
    *g.offset(21_isize) = *cpz.offset(1_isize) * *g.offset(19_isize)
        + *b01.offset(1_isize) * *g.offset(17_isize);
    *g.offset(22_isize) = *cpz.offset(0_isize) * *g.offset(20_isize)
        + 2_f64 * *b01.offset(0_isize) * *g.offset(18_isize);
    *g.offset(23_isize) = *cpz.offset(1_isize) * *g.offset(21_isize)
        + 2_f64 * *b01.offset(1_isize) * *g.offset(19_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0010(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = *cpx.offset(0_isize);
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = *cpy.offset(0_isize);
    *g.offset(5_isize) = *cpz.offset(0_isize) * *g.offset(4_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0011(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    let xkxl: f64 = (*envs).rkrl[0_usize];
    let ykyl: f64 = (*envs).rkrl[1_usize];
    let zkzl: f64 = (*envs).rkrl[2_usize];
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(4_isize) = *cpx.offset(0_isize);
    *g.offset(5_isize) = *cpx.offset(1_isize);
    *g.offset(6_isize) =
        *cpx.offset(0_isize) * (xkxl + *cpx.offset(0_isize)) + *b01.offset(0_isize);
    *g.offset(7_isize) =
        *cpx.offset(1_isize) * (xkxl + *cpx.offset(1_isize)) + *b01.offset(1_isize);
    *g.offset(2_isize) = xkxl + *cpx.offset(0_isize);
    *g.offset(3_isize) = xkxl + *cpx.offset(1_isize);
    *g.offset(12_isize) = 1_f64;
    *g.offset(13_isize) = 1_f64;
    *g.offset(16_isize) = *cpy.offset(0_isize);
    *g.offset(17_isize) = *cpy.offset(1_isize);
    *g.offset(18_isize) =
        *cpy.offset(0_isize) * (ykyl + *cpy.offset(0_isize)) + *b01.offset(0_isize);
    *g.offset(19_isize) =
        *cpy.offset(1_isize) * (ykyl + *cpy.offset(1_isize)) + *b01.offset(1_isize);
    *g.offset(14_isize) = ykyl + *cpy.offset(0_isize);
    *g.offset(15_isize) = ykyl + *cpy.offset(1_isize);
    *g.offset(28_isize) = *cpz.offset(0_isize) * *g.offset(24_isize);
    *g.offset(29_isize) = *cpz.offset(1_isize) * *g.offset(25_isize);
    *g.offset(30_isize) = *g.offset(28_isize) * (zkzl + *cpz.offset(0_isize))
        + *b01.offset(0_isize) * *g.offset(24_isize);
    *g.offset(31_isize) = *g.offset(29_isize) * (zkzl + *cpz.offset(1_isize))
        + *b01.offset(1_isize) * *g.offset(25_isize);
    *g.offset(26_isize) = *g.offset(24_isize) * (zkzl + *cpz.offset(0_isize));
    *g.offset(27_isize) = *g.offset(25_isize) * (zkzl + *cpz.offset(1_isize));
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0012(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    let xkxl: f64 = (*envs).rkrl[0_usize];
    let ykyl: f64 = (*envs).rkrl[1_usize];
    let zkzl: f64 = (*envs).rkrl[2_usize];
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(4_isize) = *cpx.offset(0_isize);
    *g.offset(5_isize) = *cpx.offset(1_isize);
    *g.offset(8_isize) =
        *cpx.offset(0_isize) * *cpx.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(9_isize) =
        *cpx.offset(1_isize) * *cpx.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(10_isize) = *g.offset(8_isize) * (xkxl + *cpx.offset(0_isize))
        + *cpx.offset(0_isize) * 2_f64 * *b01.offset(0_isize);
    *g.offset(11_isize) = *g.offset(9_isize) * (xkxl + *cpx.offset(1_isize))
        + *cpx.offset(1_isize) * 2_f64 * *b01.offset(1_isize);
    *g.offset(6_isize) =
        *cpx.offset(0_isize) * (xkxl + *cpx.offset(0_isize)) + *b01.offset(0_isize);
    *g.offset(7_isize) =
        *cpx.offset(1_isize) * (xkxl + *cpx.offset(1_isize)) + *b01.offset(1_isize);
    *g.offset(2_isize) = xkxl + *cpx.offset(0_isize);
    *g.offset(3_isize) = xkxl + *cpx.offset(1_isize);
    *g.offset(16_isize) = 1_f64;
    *g.offset(17_isize) = 1_f64;
    *g.offset(20_isize) = *cpy.offset(0_isize);
    *g.offset(21_isize) = *cpy.offset(1_isize);
    *g.offset(24_isize) =
        *cpy.offset(0_isize) * *cpy.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(25_isize) =
        *cpy.offset(1_isize) * *cpy.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(26_isize) = *g.offset(24_isize) * (ykyl + *cpy.offset(0_isize))
        + *cpy.offset(0_isize) * 2_f64 * *b01.offset(0_isize);
    *g.offset(27_isize) = *g.offset(25_isize) * (ykyl + *cpy.offset(1_isize))
        + *cpy.offset(1_isize) * 2_f64 * *b01.offset(1_isize);
    *g.offset(22_isize) =
        *cpy.offset(0_isize) * (ykyl + *cpy.offset(0_isize)) + *b01.offset(0_isize);
    *g.offset(23_isize) =
        *cpy.offset(1_isize) * (ykyl + *cpy.offset(1_isize)) + *b01.offset(1_isize);
    *g.offset(18_isize) = ykyl + *cpy.offset(0_isize);
    *g.offset(19_isize) = ykyl + *cpy.offset(1_isize);
    *g.offset(36_isize) = *cpz.offset(0_isize) * *g.offset(32_isize);
    *g.offset(37_isize) = *cpz.offset(1_isize) * *g.offset(33_isize);
    *g.offset(40_isize) = *cpz.offset(0_isize) * *g.offset(36_isize)
        + *b01.offset(0_isize) * *g.offset(32_isize);
    *g.offset(41_isize) = *cpz.offset(1_isize) * *g.offset(37_isize)
        + *b01.offset(1_isize) * *g.offset(33_isize);
    *g.offset(42_isize) = *g.offset(40_isize) * (zkzl + *cpz.offset(0_isize))
        + 2_f64 * *b01.offset(0_isize) * *g.offset(36_isize);
    *g.offset(43_isize) = *g.offset(41_isize) * (zkzl + *cpz.offset(1_isize))
        + 2_f64 * *b01.offset(1_isize) * *g.offset(37_isize);
    *g.offset(38_isize) = *g.offset(36_isize) * (zkzl + *cpz.offset(0_isize))
        + *b01.offset(0_isize) * *g.offset(32_isize);
    *g.offset(39_isize) = *g.offset(37_isize) * (zkzl + *cpz.offset(1_isize))
        + *b01.offset(1_isize) * *g.offset(33_isize);
    *g.offset(34_isize) = *g.offset(32_isize) * (zkzl + *cpz.offset(0_isize));
    *g.offset(35_isize) = *g.offset(33_isize) * (zkzl + *cpz.offset(1_isize));
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0020(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *cpx.offset(0_isize);
    *g.offset(3_isize) = *cpx.offset(1_isize);
    *g.offset(4_isize) =
        *cpx.offset(0_isize) * *cpx.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(5_isize) =
        *cpx.offset(1_isize) * *cpx.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(6_isize) = 1_f64;
    *g.offset(7_isize) = 1_f64;
    *g.offset(8_isize) = *cpy.offset(0_isize);
    *g.offset(9_isize) = *cpy.offset(1_isize);
    *g.offset(10_isize) =
        *cpy.offset(0_isize) * *cpy.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(11_isize) =
        *cpy.offset(1_isize) * *cpy.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(14_isize) = *cpz.offset(0_isize) * *g.offset(12_isize);
    *g.offset(15_isize) = *cpz.offset(1_isize) * *g.offset(13_isize);
    *g.offset(16_isize) = *cpz.offset(0_isize) * *g.offset(14_isize)
        + *b01.offset(0_isize) * *g.offset(12_isize);
    *g.offset(17_isize) = *cpz.offset(1_isize) * *g.offset(15_isize)
        + *b01.offset(1_isize) * *g.offset(13_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0021(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    let xkxl: f64 = (*envs).rkrl[0_usize];
    let ykyl: f64 = (*envs).rkrl[1_usize];
    let zkzl: f64 = (*envs).rkrl[2_usize];
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *cpx.offset(0_isize);
    *g.offset(3_isize) = *cpx.offset(1_isize);
    *g.offset(4_isize) =
        *cpx.offset(0_isize) * *cpx.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(5_isize) =
        *cpx.offset(1_isize) * *cpx.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(8_isize) = xkxl + *cpx.offset(0_isize);
    *g.offset(9_isize) = xkxl + *cpx.offset(1_isize);
    *g.offset(10_isize) =
        *cpx.offset(0_isize) * (xkxl + *cpx.offset(0_isize)) + *b01.offset(0_isize);
    *g.offset(11_isize) =
        *cpx.offset(1_isize) * (xkxl + *cpx.offset(1_isize)) + *b01.offset(1_isize);
    *g.offset(12_isize) = *g.offset(4_isize) * (xkxl + *cpx.offset(0_isize))
        + *cpx.offset(0_isize) * 2_f64 * *b01.offset(0_isize);
    *g.offset(13_isize) = *g.offset(5_isize) * (xkxl + *cpx.offset(1_isize))
        + *cpx.offset(1_isize) * 2_f64 * *b01.offset(1_isize);
    *g.offset(16_isize) = 1_f64;
    *g.offset(17_isize) = 1_f64;
    *g.offset(18_isize) = *cpy.offset(0_isize);
    *g.offset(19_isize) = *cpy.offset(1_isize);
    *g.offset(20_isize) =
        *cpy.offset(0_isize) * *cpy.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(21_isize) =
        *cpy.offset(1_isize) * *cpy.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(24_isize) = ykyl + *cpy.offset(0_isize);
    *g.offset(25_isize) = ykyl + *cpy.offset(1_isize);
    *g.offset(26_isize) =
        *cpy.offset(0_isize) * (ykyl + *cpy.offset(0_isize)) + *b01.offset(0_isize);
    *g.offset(27_isize) =
        *cpy.offset(1_isize) * (ykyl + *cpy.offset(1_isize)) + *b01.offset(1_isize);
    *g.offset(28_isize) = *g.offset(20_isize) * (ykyl + *cpy.offset(0_isize))
        + *cpy.offset(0_isize) * 2_f64 * *b01.offset(0_isize);
    *g.offset(29_isize) = *g.offset(21_isize) * (ykyl + *cpy.offset(1_isize))
        + *cpy.offset(1_isize) * 2_f64 * *b01.offset(1_isize);
    *g.offset(34_isize) = *cpz.offset(0_isize) * *g.offset(32_isize);
    *g.offset(35_isize) = *cpz.offset(1_isize) * *g.offset(33_isize);
    *g.offset(36_isize) = *cpz.offset(0_isize) * *g.offset(34_isize)
        + *b01.offset(0_isize) * *g.offset(32_isize);
    *g.offset(37_isize) = *cpz.offset(1_isize) * *g.offset(35_isize)
        + *b01.offset(1_isize) * *g.offset(33_isize);
    *g.offset(40_isize) = *g.offset(32_isize) * (zkzl + *cpz.offset(0_isize));
    *g.offset(41_isize) = *g.offset(33_isize) * (zkzl + *cpz.offset(1_isize));
    *g.offset(42_isize) = *g.offset(34_isize) * (zkzl + *cpz.offset(0_isize))
        + *b01.offset(0_isize) * *g.offset(32_isize);
    *g.offset(43_isize) = *g.offset(35_isize) * (zkzl + *cpz.offset(1_isize))
        + *b01.offset(1_isize) * *g.offset(33_isize);
    *g.offset(44_isize) = *g.offset(36_isize) * (zkzl + *cpz.offset(0_isize))
        + 2_f64 * *b01.offset(0_isize) * *g.offset(34_isize);
    *g.offset(45_isize) = *g.offset(37_isize) * (zkzl + *cpz.offset(1_isize))
        + 2_f64 * *b01.offset(1_isize) * *g.offset(35_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0030(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *cpx.offset(0_isize);
    *g.offset(3_isize) = *cpx.offset(1_isize);
    *g.offset(4_isize) =
        *cpx.offset(0_isize) * *cpx.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(5_isize) =
        *cpx.offset(1_isize) * *cpx.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(6_isize) =
        *cpx.offset(0_isize) * (*g.offset(4_isize) + 2_f64 * *b01.offset(0_isize));
    *g.offset(7_isize) =
        *cpx.offset(1_isize) * (*g.offset(5_isize) + 2_f64 * *b01.offset(1_isize));
    *g.offset(8_isize) = 1_f64;
    *g.offset(9_isize) = 1_f64;
    *g.offset(10_isize) = *cpy.offset(0_isize);
    *g.offset(11_isize) = *cpy.offset(1_isize);
    *g.offset(12_isize) =
        *cpy.offset(0_isize) * *cpy.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(13_isize) =
        *cpy.offset(1_isize) * *cpy.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(14_isize) =
        *cpy.offset(0_isize) * (*g.offset(12_isize) + 2_f64 * *b01.offset(0_isize));
    *g.offset(15_isize) =
        *cpy.offset(1_isize) * (*g.offset(13_isize) + 2_f64 * *b01.offset(1_isize));
    *g.offset(18_isize) = *cpz.offset(0_isize) * *g.offset(16_isize);
    *g.offset(19_isize) = *cpz.offset(1_isize) * *g.offset(17_isize);
    *g.offset(20_isize) = *cpz.offset(0_isize) * *g.offset(18_isize)
        + *b01.offset(0_isize) * *g.offset(16_isize);
    *g.offset(21_isize) = *cpz.offset(1_isize) * *g.offset(19_isize)
        + *b01.offset(1_isize) * *g.offset(17_isize);
    *g.offset(22_isize) = *cpz.offset(0_isize) * *g.offset(20_isize)
        + 2_f64 * *b01.offset(0_isize) * *g.offset(18_isize);
    *g.offset(23_isize) = *cpz.offset(1_isize) * *g.offset(21_isize)
        + 2_f64 * *b01.offset(1_isize) * *g.offset(19_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0100(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = *c0x.offset(0_isize);
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = *c0y.offset(0_isize);
    *g.offset(5_isize) = *c0z.offset(0_isize) * *g.offset(4_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0101(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *cpx.offset(0_isize);
    *g.offset(3_isize) = *cpx.offset(1_isize);
    *g.offset(4_isize) = *c0x.offset(0_isize);
    *g.offset(5_isize) = *c0x.offset(1_isize);
    *g.offset(6_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(7_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(8_isize) = 1_f64;
    *g.offset(9_isize) = 1_f64;
    *g.offset(10_isize) = *cpy.offset(0_isize);
    *g.offset(11_isize) = *cpy.offset(1_isize);
    *g.offset(12_isize) = *c0y.offset(0_isize);
    *g.offset(13_isize) = *c0y.offset(1_isize);
    *g.offset(14_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(15_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(18_isize) = *cpz.offset(0_isize) * *g.offset(16_isize);
    *g.offset(19_isize) = *cpz.offset(1_isize) * *g.offset(17_isize);
    *g.offset(20_isize) = *c0z.offset(0_isize) * *g.offset(16_isize);
    *g.offset(21_isize) = *c0z.offset(1_isize) * *g.offset(17_isize);
    *g.offset(22_isize) = *cpz.offset(0_isize) * *g.offset(20_isize)
        + *b00.offset(0_isize) * *g.offset(16_isize);
    *g.offset(23_isize) = *cpz.offset(1_isize) * *g.offset(21_isize)
        + *b00.offset(1_isize) * *g.offset(17_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0102(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *cpx.offset(0_isize);
    *g.offset(3_isize) = *cpx.offset(1_isize);
    *g.offset(6_isize) = *c0x.offset(0_isize);
    *g.offset(7_isize) = *c0x.offset(1_isize);
    *g.offset(4_isize) =
        *cpx.offset(0_isize) * *cpx.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(5_isize) =
        *cpx.offset(1_isize) * *cpx.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(8_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(9_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(10_isize) = *cpx.offset(0_isize)
        * (*g.offset(8_isize) + *b00.offset(0_isize))
        + *b01.offset(0_isize) * *c0x.offset(0_isize);
    *g.offset(11_isize) = *cpx.offset(1_isize)
        * (*g.offset(9_isize) + *b00.offset(1_isize))
        + *b01.offset(1_isize) * *c0x.offset(1_isize);
    *g.offset(12_isize) = 1_f64;
    *g.offset(13_isize) = 1_f64;
    *g.offset(14_isize) = *cpy.offset(0_isize);
    *g.offset(15_isize) = *cpy.offset(1_isize);
    *g.offset(18_isize) = *c0y.offset(0_isize);
    *g.offset(19_isize) = *c0y.offset(1_isize);
    *g.offset(16_isize) =
        *cpy.offset(0_isize) * *cpy.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(17_isize) =
        *cpy.offset(1_isize) * *cpy.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(20_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(21_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(22_isize) = *cpy.offset(0_isize)
        * (*g.offset(20_isize) + *b00.offset(0_isize))
        + *b01.offset(0_isize) * *c0y.offset(0_isize);
    *g.offset(23_isize) = *cpy.offset(1_isize)
        * (*g.offset(21_isize) + *b00.offset(1_isize))
        + *b01.offset(1_isize) * *c0y.offset(1_isize);
    *g.offset(26_isize) = *cpz.offset(0_isize) * *g.offset(24_isize);
    *g.offset(27_isize) = *cpz.offset(1_isize) * *g.offset(25_isize);
    *g.offset(30_isize) = *c0z.offset(0_isize) * *g.offset(24_isize);
    *g.offset(31_isize) = *c0z.offset(1_isize) * *g.offset(25_isize);
    *g.offset(28_isize) = *cpz.offset(0_isize) * *g.offset(26_isize)
        + *b01.offset(0_isize) * *g.offset(24_isize);
    *g.offset(29_isize) = *cpz.offset(1_isize) * *g.offset(27_isize)
        + *b01.offset(1_isize) * *g.offset(25_isize);
    *g.offset(32_isize) = *cpz.offset(0_isize) * *g.offset(30_isize)
        + *b00.offset(0_isize) * *g.offset(24_isize);
    *g.offset(33_isize) = *cpz.offset(1_isize) * *g.offset(31_isize)
        + *b00.offset(1_isize) * *g.offset(25_isize);
    *g.offset(34_isize) = *cpz.offset(0_isize) * *g.offset(32_isize)
        + *b01.offset(0_isize) * *g.offset(30_isize)
        + *b00.offset(0_isize) * *g.offset(26_isize);
    *g.offset(35_isize) = *cpz.offset(1_isize) * *g.offset(33_isize)
        + *b01.offset(1_isize) * *g.offset(31_isize)
        + *b00.offset(1_isize) * *g.offset(27_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0110(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *cpx.offset(0_isize);
    *g.offset(3_isize) = *cpx.offset(1_isize);
    *g.offset(4_isize) = *c0x.offset(0_isize);
    *g.offset(5_isize) = *c0x.offset(1_isize);
    *g.offset(6_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(7_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(8_isize) = 1_f64;
    *g.offset(9_isize) = 1_f64;
    *g.offset(10_isize) = *cpy.offset(0_isize);
    *g.offset(11_isize) = *cpy.offset(1_isize);
    *g.offset(12_isize) = *c0y.offset(0_isize);
    *g.offset(13_isize) = *c0y.offset(1_isize);
    *g.offset(14_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(15_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(18_isize) = *cpz.offset(0_isize) * *g.offset(16_isize);
    *g.offset(19_isize) = *cpz.offset(1_isize) * *g.offset(17_isize);
    *g.offset(20_isize) = *c0z.offset(0_isize) * *g.offset(16_isize);
    *g.offset(21_isize) = *c0z.offset(1_isize) * *g.offset(17_isize);
    *g.offset(22_isize) = *cpz.offset(0_isize) * *g.offset(20_isize)
        + *b00.offset(0_isize) * *g.offset(16_isize);
    *g.offset(23_isize) = *cpz.offset(1_isize) * *g.offset(21_isize)
        + *b00.offset(1_isize) * *g.offset(17_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0111(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    let xkxl: f64 = (*envs).rkrl[0_usize];
    let ykyl: f64 = (*envs).rkrl[1_usize];
    let zkzl: f64 = (*envs).rkrl[2_usize];
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(12_isize) = *c0x.offset(0_isize);
    *g.offset(13_isize) = *c0x.offset(1_isize);
    *g.offset(4_isize) = *cpx.offset(0_isize);
    *g.offset(5_isize) = *cpx.offset(1_isize);
    *g.offset(16_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(17_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(6_isize) =
        *cpx.offset(0_isize) * (xkxl + *cpx.offset(0_isize)) + *b01.offset(0_isize);
    *g.offset(7_isize) =
        *cpx.offset(1_isize) * (xkxl + *cpx.offset(1_isize)) + *b01.offset(1_isize);
    *g.offset(18_isize) = *g.offset(16_isize) * (xkxl + *cpx.offset(0_isize))
        + *cpx.offset(0_isize) * *b00.offset(0_isize)
        + *b01.offset(0_isize) * *c0x.offset(0_isize);
    *g.offset(19_isize) = *g.offset(17_isize) * (xkxl + *cpx.offset(1_isize))
        + *cpx.offset(1_isize) * *b00.offset(1_isize)
        + *b01.offset(1_isize) * *c0x.offset(1_isize);
    *g.offset(2_isize) = xkxl + *cpx.offset(0_isize);
    *g.offset(3_isize) = xkxl + *cpx.offset(1_isize);
    *g.offset(14_isize) =
        *c0x.offset(0_isize) * (xkxl + *cpx.offset(0_isize)) + *b00.offset(0_isize);
    *g.offset(15_isize) =
        *c0x.offset(1_isize) * (xkxl + *cpx.offset(1_isize)) + *b00.offset(1_isize);
    *g.offset(24_isize) = 1_f64;
    *g.offset(25_isize) = 1_f64;
    *g.offset(36_isize) = *c0y.offset(0_isize);
    *g.offset(37_isize) = *c0y.offset(1_isize);
    *g.offset(28_isize) = *cpy.offset(0_isize);
    *g.offset(29_isize) = *cpy.offset(1_isize);
    *g.offset(40_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(41_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(30_isize) =
        *cpy.offset(0_isize) * (ykyl + *cpy.offset(0_isize)) + *b01.offset(0_isize);
    *g.offset(31_isize) =
        *cpy.offset(1_isize) * (ykyl + *cpy.offset(1_isize)) + *b01.offset(1_isize);
    *g.offset(42_isize) = *g.offset(40_isize) * (ykyl + *cpy.offset(0_isize))
        + *cpy.offset(0_isize) * *b00.offset(0_isize)
        + *b01.offset(0_isize) * *c0y.offset(0_isize);
    *g.offset(43_isize) = *g.offset(41_isize) * (ykyl + *cpy.offset(1_isize))
        + *cpy.offset(1_isize) * *b00.offset(1_isize)
        + *b01.offset(1_isize) * *c0y.offset(1_isize);
    *g.offset(26_isize) = ykyl + *cpy.offset(0_isize);
    *g.offset(27_isize) = ykyl + *cpy.offset(1_isize);
    *g.offset(38_isize) =
        *c0y.offset(0_isize) * (ykyl + *cpy.offset(0_isize)) + *b00.offset(0_isize);
    *g.offset(39_isize) =
        *c0y.offset(1_isize) * (ykyl + *cpy.offset(1_isize)) + *b00.offset(1_isize);
    *g.offset(60_isize) = *c0z.offset(0_isize) * *g.offset(48_isize);
    *g.offset(61_isize) = *c0z.offset(1_isize) * *g.offset(49_isize);
    *g.offset(52_isize) = *cpz.offset(0_isize) * *g.offset(48_isize);
    *g.offset(53_isize) = *cpz.offset(1_isize) * *g.offset(49_isize);
    *g.offset(64_isize) = *cpz.offset(0_isize) * *g.offset(60_isize)
        + *b00.offset(0_isize) * *g.offset(48_isize);
    *g.offset(65_isize) = *cpz.offset(1_isize) * *g.offset(61_isize)
        + *b00.offset(1_isize) * *g.offset(49_isize);
    *g.offset(54_isize) = *g.offset(52_isize) * (zkzl + *cpz.offset(0_isize))
        + *b01.offset(0_isize) * *g.offset(48_isize);
    *g.offset(55_isize) = *g.offset(53_isize) * (zkzl + *cpz.offset(1_isize))
        + *b01.offset(1_isize) * *g.offset(49_isize);
    *g.offset(66_isize) = *g.offset(64_isize) * (zkzl + *cpz.offset(0_isize))
        + *b01.offset(0_isize) * *g.offset(60_isize)
        + *b00.offset(0_isize) * *g.offset(52_isize);
    *g.offset(67_isize) = *g.offset(65_isize) * (zkzl + *cpz.offset(1_isize))
        + *b01.offset(1_isize) * *g.offset(61_isize)
        + *b00.offset(1_isize) * *g.offset(53_isize);
    *g.offset(50_isize) = *g.offset(48_isize) * (zkzl + *cpz.offset(0_isize));
    *g.offset(51_isize) = *g.offset(49_isize) * (zkzl + *cpz.offset(1_isize));
    *g.offset(62_isize) = *g.offset(60_isize) * (zkzl + *cpz.offset(0_isize))
        + *b00.offset(0_isize) * *g.offset(48_isize);
    *g.offset(63_isize) = *g.offset(61_isize) * (zkzl + *cpz.offset(1_isize))
        + *b00.offset(1_isize) * *g.offset(49_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0120(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *cpx.offset(0_isize);
    *g.offset(3_isize) = *cpx.offset(1_isize);
    *g.offset(6_isize) = *c0x.offset(0_isize);
    *g.offset(7_isize) = *c0x.offset(1_isize);
    *g.offset(4_isize) =
        *cpx.offset(0_isize) * *cpx.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(5_isize) =
        *cpx.offset(1_isize) * *cpx.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(8_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(9_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(10_isize) = *cpx.offset(0_isize)
        * (*g.offset(8_isize) + *b00.offset(0_isize))
        + *b01.offset(0_isize) * *c0x.offset(0_isize);
    *g.offset(11_isize) = *cpx.offset(1_isize)
        * (*g.offset(9_isize) + *b00.offset(1_isize))
        + *b01.offset(1_isize) * *c0x.offset(1_isize);
    *g.offset(12_isize) = 1_f64;
    *g.offset(13_isize) = 1_f64;
    *g.offset(14_isize) = *cpy.offset(0_isize);
    *g.offset(15_isize) = *cpy.offset(1_isize);
    *g.offset(18_isize) = *c0y.offset(0_isize);
    *g.offset(19_isize) = *c0y.offset(1_isize);
    *g.offset(16_isize) =
        *cpy.offset(0_isize) * *cpy.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(17_isize) =
        *cpy.offset(1_isize) * *cpy.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(20_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(21_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(22_isize) = *cpy.offset(0_isize)
        * (*g.offset(20_isize) + *b00.offset(0_isize))
        + *b01.offset(0_isize) * *c0y.offset(0_isize);
    *g.offset(23_isize) = *cpy.offset(1_isize)
        * (*g.offset(21_isize) + *b00.offset(1_isize))
        + *b01.offset(1_isize) * *c0y.offset(1_isize);
    *g.offset(26_isize) = *cpz.offset(0_isize) * *g.offset(24_isize);
    *g.offset(27_isize) = *cpz.offset(1_isize) * *g.offset(25_isize);
    *g.offset(30_isize) = *c0z.offset(0_isize) * *g.offset(24_isize);
    *g.offset(31_isize) = *c0z.offset(1_isize) * *g.offset(25_isize);
    *g.offset(28_isize) = *cpz.offset(0_isize) * *g.offset(26_isize)
        + *b01.offset(0_isize) * *g.offset(24_isize);
    *g.offset(29_isize) = *cpz.offset(1_isize) * *g.offset(27_isize)
        + *b01.offset(1_isize) * *g.offset(25_isize);
    *g.offset(32_isize) = *cpz.offset(0_isize) * *g.offset(30_isize)
        + *b00.offset(0_isize) * *g.offset(24_isize);
    *g.offset(33_isize) = *cpz.offset(1_isize) * *g.offset(31_isize)
        + *b00.offset(1_isize) * *g.offset(25_isize);
    *g.offset(34_isize) = *cpz.offset(0_isize) * *g.offset(32_isize)
        + *b01.offset(0_isize) * *g.offset(30_isize)
        + *b00.offset(0_isize) * *g.offset(26_isize);
    *g.offset(35_isize) = *cpz.offset(1_isize) * *g.offset(33_isize)
        + *b01.offset(1_isize) * *g.offset(31_isize)
        + *b00.offset(1_isize) * *g.offset(27_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0200(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *c0x.offset(0_isize);
    *g.offset(3_isize) = *c0x.offset(1_isize);
    *g.offset(4_isize) =
        *c0x.offset(0_isize) * *c0x.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(5_isize) =
        *c0x.offset(1_isize) * *c0x.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(6_isize) = 1_f64;
    *g.offset(7_isize) = 1_f64;
    *g.offset(8_isize) = *c0y.offset(0_isize);
    *g.offset(9_isize) = *c0y.offset(1_isize);
    *g.offset(10_isize) =
        *c0y.offset(0_isize) * *c0y.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(11_isize) =
        *c0y.offset(1_isize) * *c0y.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(14_isize) = *c0z.offset(0_isize) * *g.offset(12_isize);
    *g.offset(15_isize) = *c0z.offset(1_isize) * *g.offset(13_isize);
    *g.offset(16_isize) = *c0z.offset(0_isize) * *g.offset(14_isize)
        + *b10.offset(0_isize) * *g.offset(12_isize);
    *g.offset(17_isize) = *c0z.offset(1_isize) * *g.offset(15_isize)
        + *b10.offset(1_isize) * *g.offset(13_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0201(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(4_isize) = *c0x.offset(0_isize);
    *g.offset(5_isize) = *c0x.offset(1_isize);
    *g.offset(8_isize) =
        *c0x.offset(0_isize) * *c0x.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(9_isize) =
        *c0x.offset(1_isize) * *c0x.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(2_isize) = *cpx.offset(0_isize);
    *g.offset(3_isize) = *cpx.offset(1_isize);
    *g.offset(6_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(7_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(10_isize) = *c0x.offset(0_isize)
        * (*g.offset(6_isize) + *b00.offset(0_isize))
        + *b10.offset(0_isize) * *cpx.offset(0_isize);
    *g.offset(11_isize) = *c0x.offset(1_isize)
        * (*g.offset(7_isize) + *b00.offset(1_isize))
        + *b10.offset(1_isize) * *cpx.offset(1_isize);
    *g.offset(12_isize) = 1_f64;
    *g.offset(13_isize) = 1_f64;
    *g.offset(16_isize) = *c0y.offset(0_isize);
    *g.offset(17_isize) = *c0y.offset(1_isize);
    *g.offset(20_isize) =
        *c0y.offset(0_isize) * *c0y.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(21_isize) =
        *c0y.offset(1_isize) * *c0y.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(14_isize) = *cpy.offset(0_isize);
    *g.offset(15_isize) = *cpy.offset(1_isize);
    *g.offset(18_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(19_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(22_isize) = *c0y.offset(0_isize)
        * (*g.offset(18_isize) + *b00.offset(0_isize))
        + *b10.offset(0_isize) * *cpy.offset(0_isize);
    *g.offset(23_isize) = *c0y.offset(1_isize)
        * (*g.offset(19_isize) + *b00.offset(1_isize))
        + *b10.offset(1_isize) * *cpy.offset(1_isize);
    *g.offset(28_isize) = *c0z.offset(0_isize) * *g.offset(24_isize);
    *g.offset(29_isize) = *c0z.offset(1_isize) * *g.offset(25_isize);
    *g.offset(32_isize) = *c0z.offset(0_isize) * *g.offset(28_isize)
        + *b10.offset(0_isize) * *g.offset(24_isize);
    *g.offset(33_isize) = *c0z.offset(1_isize) * *g.offset(29_isize)
        + *b10.offset(1_isize) * *g.offset(25_isize);
    *g.offset(26_isize) = *cpz.offset(0_isize) * *g.offset(24_isize);
    *g.offset(27_isize) = *cpz.offset(1_isize) * *g.offset(25_isize);
    *g.offset(30_isize) = *cpz.offset(0_isize) * *g.offset(28_isize)
        + *b00.offset(0_isize) * *g.offset(24_isize);
    *g.offset(31_isize) = *cpz.offset(1_isize) * *g.offset(29_isize)
        + *b00.offset(1_isize) * *g.offset(25_isize);
    *g.offset(34_isize) = *c0z.offset(0_isize) * *g.offset(30_isize)
        + *b10.offset(0_isize) * *g.offset(26_isize)
        + *b00.offset(0_isize) * *g.offset(28_isize);
    *g.offset(35_isize) = *c0z.offset(1_isize) * *g.offset(31_isize)
        + *b10.offset(1_isize) * *g.offset(27_isize)
        + *b00.offset(1_isize) * *g.offset(29_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0210(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *cpx.offset(0_isize);
    *g.offset(3_isize) = *cpx.offset(1_isize);
    *g.offset(4_isize) = *c0x.offset(0_isize);
    *g.offset(5_isize) = *c0x.offset(1_isize);
    *g.offset(6_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(7_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(8_isize) =
        *c0x.offset(0_isize) * *c0x.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(9_isize) =
        *c0x.offset(1_isize) * *c0x.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(10_isize) = *c0x.offset(0_isize)
        * (*g.offset(6_isize) + *b00.offset(0_isize))
        + *b10.offset(0_isize) * *cpx.offset(0_isize);
    *g.offset(11_isize) = *c0x.offset(1_isize)
        * (*g.offset(7_isize) + *b00.offset(1_isize))
        + *b10.offset(1_isize) * *cpx.offset(1_isize);
    *g.offset(12_isize) = 1_f64;
    *g.offset(13_isize) = 1_f64;
    *g.offset(14_isize) = *cpy.offset(0_isize);
    *g.offset(15_isize) = *cpy.offset(1_isize);
    *g.offset(16_isize) = *c0y.offset(0_isize);
    *g.offset(17_isize) = *c0y.offset(1_isize);
    *g.offset(18_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(19_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(20_isize) =
        *c0y.offset(0_isize) * *c0y.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(21_isize) =
        *c0y.offset(1_isize) * *c0y.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(22_isize) = *c0y.offset(0_isize)
        * (*g.offset(18_isize) + *b00.offset(0_isize))
        + *b10.offset(0_isize) * *cpy.offset(0_isize);
    *g.offset(23_isize) = *c0y.offset(1_isize)
        * (*g.offset(19_isize) + *b00.offset(1_isize))
        + *b10.offset(1_isize) * *cpy.offset(1_isize);
    *g.offset(26_isize) = *cpz.offset(0_isize) * *g.offset(24_isize);
    *g.offset(27_isize) = *cpz.offset(1_isize) * *g.offset(25_isize);
    *g.offset(28_isize) = *c0z.offset(0_isize) * *g.offset(24_isize);
    *g.offset(29_isize) = *c0z.offset(1_isize) * *g.offset(25_isize);
    *g.offset(30_isize) = *cpz.offset(0_isize) * *g.offset(28_isize)
        + *b00.offset(0_isize) * *g.offset(24_isize);
    *g.offset(31_isize) = *cpz.offset(1_isize) * *g.offset(29_isize)
        + *b00.offset(1_isize) * *g.offset(25_isize);
    *g.offset(32_isize) = *c0z.offset(0_isize) * *g.offset(28_isize)
        + *b10.offset(0_isize) * *g.offset(24_isize);
    *g.offset(33_isize) = *c0z.offset(1_isize) * *g.offset(29_isize)
        + *b10.offset(1_isize) * *g.offset(25_isize);
    *g.offset(34_isize) = *c0z.offset(0_isize) * *g.offset(30_isize)
        + *b10.offset(0_isize) * *g.offset(26_isize)
        + *b00.offset(0_isize) * *g.offset(28_isize);
    *g.offset(35_isize) = *c0z.offset(1_isize) * *g.offset(31_isize)
        + *b10.offset(1_isize) * *g.offset(27_isize)
        + *b00.offset(1_isize) * *g.offset(29_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0300(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *c0x.offset(0_isize);
    *g.offset(3_isize) = *c0x.offset(1_isize);
    *g.offset(4_isize) =
        *c0x.offset(0_isize) * *c0x.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(5_isize) =
        *c0x.offset(1_isize) * *c0x.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(6_isize) =
        *c0x.offset(0_isize) * (*g.offset(4_isize) + 2_f64 * *b10.offset(0_isize));
    *g.offset(7_isize) =
        *c0x.offset(1_isize) * (*g.offset(5_isize) + 2_f64 * *b10.offset(1_isize));
    *g.offset(8_isize) = 1_f64;
    *g.offset(9_isize) = 1_f64;
    *g.offset(10_isize) = *c0y.offset(0_isize);
    *g.offset(11_isize) = *c0y.offset(1_isize);
    *g.offset(12_isize) =
        *c0y.offset(0_isize) * *c0y.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(13_isize) =
        *c0y.offset(1_isize) * *c0y.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(14_isize) =
        *c0y.offset(0_isize) * (*g.offset(12_isize) + 2_f64 * *b10.offset(0_isize));
    *g.offset(15_isize) =
        *c0y.offset(1_isize) * (*g.offset(13_isize) + 2_f64 * *b10.offset(1_isize));
    *g.offset(18_isize) = *c0z.offset(0_isize) * *g.offset(16_isize);
    *g.offset(19_isize) = *c0z.offset(1_isize) * *g.offset(17_isize);
    *g.offset(20_isize) = *c0z.offset(0_isize) * *g.offset(18_isize)
        + *b10.offset(0_isize) * *g.offset(16_isize);
    *g.offset(21_isize) = *c0z.offset(1_isize) * *g.offset(19_isize)
        + *b10.offset(1_isize) * *g.offset(17_isize);
    *g.offset(22_isize) = *c0z.offset(0_isize) * *g.offset(20_isize)
        + 2_f64 * *b10.offset(0_isize) * *g.offset(18_isize);
    *g.offset(23_isize) = *c0z.offset(1_isize) * *g.offset(21_isize)
        + 2_f64 * *b10.offset(1_isize) * *g.offset(19_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1000(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = *c0x.offset(0_isize);
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = *c0y.offset(0_isize);
    *g.offset(5_isize) = *c0z.offset(0_isize) * *g.offset(4_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1001(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *c0x.offset(0_isize);
    *g.offset(3_isize) = *c0x.offset(1_isize);
    *g.offset(4_isize) = *cpx.offset(0_isize);
    *g.offset(5_isize) = *cpx.offset(1_isize);
    *g.offset(6_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(7_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(8_isize) = 1_f64;
    *g.offset(9_isize) = 1_f64;
    *g.offset(10_isize) = *c0y.offset(0_isize);
    *g.offset(11_isize) = *c0y.offset(1_isize);
    *g.offset(12_isize) = *cpy.offset(0_isize);
    *g.offset(13_isize) = *cpy.offset(1_isize);
    *g.offset(14_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(15_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(18_isize) = *c0z.offset(0_isize) * *g.offset(16_isize);
    *g.offset(19_isize) = *c0z.offset(1_isize) * *g.offset(17_isize);
    *g.offset(20_isize) = *cpz.offset(0_isize) * *g.offset(16_isize);
    *g.offset(21_isize) = *cpz.offset(1_isize) * *g.offset(17_isize);
    *g.offset(22_isize) = *cpz.offset(0_isize) * *g.offset(18_isize)
        + *b00.offset(0_isize) * *g.offset(16_isize);
    *g.offset(23_isize) = *cpz.offset(1_isize) * *g.offset(19_isize)
        + *b00.offset(1_isize) * *g.offset(17_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1002(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *c0x.offset(0_isize);
    *g.offset(3_isize) = *c0x.offset(1_isize);
    *g.offset(4_isize) = *cpx.offset(0_isize);
    *g.offset(5_isize) = *cpx.offset(1_isize);
    *g.offset(6_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(7_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(8_isize) =
        *cpx.offset(0_isize) * *cpx.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(9_isize) =
        *cpx.offset(1_isize) * *cpx.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(10_isize) = *cpx.offset(0_isize)
        * (*g.offset(6_isize) + *b00.offset(0_isize))
        + *b01.offset(0_isize) * *c0x.offset(0_isize);
    *g.offset(11_isize) = *cpx.offset(1_isize)
        * (*g.offset(7_isize) + *b00.offset(1_isize))
        + *b01.offset(1_isize) * *c0x.offset(1_isize);
    *g.offset(12_isize) = 1_f64;
    *g.offset(13_isize) = 1_f64;
    *g.offset(14_isize) = *c0y.offset(0_isize);
    *g.offset(15_isize) = *c0y.offset(1_isize);
    *g.offset(16_isize) = *cpy.offset(0_isize);
    *g.offset(17_isize) = *cpy.offset(1_isize);
    *g.offset(18_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(19_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(20_isize) =
        *cpy.offset(0_isize) * *cpy.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(21_isize) =
        *cpy.offset(1_isize) * *cpy.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(22_isize) = *cpy.offset(0_isize)
        * (*g.offset(18_isize) + *b00.offset(0_isize))
        + *b01.offset(0_isize) * *c0y.offset(0_isize);
    *g.offset(23_isize) = *cpy.offset(1_isize)
        * (*g.offset(19_isize) + *b00.offset(1_isize))
        + *b01.offset(1_isize) * *c0y.offset(1_isize);
    *g.offset(26_isize) = *c0z.offset(0_isize) * *g.offset(24_isize);
    *g.offset(27_isize) = *c0z.offset(1_isize) * *g.offset(25_isize);
    *g.offset(28_isize) = *cpz.offset(0_isize) * *g.offset(24_isize);
    *g.offset(29_isize) = *cpz.offset(1_isize) * *g.offset(25_isize);
    *g.offset(30_isize) = *cpz.offset(0_isize) * *g.offset(26_isize)
        + *b00.offset(0_isize) * *g.offset(24_isize);
    *g.offset(31_isize) = *cpz.offset(1_isize) * *g.offset(27_isize)
        + *b00.offset(1_isize) * *g.offset(25_isize);
    *g.offset(32_isize) = *cpz.offset(0_isize) * *g.offset(28_isize)
        + *b01.offset(0_isize) * *g.offset(24_isize);
    *g.offset(33_isize) = *cpz.offset(1_isize) * *g.offset(29_isize)
        + *b01.offset(1_isize) * *g.offset(25_isize);
    *g.offset(34_isize) = *cpz.offset(0_isize) * *g.offset(30_isize)
        + *b01.offset(0_isize) * *g.offset(26_isize)
        + *b00.offset(0_isize) * *g.offset(28_isize);
    *g.offset(35_isize) = *cpz.offset(1_isize) * *g.offset(31_isize)
        + *b01.offset(1_isize) * *g.offset(27_isize)
        + *b00.offset(1_isize) * *g.offset(29_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1010(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *c0x.offset(0_isize);
    *g.offset(3_isize) = *c0x.offset(1_isize);
    *g.offset(4_isize) = *cpx.offset(0_isize);
    *g.offset(5_isize) = *cpx.offset(1_isize);
    *g.offset(6_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(7_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(8_isize) = 1_f64;
    *g.offset(9_isize) = 1_f64;
    *g.offset(10_isize) = *c0y.offset(0_isize);
    *g.offset(11_isize) = *c0y.offset(1_isize);
    *g.offset(12_isize) = *cpy.offset(0_isize);
    *g.offset(13_isize) = *cpy.offset(1_isize);
    *g.offset(14_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(15_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(18_isize) = *c0z.offset(0_isize) * *g.offset(16_isize);
    *g.offset(19_isize) = *c0z.offset(1_isize) * *g.offset(17_isize);
    *g.offset(20_isize) = *cpz.offset(0_isize) * *g.offset(16_isize);
    *g.offset(21_isize) = *cpz.offset(1_isize) * *g.offset(17_isize);
    *g.offset(22_isize) = *cpz.offset(0_isize) * *g.offset(18_isize)
        + *b00.offset(0_isize) * *g.offset(16_isize);
    *g.offset(23_isize) = *cpz.offset(1_isize) * *g.offset(19_isize)
        + *b00.offset(1_isize) * *g.offset(17_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1011(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    let xkxl: f64 = (*envs).rkrl[0_usize];
    let ykyl: f64 = (*envs).rkrl[1_usize];
    let zkzl: f64 = (*envs).rkrl[2_usize];
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *c0x.offset(0_isize);
    *g.offset(3_isize) = *c0x.offset(1_isize);
    *g.offset(8_isize) = *cpx.offset(0_isize);
    *g.offset(9_isize) = *cpx.offset(1_isize);
    *g.offset(10_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(11_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(12_isize) =
        *cpx.offset(0_isize) * (xkxl + *cpx.offset(0_isize)) + *b01.offset(0_isize);
    *g.offset(13_isize) =
        *cpx.offset(1_isize) * (xkxl + *cpx.offset(1_isize)) + *b01.offset(1_isize);
    *g.offset(14_isize) = *g.offset(10_isize) * (xkxl + *cpx.offset(0_isize))
        + *cpx.offset(0_isize) * *b00.offset(0_isize)
        + *b01.offset(0_isize) * *c0x.offset(0_isize);
    *g.offset(15_isize) = *g.offset(11_isize) * (xkxl + *cpx.offset(1_isize))
        + *cpx.offset(1_isize) * *b00.offset(1_isize)
        + *b01.offset(1_isize) * *c0x.offset(1_isize);
    *g.offset(4_isize) = xkxl + *cpx.offset(0_isize);
    *g.offset(5_isize) = xkxl + *cpx.offset(1_isize);
    *g.offset(6_isize) =
        *c0x.offset(0_isize) * (xkxl + *cpx.offset(0_isize)) + *b00.offset(0_isize);
    *g.offset(7_isize) =
        *c0x.offset(1_isize) * (xkxl + *cpx.offset(1_isize)) + *b00.offset(1_isize);
    *g.offset(24_isize) = 1_f64;
    *g.offset(25_isize) = 1_f64;
    *g.offset(26_isize) = *c0y.offset(0_isize);
    *g.offset(27_isize) = *c0y.offset(1_isize);
    *g.offset(32_isize) = *cpy.offset(0_isize);
    *g.offset(33_isize) = *cpy.offset(1_isize);
    *g.offset(34_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(35_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(36_isize) =
        *cpy.offset(0_isize) * (ykyl + *cpy.offset(0_isize)) + *b01.offset(0_isize);
    *g.offset(37_isize) =
        *cpy.offset(1_isize) * (ykyl + *cpy.offset(1_isize)) + *b01.offset(1_isize);
    *g.offset(38_isize) = *g.offset(34_isize) * (ykyl + *cpy.offset(0_isize))
        + *cpy.offset(0_isize) * *b00.offset(0_isize)
        + *b01.offset(0_isize) * *c0y.offset(0_isize);
    *g.offset(39_isize) = *g.offset(35_isize) * (ykyl + *cpy.offset(1_isize))
        + *cpy.offset(1_isize) * *b00.offset(1_isize)
        + *b01.offset(1_isize) * *c0y.offset(1_isize);
    *g.offset(28_isize) = ykyl + *cpy.offset(0_isize);
    *g.offset(29_isize) = ykyl + *cpy.offset(1_isize);
    *g.offset(30_isize) =
        *c0y.offset(0_isize) * (ykyl + *cpy.offset(0_isize)) + *b00.offset(0_isize);
    *g.offset(31_isize) =
        *c0y.offset(1_isize) * (ykyl + *cpy.offset(1_isize)) + *b00.offset(1_isize);
    *g.offset(50_isize) = *c0z.offset(0_isize) * *g.offset(48_isize);
    *g.offset(51_isize) = *c0z.offset(1_isize) * *g.offset(49_isize);
    *g.offset(56_isize) = *cpz.offset(0_isize) * *g.offset(48_isize);
    *g.offset(57_isize) = *cpz.offset(1_isize) * *g.offset(49_isize);
    *g.offset(58_isize) = *cpz.offset(0_isize) * *g.offset(50_isize)
        + *b00.offset(0_isize) * *g.offset(48_isize);
    *g.offset(59_isize) = *cpz.offset(1_isize) * *g.offset(51_isize)
        + *b00.offset(1_isize) * *g.offset(49_isize);
    *g.offset(60_isize) = *g.offset(56_isize) * (zkzl + *cpz.offset(0_isize))
        + *b01.offset(0_isize) * *g.offset(48_isize);
    *g.offset(61_isize) = *g.offset(57_isize) * (zkzl + *cpz.offset(1_isize))
        + *b01.offset(1_isize) * *g.offset(49_isize);
    *g.offset(62_isize) = *g.offset(58_isize) * (zkzl + *cpz.offset(0_isize))
        + *b01.offset(0_isize) * *g.offset(50_isize)
        + *b00.offset(0_isize) * *g.offset(56_isize);
    *g.offset(63_isize) = *g.offset(59_isize) * (zkzl + *cpz.offset(1_isize))
        + *b01.offset(1_isize) * *g.offset(51_isize)
        + *b00.offset(1_isize) * *g.offset(57_isize);
    *g.offset(52_isize) = *g.offset(48_isize) * (zkzl + *cpz.offset(0_isize));
    *g.offset(53_isize) = *g.offset(49_isize) * (zkzl + *cpz.offset(1_isize));
    *g.offset(54_isize) = *g.offset(50_isize) * (zkzl + *cpz.offset(0_isize))
        + *b00.offset(0_isize) * *g.offset(48_isize);
    *g.offset(55_isize) = *g.offset(51_isize) * (zkzl + *cpz.offset(1_isize))
        + *b00.offset(1_isize) * *g.offset(49_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1020(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *c0x.offset(0_isize);
    *g.offset(3_isize) = *c0x.offset(1_isize);
    *g.offset(4_isize) = *cpx.offset(0_isize);
    *g.offset(5_isize) = *cpx.offset(1_isize);
    *g.offset(6_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(7_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(8_isize) =
        *cpx.offset(0_isize) * *cpx.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(9_isize) =
        *cpx.offset(1_isize) * *cpx.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(10_isize) = *cpx.offset(0_isize)
        * (*g.offset(6_isize) + *b00.offset(0_isize))
        + *b01.offset(0_isize) * *c0x.offset(0_isize);
    *g.offset(11_isize) = *cpx.offset(1_isize)
        * (*g.offset(7_isize) + *b00.offset(1_isize))
        + *b01.offset(1_isize) * *c0x.offset(1_isize);
    *g.offset(12_isize) = 1_f64;
    *g.offset(13_isize) = 1_f64;
    *g.offset(14_isize) = *c0y.offset(0_isize);
    *g.offset(15_isize) = *c0y.offset(1_isize);
    *g.offset(16_isize) = *cpy.offset(0_isize);
    *g.offset(17_isize) = *cpy.offset(1_isize);
    *g.offset(18_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(19_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(20_isize) =
        *cpy.offset(0_isize) * *cpy.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(21_isize) =
        *cpy.offset(1_isize) * *cpy.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(22_isize) = *cpy.offset(0_isize)
        * (*g.offset(18_isize) + *b00.offset(0_isize))
        + *b01.offset(0_isize) * *c0y.offset(0_isize);
    *g.offset(23_isize) = *cpy.offset(1_isize)
        * (*g.offset(19_isize) + *b00.offset(1_isize))
        + *b01.offset(1_isize) * *c0y.offset(1_isize);
    *g.offset(26_isize) = *c0z.offset(0_isize) * *g.offset(24_isize);
    *g.offset(27_isize) = *c0z.offset(1_isize) * *g.offset(25_isize);
    *g.offset(28_isize) = *cpz.offset(0_isize) * *g.offset(24_isize);
    *g.offset(29_isize) = *cpz.offset(1_isize) * *g.offset(25_isize);
    *g.offset(30_isize) = *cpz.offset(0_isize) * *g.offset(26_isize)
        + *b00.offset(0_isize) * *g.offset(24_isize);
    *g.offset(31_isize) = *cpz.offset(1_isize) * *g.offset(27_isize)
        + *b00.offset(1_isize) * *g.offset(25_isize);
    *g.offset(32_isize) = *cpz.offset(0_isize) * *g.offset(28_isize)
        + *b01.offset(0_isize) * *g.offset(24_isize);
    *g.offset(33_isize) = *cpz.offset(1_isize) * *g.offset(29_isize)
        + *b01.offset(1_isize) * *g.offset(25_isize);
    *g.offset(34_isize) = *cpz.offset(0_isize) * *g.offset(30_isize)
        + *b01.offset(0_isize) * *g.offset(26_isize)
        + *b00.offset(0_isize) * *g.offset(28_isize);
    *g.offset(35_isize) = *cpz.offset(1_isize) * *g.offset(31_isize)
        + *b01.offset(1_isize) * *g.offset(27_isize)
        + *b00.offset(1_isize) * *g.offset(29_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1100(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    let xixj: f64 = (*envs).rirj[0_usize];
    let yiyj: f64 = (*envs).rirj[1_usize];
    let zizj: f64 = (*envs).rirj[2_usize];
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(4_isize) = *c0x.offset(0_isize);
    *g.offset(5_isize) = *c0x.offset(1_isize);
    *g.offset(6_isize) =
        *c0x.offset(0_isize) * (xixj + *c0x.offset(0_isize)) + *b10.offset(0_isize);
    *g.offset(7_isize) =
        *c0x.offset(1_isize) * (xixj + *c0x.offset(1_isize)) + *b10.offset(1_isize);
    *g.offset(2_isize) = xixj + *c0x.offset(0_isize);
    *g.offset(3_isize) = xixj + *c0x.offset(1_isize);
    *g.offset(12_isize) = 1_f64;
    *g.offset(13_isize) = 1_f64;
    *g.offset(16_isize) = *c0y.offset(0_isize);
    *g.offset(17_isize) = *c0y.offset(1_isize);
    *g.offset(18_isize) =
        *c0y.offset(0_isize) * (yiyj + *c0y.offset(0_isize)) + *b10.offset(0_isize);
    *g.offset(19_isize) =
        *c0y.offset(1_isize) * (yiyj + *c0y.offset(1_isize)) + *b10.offset(1_isize);
    *g.offset(14_isize) = yiyj + *c0y.offset(0_isize);
    *g.offset(15_isize) = yiyj + *c0y.offset(1_isize);
    *g.offset(28_isize) = *c0z.offset(0_isize) * *g.offset(24_isize);
    *g.offset(29_isize) = *c0z.offset(1_isize) * *g.offset(25_isize);
    *g.offset(30_isize) = *g.offset(28_isize) * (zizj + *c0z.offset(0_isize))
        + *b10.offset(0_isize) * *g.offset(24_isize);
    *g.offset(31_isize) = *g.offset(29_isize) * (zizj + *c0z.offset(1_isize))
        + *b10.offset(1_isize) * *g.offset(25_isize);
    *g.offset(26_isize) = *g.offset(24_isize) * (zizj + *c0z.offset(0_isize));
    *g.offset(27_isize) = *g.offset(25_isize) * (zizj + *c0z.offset(1_isize));
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1101(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    let xixj: f64 = (*envs).rirj[0_usize];
    let yiyj: f64 = (*envs).rirj[1_usize];
    let zizj: f64 = (*envs).rirj[2_usize];
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(8_isize) = *c0x.offset(0_isize);
    *g.offset(9_isize) = *c0x.offset(1_isize);
    *g.offset(4_isize) = *cpx.offset(0_isize);
    *g.offset(5_isize) = *cpx.offset(1_isize);
    *g.offset(12_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(13_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(10_isize) =
        *c0x.offset(0_isize) * (xixj + *c0x.offset(0_isize)) + *b10.offset(0_isize);
    *g.offset(11_isize) =
        *c0x.offset(1_isize) * (xixj + *c0x.offset(1_isize)) + *b10.offset(1_isize);
    *g.offset(2_isize) = xixj + *c0x.offset(0_isize);
    *g.offset(3_isize) = xixj + *c0x.offset(1_isize);
    *g.offset(14_isize) = *g.offset(12_isize) * (xixj + *c0x.offset(0_isize))
        + *c0x.offset(0_isize) * *b00.offset(0_isize)
        + *b10.offset(0_isize) * *cpx.offset(0_isize);
    *g.offset(15_isize) = *g.offset(13_isize) * (xixj + *c0x.offset(1_isize))
        + *c0x.offset(1_isize) * *b00.offset(1_isize)
        + *b10.offset(1_isize) * *cpx.offset(1_isize);
    *g.offset(6_isize) =
        *cpx.offset(0_isize) * (xixj + *c0x.offset(0_isize)) + *b00.offset(0_isize);
    *g.offset(7_isize) =
        *cpx.offset(1_isize) * (xixj + *c0x.offset(1_isize)) + *b00.offset(1_isize);
    *g.offset(24_isize) = 1_f64;
    *g.offset(25_isize) = 1_f64;
    *g.offset(32_isize) = *c0y.offset(0_isize);
    *g.offset(33_isize) = *c0y.offset(1_isize);
    *g.offset(28_isize) = *cpy.offset(0_isize);
    *g.offset(29_isize) = *cpy.offset(1_isize);
    *g.offset(36_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(37_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(34_isize) =
        *c0y.offset(0_isize) * (yiyj + *c0y.offset(0_isize)) + *b10.offset(0_isize);
    *g.offset(35_isize) =
        *c0y.offset(1_isize) * (yiyj + *c0y.offset(1_isize)) + *b10.offset(1_isize);
    *g.offset(26_isize) = yiyj + *c0y.offset(0_isize);
    *g.offset(27_isize) = yiyj + *c0y.offset(1_isize);
    *g.offset(38_isize) = *g.offset(36_isize) * (yiyj + *c0y.offset(0_isize))
        + *c0y.offset(0_isize) * *b00.offset(0_isize)
        + *b10.offset(0_isize) * *cpy.offset(0_isize);
    *g.offset(39_isize) = *g.offset(37_isize) * (yiyj + *c0y.offset(1_isize))
        + *c0y.offset(1_isize) * *b00.offset(1_isize)
        + *b10.offset(1_isize) * *cpy.offset(1_isize);
    *g.offset(30_isize) =
        *cpy.offset(0_isize) * (yiyj + *c0y.offset(0_isize)) + *b00.offset(0_isize);
    *g.offset(31_isize) =
        *cpy.offset(1_isize) * (yiyj + *c0y.offset(1_isize)) + *b00.offset(1_isize);
    *g.offset(56_isize) = *c0z.offset(0_isize) * *g.offset(48_isize);
    *g.offset(57_isize) = *c0z.offset(1_isize) * *g.offset(49_isize);
    *g.offset(52_isize) = *cpz.offset(0_isize) * *g.offset(48_isize);
    *g.offset(53_isize) = *cpz.offset(1_isize) * *g.offset(49_isize);
    *g.offset(60_isize) = *cpz.offset(0_isize) * *g.offset(56_isize)
        + *b00.offset(0_isize) * *g.offset(48_isize);
    *g.offset(61_isize) = *cpz.offset(1_isize) * *g.offset(57_isize)
        + *b00.offset(1_isize) * *g.offset(49_isize);
    *g.offset(58_isize) = *g.offset(56_isize) * (zizj + *c0z.offset(0_isize))
        + *b10.offset(0_isize) * *g.offset(48_isize);
    *g.offset(59_isize) = *g.offset(57_isize) * (zizj + *c0z.offset(1_isize))
        + *b10.offset(1_isize) * *g.offset(49_isize);
    *g.offset(50_isize) = *g.offset(48_isize) * (zizj + *c0z.offset(0_isize));
    *g.offset(51_isize) = *g.offset(49_isize) * (zizj + *c0z.offset(1_isize));
    *g.offset(62_isize) = *g.offset(60_isize) * (zizj + *c0z.offset(0_isize))
        + *b10.offset(0_isize) * *g.offset(52_isize)
        + *b00.offset(0_isize) * *g.offset(56_isize);
    *g.offset(63_isize) = *g.offset(61_isize) * (zizj + *c0z.offset(1_isize))
        + *b10.offset(1_isize) * *g.offset(53_isize)
        + *b00.offset(1_isize) * *g.offset(57_isize);
    *g.offset(54_isize) = zizj * *g.offset(52_isize)
        + *cpz.offset(0_isize) * *g.offset(56_isize)
        + *b00.offset(0_isize) * *g.offset(48_isize);
    *g.offset(55_isize) = zizj * *g.offset(53_isize)
        + *cpz.offset(1_isize) * *g.offset(57_isize)
        + *b00.offset(1_isize) * *g.offset(49_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1110(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    let xixj: f64 = (*envs).rirj[0_usize];
    let yiyj: f64 = (*envs).rirj[1_usize];
    let zizj: f64 = (*envs).rirj[2_usize];
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(8_isize) = *c0x.offset(0_isize);
    *g.offset(9_isize) = *c0x.offset(1_isize);
    *g.offset(4_isize) = *cpx.offset(0_isize);
    *g.offset(5_isize) = *cpx.offset(1_isize);
    *g.offset(12_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(13_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(10_isize) =
        *c0x.offset(0_isize) * (xixj + *c0x.offset(0_isize)) + *b10.offset(0_isize);
    *g.offset(11_isize) =
        *c0x.offset(1_isize) * (xixj + *c0x.offset(1_isize)) + *b10.offset(1_isize);
    *g.offset(2_isize) = xixj + *c0x.offset(0_isize);
    *g.offset(3_isize) = xixj + *c0x.offset(1_isize);
    *g.offset(14_isize) = *g.offset(12_isize) * (xixj + *c0x.offset(0_isize))
        + *c0x.offset(0_isize) * *b00.offset(0_isize)
        + *b10.offset(0_isize) * *cpx.offset(0_isize);
    *g.offset(15_isize) = *g.offset(13_isize) * (xixj + *c0x.offset(1_isize))
        + *c0x.offset(1_isize) * *b00.offset(1_isize)
        + *b10.offset(1_isize) * *cpx.offset(1_isize);
    *g.offset(6_isize) =
        *cpx.offset(0_isize) * (xixj + *c0x.offset(0_isize)) + *b00.offset(0_isize);
    *g.offset(7_isize) =
        *cpx.offset(1_isize) * (xixj + *c0x.offset(1_isize)) + *b00.offset(1_isize);
    *g.offset(24_isize) = 1_f64;
    *g.offset(25_isize) = 1_f64;
    *g.offset(32_isize) = *c0y.offset(0_isize);
    *g.offset(33_isize) = *c0y.offset(1_isize);
    *g.offset(28_isize) = *cpy.offset(0_isize);
    *g.offset(29_isize) = *cpy.offset(1_isize);
    *g.offset(36_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(37_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(34_isize) =
        *c0y.offset(0_isize) * (yiyj + *c0y.offset(0_isize)) + *b10.offset(0_isize);
    *g.offset(35_isize) =
        *c0y.offset(1_isize) * (yiyj + *c0y.offset(1_isize)) + *b10.offset(1_isize);
    *g.offset(26_isize) = yiyj + *c0y.offset(0_isize);
    *g.offset(27_isize) = yiyj + *c0y.offset(1_isize);
    *g.offset(38_isize) = *g.offset(36_isize) * (yiyj + *c0y.offset(0_isize))
        + *c0y.offset(0_isize) * *b00.offset(0_isize)
        + *b10.offset(0_isize) * *cpy.offset(0_isize);
    *g.offset(39_isize) = *g.offset(37_isize) * (yiyj + *c0y.offset(1_isize))
        + *c0y.offset(1_isize) * *b00.offset(1_isize)
        + *b10.offset(1_isize) * *cpy.offset(1_isize);
    *g.offset(30_isize) =
        *cpy.offset(0_isize) * (yiyj + *c0y.offset(0_isize)) + *b00.offset(0_isize);
    *g.offset(31_isize) =
        *cpy.offset(1_isize) * (yiyj + *c0y.offset(1_isize)) + *b00.offset(1_isize);
    *g.offset(56_isize) = *c0z.offset(0_isize) * *g.offset(48_isize);
    *g.offset(57_isize) = *c0z.offset(1_isize) * *g.offset(49_isize);
    *g.offset(52_isize) = *cpz.offset(0_isize) * *g.offset(48_isize);
    *g.offset(53_isize) = *cpz.offset(1_isize) * *g.offset(49_isize);
    *g.offset(60_isize) = *cpz.offset(0_isize) * *g.offset(56_isize)
        + *b00.offset(0_isize) * *g.offset(48_isize);
    *g.offset(61_isize) = *cpz.offset(1_isize) * *g.offset(57_isize)
        + *b00.offset(1_isize) * *g.offset(49_isize);
    *g.offset(58_isize) = *g.offset(56_isize) * (zizj + *c0z.offset(0_isize))
        + *b10.offset(0_isize) * *g.offset(48_isize);
    *g.offset(59_isize) = *g.offset(57_isize) * (zizj + *c0z.offset(1_isize))
        + *b10.offset(1_isize) * *g.offset(49_isize);
    *g.offset(50_isize) = *g.offset(48_isize) * (zizj + *c0z.offset(0_isize));
    *g.offset(51_isize) = *g.offset(49_isize) * (zizj + *c0z.offset(1_isize));
    *g.offset(62_isize) = *g.offset(60_isize) * (zizj + *c0z.offset(0_isize))
        + *b10.offset(0_isize) * *g.offset(52_isize)
        + *b00.offset(0_isize) * *g.offset(56_isize);
    *g.offset(63_isize) = *g.offset(61_isize) * (zizj + *c0z.offset(1_isize))
        + *b10.offset(1_isize) * *g.offset(53_isize)
        + *b00.offset(1_isize) * *g.offset(57_isize);
    *g.offset(54_isize) = zizj * *g.offset(52_isize)
        + *cpz.offset(0_isize) * *g.offset(56_isize)
        + *b00.offset(0_isize) * *g.offset(48_isize);
    *g.offset(55_isize) = zizj * *g.offset(53_isize)
        + *cpz.offset(1_isize) * *g.offset(57_isize)
        + *b00.offset(1_isize) * *g.offset(49_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1200(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    let xixj: f64 = (*envs).rirj[0_usize];
    let yiyj: f64 = (*envs).rirj[1_usize];
    let zizj: f64 = (*envs).rirj[2_usize];
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(4_isize) = *c0x.offset(0_isize);
    *g.offset(5_isize) = *c0x.offset(1_isize);
    *g.offset(8_isize) =
        *c0x.offset(0_isize) * *c0x.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(9_isize) =
        *c0x.offset(1_isize) * *c0x.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(10_isize) = *g.offset(8_isize) * (xixj + *c0x.offset(0_isize))
        + *c0x.offset(0_isize) * 2_f64 * *b10.offset(0_isize);
    *g.offset(11_isize) = *g.offset(9_isize) * (xixj + *c0x.offset(1_isize))
        + *c0x.offset(1_isize) * 2_f64 * *b10.offset(1_isize);
    *g.offset(6_isize) =
        *c0x.offset(0_isize) * (xixj + *c0x.offset(0_isize)) + *b10.offset(0_isize);
    *g.offset(7_isize) =
        *c0x.offset(1_isize) * (xixj + *c0x.offset(1_isize)) + *b10.offset(1_isize);
    *g.offset(2_isize) = xixj + *c0x.offset(0_isize);
    *g.offset(3_isize) = xixj + *c0x.offset(1_isize);
    *g.offset(16_isize) = 1_f64;
    *g.offset(17_isize) = 1_f64;
    *g.offset(20_isize) = *c0y.offset(0_isize);
    *g.offset(21_isize) = *c0y.offset(1_isize);
    *g.offset(24_isize) =
        *c0y.offset(0_isize) * *c0y.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(25_isize) =
        *c0y.offset(1_isize) * *c0y.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(26_isize) = *g.offset(24_isize) * (yiyj + *c0y.offset(0_isize))
        + *c0y.offset(0_isize) * 2_f64 * *b10.offset(0_isize);
    *g.offset(27_isize) = *g.offset(25_isize) * (yiyj + *c0y.offset(1_isize))
        + *c0y.offset(1_isize) * 2_f64 * *b10.offset(1_isize);
    *g.offset(22_isize) =
        *c0y.offset(0_isize) * (yiyj + *c0y.offset(0_isize)) + *b10.offset(0_isize);
    *g.offset(23_isize) =
        *c0y.offset(1_isize) * (yiyj + *c0y.offset(1_isize)) + *b10.offset(1_isize);
    *g.offset(18_isize) = yiyj + *c0y.offset(0_isize);
    *g.offset(19_isize) = yiyj + *c0y.offset(1_isize);
    *g.offset(36_isize) = *c0z.offset(0_isize) * *g.offset(32_isize);
    *g.offset(37_isize) = *c0z.offset(1_isize) * *g.offset(33_isize);
    *g.offset(40_isize) = *c0z.offset(0_isize) * *g.offset(36_isize)
        + *b10.offset(0_isize) * *g.offset(32_isize);
    *g.offset(41_isize) = *c0z.offset(1_isize) * *g.offset(37_isize)
        + *b10.offset(1_isize) * *g.offset(33_isize);
    *g.offset(42_isize) = *g.offset(40_isize) * (zizj + *c0z.offset(0_isize))
        + 2_f64 * *b10.offset(0_isize) * *g.offset(36_isize);
    *g.offset(43_isize) = *g.offset(41_isize) * (zizj + *c0z.offset(1_isize))
        + 2_f64 * *b10.offset(1_isize) * *g.offset(37_isize);
    *g.offset(38_isize) = *g.offset(36_isize) * (zizj + *c0z.offset(0_isize))
        + *b10.offset(0_isize) * *g.offset(32_isize);
    *g.offset(39_isize) = *g.offset(37_isize) * (zizj + *c0z.offset(1_isize))
        + *b10.offset(1_isize) * *g.offset(33_isize);
    *g.offset(34_isize) = *g.offset(32_isize) * (zizj + *c0z.offset(0_isize));
    *g.offset(35_isize) = *g.offset(33_isize) * (zizj + *c0z.offset(1_isize));
}
#[inline]
unsafe extern "C" fn _g0_2d4d_2000(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *c0x.offset(0_isize);
    *g.offset(3_isize) = *c0x.offset(1_isize);
    *g.offset(4_isize) =
        *c0x.offset(0_isize) * *c0x.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(5_isize) =
        *c0x.offset(1_isize) * *c0x.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(6_isize) = 1_f64;
    *g.offset(7_isize) = 1_f64;
    *g.offset(8_isize) = *c0y.offset(0_isize);
    *g.offset(9_isize) = *c0y.offset(1_isize);
    *g.offset(10_isize) =
        *c0y.offset(0_isize) * *c0y.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(11_isize) =
        *c0y.offset(1_isize) * *c0y.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(14_isize) = *c0z.offset(0_isize) * *g.offset(12_isize);
    *g.offset(15_isize) = *c0z.offset(1_isize) * *g.offset(13_isize);
    *g.offset(16_isize) = *c0z.offset(0_isize) * *g.offset(14_isize)
        + *b10.offset(0_isize) * *g.offset(12_isize);
    *g.offset(17_isize) = *c0z.offset(1_isize) * *g.offset(15_isize)
        + *b10.offset(1_isize) * *g.offset(13_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_2001(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *c0x.offset(0_isize);
    *g.offset(3_isize) = *c0x.offset(1_isize);
    *g.offset(4_isize) =
        *c0x.offset(0_isize) * *c0x.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(5_isize) =
        *c0x.offset(1_isize) * *c0x.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(6_isize) = *cpx.offset(0_isize);
    *g.offset(7_isize) = *cpx.offset(1_isize);
    *g.offset(8_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(9_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(10_isize) = *c0x.offset(0_isize)
        * (*g.offset(8_isize) + *b00.offset(0_isize))
        + *b10.offset(0_isize) * *cpx.offset(0_isize);
    *g.offset(11_isize) = *c0x.offset(1_isize)
        * (*g.offset(9_isize) + *b00.offset(1_isize))
        + *b10.offset(1_isize) * *cpx.offset(1_isize);
    *g.offset(12_isize) = 1_f64;
    *g.offset(13_isize) = 1_f64;
    *g.offset(14_isize) = *c0y.offset(0_isize);
    *g.offset(15_isize) = *c0y.offset(1_isize);
    *g.offset(16_isize) =
        *c0y.offset(0_isize) * *c0y.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(17_isize) =
        *c0y.offset(1_isize) * *c0y.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(18_isize) = *cpy.offset(0_isize);
    *g.offset(19_isize) = *cpy.offset(1_isize);
    *g.offset(20_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(21_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(22_isize) = *c0y.offset(0_isize)
        * (*g.offset(20_isize) + *b00.offset(0_isize))
        + *b10.offset(0_isize) * *cpy.offset(0_isize);
    *g.offset(23_isize) = *c0y.offset(1_isize)
        * (*g.offset(21_isize) + *b00.offset(1_isize))
        + *b10.offset(1_isize) * *cpy.offset(1_isize);
    *g.offset(26_isize) = *c0z.offset(0_isize) * *g.offset(24_isize);
    *g.offset(27_isize) = *c0z.offset(1_isize) * *g.offset(25_isize);
    *g.offset(28_isize) = *c0z.offset(0_isize) * *g.offset(26_isize)
        + *b10.offset(0_isize) * *g.offset(24_isize);
    *g.offset(29_isize) = *c0z.offset(1_isize) * *g.offset(27_isize)
        + *b10.offset(1_isize) * *g.offset(25_isize);
    *g.offset(30_isize) = *cpz.offset(0_isize) * *g.offset(24_isize);
    *g.offset(31_isize) = *cpz.offset(1_isize) * *g.offset(25_isize);
    *g.offset(32_isize) = *cpz.offset(0_isize) * *g.offset(26_isize)
        + *b00.offset(0_isize) * *g.offset(24_isize);
    *g.offset(33_isize) = *cpz.offset(1_isize) * *g.offset(27_isize)
        + *b00.offset(1_isize) * *g.offset(25_isize);
    *g.offset(34_isize) = *c0z.offset(0_isize) * *g.offset(32_isize)
        + *b10.offset(0_isize) * *g.offset(30_isize)
        + *b00.offset(0_isize) * *g.offset(26_isize);
    *g.offset(35_isize) = *c0z.offset(1_isize) * *g.offset(33_isize)
        + *b10.offset(1_isize) * *g.offset(31_isize)
        + *b00.offset(1_isize) * *g.offset(27_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_2010(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *c0x.offset(0_isize);
    *g.offset(3_isize) = *c0x.offset(1_isize);
    *g.offset(4_isize) =
        *c0x.offset(0_isize) * *c0x.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(5_isize) =
        *c0x.offset(1_isize) * *c0x.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(6_isize) = *cpx.offset(0_isize);
    *g.offset(7_isize) = *cpx.offset(1_isize);
    *g.offset(8_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(9_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(10_isize) = *c0x.offset(0_isize)
        * (*g.offset(8_isize) + *b00.offset(0_isize))
        + *b10.offset(0_isize) * *cpx.offset(0_isize);
    *g.offset(11_isize) = *c0x.offset(1_isize)
        * (*g.offset(9_isize) + *b00.offset(1_isize))
        + *b10.offset(1_isize) * *cpx.offset(1_isize);
    *g.offset(12_isize) = 1_f64;
    *g.offset(13_isize) = 1_f64;
    *g.offset(14_isize) = *c0y.offset(0_isize);
    *g.offset(15_isize) = *c0y.offset(1_isize);
    *g.offset(16_isize) =
        *c0y.offset(0_isize) * *c0y.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(17_isize) =
        *c0y.offset(1_isize) * *c0y.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(18_isize) = *cpy.offset(0_isize);
    *g.offset(19_isize) = *cpy.offset(1_isize);
    *g.offset(20_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(21_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(22_isize) = *c0y.offset(0_isize)
        * (*g.offset(20_isize) + *b00.offset(0_isize))
        + *b10.offset(0_isize) * *cpy.offset(0_isize);
    *g.offset(23_isize) = *c0y.offset(1_isize)
        * (*g.offset(21_isize) + *b00.offset(1_isize))
        + *b10.offset(1_isize) * *cpy.offset(1_isize);
    *g.offset(26_isize) = *c0z.offset(0_isize) * *g.offset(24_isize);
    *g.offset(27_isize) = *c0z.offset(1_isize) * *g.offset(25_isize);
    *g.offset(28_isize) = *c0z.offset(0_isize) * *g.offset(26_isize)
        + *b10.offset(0_isize) * *g.offset(24_isize);
    *g.offset(29_isize) = *c0z.offset(1_isize) * *g.offset(27_isize)
        + *b10.offset(1_isize) * *g.offset(25_isize);
    *g.offset(30_isize) = *cpz.offset(0_isize) * *g.offset(24_isize);
    *g.offset(31_isize) = *cpz.offset(1_isize) * *g.offset(25_isize);
    *g.offset(32_isize) = *cpz.offset(0_isize) * *g.offset(26_isize)
        + *b00.offset(0_isize) * *g.offset(24_isize);
    *g.offset(33_isize) = *cpz.offset(1_isize) * *g.offset(27_isize)
        + *b00.offset(1_isize) * *g.offset(25_isize);
    *g.offset(34_isize) = *c0z.offset(0_isize) * *g.offset(32_isize)
        + *b10.offset(0_isize) * *g.offset(30_isize)
        + *b00.offset(0_isize) * *g.offset(26_isize);
    *g.offset(35_isize) = *c0z.offset(1_isize) * *g.offset(33_isize)
        + *b10.offset(1_isize) * *g.offset(31_isize)
        + *b00.offset(1_isize) * *g.offset(27_isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_2100(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    let xixj: f64 = (*envs).rirj[0_usize];
    let yiyj: f64 = (*envs).rirj[1_usize];
    let zizj: f64 = (*envs).rirj[2_usize];
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *c0x.offset(0_isize);
    *g.offset(3_isize) = *c0x.offset(1_isize);
    *g.offset(4_isize) =
        *c0x.offset(0_isize) * *c0x.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(5_isize) =
        *c0x.offset(1_isize) * *c0x.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(12_isize) = *g.offset(4_isize) * (xixj + *c0x.offset(0_isize))
        + *c0x.offset(0_isize) * 2_f64 * *b10.offset(0_isize);
    *g.offset(13_isize) = *g.offset(5_isize) * (xixj + *c0x.offset(1_isize))
        + *c0x.offset(1_isize) * 2_f64 * *b10.offset(1_isize);
    *g.offset(10_isize) =
        *c0x.offset(0_isize) * (xixj + *c0x.offset(0_isize)) + *b10.offset(0_isize);
    *g.offset(11_isize) =
        *c0x.offset(1_isize) * (xixj + *c0x.offset(1_isize)) + *b10.offset(1_isize);
    *g.offset(8_isize) = xixj + *c0x.offset(0_isize);
    *g.offset(9_isize) = xixj + *c0x.offset(1_isize);
    *g.offset(16_isize) = 1_f64;
    *g.offset(17_isize) = 1_f64;
    *g.offset(18_isize) = *c0y.offset(0_isize);
    *g.offset(19_isize) = *c0y.offset(1_isize);
    *g.offset(20_isize) =
        *c0y.offset(0_isize) * *c0y.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(21_isize) =
        *c0y.offset(1_isize) * *c0y.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(28_isize) = *g.offset(20_isize) * (yiyj + *c0y.offset(0_isize))
        + *c0y.offset(0_isize) * 2_f64 * *b10.offset(0_isize);
    *g.offset(29_isize) = *g.offset(21_isize) * (yiyj + *c0y.offset(1_isize))
        + *c0y.offset(1_isize) * 2_f64 * *b10.offset(1_isize);
    *g.offset(26_isize) =
        *c0y.offset(0_isize) * (yiyj + *c0y.offset(0_isize)) + *b10.offset(0_isize);
    *g.offset(27_isize) =
        *c0y.offset(1_isize) * (yiyj + *c0y.offset(1_isize)) + *b10.offset(1_isize);
    *g.offset(24_isize) = yiyj + *c0y.offset(0_isize);
    *g.offset(25_isize) = yiyj + *c0y.offset(1_isize);
    *g.offset(34_isize) = *c0z.offset(0_isize) * *g.offset(32_isize);
    *g.offset(35_isize) = *c0z.offset(1_isize) * *g.offset(33_isize);
    *g.offset(36_isize) = *c0z.offset(0_isize) * *g.offset(34_isize)
        + *b10.offset(0_isize) * *g.offset(32_isize);
    *g.offset(37_isize) = *c0z.offset(1_isize) * *g.offset(35_isize)
        + *b10.offset(1_isize) * *g.offset(33_isize);
    *g.offset(44_isize) = *g.offset(36_isize) * (zizj + *c0z.offset(0_isize))
        + 2_f64 * *b10.offset(0_isize) * *g.offset(34_isize);
    *g.offset(45_isize) = *g.offset(37_isize) * (zizj + *c0z.offset(1_isize))
        + 2_f64 * *b10.offset(1_isize) * *g.offset(35_isize);
    *g.offset(42_isize) = *g.offset(34_isize) * (zizj + *c0z.offset(0_isize))
        + *b10.offset(0_isize) * *g.offset(32_isize);
    *g.offset(43_isize) = *g.offset(35_isize) * (zizj + *c0z.offset(1_isize))
        + *b10.offset(1_isize) * *g.offset(33_isize);
    *g.offset(40_isize) = *g.offset(32_isize) * (zizj + *c0z.offset(0_isize));
    *g.offset(41_isize) = *g.offset(33_isize) * (zizj + *c0z.offset(1_isize));
}
#[inline]
unsafe extern "C" fn _g0_2d4d_3000(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *c0x.offset(0_isize);
    *g.offset(3_isize) = *c0x.offset(1_isize);
    *g.offset(4_isize) =
        *c0x.offset(0_isize) * *c0x.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(5_isize) =
        *c0x.offset(1_isize) * *c0x.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(6_isize) =
        *c0x.offset(0_isize) * (*g.offset(4_isize) + 2_f64 * *b10.offset(0_isize));
    *g.offset(7_isize) =
        *c0x.offset(1_isize) * (*g.offset(5_isize) + 2_f64 * *b10.offset(1_isize));
    *g.offset(8_isize) = 1_f64;
    *g.offset(9_isize) = 1_f64;
    *g.offset(10_isize) = *c0y.offset(0_isize);
    *g.offset(11_isize) = *c0y.offset(1_isize);
    *g.offset(12_isize) =
        *c0y.offset(0_isize) * *c0y.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(13_isize) =
        *c0y.offset(1_isize) * *c0y.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(14_isize) =
        *c0y.offset(0_isize) * (*g.offset(12_isize) + 2_f64 * *b10.offset(0_isize));
    *g.offset(15_isize) =
        *c0y.offset(1_isize) * (*g.offset(13_isize) + 2_f64 * *b10.offset(1_isize));
    *g.offset(18_isize) = *c0z.offset(0_isize) * *g.offset(16_isize);
    *g.offset(19_isize) = *c0z.offset(1_isize) * *g.offset(17_isize);
    *g.offset(20_isize) = *c0z.offset(0_isize) * *g.offset(18_isize)
        + *b10.offset(0_isize) * *g.offset(16_isize);
    *g.offset(21_isize) = *c0z.offset(1_isize) * *g.offset(19_isize)
        + *b10.offset(1_isize) * *g.offset(17_isize);
    *g.offset(22_isize) = *c0z.offset(0_isize) * *g.offset(20_isize)
        + 2_f64 * *b10.offset(0_isize) * *g.offset(18_isize);
    *g.offset(23_isize) = *c0z.offset(1_isize) * *g.offset(21_isize)
        + 2_f64 * *b10.offset(1_isize) * *g.offset(19_isize);
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e_2d4d_unrolled(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    let type_ijkl: i32 = (*envs).li_ceil << 6_i32
        | (*envs).lj_ceil << 4_i32
        | (*envs).lk_ceil << 2_i32
        | (*envs).ll_ceil;
    match type_ijkl {
        0 => {
            _g0_2d4d_0000(g, bc, envs);
            return;
        }
        1 => {
            _g0_2d4d_0001(g, bc, envs);
            return;
        }
        2 => {
            _g0_2d4d_0002(g, bc, envs);
            return;
        }
        3 => {
            _g0_2d4d_0003(g, bc, envs);
            return;
        }
        4 => {
            _g0_2d4d_0010(g, bc, envs);
            return;
        }
        5 => {
            _g0_2d4d_0011(g, bc, envs);
            return;
        }
        6 => {
            _g0_2d4d_0012(g, bc, envs);
            return;
        }
        8 => {
            _g0_2d4d_0020(g, bc, envs);
            return;
        }
        9 => {
            _g0_2d4d_0021(g, bc, envs);
            return;
        }
        12 => {
            _g0_2d4d_0030(g, bc, envs);
            return;
        }
        16 => {
            _g0_2d4d_0100(g, bc, envs);
            return;
        }
        17 => {
            _g0_2d4d_0101(g, bc, envs);
            return;
        }
        18 => {
            _g0_2d4d_0102(g, bc, envs);
            return;
        }
        20 => {
            _g0_2d4d_0110(g, bc, envs);
            return;
        }
        21 => {
            _g0_2d4d_0111(g, bc, envs);
            return;
        }
        24 => {
            _g0_2d4d_0120(g, bc, envs);
            return;
        }
        32 => {
            _g0_2d4d_0200(g, bc, envs);
            return;
        }
        33 => {
            _g0_2d4d_0201(g, bc, envs);
            return;
        }
        36 => {
            _g0_2d4d_0210(g, bc, envs);
            return;
        }
        48 => {
            _g0_2d4d_0300(g, bc, envs);
            return;
        }
        64 => {
            _g0_2d4d_1000(g, bc, envs);
            return;
        }
        65 => {
            _g0_2d4d_1001(g, bc, envs);
            return;
        }
        66 => {
            _g0_2d4d_1002(g, bc, envs);
            return;
        }
        68 => {
            _g0_2d4d_1010(g, bc, envs);
            return;
        }
        69 => {
            _g0_2d4d_1011(g, bc, envs);
            return;
        }
        72 => {
            _g0_2d4d_1020(g, bc, envs);
            return;
        }
        80 => {
            _g0_2d4d_1100(g, bc, envs);
            return;
        }
        81 => {
            _g0_2d4d_1101(g, bc, envs);
            return;
        }
        84 => {
            _g0_2d4d_1110(g, bc, envs);
            return;
        }
        96 => {
            _g0_2d4d_1200(g, bc, envs);
            return;
        }
        128 => {
            _g0_2d4d_2000(g, bc, envs);
            return;
        }
        129 => {
            _g0_2d4d_2001(g, bc, envs);
            return;
        }
        132 => {
            _g0_2d4d_2010(g, bc, envs);
            return;
        }
        144 => {
            _g0_2d4d_2100(g, bc, envs);
            return;
        }
        192 => {
            _g0_2d4d_3000(g, bc, envs);
            return;
        }
        _ => {}
    }
    eprintln!(
        "Dimension error for CINTg0_2e_lj2d4d: iklj = {} {} {} {}",
        (*envs).li_ceil,
        (*envs).lk_ceil,
        (*envs).ll_ceil,
        (*envs).lj_ceil
    );
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0000(
    g: *mut f64,
    _bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0001(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *cpx.offset(0_isize);
    *g.offset(3_isize) = *cpx.offset(1_isize);
    *g.offset(4_isize) = 1_f64;
    *g.offset(5_isize) = 1_f64;
    *g.offset(6_isize) = *cpy.offset(0_isize);
    *g.offset(7_isize) = *cpy.offset(1_isize);
    *g.offset(10_isize) = *cpz.offset(0_isize) * *g.offset(8_isize);
    *g.offset(11_isize) = *cpz.offset(1_isize) * *g.offset(9_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0002(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(4_isize) = *cpx.offset(0_isize);
    *g.offset(5_isize) = *cpx.offset(1_isize);
    *g.offset(6_isize) = *cpx.offset(2_isize);
    *g.offset(7_isize) = *cpx.offset(3_isize);
    *g.offset(8_isize) =
        *cpx.offset(0_isize) * *cpx.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(9_isize) =
        *cpx.offset(1_isize) * *cpx.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(10_isize) =
        *cpx.offset(2_isize) * *cpx.offset(2_isize) + *b01.offset(2_isize);
    *g.offset(11_isize) =
        *cpx.offset(3_isize) * *cpx.offset(3_isize) + *b01.offset(3_isize);
    *g.offset(12_isize) = 1_f64;
    *g.offset(13_isize) = 1_f64;
    *g.offset(14_isize) = 1_f64;
    *g.offset(15_isize) = 1_f64;
    *g.offset(16_isize) = *cpy.offset(0_isize);
    *g.offset(17_isize) = *cpy.offset(1_isize);
    *g.offset(18_isize) = *cpy.offset(2_isize);
    *g.offset(19_isize) = *cpy.offset(3_isize);
    *g.offset(20_isize) =
        *cpy.offset(0_isize) * *cpy.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(21_isize) =
        *cpy.offset(1_isize) * *cpy.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(22_isize) =
        *cpy.offset(2_isize) * *cpy.offset(2_isize) + *b01.offset(2_isize);
    *g.offset(23_isize) =
        *cpy.offset(3_isize) * *cpy.offset(3_isize) + *b01.offset(3_isize);
    *g.offset(28_isize) = *cpz.offset(0_isize) * *g.offset(24_isize);
    *g.offset(29_isize) = *cpz.offset(1_isize) * *g.offset(25_isize);
    *g.offset(30_isize) = *cpz.offset(2_isize) * *g.offset(26_isize);
    *g.offset(31_isize) = *cpz.offset(3_isize) * *g.offset(27_isize);
    *g.offset(32_isize) = *cpz.offset(0_isize) * *g.offset(28_isize)
        + *b01.offset(0_isize) * *g.offset(24_isize);
    *g.offset(33_isize) = *cpz.offset(1_isize) * *g.offset(29_isize)
        + *b01.offset(1_isize) * *g.offset(25_isize);
    *g.offset(34_isize) = *cpz.offset(2_isize) * *g.offset(30_isize)
        + *b01.offset(2_isize) * *g.offset(26_isize);
    *g.offset(35_isize) = *cpz.offset(3_isize) * *g.offset(31_isize)
        + *b01.offset(3_isize) * *g.offset(27_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0003(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(4_isize) = *cpx.offset(0_isize);
    *g.offset(5_isize) = *cpx.offset(1_isize);
    *g.offset(6_isize) = *cpx.offset(2_isize);
    *g.offset(7_isize) = *cpx.offset(3_isize);
    *g.offset(8_isize) =
        *cpx.offset(0_isize) * *cpx.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(9_isize) =
        *cpx.offset(1_isize) * *cpx.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(10_isize) =
        *cpx.offset(2_isize) * *cpx.offset(2_isize) + *b01.offset(2_isize);
    *g.offset(11_isize) =
        *cpx.offset(3_isize) * *cpx.offset(3_isize) + *b01.offset(3_isize);
    *g.offset(12_isize) =
        *cpx.offset(0_isize) * (*g.offset(8_isize) + 2_f64 * *b01.offset(0_isize));
    *g.offset(13_isize) =
        *cpx.offset(1_isize) * (*g.offset(9_isize) + 2_f64 * *b01.offset(1_isize));
    *g.offset(14_isize) =
        *cpx.offset(2_isize) * (*g.offset(10_isize) + 2_f64 * *b01.offset(2_isize));
    *g.offset(15_isize) =
        *cpx.offset(3_isize) * (*g.offset(11_isize) + 2_f64 * *b01.offset(3_isize));
    *g.offset(16_isize) = 1_f64;
    *g.offset(17_isize) = 1_f64;
    *g.offset(18_isize) = 1_f64;
    *g.offset(19_isize) = 1_f64;
    *g.offset(20_isize) = *cpy.offset(0_isize);
    *g.offset(21_isize) = *cpy.offset(1_isize);
    *g.offset(22_isize) = *cpy.offset(2_isize);
    *g.offset(23_isize) = *cpy.offset(3_isize);
    *g.offset(24_isize) =
        *cpy.offset(0_isize) * *cpy.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(25_isize) =
        *cpy.offset(1_isize) * *cpy.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(26_isize) =
        *cpy.offset(2_isize) * *cpy.offset(2_isize) + *b01.offset(2_isize);
    *g.offset(27_isize) =
        *cpy.offset(3_isize) * *cpy.offset(3_isize) + *b01.offset(3_isize);
    *g.offset(28_isize) =
        *cpy.offset(0_isize) * (*g.offset(24_isize) + 2_f64 * *b01.offset(0_isize));
    *g.offset(29_isize) =
        *cpy.offset(1_isize) * (*g.offset(25_isize) + 2_f64 * *b01.offset(1_isize));
    *g.offset(30_isize) =
        *cpy.offset(2_isize) * (*g.offset(26_isize) + 2_f64 * *b01.offset(2_isize));
    *g.offset(31_isize) =
        *cpy.offset(3_isize) * (*g.offset(27_isize) + 2_f64 * *b01.offset(3_isize));
    *g.offset(36_isize) = *cpz.offset(0_isize) * *g.offset(32_isize);
    *g.offset(37_isize) = *cpz.offset(1_isize) * *g.offset(33_isize);
    *g.offset(38_isize) = *cpz.offset(2_isize) * *g.offset(34_isize);
    *g.offset(39_isize) = *cpz.offset(3_isize) * *g.offset(35_isize);
    *g.offset(40_isize) = *cpz.offset(0_isize) * *g.offset(36_isize)
        + *b01.offset(0_isize) * *g.offset(32_isize);
    *g.offset(41_isize) = *cpz.offset(1_isize) * *g.offset(37_isize)
        + *b01.offset(1_isize) * *g.offset(33_isize);
    *g.offset(42_isize) = *cpz.offset(2_isize) * *g.offset(38_isize)
        + *b01.offset(2_isize) * *g.offset(34_isize);
    *g.offset(43_isize) = *cpz.offset(3_isize) * *g.offset(39_isize)
        + *b01.offset(3_isize) * *g.offset(35_isize);
    *g.offset(44_isize) = *cpz.offset(0_isize) * *g.offset(40_isize)
        + 2_f64 * *b01.offset(0_isize) * *g.offset(36_isize);
    *g.offset(45_isize) = *cpz.offset(1_isize) * *g.offset(41_isize)
        + 2_f64 * *b01.offset(1_isize) * *g.offset(37_isize);
    *g.offset(46_isize) = *cpz.offset(2_isize) * *g.offset(42_isize)
        + 2_f64 * *b01.offset(2_isize) * *g.offset(38_isize);
    *g.offset(47_isize) = *cpz.offset(3_isize) * *g.offset(43_isize)
        + 2_f64 * *b01.offset(3_isize) * *g.offset(39_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0010(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *cpx.offset(0_isize);
    *g.offset(3_isize) = *cpx.offset(1_isize);
    *g.offset(4_isize) = 1_f64;
    *g.offset(5_isize) = 1_f64;
    *g.offset(6_isize) = *cpy.offset(0_isize);
    *g.offset(7_isize) = *cpy.offset(1_isize);
    *g.offset(10_isize) = *cpz.offset(0_isize) * *g.offset(8_isize);
    *g.offset(11_isize) = *cpz.offset(1_isize) * *g.offset(9_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0011(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    let xkxl: f64 = (*envs).rkrl[0_usize];
    let ykyl: f64 = (*envs).rkrl[1_usize];
    let zkzl: f64 = (*envs).rkrl[2_usize];
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(8_isize) = *cpx.offset(0_isize);
    *g.offset(9_isize) = *cpx.offset(1_isize);
    *g.offset(10_isize) = *cpx.offset(2_isize);
    *g.offset(11_isize) = *cpx.offset(3_isize);
    *g.offset(12_isize) =
        *cpx.offset(0_isize) * (xkxl + *cpx.offset(0_isize)) + *b01.offset(0_isize);
    *g.offset(13_isize) =
        *cpx.offset(1_isize) * (xkxl + *cpx.offset(1_isize)) + *b01.offset(1_isize);
    *g.offset(14_isize) =
        *cpx.offset(2_isize) * (xkxl + *cpx.offset(2_isize)) + *b01.offset(2_isize);
    *g.offset(15_isize) =
        *cpx.offset(3_isize) * (xkxl + *cpx.offset(3_isize)) + *b01.offset(3_isize);
    *g.offset(4_isize) = xkxl + *cpx.offset(0_isize);
    *g.offset(5_isize) = xkxl + *cpx.offset(1_isize);
    *g.offset(6_isize) = xkxl + *cpx.offset(2_isize);
    *g.offset(7_isize) = xkxl + *cpx.offset(3_isize);
    *g.offset(24_isize) = 1_f64;
    *g.offset(25_isize) = 1_f64;
    *g.offset(26_isize) = 1_f64;
    *g.offset(27_isize) = 1_f64;
    *g.offset(32_isize) = *cpy.offset(0_isize);
    *g.offset(33_isize) = *cpy.offset(1_isize);
    *g.offset(34_isize) = *cpy.offset(2_isize);
    *g.offset(35_isize) = *cpy.offset(3_isize);
    *g.offset(36_isize) =
        *cpy.offset(0_isize) * (ykyl + *cpy.offset(0_isize)) + *b01.offset(0_isize);
    *g.offset(37_isize) =
        *cpy.offset(1_isize) * (ykyl + *cpy.offset(1_isize)) + *b01.offset(1_isize);
    *g.offset(38_isize) =
        *cpy.offset(2_isize) * (ykyl + *cpy.offset(2_isize)) + *b01.offset(2_isize);
    *g.offset(39_isize) =
        *cpy.offset(3_isize) * (ykyl + *cpy.offset(3_isize)) + *b01.offset(3_isize);
    *g.offset(28_isize) = ykyl + *cpy.offset(0_isize);
    *g.offset(29_isize) = ykyl + *cpy.offset(1_isize);
    *g.offset(30_isize) = ykyl + *cpy.offset(2_isize);
    *g.offset(31_isize) = ykyl + *cpy.offset(3_isize);
    *g.offset(56_isize) = *cpz.offset(0_isize) * *g.offset(48_isize);
    *g.offset(57_isize) = *cpz.offset(1_isize) * *g.offset(49_isize);
    *g.offset(58_isize) = *cpz.offset(2_isize) * *g.offset(50_isize);
    *g.offset(59_isize) = *cpz.offset(3_isize) * *g.offset(51_isize);
    *g.offset(60_isize) = *g.offset(56_isize) * (zkzl + *cpz.offset(0_isize))
        + *b01.offset(0_isize) * *g.offset(48_isize);
    *g.offset(61_isize) = *g.offset(57_isize) * (zkzl + *cpz.offset(1_isize))
        + *b01.offset(1_isize) * *g.offset(49_isize);
    *g.offset(62_isize) = *g.offset(58_isize) * (zkzl + *cpz.offset(2_isize))
        + *b01.offset(2_isize) * *g.offset(50_isize);
    *g.offset(63_isize) = *g.offset(59_isize) * (zkzl + *cpz.offset(3_isize))
        + *b01.offset(3_isize) * *g.offset(51_isize);
    *g.offset(52_isize) = *g.offset(48_isize) * (zkzl + *cpz.offset(0_isize));
    *g.offset(53_isize) = *g.offset(49_isize) * (zkzl + *cpz.offset(1_isize));
    *g.offset(54_isize) = *g.offset(50_isize) * (zkzl + *cpz.offset(2_isize));
    *g.offset(55_isize) = *g.offset(51_isize) * (zkzl + *cpz.offset(3_isize));
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0012(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    let xkxl: f64 = (*envs).rkrl[0_usize];
    let ykyl: f64 = (*envs).rkrl[1_usize];
    let zkzl: f64 = (*envs).rkrl[2_usize];
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(8_isize) = *cpx.offset(0_isize);
    *g.offset(9_isize) = *cpx.offset(1_isize);
    *g.offset(10_isize) = *cpx.offset(2_isize);
    *g.offset(11_isize) = *cpx.offset(3_isize);
    *g.offset(16_isize) =
        *cpx.offset(0_isize) * *cpx.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(17_isize) =
        *cpx.offset(1_isize) * *cpx.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(18_isize) =
        *cpx.offset(2_isize) * *cpx.offset(2_isize) + *b01.offset(2_isize);
    *g.offset(19_isize) =
        *cpx.offset(3_isize) * *cpx.offset(3_isize) + *b01.offset(3_isize);
    *g.offset(20_isize) = *g.offset(16_isize) * (xkxl + *cpx.offset(0_isize))
        + *cpx.offset(0_isize) * 2_f64 * *b01.offset(0_isize);
    *g.offset(21_isize) = *g.offset(17_isize) * (xkxl + *cpx.offset(1_isize))
        + *cpx.offset(1_isize) * 2_f64 * *b01.offset(1_isize);
    *g.offset(22_isize) = *g.offset(18_isize) * (xkxl + *cpx.offset(2_isize))
        + *cpx.offset(2_isize) * 2_f64 * *b01.offset(2_isize);
    *g.offset(23_isize) = *g.offset(19_isize) * (xkxl + *cpx.offset(3_isize))
        + *cpx.offset(3_isize) * 2_f64 * *b01.offset(3_isize);
    *g.offset(12_isize) =
        *cpx.offset(0_isize) * (xkxl + *cpx.offset(0_isize)) + *b01.offset(0_isize);
    *g.offset(13_isize) =
        *cpx.offset(1_isize) * (xkxl + *cpx.offset(1_isize)) + *b01.offset(1_isize);
    *g.offset(14_isize) =
        *cpx.offset(2_isize) * (xkxl + *cpx.offset(2_isize)) + *b01.offset(2_isize);
    *g.offset(15_isize) =
        *cpx.offset(3_isize) * (xkxl + *cpx.offset(3_isize)) + *b01.offset(3_isize);
    *g.offset(4_isize) = xkxl + *cpx.offset(0_isize);
    *g.offset(5_isize) = xkxl + *cpx.offset(1_isize);
    *g.offset(6_isize) = xkxl + *cpx.offset(2_isize);
    *g.offset(7_isize) = xkxl + *cpx.offset(3_isize);
    *g.offset(32_isize) = 1_f64;
    *g.offset(33_isize) = 1_f64;
    *g.offset(34_isize) = 1_f64;
    *g.offset(35_isize) = 1_f64;
    *g.offset(40_isize) = *cpy.offset(0_isize);
    *g.offset(41_isize) = *cpy.offset(1_isize);
    *g.offset(42_isize) = *cpy.offset(2_isize);
    *g.offset(43_isize) = *cpy.offset(3_isize);
    *g.offset(48_isize) =
        *cpy.offset(0_isize) * *cpy.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(49_isize) =
        *cpy.offset(1_isize) * *cpy.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(50_isize) =
        *cpy.offset(2_isize) * *cpy.offset(2_isize) + *b01.offset(2_isize);
    *g.offset(51_isize) =
        *cpy.offset(3_isize) * *cpy.offset(3_isize) + *b01.offset(3_isize);
    *g.offset(52_isize) = *g.offset(48_isize) * (ykyl + *cpy.offset(0_isize))
        + *cpy.offset(0_isize) * 2_f64 * *b01.offset(0_isize);
    *g.offset(53_isize) = *g.offset(49_isize) * (ykyl + *cpy.offset(1_isize))
        + *cpy.offset(1_isize) * 2_f64 * *b01.offset(1_isize);
    *g.offset(54_isize) = *g.offset(50_isize) * (ykyl + *cpy.offset(2_isize))
        + *cpy.offset(2_isize) * 2_f64 * *b01.offset(2_isize);
    *g.offset(55_isize) = *g.offset(51_isize) * (ykyl + *cpy.offset(3_isize))
        + *cpy.offset(3_isize) * 2_f64 * *b01.offset(3_isize);
    *g.offset(44_isize) =
        *cpy.offset(0_isize) * (ykyl + *cpy.offset(0_isize)) + *b01.offset(0_isize);
    *g.offset(45_isize) =
        *cpy.offset(1_isize) * (ykyl + *cpy.offset(1_isize)) + *b01.offset(1_isize);
    *g.offset(46_isize) =
        *cpy.offset(2_isize) * (ykyl + *cpy.offset(2_isize)) + *b01.offset(2_isize);
    *g.offset(47_isize) =
        *cpy.offset(3_isize) * (ykyl + *cpy.offset(3_isize)) + *b01.offset(3_isize);
    *g.offset(36_isize) = ykyl + *cpy.offset(0_isize);
    *g.offset(37_isize) = ykyl + *cpy.offset(1_isize);
    *g.offset(38_isize) = ykyl + *cpy.offset(2_isize);
    *g.offset(39_isize) = ykyl + *cpy.offset(3_isize);
    *g.offset(72_isize) = *cpz.offset(0_isize) * *g.offset(64_isize);
    *g.offset(73_isize) = *cpz.offset(1_isize) * *g.offset(65_isize);
    *g.offset(74_isize) = *cpz.offset(2_isize) * *g.offset(66_isize);
    *g.offset(75_isize) = *cpz.offset(3_isize) * *g.offset(67_isize);
    *g.offset(80_isize) = *cpz.offset(0_isize) * *g.offset(72_isize)
        + *b01.offset(0_isize) * *g.offset(64_isize);
    *g.offset(81_isize) = *cpz.offset(1_isize) * *g.offset(73_isize)
        + *b01.offset(1_isize) * *g.offset(65_isize);
    *g.offset(82_isize) = *cpz.offset(2_isize) * *g.offset(74_isize)
        + *b01.offset(2_isize) * *g.offset(66_isize);
    *g.offset(83_isize) = *cpz.offset(3_isize) * *g.offset(75_isize)
        + *b01.offset(3_isize) * *g.offset(67_isize);
    *g.offset(84_isize) = *g.offset(80_isize) * (zkzl + *cpz.offset(0_isize))
        + 2_f64 * *b01.offset(0_isize) * *g.offset(72_isize);
    *g.offset(85_isize) = *g.offset(81_isize) * (zkzl + *cpz.offset(1_isize))
        + 2_f64 * *b01.offset(1_isize) * *g.offset(73_isize);
    *g.offset(86_isize) = *g.offset(82_isize) * (zkzl + *cpz.offset(2_isize))
        + 2_f64 * *b01.offset(2_isize) * *g.offset(74_isize);
    *g.offset(87_isize) = *g.offset(83_isize) * (zkzl + *cpz.offset(3_isize))
        + 2_f64 * *b01.offset(3_isize) * *g.offset(75_isize);
    *g.offset(76_isize) = *g.offset(72_isize) * (zkzl + *cpz.offset(0_isize))
        + *b01.offset(0_isize) * *g.offset(64_isize);
    *g.offset(77_isize) = *g.offset(73_isize) * (zkzl + *cpz.offset(1_isize))
        + *b01.offset(1_isize) * *g.offset(65_isize);
    *g.offset(78_isize) = *g.offset(74_isize) * (zkzl + *cpz.offset(2_isize))
        + *b01.offset(2_isize) * *g.offset(66_isize);
    *g.offset(79_isize) = *g.offset(75_isize) * (zkzl + *cpz.offset(3_isize))
        + *b01.offset(3_isize) * *g.offset(67_isize);
    *g.offset(68_isize) = *g.offset(64_isize) * (zkzl + *cpz.offset(0_isize));
    *g.offset(69_isize) = *g.offset(65_isize) * (zkzl + *cpz.offset(1_isize));
    *g.offset(70_isize) = *g.offset(66_isize) * (zkzl + *cpz.offset(2_isize));
    *g.offset(71_isize) = *g.offset(67_isize) * (zkzl + *cpz.offset(3_isize));
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0020(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(4_isize) = *cpx.offset(0_isize);
    *g.offset(5_isize) = *cpx.offset(1_isize);
    *g.offset(6_isize) = *cpx.offset(2_isize);
    *g.offset(7_isize) = *cpx.offset(3_isize);
    *g.offset(8_isize) =
        *cpx.offset(0_isize) * *cpx.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(9_isize) =
        *cpx.offset(1_isize) * *cpx.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(10_isize) =
        *cpx.offset(2_isize) * *cpx.offset(2_isize) + *b01.offset(2_isize);
    *g.offset(11_isize) =
        *cpx.offset(3_isize) * *cpx.offset(3_isize) + *b01.offset(3_isize);
    *g.offset(12_isize) = 1_f64;
    *g.offset(13_isize) = 1_f64;
    *g.offset(14_isize) = 1_f64;
    *g.offset(15_isize) = 1_f64;
    *g.offset(16_isize) = *cpy.offset(0_isize);
    *g.offset(17_isize) = *cpy.offset(1_isize);
    *g.offset(18_isize) = *cpy.offset(2_isize);
    *g.offset(19_isize) = *cpy.offset(3_isize);
    *g.offset(20_isize) =
        *cpy.offset(0_isize) * *cpy.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(21_isize) =
        *cpy.offset(1_isize) * *cpy.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(22_isize) =
        *cpy.offset(2_isize) * *cpy.offset(2_isize) + *b01.offset(2_isize);
    *g.offset(23_isize) =
        *cpy.offset(3_isize) * *cpy.offset(3_isize) + *b01.offset(3_isize);
    *g.offset(28_isize) = *cpz.offset(0_isize) * *g.offset(24_isize);
    *g.offset(29_isize) = *cpz.offset(1_isize) * *g.offset(25_isize);
    *g.offset(30_isize) = *cpz.offset(2_isize) * *g.offset(26_isize);
    *g.offset(31_isize) = *cpz.offset(3_isize) * *g.offset(27_isize);
    *g.offset(32_isize) = *cpz.offset(0_isize) * *g.offset(28_isize)
        + *b01.offset(0_isize) * *g.offset(24_isize);
    *g.offset(33_isize) = *cpz.offset(1_isize) * *g.offset(29_isize)
        + *b01.offset(1_isize) * *g.offset(25_isize);
    *g.offset(34_isize) = *cpz.offset(2_isize) * *g.offset(30_isize)
        + *b01.offset(2_isize) * *g.offset(26_isize);
    *g.offset(35_isize) = *cpz.offset(3_isize) * *g.offset(31_isize)
        + *b01.offset(3_isize) * *g.offset(27_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0021(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    let xkxl: f64 = (*envs).rkrl[0_usize];
    let ykyl: f64 = (*envs).rkrl[1_usize];
    let zkzl: f64 = (*envs).rkrl[2_usize];
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(4_isize) = *cpx.offset(0_isize);
    *g.offset(5_isize) = *cpx.offset(1_isize);
    *g.offset(6_isize) = *cpx.offset(2_isize);
    *g.offset(7_isize) = *cpx.offset(3_isize);
    *g.offset(8_isize) =
        *cpx.offset(0_isize) * *cpx.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(9_isize) =
        *cpx.offset(1_isize) * *cpx.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(10_isize) =
        *cpx.offset(2_isize) * *cpx.offset(2_isize) + *b01.offset(2_isize);
    *g.offset(11_isize) =
        *cpx.offset(3_isize) * *cpx.offset(3_isize) + *b01.offset(3_isize);
    *g.offset(16_isize) = xkxl + *cpx.offset(0_isize);
    *g.offset(17_isize) = xkxl + *cpx.offset(1_isize);
    *g.offset(18_isize) = xkxl + *cpx.offset(2_isize);
    *g.offset(19_isize) = xkxl + *cpx.offset(3_isize);
    *g.offset(20_isize) =
        *cpx.offset(0_isize) * (xkxl + *cpx.offset(0_isize)) + *b01.offset(0_isize);
    *g.offset(21_isize) =
        *cpx.offset(1_isize) * (xkxl + *cpx.offset(1_isize)) + *b01.offset(1_isize);
    *g.offset(22_isize) =
        *cpx.offset(2_isize) * (xkxl + *cpx.offset(2_isize)) + *b01.offset(2_isize);
    *g.offset(23_isize) =
        *cpx.offset(3_isize) * (xkxl + *cpx.offset(3_isize)) + *b01.offset(3_isize);
    *g.offset(24_isize) = *g.offset(8_isize) * (xkxl + *cpx.offset(0_isize))
        + *cpx.offset(0_isize) * 2_f64 * *b01.offset(0_isize);
    *g.offset(25_isize) = *g.offset(9_isize) * (xkxl + *cpx.offset(1_isize))
        + *cpx.offset(1_isize) * 2_f64 * *b01.offset(1_isize);
    *g.offset(26_isize) = *g.offset(10_isize) * (xkxl + *cpx.offset(2_isize))
        + *cpx.offset(2_isize) * 2_f64 * *b01.offset(2_isize);
    *g.offset(27_isize) = *g.offset(11_isize) * (xkxl + *cpx.offset(3_isize))
        + *cpx.offset(3_isize) * 2_f64 * *b01.offset(3_isize);
    *g.offset(32_isize) = 1_f64;
    *g.offset(33_isize) = 1_f64;
    *g.offset(34_isize) = 1_f64;
    *g.offset(35_isize) = 1_f64;
    *g.offset(36_isize) = *cpy.offset(0_isize);
    *g.offset(37_isize) = *cpy.offset(1_isize);
    *g.offset(38_isize) = *cpy.offset(2_isize);
    *g.offset(39_isize) = *cpy.offset(3_isize);
    *g.offset(40_isize) =
        *cpy.offset(0_isize) * *cpy.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(41_isize) =
        *cpy.offset(1_isize) * *cpy.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(42_isize) =
        *cpy.offset(2_isize) * *cpy.offset(2_isize) + *b01.offset(2_isize);
    *g.offset(43_isize) =
        *cpy.offset(3_isize) * *cpy.offset(3_isize) + *b01.offset(3_isize);
    *g.offset(48_isize) = ykyl + *cpy.offset(0_isize);
    *g.offset(49_isize) = ykyl + *cpy.offset(1_isize);
    *g.offset(50_isize) = ykyl + *cpy.offset(2_isize);
    *g.offset(51_isize) = ykyl + *cpy.offset(3_isize);
    *g.offset(52_isize) =
        *cpy.offset(0_isize) * (ykyl + *cpy.offset(0_isize)) + *b01.offset(0_isize);
    *g.offset(53_isize) =
        *cpy.offset(1_isize) * (ykyl + *cpy.offset(1_isize)) + *b01.offset(1_isize);
    *g.offset(54_isize) =
        *cpy.offset(2_isize) * (ykyl + *cpy.offset(2_isize)) + *b01.offset(2_isize);
    *g.offset(55_isize) =
        *cpy.offset(3_isize) * (ykyl + *cpy.offset(3_isize)) + *b01.offset(3_isize);
    *g.offset(56_isize) = *g.offset(40_isize) * (ykyl + *cpy.offset(0_isize))
        + *cpy.offset(0_isize) * 2_f64 * *b01.offset(0_isize);
    *g.offset(57_isize) = *g.offset(41_isize) * (ykyl + *cpy.offset(1_isize))
        + *cpy.offset(1_isize) * 2_f64 * *b01.offset(1_isize);
    *g.offset(58_isize) = *g.offset(42_isize) * (ykyl + *cpy.offset(2_isize))
        + *cpy.offset(2_isize) * 2_f64 * *b01.offset(2_isize);
    *g.offset(59_isize) = *g.offset(43_isize) * (ykyl + *cpy.offset(3_isize))
        + *cpy.offset(3_isize) * 2_f64 * *b01.offset(3_isize);
    *g.offset(68_isize) = *cpz.offset(0_isize) * *g.offset(64_isize);
    *g.offset(69_isize) = *cpz.offset(1_isize) * *g.offset(65_isize);
    *g.offset(70_isize) = *cpz.offset(2_isize) * *g.offset(66_isize);
    *g.offset(71_isize) = *cpz.offset(3_isize) * *g.offset(67_isize);
    *g.offset(72_isize) = *cpz.offset(0_isize) * *g.offset(68_isize)
        + *b01.offset(0_isize) * *g.offset(64_isize);
    *g.offset(73_isize) = *cpz.offset(1_isize) * *g.offset(69_isize)
        + *b01.offset(1_isize) * *g.offset(65_isize);
    *g.offset(74_isize) = *cpz.offset(2_isize) * *g.offset(70_isize)
        + *b01.offset(2_isize) * *g.offset(66_isize);
    *g.offset(75_isize) = *cpz.offset(3_isize) * *g.offset(71_isize)
        + *b01.offset(3_isize) * *g.offset(67_isize);
    *g.offset(80_isize) = *g.offset(64_isize) * (zkzl + *cpz.offset(0_isize));
    *g.offset(81_isize) = *g.offset(65_isize) * (zkzl + *cpz.offset(1_isize));
    *g.offset(82_isize) = *g.offset(66_isize) * (zkzl + *cpz.offset(2_isize));
    *g.offset(83_isize) = *g.offset(67_isize) * (zkzl + *cpz.offset(3_isize));
    *g.offset(84_isize) = *g.offset(68_isize) * (zkzl + *cpz.offset(0_isize))
        + *b01.offset(0_isize) * *g.offset(64_isize);
    *g.offset(85_isize) = *g.offset(69_isize) * (zkzl + *cpz.offset(1_isize))
        + *b01.offset(1_isize) * *g.offset(65_isize);
    *g.offset(86_isize) = *g.offset(70_isize) * (zkzl + *cpz.offset(2_isize))
        + *b01.offset(2_isize) * *g.offset(66_isize);
    *g.offset(87_isize) = *g.offset(71_isize) * (zkzl + *cpz.offset(3_isize))
        + *b01.offset(3_isize) * *g.offset(67_isize);
    *g.offset(88_isize) = *g.offset(72_isize) * (zkzl + *cpz.offset(0_isize))
        + 2_f64 * *b01.offset(0_isize) * *g.offset(68_isize);
    *g.offset(89_isize) = *g.offset(73_isize) * (zkzl + *cpz.offset(1_isize))
        + 2_f64 * *b01.offset(1_isize) * *g.offset(69_isize);
    *g.offset(90_isize) = *g.offset(74_isize) * (zkzl + *cpz.offset(2_isize))
        + 2_f64 * *b01.offset(2_isize) * *g.offset(70_isize);
    *g.offset(91_isize) = *g.offset(75_isize) * (zkzl + *cpz.offset(3_isize))
        + 2_f64 * *b01.offset(3_isize) * *g.offset(71_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0030(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(4_isize) = *cpx.offset(0_isize);
    *g.offset(5_isize) = *cpx.offset(1_isize);
    *g.offset(6_isize) = *cpx.offset(2_isize);
    *g.offset(7_isize) = *cpx.offset(3_isize);
    *g.offset(8_isize) =
        *cpx.offset(0_isize) * *cpx.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(9_isize) =
        *cpx.offset(1_isize) * *cpx.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(10_isize) =
        *cpx.offset(2_isize) * *cpx.offset(2_isize) + *b01.offset(2_isize);
    *g.offset(11_isize) =
        *cpx.offset(3_isize) * *cpx.offset(3_isize) + *b01.offset(3_isize);
    *g.offset(12_isize) =
        *cpx.offset(0_isize) * (*g.offset(8_isize) + 2_f64 * *b01.offset(0_isize));
    *g.offset(13_isize) =
        *cpx.offset(1_isize) * (*g.offset(9_isize) + 2_f64 * *b01.offset(1_isize));
    *g.offset(14_isize) =
        *cpx.offset(2_isize) * (*g.offset(10_isize) + 2_f64 * *b01.offset(2_isize));
    *g.offset(15_isize) =
        *cpx.offset(3_isize) * (*g.offset(11_isize) + 2_f64 * *b01.offset(3_isize));
    *g.offset(16_isize) = 1_f64;
    *g.offset(17_isize) = 1_f64;
    *g.offset(18_isize) = 1_f64;
    *g.offset(19_isize) = 1_f64;
    *g.offset(20_isize) = *cpy.offset(0_isize);
    *g.offset(21_isize) = *cpy.offset(1_isize);
    *g.offset(22_isize) = *cpy.offset(2_isize);
    *g.offset(23_isize) = *cpy.offset(3_isize);
    *g.offset(24_isize) =
        *cpy.offset(0_isize) * *cpy.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(25_isize) =
        *cpy.offset(1_isize) * *cpy.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(26_isize) =
        *cpy.offset(2_isize) * *cpy.offset(2_isize) + *b01.offset(2_isize);
    *g.offset(27_isize) =
        *cpy.offset(3_isize) * *cpy.offset(3_isize) + *b01.offset(3_isize);
    *g.offset(28_isize) =
        *cpy.offset(0_isize) * (*g.offset(24_isize) + 2_f64 * *b01.offset(0_isize));
    *g.offset(29_isize) =
        *cpy.offset(1_isize) * (*g.offset(25_isize) + 2_f64 * *b01.offset(1_isize));
    *g.offset(30_isize) =
        *cpy.offset(2_isize) * (*g.offset(26_isize) + 2_f64 * *b01.offset(2_isize));
    *g.offset(31_isize) =
        *cpy.offset(3_isize) * (*g.offset(27_isize) + 2_f64 * *b01.offset(3_isize));
    *g.offset(36_isize) = *cpz.offset(0_isize) * *g.offset(32_isize);
    *g.offset(37_isize) = *cpz.offset(1_isize) * *g.offset(33_isize);
    *g.offset(38_isize) = *cpz.offset(2_isize) * *g.offset(34_isize);
    *g.offset(39_isize) = *cpz.offset(3_isize) * *g.offset(35_isize);
    *g.offset(40_isize) = *cpz.offset(0_isize) * *g.offset(36_isize)
        + *b01.offset(0_isize) * *g.offset(32_isize);
    *g.offset(41_isize) = *cpz.offset(1_isize) * *g.offset(37_isize)
        + *b01.offset(1_isize) * *g.offset(33_isize);
    *g.offset(42_isize) = *cpz.offset(2_isize) * *g.offset(38_isize)
        + *b01.offset(2_isize) * *g.offset(34_isize);
    *g.offset(43_isize) = *cpz.offset(3_isize) * *g.offset(39_isize)
        + *b01.offset(3_isize) * *g.offset(35_isize);
    *g.offset(44_isize) = *cpz.offset(0_isize) * *g.offset(40_isize)
        + 2_f64 * *b01.offset(0_isize) * *g.offset(36_isize);
    *g.offset(45_isize) = *cpz.offset(1_isize) * *g.offset(41_isize)
        + 2_f64 * *b01.offset(1_isize) * *g.offset(37_isize);
    *g.offset(46_isize) = *cpz.offset(2_isize) * *g.offset(42_isize)
        + 2_f64 * *b01.offset(2_isize) * *g.offset(38_isize);
    *g.offset(47_isize) = *cpz.offset(3_isize) * *g.offset(43_isize)
        + 2_f64 * *b01.offset(3_isize) * *g.offset(39_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0100(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *c0x.offset(0_isize);
    *g.offset(3_isize) = *c0x.offset(1_isize);
    *g.offset(4_isize) = 1_f64;
    *g.offset(5_isize) = 1_f64;
    *g.offset(6_isize) = *c0y.offset(0_isize);
    *g.offset(7_isize) = *c0y.offset(1_isize);
    *g.offset(10_isize) = *c0z.offset(0_isize) * *g.offset(8_isize);
    *g.offset(11_isize) = *c0z.offset(1_isize) * *g.offset(9_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0101(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(4_isize) = *cpx.offset(0_isize);
    *g.offset(5_isize) = *cpx.offset(1_isize);
    *g.offset(6_isize) = *cpx.offset(2_isize);
    *g.offset(7_isize) = *cpx.offset(3_isize);
    *g.offset(8_isize) = *c0x.offset(0_isize);
    *g.offset(9_isize) = *c0x.offset(1_isize);
    *g.offset(10_isize) = *c0x.offset(2_isize);
    *g.offset(11_isize) = *c0x.offset(3_isize);
    *g.offset(12_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(13_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(14_isize) =
        *cpx.offset(2_isize) * *c0x.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(15_isize) =
        *cpx.offset(3_isize) * *c0x.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(16_isize) = 1_f64;
    *g.offset(17_isize) = 1_f64;
    *g.offset(18_isize) = 1_f64;
    *g.offset(19_isize) = 1_f64;
    *g.offset(20_isize) = *cpy.offset(0_isize);
    *g.offset(21_isize) = *cpy.offset(1_isize);
    *g.offset(22_isize) = *cpy.offset(2_isize);
    *g.offset(23_isize) = *cpy.offset(3_isize);
    *g.offset(24_isize) = *c0y.offset(0_isize);
    *g.offset(25_isize) = *c0y.offset(1_isize);
    *g.offset(26_isize) = *c0y.offset(2_isize);
    *g.offset(27_isize) = *c0y.offset(3_isize);
    *g.offset(28_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(29_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(30_isize) =
        *cpy.offset(2_isize) * *c0y.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(31_isize) =
        *cpy.offset(3_isize) * *c0y.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(36_isize) = *cpz.offset(0_isize) * *g.offset(32_isize);
    *g.offset(37_isize) = *cpz.offset(1_isize) * *g.offset(33_isize);
    *g.offset(38_isize) = *cpz.offset(2_isize) * *g.offset(34_isize);
    *g.offset(39_isize) = *cpz.offset(3_isize) * *g.offset(35_isize);
    *g.offset(40_isize) = *c0z.offset(0_isize) * *g.offset(32_isize);
    *g.offset(41_isize) = *c0z.offset(1_isize) * *g.offset(33_isize);
    *g.offset(42_isize) = *c0z.offset(2_isize) * *g.offset(34_isize);
    *g.offset(43_isize) = *c0z.offset(3_isize) * *g.offset(35_isize);
    *g.offset(44_isize) = *cpz.offset(0_isize) * *g.offset(40_isize)
        + *b00.offset(0_isize) * *g.offset(32_isize);
    *g.offset(45_isize) = *cpz.offset(1_isize) * *g.offset(41_isize)
        + *b00.offset(1_isize) * *g.offset(33_isize);
    *g.offset(46_isize) = *cpz.offset(2_isize) * *g.offset(42_isize)
        + *b00.offset(2_isize) * *g.offset(34_isize);
    *g.offset(47_isize) = *cpz.offset(3_isize) * *g.offset(43_isize)
        + *b00.offset(3_isize) * *g.offset(35_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0102(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(4_isize) = *cpx.offset(0_isize);
    *g.offset(5_isize) = *cpx.offset(1_isize);
    *g.offset(6_isize) = *cpx.offset(2_isize);
    *g.offset(7_isize) = *cpx.offset(3_isize);
    *g.offset(12_isize) = *c0x.offset(0_isize);
    *g.offset(13_isize) = *c0x.offset(1_isize);
    *g.offset(14_isize) = *c0x.offset(2_isize);
    *g.offset(15_isize) = *c0x.offset(3_isize);
    *g.offset(8_isize) =
        *cpx.offset(0_isize) * *cpx.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(9_isize) =
        *cpx.offset(1_isize) * *cpx.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(10_isize) =
        *cpx.offset(2_isize) * *cpx.offset(2_isize) + *b01.offset(2_isize);
    *g.offset(11_isize) =
        *cpx.offset(3_isize) * *cpx.offset(3_isize) + *b01.offset(3_isize);
    *g.offset(16_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(17_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(18_isize) =
        *cpx.offset(2_isize) * *c0x.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(19_isize) =
        *cpx.offset(3_isize) * *c0x.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(20_isize) = *cpx.offset(0_isize)
        * (*g.offset(16_isize) + *b00.offset(0_isize))
        + *b01.offset(0_isize) * *c0x.offset(0_isize);
    *g.offset(21_isize) = *cpx.offset(1_isize)
        * (*g.offset(17_isize) + *b00.offset(1_isize))
        + *b01.offset(1_isize) * *c0x.offset(1_isize);
    *g.offset(22_isize) = *cpx.offset(2_isize)
        * (*g.offset(18_isize) + *b00.offset(2_isize))
        + *b01.offset(2_isize) * *c0x.offset(2_isize);
    *g.offset(23_isize) = *cpx.offset(3_isize)
        * (*g.offset(19_isize) + *b00.offset(3_isize))
        + *b01.offset(3_isize) * *c0x.offset(3_isize);
    *g.offset(24_isize) = 1_f64;
    *g.offset(25_isize) = 1_f64;
    *g.offset(26_isize) = 1_f64;
    *g.offset(27_isize) = 1_f64;
    *g.offset(28_isize) = *cpy.offset(0_isize);
    *g.offset(29_isize) = *cpy.offset(1_isize);
    *g.offset(30_isize) = *cpy.offset(2_isize);
    *g.offset(31_isize) = *cpy.offset(3_isize);
    *g.offset(36_isize) = *c0y.offset(0_isize);
    *g.offset(37_isize) = *c0y.offset(1_isize);
    *g.offset(38_isize) = *c0y.offset(2_isize);
    *g.offset(39_isize) = *c0y.offset(3_isize);
    *g.offset(32_isize) =
        *cpy.offset(0_isize) * *cpy.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(33_isize) =
        *cpy.offset(1_isize) * *cpy.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(34_isize) =
        *cpy.offset(2_isize) * *cpy.offset(2_isize) + *b01.offset(2_isize);
    *g.offset(35_isize) =
        *cpy.offset(3_isize) * *cpy.offset(3_isize) + *b01.offset(3_isize);
    *g.offset(40_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(41_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(42_isize) =
        *cpy.offset(2_isize) * *c0y.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(43_isize) =
        *cpy.offset(3_isize) * *c0y.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(44_isize) = *cpy.offset(0_isize)
        * (*g.offset(40_isize) + *b00.offset(0_isize))
        + *b01.offset(0_isize) * *c0y.offset(0_isize);
    *g.offset(45_isize) = *cpy.offset(1_isize)
        * (*g.offset(41_isize) + *b00.offset(1_isize))
        + *b01.offset(1_isize) * *c0y.offset(1_isize);
    *g.offset(46_isize) = *cpy.offset(2_isize)
        * (*g.offset(42_isize) + *b00.offset(2_isize))
        + *b01.offset(2_isize) * *c0y.offset(2_isize);
    *g.offset(47_isize) = *cpy.offset(3_isize)
        * (*g.offset(43_isize) + *b00.offset(3_isize))
        + *b01.offset(3_isize) * *c0y.offset(3_isize);
    *g.offset(52_isize) = *cpz.offset(0_isize) * *g.offset(48_isize);
    *g.offset(53_isize) = *cpz.offset(1_isize) * *g.offset(49_isize);
    *g.offset(54_isize) = *cpz.offset(2_isize) * *g.offset(50_isize);
    *g.offset(55_isize) = *cpz.offset(3_isize) * *g.offset(51_isize);
    *g.offset(60_isize) = *c0z.offset(0_isize) * *g.offset(48_isize);
    *g.offset(61_isize) = *c0z.offset(1_isize) * *g.offset(49_isize);
    *g.offset(62_isize) = *c0z.offset(2_isize) * *g.offset(50_isize);
    *g.offset(63_isize) = *c0z.offset(3_isize) * *g.offset(51_isize);
    *g.offset(56_isize) = *cpz.offset(0_isize) * *g.offset(52_isize)
        + *b01.offset(0_isize) * *g.offset(48_isize);
    *g.offset(57_isize) = *cpz.offset(1_isize) * *g.offset(53_isize)
        + *b01.offset(1_isize) * *g.offset(49_isize);
    *g.offset(58_isize) = *cpz.offset(2_isize) * *g.offset(54_isize)
        + *b01.offset(2_isize) * *g.offset(50_isize);
    *g.offset(59_isize) = *cpz.offset(3_isize) * *g.offset(55_isize)
        + *b01.offset(3_isize) * *g.offset(51_isize);
    *g.offset(64_isize) = *cpz.offset(0_isize) * *g.offset(60_isize)
        + *b00.offset(0_isize) * *g.offset(48_isize);
    *g.offset(65_isize) = *cpz.offset(1_isize) * *g.offset(61_isize)
        + *b00.offset(1_isize) * *g.offset(49_isize);
    *g.offset(66_isize) = *cpz.offset(2_isize) * *g.offset(62_isize)
        + *b00.offset(2_isize) * *g.offset(50_isize);
    *g.offset(67_isize) = *cpz.offset(3_isize) * *g.offset(63_isize)
        + *b00.offset(3_isize) * *g.offset(51_isize);
    *g.offset(68_isize) = *cpz.offset(0_isize) * *g.offset(64_isize)
        + *b01.offset(0_isize) * *g.offset(60_isize)
        + *b00.offset(0_isize) * *g.offset(52_isize);
    *g.offset(69_isize) = *cpz.offset(1_isize) * *g.offset(65_isize)
        + *b01.offset(1_isize) * *g.offset(61_isize)
        + *b00.offset(1_isize) * *g.offset(53_isize);
    *g.offset(70_isize) = *cpz.offset(2_isize) * *g.offset(66_isize)
        + *b01.offset(2_isize) * *g.offset(62_isize)
        + *b00.offset(2_isize) * *g.offset(54_isize);
    *g.offset(71_isize) = *cpz.offset(3_isize) * *g.offset(67_isize)
        + *b01.offset(3_isize) * *g.offset(63_isize)
        + *b00.offset(3_isize) * *g.offset(55_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0110(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(4_isize) = *cpx.offset(0_isize);
    *g.offset(5_isize) = *cpx.offset(1_isize);
    *g.offset(6_isize) = *cpx.offset(2_isize);
    *g.offset(7_isize) = *cpx.offset(3_isize);
    *g.offset(8_isize) = *c0x.offset(0_isize);
    *g.offset(9_isize) = *c0x.offset(1_isize);
    *g.offset(10_isize) = *c0x.offset(2_isize);
    *g.offset(11_isize) = *c0x.offset(3_isize);
    *g.offset(12_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(13_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(14_isize) =
        *cpx.offset(2_isize) * *c0x.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(15_isize) =
        *cpx.offset(3_isize) * *c0x.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(16_isize) = 1_f64;
    *g.offset(17_isize) = 1_f64;
    *g.offset(18_isize) = 1_f64;
    *g.offset(19_isize) = 1_f64;
    *g.offset(20_isize) = *cpy.offset(0_isize);
    *g.offset(21_isize) = *cpy.offset(1_isize);
    *g.offset(22_isize) = *cpy.offset(2_isize);
    *g.offset(23_isize) = *cpy.offset(3_isize);
    *g.offset(24_isize) = *c0y.offset(0_isize);
    *g.offset(25_isize) = *c0y.offset(1_isize);
    *g.offset(26_isize) = *c0y.offset(2_isize);
    *g.offset(27_isize) = *c0y.offset(3_isize);
    *g.offset(28_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(29_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(30_isize) =
        *cpy.offset(2_isize) * *c0y.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(31_isize) =
        *cpy.offset(3_isize) * *c0y.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(36_isize) = *cpz.offset(0_isize) * *g.offset(32_isize);
    *g.offset(37_isize) = *cpz.offset(1_isize) * *g.offset(33_isize);
    *g.offset(38_isize) = *cpz.offset(2_isize) * *g.offset(34_isize);
    *g.offset(39_isize) = *cpz.offset(3_isize) * *g.offset(35_isize);
    *g.offset(40_isize) = *c0z.offset(0_isize) * *g.offset(32_isize);
    *g.offset(41_isize) = *c0z.offset(1_isize) * *g.offset(33_isize);
    *g.offset(42_isize) = *c0z.offset(2_isize) * *g.offset(34_isize);
    *g.offset(43_isize) = *c0z.offset(3_isize) * *g.offset(35_isize);
    *g.offset(44_isize) = *cpz.offset(0_isize) * *g.offset(40_isize)
        + *b00.offset(0_isize) * *g.offset(32_isize);
    *g.offset(45_isize) = *cpz.offset(1_isize) * *g.offset(41_isize)
        + *b00.offset(1_isize) * *g.offset(33_isize);
    *g.offset(46_isize) = *cpz.offset(2_isize) * *g.offset(42_isize)
        + *b00.offset(2_isize) * *g.offset(34_isize);
    *g.offset(47_isize) = *cpz.offset(3_isize) * *g.offset(43_isize)
        + *b00.offset(3_isize) * *g.offset(35_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0111(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    let xkxl: f64 = (*envs).rkrl[0_usize];
    let ykyl: f64 = (*envs).rkrl[1_usize];
    let zkzl: f64 = (*envs).rkrl[2_usize];
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(24_isize) = *c0x.offset(0_isize);
    *g.offset(25_isize) = *c0x.offset(1_isize);
    *g.offset(26_isize) = *c0x.offset(2_isize);
    *g.offset(27_isize) = *c0x.offset(3_isize);
    *g.offset(8_isize) = *cpx.offset(0_isize);
    *g.offset(9_isize) = *cpx.offset(1_isize);
    *g.offset(10_isize) = *cpx.offset(2_isize);
    *g.offset(11_isize) = *cpx.offset(3_isize);
    *g.offset(32_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(33_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(34_isize) =
        *cpx.offset(2_isize) * *c0x.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(35_isize) =
        *cpx.offset(3_isize) * *c0x.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(12_isize) =
        *cpx.offset(0_isize) * (xkxl + *cpx.offset(0_isize)) + *b01.offset(0_isize);
    *g.offset(13_isize) =
        *cpx.offset(1_isize) * (xkxl + *cpx.offset(1_isize)) + *b01.offset(1_isize);
    *g.offset(14_isize) =
        *cpx.offset(2_isize) * (xkxl + *cpx.offset(2_isize)) + *b01.offset(2_isize);
    *g.offset(15_isize) =
        *cpx.offset(3_isize) * (xkxl + *cpx.offset(3_isize)) + *b01.offset(3_isize);
    *g.offset(36_isize) = *g.offset(32_isize) * (xkxl + *cpx.offset(0_isize))
        + *cpx.offset(0_isize) * *b00.offset(0_isize)
        + *b01.offset(0_isize) * *c0x.offset(0_isize);
    *g.offset(37_isize) = *g.offset(33_isize) * (xkxl + *cpx.offset(1_isize))
        + *cpx.offset(1_isize) * *b00.offset(1_isize)
        + *b01.offset(1_isize) * *c0x.offset(1_isize);
    *g.offset(38_isize) = *g.offset(34_isize) * (xkxl + *cpx.offset(2_isize))
        + *cpx.offset(2_isize) * *b00.offset(2_isize)
        + *b01.offset(2_isize) * *c0x.offset(2_isize);
    *g.offset(39_isize) = *g.offset(35_isize) * (xkxl + *cpx.offset(3_isize))
        + *cpx.offset(3_isize) * *b00.offset(3_isize)
        + *b01.offset(3_isize) * *c0x.offset(3_isize);
    *g.offset(4_isize) = xkxl + *cpx.offset(0_isize);
    *g.offset(5_isize) = xkxl + *cpx.offset(1_isize);
    *g.offset(6_isize) = xkxl + *cpx.offset(2_isize);
    *g.offset(7_isize) = xkxl + *cpx.offset(3_isize);
    *g.offset(28_isize) =
        *c0x.offset(0_isize) * (xkxl + *cpx.offset(0_isize)) + *b00.offset(0_isize);
    *g.offset(29_isize) =
        *c0x.offset(1_isize) * (xkxl + *cpx.offset(1_isize)) + *b00.offset(1_isize);
    *g.offset(30_isize) =
        *c0x.offset(2_isize) * (xkxl + *cpx.offset(2_isize)) + *b00.offset(2_isize);
    *g.offset(31_isize) =
        *c0x.offset(3_isize) * (xkxl + *cpx.offset(3_isize)) + *b00.offset(3_isize);
    *g.offset(48_isize) = 1_f64;
    *g.offset(49_isize) = 1_f64;
    *g.offset(50_isize) = 1_f64;
    *g.offset(51_isize) = 1_f64;
    *g.offset(72_isize) = *c0y.offset(0_isize);
    *g.offset(73_isize) = *c0y.offset(1_isize);
    *g.offset(74_isize) = *c0y.offset(2_isize);
    *g.offset(75_isize) = *c0y.offset(3_isize);
    *g.offset(56_isize) = *cpy.offset(0_isize);
    *g.offset(57_isize) = *cpy.offset(1_isize);
    *g.offset(58_isize) = *cpy.offset(2_isize);
    *g.offset(59_isize) = *cpy.offset(3_isize);
    *g.offset(80_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(81_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(82_isize) =
        *cpy.offset(2_isize) * *c0y.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(83_isize) =
        *cpy.offset(3_isize) * *c0y.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(60_isize) =
        *cpy.offset(0_isize) * (ykyl + *cpy.offset(0_isize)) + *b01.offset(0_isize);
    *g.offset(61_isize) =
        *cpy.offset(1_isize) * (ykyl + *cpy.offset(1_isize)) + *b01.offset(1_isize);
    *g.offset(62_isize) =
        *cpy.offset(2_isize) * (ykyl + *cpy.offset(2_isize)) + *b01.offset(2_isize);
    *g.offset(63_isize) =
        *cpy.offset(3_isize) * (ykyl + *cpy.offset(3_isize)) + *b01.offset(3_isize);
    *g.offset(84_isize) = *g.offset(80_isize) * (ykyl + *cpy.offset(0_isize))
        + *cpy.offset(0_isize) * *b00.offset(0_isize)
        + *b01.offset(0_isize) * *c0y.offset(0_isize);
    *g.offset(85_isize) = *g.offset(81_isize) * (ykyl + *cpy.offset(1_isize))
        + *cpy.offset(1_isize) * *b00.offset(1_isize)
        + *b01.offset(1_isize) * *c0y.offset(1_isize);
    *g.offset(86_isize) = *g.offset(82_isize) * (ykyl + *cpy.offset(2_isize))
        + *cpy.offset(2_isize) * *b00.offset(2_isize)
        + *b01.offset(2_isize) * *c0y.offset(2_isize);
    *g.offset(87_isize) = *g.offset(83_isize) * (ykyl + *cpy.offset(3_isize))
        + *cpy.offset(3_isize) * *b00.offset(3_isize)
        + *b01.offset(3_isize) * *c0y.offset(3_isize);
    *g.offset(52_isize) = ykyl + *cpy.offset(0_isize);
    *g.offset(53_isize) = ykyl + *cpy.offset(1_isize);
    *g.offset(54_isize) = ykyl + *cpy.offset(2_isize);
    *g.offset(55_isize) = ykyl + *cpy.offset(3_isize);
    *g.offset(76_isize) =
        *c0y.offset(0_isize) * (ykyl + *cpy.offset(0_isize)) + *b00.offset(0_isize);
    *g.offset(77_isize) =
        *c0y.offset(1_isize) * (ykyl + *cpy.offset(1_isize)) + *b00.offset(1_isize);
    *g.offset(78_isize) =
        *c0y.offset(2_isize) * (ykyl + *cpy.offset(2_isize)) + *b00.offset(2_isize);
    *g.offset(79_isize) =
        *c0y.offset(3_isize) * (ykyl + *cpy.offset(3_isize)) + *b00.offset(3_isize);
    *g.offset(120_isize) = *c0z.offset(0_isize) * *g.offset(96_isize);
    *g.offset(121_isize) = *c0z.offset(1_isize) * *g.offset(97_isize);
    *g.offset(122_isize) = *c0z.offset(2_isize) * *g.offset(98_isize);
    *g.offset(123_isize) = *c0z.offset(3_isize) * *g.offset(99_isize);
    *g.offset(104_isize) = *cpz.offset(0_isize) * *g.offset(96_isize);
    *g.offset(105_isize) = *cpz.offset(1_isize) * *g.offset(97_isize);
    *g.offset(106_isize) = *cpz.offset(2_isize) * *g.offset(98_isize);
    *g.offset(107_isize) = *cpz.offset(3_isize) * *g.offset(99_isize);
    *g.offset(128_isize) = *cpz.offset(0_isize) * *g.offset(120_isize)
        + *b00.offset(0_isize) * *g.offset(96_isize);
    *g.offset(129_isize) = *cpz.offset(1_isize) * *g.offset(121_isize)
        + *b00.offset(1_isize) * *g.offset(97_isize);
    *g.offset(130_isize) = *cpz.offset(2_isize) * *g.offset(122_isize)
        + *b00.offset(2_isize) * *g.offset(98_isize);
    *g.offset(131_isize) = *cpz.offset(3_isize) * *g.offset(123_isize)
        + *b00.offset(3_isize) * *g.offset(99_isize);
    *g.offset(108_isize) = *g.offset(104_isize) * (zkzl + *cpz.offset(0_isize))
        + *b01.offset(0_isize) * *g.offset(96_isize);
    *g.offset(109_isize) = *g.offset(105_isize) * (zkzl + *cpz.offset(1_isize))
        + *b01.offset(1_isize) * *g.offset(97_isize);
    *g.offset(110_isize) = *g.offset(106_isize) * (zkzl + *cpz.offset(2_isize))
        + *b01.offset(2_isize) * *g.offset(98_isize);
    *g.offset(111_isize) = *g.offset(107_isize) * (zkzl + *cpz.offset(3_isize))
        + *b01.offset(3_isize) * *g.offset(99_isize);
    *g.offset(132_isize) = *g.offset(128_isize) * (zkzl + *cpz.offset(0_isize))
        + *b01.offset(0_isize) * *g.offset(120_isize)
        + *b00.offset(0_isize) * *g.offset(104_isize);
    *g.offset(133_isize) = *g.offset(129_isize) * (zkzl + *cpz.offset(1_isize))
        + *b01.offset(1_isize) * *g.offset(121_isize)
        + *b00.offset(1_isize) * *g.offset(105_isize);
    *g.offset(134_isize) = *g.offset(130_isize) * (zkzl + *cpz.offset(2_isize))
        + *b01.offset(2_isize) * *g.offset(122_isize)
        + *b00.offset(2_isize) * *g.offset(106_isize);
    *g.offset(135_isize) = *g.offset(131_isize) * (zkzl + *cpz.offset(3_isize))
        + *b01.offset(3_isize) * *g.offset(123_isize)
        + *b00.offset(3_isize) * *g.offset(107_isize);
    *g.offset(100_isize) = *g.offset(96_isize) * (zkzl + *cpz.offset(0_isize));
    *g.offset(101_isize) = *g.offset(97_isize) * (zkzl + *cpz.offset(1_isize));
    *g.offset(102_isize) = *g.offset(98_isize) * (zkzl + *cpz.offset(2_isize));
    *g.offset(103_isize) = *g.offset(99_isize) * (zkzl + *cpz.offset(3_isize));
    *g.offset(124_isize) = *g.offset(120_isize) * (zkzl + *cpz.offset(0_isize))
        + *b00.offset(0_isize) * *g.offset(96_isize);
    *g.offset(125_isize) = *g.offset(121_isize) * (zkzl + *cpz.offset(1_isize))
        + *b00.offset(1_isize) * *g.offset(97_isize);
    *g.offset(126_isize) = *g.offset(122_isize) * (zkzl + *cpz.offset(2_isize))
        + *b00.offset(2_isize) * *g.offset(98_isize);
    *g.offset(127_isize) = *g.offset(123_isize) * (zkzl + *cpz.offset(3_isize))
        + *b00.offset(3_isize) * *g.offset(99_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0120(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(4_isize) = *cpx.offset(0_isize);
    *g.offset(5_isize) = *cpx.offset(1_isize);
    *g.offset(6_isize) = *cpx.offset(2_isize);
    *g.offset(7_isize) = *cpx.offset(3_isize);
    *g.offset(12_isize) = *c0x.offset(0_isize);
    *g.offset(13_isize) = *c0x.offset(1_isize);
    *g.offset(14_isize) = *c0x.offset(2_isize);
    *g.offset(15_isize) = *c0x.offset(3_isize);
    *g.offset(8_isize) =
        *cpx.offset(0_isize) * *cpx.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(9_isize) =
        *cpx.offset(1_isize) * *cpx.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(10_isize) =
        *cpx.offset(2_isize) * *cpx.offset(2_isize) + *b01.offset(2_isize);
    *g.offset(11_isize) =
        *cpx.offset(3_isize) * *cpx.offset(3_isize) + *b01.offset(3_isize);
    *g.offset(16_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(17_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(18_isize) =
        *cpx.offset(2_isize) * *c0x.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(19_isize) =
        *cpx.offset(3_isize) * *c0x.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(20_isize) = *cpx.offset(0_isize)
        * (*g.offset(16_isize) + *b00.offset(0_isize))
        + *b01.offset(0_isize) * *c0x.offset(0_isize);
    *g.offset(21_isize) = *cpx.offset(1_isize)
        * (*g.offset(17_isize) + *b00.offset(1_isize))
        + *b01.offset(1_isize) * *c0x.offset(1_isize);
    *g.offset(22_isize) = *cpx.offset(2_isize)
        * (*g.offset(18_isize) + *b00.offset(2_isize))
        + *b01.offset(2_isize) * *c0x.offset(2_isize);
    *g.offset(23_isize) = *cpx.offset(3_isize)
        * (*g.offset(19_isize) + *b00.offset(3_isize))
        + *b01.offset(3_isize) * *c0x.offset(3_isize);
    *g.offset(24_isize) = 1_f64;
    *g.offset(25_isize) = 1_f64;
    *g.offset(26_isize) = 1_f64;
    *g.offset(27_isize) = 1_f64;
    *g.offset(28_isize) = *cpy.offset(0_isize);
    *g.offset(29_isize) = *cpy.offset(1_isize);
    *g.offset(30_isize) = *cpy.offset(2_isize);
    *g.offset(31_isize) = *cpy.offset(3_isize);
    *g.offset(36_isize) = *c0y.offset(0_isize);
    *g.offset(37_isize) = *c0y.offset(1_isize);
    *g.offset(38_isize) = *c0y.offset(2_isize);
    *g.offset(39_isize) = *c0y.offset(3_isize);
    *g.offset(32_isize) =
        *cpy.offset(0_isize) * *cpy.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(33_isize) =
        *cpy.offset(1_isize) * *cpy.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(34_isize) =
        *cpy.offset(2_isize) * *cpy.offset(2_isize) + *b01.offset(2_isize);
    *g.offset(35_isize) =
        *cpy.offset(3_isize) * *cpy.offset(3_isize) + *b01.offset(3_isize);
    *g.offset(40_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(41_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(42_isize) =
        *cpy.offset(2_isize) * *c0y.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(43_isize) =
        *cpy.offset(3_isize) * *c0y.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(44_isize) = *cpy.offset(0_isize)
        * (*g.offset(40_isize) + *b00.offset(0_isize))
        + *b01.offset(0_isize) * *c0y.offset(0_isize);
    *g.offset(45_isize) = *cpy.offset(1_isize)
        * (*g.offset(41_isize) + *b00.offset(1_isize))
        + *b01.offset(1_isize) * *c0y.offset(1_isize);
    *g.offset(46_isize) = *cpy.offset(2_isize)
        * (*g.offset(42_isize) + *b00.offset(2_isize))
        + *b01.offset(2_isize) * *c0y.offset(2_isize);
    *g.offset(47_isize) = *cpy.offset(3_isize)
        * (*g.offset(43_isize) + *b00.offset(3_isize))
        + *b01.offset(3_isize) * *c0y.offset(3_isize);
    *g.offset(52_isize) = *cpz.offset(0_isize) * *g.offset(48_isize);
    *g.offset(53_isize) = *cpz.offset(1_isize) * *g.offset(49_isize);
    *g.offset(54_isize) = *cpz.offset(2_isize) * *g.offset(50_isize);
    *g.offset(55_isize) = *cpz.offset(3_isize) * *g.offset(51_isize);
    *g.offset(60_isize) = *c0z.offset(0_isize) * *g.offset(48_isize);
    *g.offset(61_isize) = *c0z.offset(1_isize) * *g.offset(49_isize);
    *g.offset(62_isize) = *c0z.offset(2_isize) * *g.offset(50_isize);
    *g.offset(63_isize) = *c0z.offset(3_isize) * *g.offset(51_isize);
    *g.offset(56_isize) = *cpz.offset(0_isize) * *g.offset(52_isize)
        + *b01.offset(0_isize) * *g.offset(48_isize);
    *g.offset(57_isize) = *cpz.offset(1_isize) * *g.offset(53_isize)
        + *b01.offset(1_isize) * *g.offset(49_isize);
    *g.offset(58_isize) = *cpz.offset(2_isize) * *g.offset(54_isize)
        + *b01.offset(2_isize) * *g.offset(50_isize);
    *g.offset(59_isize) = *cpz.offset(3_isize) * *g.offset(55_isize)
        + *b01.offset(3_isize) * *g.offset(51_isize);
    *g.offset(64_isize) = *cpz.offset(0_isize) * *g.offset(60_isize)
        + *b00.offset(0_isize) * *g.offset(48_isize);
    *g.offset(65_isize) = *cpz.offset(1_isize) * *g.offset(61_isize)
        + *b00.offset(1_isize) * *g.offset(49_isize);
    *g.offset(66_isize) = *cpz.offset(2_isize) * *g.offset(62_isize)
        + *b00.offset(2_isize) * *g.offset(50_isize);
    *g.offset(67_isize) = *cpz.offset(3_isize) * *g.offset(63_isize)
        + *b00.offset(3_isize) * *g.offset(51_isize);
    *g.offset(68_isize) = *cpz.offset(0_isize) * *g.offset(64_isize)
        + *b01.offset(0_isize) * *g.offset(60_isize)
        + *b00.offset(0_isize) * *g.offset(52_isize);
    *g.offset(69_isize) = *cpz.offset(1_isize) * *g.offset(65_isize)
        + *b01.offset(1_isize) * *g.offset(61_isize)
        + *b00.offset(1_isize) * *g.offset(53_isize);
    *g.offset(70_isize) = *cpz.offset(2_isize) * *g.offset(66_isize)
        + *b01.offset(2_isize) * *g.offset(62_isize)
        + *b00.offset(2_isize) * *g.offset(54_isize);
    *g.offset(71_isize) = *cpz.offset(3_isize) * *g.offset(67_isize)
        + *b01.offset(3_isize) * *g.offset(63_isize)
        + *b00.offset(3_isize) * *g.offset(55_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0200(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(4_isize) = *c0x.offset(0_isize);
    *g.offset(5_isize) = *c0x.offset(1_isize);
    *g.offset(6_isize) = *c0x.offset(2_isize);
    *g.offset(7_isize) = *c0x.offset(3_isize);
    *g.offset(8_isize) =
        *c0x.offset(0_isize) * *c0x.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(9_isize) =
        *c0x.offset(1_isize) * *c0x.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(10_isize) =
        *c0x.offset(2_isize) * *c0x.offset(2_isize) + *b10.offset(2_isize);
    *g.offset(11_isize) =
        *c0x.offset(3_isize) * *c0x.offset(3_isize) + *b10.offset(3_isize);
    *g.offset(12_isize) = 1_f64;
    *g.offset(13_isize) = 1_f64;
    *g.offset(14_isize) = 1_f64;
    *g.offset(15_isize) = 1_f64;
    *g.offset(16_isize) = *c0y.offset(0_isize);
    *g.offset(17_isize) = *c0y.offset(1_isize);
    *g.offset(18_isize) = *c0y.offset(2_isize);
    *g.offset(19_isize) = *c0y.offset(3_isize);
    *g.offset(20_isize) =
        *c0y.offset(0_isize) * *c0y.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(21_isize) =
        *c0y.offset(1_isize) * *c0y.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(22_isize) =
        *c0y.offset(2_isize) * *c0y.offset(2_isize) + *b10.offset(2_isize);
    *g.offset(23_isize) =
        *c0y.offset(3_isize) * *c0y.offset(3_isize) + *b10.offset(3_isize);
    *g.offset(28_isize) = *c0z.offset(0_isize) * *g.offset(24_isize);
    *g.offset(29_isize) = *c0z.offset(1_isize) * *g.offset(25_isize);
    *g.offset(30_isize) = *c0z.offset(2_isize) * *g.offset(26_isize);
    *g.offset(31_isize) = *c0z.offset(3_isize) * *g.offset(27_isize);
    *g.offset(32_isize) = *c0z.offset(0_isize) * *g.offset(28_isize)
        + *b10.offset(0_isize) * *g.offset(24_isize);
    *g.offset(33_isize) = *c0z.offset(1_isize) * *g.offset(29_isize)
        + *b10.offset(1_isize) * *g.offset(25_isize);
    *g.offset(34_isize) = *c0z.offset(2_isize) * *g.offset(30_isize)
        + *b10.offset(2_isize) * *g.offset(26_isize);
    *g.offset(35_isize) = *c0z.offset(3_isize) * *g.offset(31_isize)
        + *b10.offset(3_isize) * *g.offset(27_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0201(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(8_isize) = *c0x.offset(0_isize);
    *g.offset(9_isize) = *c0x.offset(1_isize);
    *g.offset(10_isize) = *c0x.offset(2_isize);
    *g.offset(11_isize) = *c0x.offset(3_isize);
    *g.offset(16_isize) =
        *c0x.offset(0_isize) * *c0x.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(17_isize) =
        *c0x.offset(1_isize) * *c0x.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(18_isize) =
        *c0x.offset(2_isize) * *c0x.offset(2_isize) + *b10.offset(2_isize);
    *g.offset(19_isize) =
        *c0x.offset(3_isize) * *c0x.offset(3_isize) + *b10.offset(3_isize);
    *g.offset(4_isize) = *cpx.offset(0_isize);
    *g.offset(5_isize) = *cpx.offset(1_isize);
    *g.offset(6_isize) = *cpx.offset(2_isize);
    *g.offset(7_isize) = *cpx.offset(3_isize);
    *g.offset(12_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(13_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(14_isize) =
        *cpx.offset(2_isize) * *c0x.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(15_isize) =
        *cpx.offset(3_isize) * *c0x.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(20_isize) = *c0x.offset(0_isize)
        * (*g.offset(12_isize) + *b00.offset(0_isize))
        + *b10.offset(0_isize) * *cpx.offset(0_isize);
    *g.offset(21_isize) = *c0x.offset(1_isize)
        * (*g.offset(13_isize) + *b00.offset(1_isize))
        + *b10.offset(1_isize) * *cpx.offset(1_isize);
    *g.offset(22_isize) = *c0x.offset(2_isize)
        * (*g.offset(14_isize) + *b00.offset(2_isize))
        + *b10.offset(2_isize) * *cpx.offset(2_isize);
    *g.offset(23_isize) = *c0x.offset(3_isize)
        * (*g.offset(15_isize) + *b00.offset(3_isize))
        + *b10.offset(3_isize) * *cpx.offset(3_isize);
    *g.offset(24_isize) = 1_f64;
    *g.offset(25_isize) = 1_f64;
    *g.offset(26_isize) = 1_f64;
    *g.offset(27_isize) = 1_f64;
    *g.offset(32_isize) = *c0y.offset(0_isize);
    *g.offset(33_isize) = *c0y.offset(1_isize);
    *g.offset(34_isize) = *c0y.offset(2_isize);
    *g.offset(35_isize) = *c0y.offset(3_isize);
    *g.offset(40_isize) =
        *c0y.offset(0_isize) * *c0y.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(41_isize) =
        *c0y.offset(1_isize) * *c0y.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(42_isize) =
        *c0y.offset(2_isize) * *c0y.offset(2_isize) + *b10.offset(2_isize);
    *g.offset(43_isize) =
        *c0y.offset(3_isize) * *c0y.offset(3_isize) + *b10.offset(3_isize);
    *g.offset(28_isize) = *cpy.offset(0_isize);
    *g.offset(29_isize) = *cpy.offset(1_isize);
    *g.offset(30_isize) = *cpy.offset(2_isize);
    *g.offset(31_isize) = *cpy.offset(3_isize);
    *g.offset(36_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(37_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(38_isize) =
        *cpy.offset(2_isize) * *c0y.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(39_isize) =
        *cpy.offset(3_isize) * *c0y.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(44_isize) = *c0y.offset(0_isize)
        * (*g.offset(36_isize) + *b00.offset(0_isize))
        + *b10.offset(0_isize) * *cpy.offset(0_isize);
    *g.offset(45_isize) = *c0y.offset(1_isize)
        * (*g.offset(37_isize) + *b00.offset(1_isize))
        + *b10.offset(1_isize) * *cpy.offset(1_isize);
    *g.offset(46_isize) = *c0y.offset(2_isize)
        * (*g.offset(38_isize) + *b00.offset(2_isize))
        + *b10.offset(2_isize) * *cpy.offset(2_isize);
    *g.offset(47_isize) = *c0y.offset(3_isize)
        * (*g.offset(39_isize) + *b00.offset(3_isize))
        + *b10.offset(3_isize) * *cpy.offset(3_isize);
    *g.offset(56_isize) = *c0z.offset(0_isize) * *g.offset(48_isize);
    *g.offset(57_isize) = *c0z.offset(1_isize) * *g.offset(49_isize);
    *g.offset(58_isize) = *c0z.offset(2_isize) * *g.offset(50_isize);
    *g.offset(59_isize) = *c0z.offset(3_isize) * *g.offset(51_isize);
    *g.offset(64_isize) = *c0z.offset(0_isize) * *g.offset(56_isize)
        + *b10.offset(0_isize) * *g.offset(48_isize);
    *g.offset(65_isize) = *c0z.offset(1_isize) * *g.offset(57_isize)
        + *b10.offset(1_isize) * *g.offset(49_isize);
    *g.offset(66_isize) = *c0z.offset(2_isize) * *g.offset(58_isize)
        + *b10.offset(2_isize) * *g.offset(50_isize);
    *g.offset(67_isize) = *c0z.offset(3_isize) * *g.offset(59_isize)
        + *b10.offset(3_isize) * *g.offset(51_isize);
    *g.offset(52_isize) = *cpz.offset(0_isize) * *g.offset(48_isize);
    *g.offset(53_isize) = *cpz.offset(1_isize) * *g.offset(49_isize);
    *g.offset(54_isize) = *cpz.offset(2_isize) * *g.offset(50_isize);
    *g.offset(55_isize) = *cpz.offset(3_isize) * *g.offset(51_isize);
    *g.offset(60_isize) = *cpz.offset(0_isize) * *g.offset(56_isize)
        + *b00.offset(0_isize) * *g.offset(48_isize);
    *g.offset(61_isize) = *cpz.offset(1_isize) * *g.offset(57_isize)
        + *b00.offset(1_isize) * *g.offset(49_isize);
    *g.offset(62_isize) = *cpz.offset(2_isize) * *g.offset(58_isize)
        + *b00.offset(2_isize) * *g.offset(50_isize);
    *g.offset(63_isize) = *cpz.offset(3_isize) * *g.offset(59_isize)
        + *b00.offset(3_isize) * *g.offset(51_isize);
    *g.offset(68_isize) = *c0z.offset(0_isize) * *g.offset(60_isize)
        + *b10.offset(0_isize) * *g.offset(52_isize)
        + *b00.offset(0_isize) * *g.offset(56_isize);
    *g.offset(69_isize) = *c0z.offset(1_isize) * *g.offset(61_isize)
        + *b10.offset(1_isize) * *g.offset(53_isize)
        + *b00.offset(1_isize) * *g.offset(57_isize);
    *g.offset(70_isize) = *c0z.offset(2_isize) * *g.offset(62_isize)
        + *b10.offset(2_isize) * *g.offset(54_isize)
        + *b00.offset(2_isize) * *g.offset(58_isize);
    *g.offset(71_isize) = *c0z.offset(3_isize) * *g.offset(63_isize)
        + *b10.offset(3_isize) * *g.offset(55_isize)
        + *b00.offset(3_isize) * *g.offset(59_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0210(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(4_isize) = *cpx.offset(0_isize);
    *g.offset(5_isize) = *cpx.offset(1_isize);
    *g.offset(6_isize) = *cpx.offset(2_isize);
    *g.offset(7_isize) = *cpx.offset(3_isize);
    *g.offset(8_isize) = *c0x.offset(0_isize);
    *g.offset(9_isize) = *c0x.offset(1_isize);
    *g.offset(10_isize) = *c0x.offset(2_isize);
    *g.offset(11_isize) = *c0x.offset(3_isize);
    *g.offset(12_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(13_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(14_isize) =
        *cpx.offset(2_isize) * *c0x.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(15_isize) =
        *cpx.offset(3_isize) * *c0x.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(16_isize) =
        *c0x.offset(0_isize) * *c0x.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(17_isize) =
        *c0x.offset(1_isize) * *c0x.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(18_isize) =
        *c0x.offset(2_isize) * *c0x.offset(2_isize) + *b10.offset(2_isize);
    *g.offset(19_isize) =
        *c0x.offset(3_isize) * *c0x.offset(3_isize) + *b10.offset(3_isize);
    *g.offset(20_isize) = *c0x.offset(0_isize)
        * (*g.offset(12_isize) + *b00.offset(0_isize))
        + *b10.offset(0_isize) * *cpx.offset(0_isize);
    *g.offset(21_isize) = *c0x.offset(1_isize)
        * (*g.offset(13_isize) + *b00.offset(1_isize))
        + *b10.offset(1_isize) * *cpx.offset(1_isize);
    *g.offset(22_isize) = *c0x.offset(2_isize)
        * (*g.offset(14_isize) + *b00.offset(2_isize))
        + *b10.offset(2_isize) * *cpx.offset(2_isize);
    *g.offset(23_isize) = *c0x.offset(3_isize)
        * (*g.offset(15_isize) + *b00.offset(3_isize))
        + *b10.offset(3_isize) * *cpx.offset(3_isize);
    *g.offset(24_isize) = 1_f64;
    *g.offset(25_isize) = 1_f64;
    *g.offset(26_isize) = 1_f64;
    *g.offset(27_isize) = 1_f64;
    *g.offset(28_isize) = *cpy.offset(0_isize);
    *g.offset(29_isize) = *cpy.offset(1_isize);
    *g.offset(30_isize) = *cpy.offset(2_isize);
    *g.offset(31_isize) = *cpy.offset(3_isize);
    *g.offset(32_isize) = *c0y.offset(0_isize);
    *g.offset(33_isize) = *c0y.offset(1_isize);
    *g.offset(34_isize) = *c0y.offset(2_isize);
    *g.offset(35_isize) = *c0y.offset(3_isize);
    *g.offset(36_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(37_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(38_isize) =
        *cpy.offset(2_isize) * *c0y.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(39_isize) =
        *cpy.offset(3_isize) * *c0y.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(40_isize) =
        *c0y.offset(0_isize) * *c0y.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(41_isize) =
        *c0y.offset(1_isize) * *c0y.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(42_isize) =
        *c0y.offset(2_isize) * *c0y.offset(2_isize) + *b10.offset(2_isize);
    *g.offset(43_isize) =
        *c0y.offset(3_isize) * *c0y.offset(3_isize) + *b10.offset(3_isize);
    *g.offset(44_isize) = *c0y.offset(0_isize)
        * (*g.offset(36_isize) + *b00.offset(0_isize))
        + *b10.offset(0_isize) * *cpy.offset(0_isize);
    *g.offset(45_isize) = *c0y.offset(1_isize)
        * (*g.offset(37_isize) + *b00.offset(1_isize))
        + *b10.offset(1_isize) * *cpy.offset(1_isize);
    *g.offset(46_isize) = *c0y.offset(2_isize)
        * (*g.offset(38_isize) + *b00.offset(2_isize))
        + *b10.offset(2_isize) * *cpy.offset(2_isize);
    *g.offset(47_isize) = *c0y.offset(3_isize)
        * (*g.offset(39_isize) + *b00.offset(3_isize))
        + *b10.offset(3_isize) * *cpy.offset(3_isize);
    *g.offset(52_isize) = *cpz.offset(0_isize) * *g.offset(48_isize);
    *g.offset(53_isize) = *cpz.offset(1_isize) * *g.offset(49_isize);
    *g.offset(54_isize) = *cpz.offset(2_isize) * *g.offset(50_isize);
    *g.offset(55_isize) = *cpz.offset(3_isize) * *g.offset(51_isize);
    *g.offset(56_isize) = *c0z.offset(0_isize) * *g.offset(48_isize);
    *g.offset(57_isize) = *c0z.offset(1_isize) * *g.offset(49_isize);
    *g.offset(58_isize) = *c0z.offset(2_isize) * *g.offset(50_isize);
    *g.offset(59_isize) = *c0z.offset(3_isize) * *g.offset(51_isize);
    *g.offset(60_isize) = *cpz.offset(0_isize) * *g.offset(56_isize)
        + *b00.offset(0_isize) * *g.offset(48_isize);
    *g.offset(61_isize) = *cpz.offset(1_isize) * *g.offset(57_isize)
        + *b00.offset(1_isize) * *g.offset(49_isize);
    *g.offset(62_isize) = *cpz.offset(2_isize) * *g.offset(58_isize)
        + *b00.offset(2_isize) * *g.offset(50_isize);
    *g.offset(63_isize) = *cpz.offset(3_isize) * *g.offset(59_isize)
        + *b00.offset(3_isize) * *g.offset(51_isize);
    *g.offset(64_isize) = *c0z.offset(0_isize) * *g.offset(56_isize)
        + *b10.offset(0_isize) * *g.offset(48_isize);
    *g.offset(65_isize) = *c0z.offset(1_isize) * *g.offset(57_isize)
        + *b10.offset(1_isize) * *g.offset(49_isize);
    *g.offset(66_isize) = *c0z.offset(2_isize) * *g.offset(58_isize)
        + *b10.offset(2_isize) * *g.offset(50_isize);
    *g.offset(67_isize) = *c0z.offset(3_isize) * *g.offset(59_isize)
        + *b10.offset(3_isize) * *g.offset(51_isize);
    *g.offset(68_isize) = *c0z.offset(0_isize) * *g.offset(60_isize)
        + *b10.offset(0_isize) * *g.offset(52_isize)
        + *b00.offset(0_isize) * *g.offset(56_isize);
    *g.offset(69_isize) = *c0z.offset(1_isize) * *g.offset(61_isize)
        + *b10.offset(1_isize) * *g.offset(53_isize)
        + *b00.offset(1_isize) * *g.offset(57_isize);
    *g.offset(70_isize) = *c0z.offset(2_isize) * *g.offset(62_isize)
        + *b10.offset(2_isize) * *g.offset(54_isize)
        + *b00.offset(2_isize) * *g.offset(58_isize);
    *g.offset(71_isize) = *c0z.offset(3_isize) * *g.offset(63_isize)
        + *b10.offset(3_isize) * *g.offset(55_isize)
        + *b00.offset(3_isize) * *g.offset(59_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0300(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(4_isize) = *c0x.offset(0_isize);
    *g.offset(5_isize) = *c0x.offset(1_isize);
    *g.offset(6_isize) = *c0x.offset(2_isize);
    *g.offset(7_isize) = *c0x.offset(3_isize);
    *g.offset(8_isize) =
        *c0x.offset(0_isize) * *c0x.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(9_isize) =
        *c0x.offset(1_isize) * *c0x.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(10_isize) =
        *c0x.offset(2_isize) * *c0x.offset(2_isize) + *b10.offset(2_isize);
    *g.offset(11_isize) =
        *c0x.offset(3_isize) * *c0x.offset(3_isize) + *b10.offset(3_isize);
    *g.offset(12_isize) =
        *c0x.offset(0_isize) * (*g.offset(8_isize) + 2_f64 * *b10.offset(0_isize));
    *g.offset(13_isize) =
        *c0x.offset(1_isize) * (*g.offset(9_isize) + 2_f64 * *b10.offset(1_isize));
    *g.offset(14_isize) =
        *c0x.offset(2_isize) * (*g.offset(10_isize) + 2_f64 * *b10.offset(2_isize));
    *g.offset(15_isize) =
        *c0x.offset(3_isize) * (*g.offset(11_isize) + 2_f64 * *b10.offset(3_isize));
    *g.offset(16_isize) = 1_f64;
    *g.offset(17_isize) = 1_f64;
    *g.offset(18_isize) = 1_f64;
    *g.offset(19_isize) = 1_f64;
    *g.offset(20_isize) = *c0y.offset(0_isize);
    *g.offset(21_isize) = *c0y.offset(1_isize);
    *g.offset(22_isize) = *c0y.offset(2_isize);
    *g.offset(23_isize) = *c0y.offset(3_isize);
    *g.offset(24_isize) =
        *c0y.offset(0_isize) * *c0y.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(25_isize) =
        *c0y.offset(1_isize) * *c0y.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(26_isize) =
        *c0y.offset(2_isize) * *c0y.offset(2_isize) + *b10.offset(2_isize);
    *g.offset(27_isize) =
        *c0y.offset(3_isize) * *c0y.offset(3_isize) + *b10.offset(3_isize);
    *g.offset(28_isize) =
        *c0y.offset(0_isize) * (*g.offset(24_isize) + 2_f64 * *b10.offset(0_isize));
    *g.offset(29_isize) =
        *c0y.offset(1_isize) * (*g.offset(25_isize) + 2_f64 * *b10.offset(1_isize));
    *g.offset(30_isize) =
        *c0y.offset(2_isize) * (*g.offset(26_isize) + 2_f64 * *b10.offset(2_isize));
    *g.offset(31_isize) =
        *c0y.offset(3_isize) * (*g.offset(27_isize) + 2_f64 * *b10.offset(3_isize));
    *g.offset(36_isize) = *c0z.offset(0_isize) * *g.offset(32_isize);
    *g.offset(37_isize) = *c0z.offset(1_isize) * *g.offset(33_isize);
    *g.offset(38_isize) = *c0z.offset(2_isize) * *g.offset(34_isize);
    *g.offset(39_isize) = *c0z.offset(3_isize) * *g.offset(35_isize);
    *g.offset(40_isize) = *c0z.offset(0_isize) * *g.offset(36_isize)
        + *b10.offset(0_isize) * *g.offset(32_isize);
    *g.offset(41_isize) = *c0z.offset(1_isize) * *g.offset(37_isize)
        + *b10.offset(1_isize) * *g.offset(33_isize);
    *g.offset(42_isize) = *c0z.offset(2_isize) * *g.offset(38_isize)
        + *b10.offset(2_isize) * *g.offset(34_isize);
    *g.offset(43_isize) = *c0z.offset(3_isize) * *g.offset(39_isize)
        + *b10.offset(3_isize) * *g.offset(35_isize);
    *g.offset(44_isize) = *c0z.offset(0_isize) * *g.offset(40_isize)
        + 2_f64 * *b10.offset(0_isize) * *g.offset(36_isize);
    *g.offset(45_isize) = *c0z.offset(1_isize) * *g.offset(41_isize)
        + 2_f64 * *b10.offset(1_isize) * *g.offset(37_isize);
    *g.offset(46_isize) = *c0z.offset(2_isize) * *g.offset(42_isize)
        + 2_f64 * *b10.offset(2_isize) * *g.offset(38_isize);
    *g.offset(47_isize) = *c0z.offset(3_isize) * *g.offset(43_isize)
        + 2_f64 * *b10.offset(3_isize) * *g.offset(39_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1000(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = *c0x.offset(0_isize);
    *g.offset(3_isize) = *c0x.offset(1_isize);
    *g.offset(4_isize) = 1_f64;
    *g.offset(5_isize) = 1_f64;
    *g.offset(6_isize) = *c0y.offset(0_isize);
    *g.offset(7_isize) = *c0y.offset(1_isize);
    *g.offset(10_isize) = *c0z.offset(0_isize) * *g.offset(8_isize);
    *g.offset(11_isize) = *c0z.offset(1_isize) * *g.offset(9_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1001(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(4_isize) = *c0x.offset(0_isize);
    *g.offset(5_isize) = *c0x.offset(1_isize);
    *g.offset(6_isize) = *c0x.offset(2_isize);
    *g.offset(7_isize) = *c0x.offset(3_isize);
    *g.offset(8_isize) = *cpx.offset(0_isize);
    *g.offset(9_isize) = *cpx.offset(1_isize);
    *g.offset(10_isize) = *cpx.offset(2_isize);
    *g.offset(11_isize) = *cpx.offset(3_isize);
    *g.offset(12_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(13_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(14_isize) =
        *cpx.offset(2_isize) * *c0x.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(15_isize) =
        *cpx.offset(3_isize) * *c0x.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(16_isize) = 1_f64;
    *g.offset(17_isize) = 1_f64;
    *g.offset(18_isize) = 1_f64;
    *g.offset(19_isize) = 1_f64;
    *g.offset(20_isize) = *c0y.offset(0_isize);
    *g.offset(21_isize) = *c0y.offset(1_isize);
    *g.offset(22_isize) = *c0y.offset(2_isize);
    *g.offset(23_isize) = *c0y.offset(3_isize);
    *g.offset(24_isize) = *cpy.offset(0_isize);
    *g.offset(25_isize) = *cpy.offset(1_isize);
    *g.offset(26_isize) = *cpy.offset(2_isize);
    *g.offset(27_isize) = *cpy.offset(3_isize);
    *g.offset(28_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(29_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(30_isize) =
        *cpy.offset(2_isize) * *c0y.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(31_isize) =
        *cpy.offset(3_isize) * *c0y.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(36_isize) = *c0z.offset(0_isize) * *g.offset(32_isize);
    *g.offset(37_isize) = *c0z.offset(1_isize) * *g.offset(33_isize);
    *g.offset(38_isize) = *c0z.offset(2_isize) * *g.offset(34_isize);
    *g.offset(39_isize) = *c0z.offset(3_isize) * *g.offset(35_isize);
    *g.offset(40_isize) = *cpz.offset(0_isize) * *g.offset(32_isize);
    *g.offset(41_isize) = *cpz.offset(1_isize) * *g.offset(33_isize);
    *g.offset(42_isize) = *cpz.offset(2_isize) * *g.offset(34_isize);
    *g.offset(43_isize) = *cpz.offset(3_isize) * *g.offset(35_isize);
    *g.offset(44_isize) = *cpz.offset(0_isize) * *g.offset(36_isize)
        + *b00.offset(0_isize) * *g.offset(32_isize);
    *g.offset(45_isize) = *cpz.offset(1_isize) * *g.offset(37_isize)
        + *b00.offset(1_isize) * *g.offset(33_isize);
    *g.offset(46_isize) = *cpz.offset(2_isize) * *g.offset(38_isize)
        + *b00.offset(2_isize) * *g.offset(34_isize);
    *g.offset(47_isize) = *cpz.offset(3_isize) * *g.offset(39_isize)
        + *b00.offset(3_isize) * *g.offset(35_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1002(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(4_isize) = *c0x.offset(0_isize);
    *g.offset(5_isize) = *c0x.offset(1_isize);
    *g.offset(6_isize) = *c0x.offset(2_isize);
    *g.offset(7_isize) = *c0x.offset(3_isize);
    *g.offset(8_isize) = *cpx.offset(0_isize);
    *g.offset(9_isize) = *cpx.offset(1_isize);
    *g.offset(10_isize) = *cpx.offset(2_isize);
    *g.offset(11_isize) = *cpx.offset(3_isize);
    *g.offset(12_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(13_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(14_isize) =
        *cpx.offset(2_isize) * *c0x.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(15_isize) =
        *cpx.offset(3_isize) * *c0x.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(16_isize) =
        *cpx.offset(0_isize) * *cpx.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(17_isize) =
        *cpx.offset(1_isize) * *cpx.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(18_isize) =
        *cpx.offset(2_isize) * *cpx.offset(2_isize) + *b01.offset(2_isize);
    *g.offset(19_isize) =
        *cpx.offset(3_isize) * *cpx.offset(3_isize) + *b01.offset(3_isize);
    *g.offset(20_isize) = *cpx.offset(0_isize)
        * (*g.offset(12_isize) + *b00.offset(0_isize))
        + *b01.offset(0_isize) * *c0x.offset(0_isize);
    *g.offset(21_isize) = *cpx.offset(1_isize)
        * (*g.offset(13_isize) + *b00.offset(1_isize))
        + *b01.offset(1_isize) * *c0x.offset(1_isize);
    *g.offset(22_isize) = *cpx.offset(2_isize)
        * (*g.offset(14_isize) + *b00.offset(2_isize))
        + *b01.offset(2_isize) * *c0x.offset(2_isize);
    *g.offset(23_isize) = *cpx.offset(3_isize)
        * (*g.offset(15_isize) + *b00.offset(3_isize))
        + *b01.offset(3_isize) * *c0x.offset(3_isize);
    *g.offset(24_isize) = 1_f64;
    *g.offset(25_isize) = 1_f64;
    *g.offset(26_isize) = 1_f64;
    *g.offset(27_isize) = 1_f64;
    *g.offset(28_isize) = *c0y.offset(0_isize);
    *g.offset(29_isize) = *c0y.offset(1_isize);
    *g.offset(30_isize) = *c0y.offset(2_isize);
    *g.offset(31_isize) = *c0y.offset(3_isize);
    *g.offset(32_isize) = *cpy.offset(0_isize);
    *g.offset(33_isize) = *cpy.offset(1_isize);
    *g.offset(34_isize) = *cpy.offset(2_isize);
    *g.offset(35_isize) = *cpy.offset(3_isize);
    *g.offset(36_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(37_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(38_isize) =
        *cpy.offset(2_isize) * *c0y.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(39_isize) =
        *cpy.offset(3_isize) * *c0y.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(40_isize) =
        *cpy.offset(0_isize) * *cpy.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(41_isize) =
        *cpy.offset(1_isize) * *cpy.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(42_isize) =
        *cpy.offset(2_isize) * *cpy.offset(2_isize) + *b01.offset(2_isize);
    *g.offset(43_isize) =
        *cpy.offset(3_isize) * *cpy.offset(3_isize) + *b01.offset(3_isize);
    *g.offset(44_isize) = *cpy.offset(0_isize)
        * (*g.offset(36_isize) + *b00.offset(0_isize))
        + *b01.offset(0_isize) * *c0y.offset(0_isize);
    *g.offset(45_isize) = *cpy.offset(1_isize)
        * (*g.offset(37_isize) + *b00.offset(1_isize))
        + *b01.offset(1_isize) * *c0y.offset(1_isize);
    *g.offset(46_isize) = *cpy.offset(2_isize)
        * (*g.offset(38_isize) + *b00.offset(2_isize))
        + *b01.offset(2_isize) * *c0y.offset(2_isize);
    *g.offset(47_isize) = *cpy.offset(3_isize)
        * (*g.offset(39_isize) + *b00.offset(3_isize))
        + *b01.offset(3_isize) * *c0y.offset(3_isize);
    *g.offset(52_isize) = *c0z.offset(0_isize) * *g.offset(48_isize);
    *g.offset(53_isize) = *c0z.offset(1_isize) * *g.offset(49_isize);
    *g.offset(54_isize) = *c0z.offset(2_isize) * *g.offset(50_isize);
    *g.offset(55_isize) = *c0z.offset(3_isize) * *g.offset(51_isize);
    *g.offset(56_isize) = *cpz.offset(0_isize) * *g.offset(48_isize);
    *g.offset(57_isize) = *cpz.offset(1_isize) * *g.offset(49_isize);
    *g.offset(58_isize) = *cpz.offset(2_isize) * *g.offset(50_isize);
    *g.offset(59_isize) = *cpz.offset(3_isize) * *g.offset(51_isize);
    *g.offset(60_isize) = *cpz.offset(0_isize) * *g.offset(52_isize)
        + *b00.offset(0_isize) * *g.offset(48_isize);
    *g.offset(61_isize) = *cpz.offset(1_isize) * *g.offset(53_isize)
        + *b00.offset(1_isize) * *g.offset(49_isize);
    *g.offset(62_isize) = *cpz.offset(2_isize) * *g.offset(54_isize)
        + *b00.offset(2_isize) * *g.offset(50_isize);
    *g.offset(63_isize) = *cpz.offset(3_isize) * *g.offset(55_isize)
        + *b00.offset(3_isize) * *g.offset(51_isize);
    *g.offset(64_isize) = *cpz.offset(0_isize) * *g.offset(56_isize)
        + *b01.offset(0_isize) * *g.offset(48_isize);
    *g.offset(65_isize) = *cpz.offset(1_isize) * *g.offset(57_isize)
        + *b01.offset(1_isize) * *g.offset(49_isize);
    *g.offset(66_isize) = *cpz.offset(2_isize) * *g.offset(58_isize)
        + *b01.offset(2_isize) * *g.offset(50_isize);
    *g.offset(67_isize) = *cpz.offset(3_isize) * *g.offset(59_isize)
        + *b01.offset(3_isize) * *g.offset(51_isize);
    *g.offset(68_isize) = *cpz.offset(0_isize) * *g.offset(60_isize)
        + *b01.offset(0_isize) * *g.offset(52_isize)
        + *b00.offset(0_isize) * *g.offset(56_isize);
    *g.offset(69_isize) = *cpz.offset(1_isize) * *g.offset(61_isize)
        + *b01.offset(1_isize) * *g.offset(53_isize)
        + *b00.offset(1_isize) * *g.offset(57_isize);
    *g.offset(70_isize) = *cpz.offset(2_isize) * *g.offset(62_isize)
        + *b01.offset(2_isize) * *g.offset(54_isize)
        + *b00.offset(2_isize) * *g.offset(58_isize);
    *g.offset(71_isize) = *cpz.offset(3_isize) * *g.offset(63_isize)
        + *b01.offset(3_isize) * *g.offset(55_isize)
        + *b00.offset(3_isize) * *g.offset(59_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1010(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(4_isize) = *c0x.offset(0_isize);
    *g.offset(5_isize) = *c0x.offset(1_isize);
    *g.offset(6_isize) = *c0x.offset(2_isize);
    *g.offset(7_isize) = *c0x.offset(3_isize);
    *g.offset(8_isize) = *cpx.offset(0_isize);
    *g.offset(9_isize) = *cpx.offset(1_isize);
    *g.offset(10_isize) = *cpx.offset(2_isize);
    *g.offset(11_isize) = *cpx.offset(3_isize);
    *g.offset(12_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(13_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(14_isize) =
        *cpx.offset(2_isize) * *c0x.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(15_isize) =
        *cpx.offset(3_isize) * *c0x.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(16_isize) = 1_f64;
    *g.offset(17_isize) = 1_f64;
    *g.offset(18_isize) = 1_f64;
    *g.offset(19_isize) = 1_f64;
    *g.offset(20_isize) = *c0y.offset(0_isize);
    *g.offset(21_isize) = *c0y.offset(1_isize);
    *g.offset(22_isize) = *c0y.offset(2_isize);
    *g.offset(23_isize) = *c0y.offset(3_isize);
    *g.offset(24_isize) = *cpy.offset(0_isize);
    *g.offset(25_isize) = *cpy.offset(1_isize);
    *g.offset(26_isize) = *cpy.offset(2_isize);
    *g.offset(27_isize) = *cpy.offset(3_isize);
    *g.offset(28_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(29_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(30_isize) =
        *cpy.offset(2_isize) * *c0y.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(31_isize) =
        *cpy.offset(3_isize) * *c0y.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(36_isize) = *c0z.offset(0_isize) * *g.offset(32_isize);
    *g.offset(37_isize) = *c0z.offset(1_isize) * *g.offset(33_isize);
    *g.offset(38_isize) = *c0z.offset(2_isize) * *g.offset(34_isize);
    *g.offset(39_isize) = *c0z.offset(3_isize) * *g.offset(35_isize);
    *g.offset(40_isize) = *cpz.offset(0_isize) * *g.offset(32_isize);
    *g.offset(41_isize) = *cpz.offset(1_isize) * *g.offset(33_isize);
    *g.offset(42_isize) = *cpz.offset(2_isize) * *g.offset(34_isize);
    *g.offset(43_isize) = *cpz.offset(3_isize) * *g.offset(35_isize);
    *g.offset(44_isize) = *cpz.offset(0_isize) * *g.offset(36_isize)
        + *b00.offset(0_isize) * *g.offset(32_isize);
    *g.offset(45_isize) = *cpz.offset(1_isize) * *g.offset(37_isize)
        + *b00.offset(1_isize) * *g.offset(33_isize);
    *g.offset(46_isize) = *cpz.offset(2_isize) * *g.offset(38_isize)
        + *b00.offset(2_isize) * *g.offset(34_isize);
    *g.offset(47_isize) = *cpz.offset(3_isize) * *g.offset(39_isize)
        + *b00.offset(3_isize) * *g.offset(35_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1011(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    let xkxl: f64 = (*envs).rkrl[0_usize];
    let ykyl: f64 = (*envs).rkrl[1_usize];
    let zkzl: f64 = (*envs).rkrl[2_usize];
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(4_isize) = *c0x.offset(0_isize);
    *g.offset(5_isize) = *c0x.offset(1_isize);
    *g.offset(6_isize) = *c0x.offset(2_isize);
    *g.offset(7_isize) = *c0x.offset(3_isize);
    *g.offset(16_isize) = *cpx.offset(0_isize);
    *g.offset(17_isize) = *cpx.offset(1_isize);
    *g.offset(18_isize) = *cpx.offset(2_isize);
    *g.offset(19_isize) = *cpx.offset(3_isize);
    *g.offset(20_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(21_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(22_isize) =
        *cpx.offset(2_isize) * *c0x.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(23_isize) =
        *cpx.offset(3_isize) * *c0x.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(24_isize) =
        *cpx.offset(0_isize) * (xkxl + *cpx.offset(0_isize)) + *b01.offset(0_isize);
    *g.offset(25_isize) =
        *cpx.offset(1_isize) * (xkxl + *cpx.offset(1_isize)) + *b01.offset(1_isize);
    *g.offset(26_isize) =
        *cpx.offset(2_isize) * (xkxl + *cpx.offset(2_isize)) + *b01.offset(2_isize);
    *g.offset(27_isize) =
        *cpx.offset(3_isize) * (xkxl + *cpx.offset(3_isize)) + *b01.offset(3_isize);
    *g.offset(28_isize) = *g.offset(20_isize) * (xkxl + *cpx.offset(0_isize))
        + *cpx.offset(0_isize) * *b00.offset(0_isize)
        + *b01.offset(0_isize) * *c0x.offset(0_isize);
    *g.offset(29_isize) = *g.offset(21_isize) * (xkxl + *cpx.offset(1_isize))
        + *cpx.offset(1_isize) * *b00.offset(1_isize)
        + *b01.offset(1_isize) * *c0x.offset(1_isize);
    *g.offset(30_isize) = *g.offset(22_isize) * (xkxl + *cpx.offset(2_isize))
        + *cpx.offset(2_isize) * *b00.offset(2_isize)
        + *b01.offset(2_isize) * *c0x.offset(2_isize);
    *g.offset(31_isize) = *g.offset(23_isize) * (xkxl + *cpx.offset(3_isize))
        + *cpx.offset(3_isize) * *b00.offset(3_isize)
        + *b01.offset(3_isize) * *c0x.offset(3_isize);
    *g.offset(8_isize) = xkxl + *cpx.offset(0_isize);
    *g.offset(9_isize) = xkxl + *cpx.offset(1_isize);
    *g.offset(10_isize) = xkxl + *cpx.offset(2_isize);
    *g.offset(11_isize) = xkxl + *cpx.offset(3_isize);
    *g.offset(12_isize) =
        *c0x.offset(0_isize) * (xkxl + *cpx.offset(0_isize)) + *b00.offset(0_isize);
    *g.offset(13_isize) =
        *c0x.offset(1_isize) * (xkxl + *cpx.offset(1_isize)) + *b00.offset(1_isize);
    *g.offset(14_isize) =
        *c0x.offset(2_isize) * (xkxl + *cpx.offset(2_isize)) + *b00.offset(2_isize);
    *g.offset(15_isize) =
        *c0x.offset(3_isize) * (xkxl + *cpx.offset(3_isize)) + *b00.offset(3_isize);
    *g.offset(48_isize) = 1_f64;
    *g.offset(49_isize) = 1_f64;
    *g.offset(50_isize) = 1_f64;
    *g.offset(51_isize) = 1_f64;
    *g.offset(52_isize) = *c0y.offset(0_isize);
    *g.offset(53_isize) = *c0y.offset(1_isize);
    *g.offset(54_isize) = *c0y.offset(2_isize);
    *g.offset(55_isize) = *c0y.offset(3_isize);
    *g.offset(64_isize) = *cpy.offset(0_isize);
    *g.offset(65_isize) = *cpy.offset(1_isize);
    *g.offset(66_isize) = *cpy.offset(2_isize);
    *g.offset(67_isize) = *cpy.offset(3_isize);
    *g.offset(68_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(69_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(70_isize) =
        *cpy.offset(2_isize) * *c0y.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(71_isize) =
        *cpy.offset(3_isize) * *c0y.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(72_isize) =
        *cpy.offset(0_isize) * (ykyl + *cpy.offset(0_isize)) + *b01.offset(0_isize);
    *g.offset(73_isize) =
        *cpy.offset(1_isize) * (ykyl + *cpy.offset(1_isize)) + *b01.offset(1_isize);
    *g.offset(74_isize) =
        *cpy.offset(2_isize) * (ykyl + *cpy.offset(2_isize)) + *b01.offset(2_isize);
    *g.offset(75_isize) =
        *cpy.offset(3_isize) * (ykyl + *cpy.offset(3_isize)) + *b01.offset(3_isize);
    *g.offset(76_isize) = *g.offset(68_isize) * (ykyl + *cpy.offset(0_isize))
        + *cpy.offset(0_isize) * *b00.offset(0_isize)
        + *b01.offset(0_isize) * *c0y.offset(0_isize);
    *g.offset(77_isize) = *g.offset(69_isize) * (ykyl + *cpy.offset(1_isize))
        + *cpy.offset(1_isize) * *b00.offset(1_isize)
        + *b01.offset(1_isize) * *c0y.offset(1_isize);
    *g.offset(78_isize) = *g.offset(70_isize) * (ykyl + *cpy.offset(2_isize))
        + *cpy.offset(2_isize) * *b00.offset(2_isize)
        + *b01.offset(2_isize) * *c0y.offset(2_isize);
    *g.offset(79_isize) = *g.offset(71_isize) * (ykyl + *cpy.offset(3_isize))
        + *cpy.offset(3_isize) * *b00.offset(3_isize)
        + *b01.offset(3_isize) * *c0y.offset(3_isize);
    *g.offset(56_isize) = ykyl + *cpy.offset(0_isize);
    *g.offset(57_isize) = ykyl + *cpy.offset(1_isize);
    *g.offset(58_isize) = ykyl + *cpy.offset(2_isize);
    *g.offset(59_isize) = ykyl + *cpy.offset(3_isize);
    *g.offset(60_isize) =
        *c0y.offset(0_isize) * (ykyl + *cpy.offset(0_isize)) + *b00.offset(0_isize);
    *g.offset(61_isize) =
        *c0y.offset(1_isize) * (ykyl + *cpy.offset(1_isize)) + *b00.offset(1_isize);
    *g.offset(62_isize) =
        *c0y.offset(2_isize) * (ykyl + *cpy.offset(2_isize)) + *b00.offset(2_isize);
    *g.offset(63_isize) =
        *c0y.offset(3_isize) * (ykyl + *cpy.offset(3_isize)) + *b00.offset(3_isize);
    *g.offset(100_isize) = *c0z.offset(0_isize) * *g.offset(96_isize);
    *g.offset(101_isize) = *c0z.offset(1_isize) * *g.offset(97_isize);
    *g.offset(102_isize) = *c0z.offset(2_isize) * *g.offset(98_isize);
    *g.offset(103_isize) = *c0z.offset(3_isize) * *g.offset(99_isize);
    *g.offset(112_isize) = *cpz.offset(0_isize) * *g.offset(96_isize);
    *g.offset(113_isize) = *cpz.offset(1_isize) * *g.offset(97_isize);
    *g.offset(114_isize) = *cpz.offset(2_isize) * *g.offset(98_isize);
    *g.offset(115_isize) = *cpz.offset(3_isize) * *g.offset(99_isize);
    *g.offset(116_isize) = *cpz.offset(0_isize) * *g.offset(100_isize)
        + *b00.offset(0_isize) * *g.offset(96_isize);
    *g.offset(117_isize) = *cpz.offset(1_isize) * *g.offset(101_isize)
        + *b00.offset(1_isize) * *g.offset(97_isize);
    *g.offset(118_isize) = *cpz.offset(2_isize) * *g.offset(102_isize)
        + *b00.offset(2_isize) * *g.offset(98_isize);
    *g.offset(119_isize) = *cpz.offset(3_isize) * *g.offset(103_isize)
        + *b00.offset(3_isize) * *g.offset(99_isize);
    *g.offset(120_isize) = *g.offset(112_isize) * (zkzl + *cpz.offset(0_isize))
        + *b01.offset(0_isize) * *g.offset(96_isize);
    *g.offset(121_isize) = *g.offset(113_isize) * (zkzl + *cpz.offset(1_isize))
        + *b01.offset(1_isize) * *g.offset(97_isize);
    *g.offset(122_isize) = *g.offset(114_isize) * (zkzl + *cpz.offset(2_isize))
        + *b01.offset(2_isize) * *g.offset(98_isize);
    *g.offset(123_isize) = *g.offset(115_isize) * (zkzl + *cpz.offset(3_isize))
        + *b01.offset(3_isize) * *g.offset(99_isize);
    *g.offset(124_isize) = *g.offset(116_isize) * (zkzl + *cpz.offset(0_isize))
        + *b01.offset(0_isize) * *g.offset(100_isize)
        + *b00.offset(0_isize) * *g.offset(112_isize);
    *g.offset(125_isize) = *g.offset(117_isize) * (zkzl + *cpz.offset(1_isize))
        + *b01.offset(1_isize) * *g.offset(101_isize)
        + *b00.offset(1_isize) * *g.offset(113_isize);
    *g.offset(126_isize) = *g.offset(118_isize) * (zkzl + *cpz.offset(2_isize))
        + *b01.offset(2_isize) * *g.offset(102_isize)
        + *b00.offset(2_isize) * *g.offset(114_isize);
    *g.offset(127_isize) = *g.offset(119_isize) * (zkzl + *cpz.offset(3_isize))
        + *b01.offset(3_isize) * *g.offset(103_isize)
        + *b00.offset(3_isize) * *g.offset(115_isize);
    *g.offset(104_isize) = *g.offset(96_isize) * (zkzl + *cpz.offset(0_isize));
    *g.offset(105_isize) = *g.offset(97_isize) * (zkzl + *cpz.offset(1_isize));
    *g.offset(106_isize) = *g.offset(98_isize) * (zkzl + *cpz.offset(2_isize));
    *g.offset(107_isize) = *g.offset(99_isize) * (zkzl + *cpz.offset(3_isize));
    *g.offset(108_isize) = *g.offset(100_isize) * (zkzl + *cpz.offset(0_isize))
        + *b00.offset(0_isize) * *g.offset(96_isize);
    *g.offset(109_isize) = *g.offset(101_isize) * (zkzl + *cpz.offset(1_isize))
        + *b00.offset(1_isize) * *g.offset(97_isize);
    *g.offset(110_isize) = *g.offset(102_isize) * (zkzl + *cpz.offset(2_isize))
        + *b00.offset(2_isize) * *g.offset(98_isize);
    *g.offset(111_isize) = *g.offset(103_isize) * (zkzl + *cpz.offset(3_isize))
        + *b00.offset(3_isize) * *g.offset(99_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1020(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(4_isize) = *c0x.offset(0_isize);
    *g.offset(5_isize) = *c0x.offset(1_isize);
    *g.offset(6_isize) = *c0x.offset(2_isize);
    *g.offset(7_isize) = *c0x.offset(3_isize);
    *g.offset(8_isize) = *cpx.offset(0_isize);
    *g.offset(9_isize) = *cpx.offset(1_isize);
    *g.offset(10_isize) = *cpx.offset(2_isize);
    *g.offset(11_isize) = *cpx.offset(3_isize);
    *g.offset(12_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(13_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(14_isize) =
        *cpx.offset(2_isize) * *c0x.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(15_isize) =
        *cpx.offset(3_isize) * *c0x.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(16_isize) =
        *cpx.offset(0_isize) * *cpx.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(17_isize) =
        *cpx.offset(1_isize) * *cpx.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(18_isize) =
        *cpx.offset(2_isize) * *cpx.offset(2_isize) + *b01.offset(2_isize);
    *g.offset(19_isize) =
        *cpx.offset(3_isize) * *cpx.offset(3_isize) + *b01.offset(3_isize);
    *g.offset(20_isize) = *cpx.offset(0_isize)
        * (*g.offset(12_isize) + *b00.offset(0_isize))
        + *b01.offset(0_isize) * *c0x.offset(0_isize);
    *g.offset(21_isize) = *cpx.offset(1_isize)
        * (*g.offset(13_isize) + *b00.offset(1_isize))
        + *b01.offset(1_isize) * *c0x.offset(1_isize);
    *g.offset(22_isize) = *cpx.offset(2_isize)
        * (*g.offset(14_isize) + *b00.offset(2_isize))
        + *b01.offset(2_isize) * *c0x.offset(2_isize);
    *g.offset(23_isize) = *cpx.offset(3_isize)
        * (*g.offset(15_isize) + *b00.offset(3_isize))
        + *b01.offset(3_isize) * *c0x.offset(3_isize);
    *g.offset(24_isize) = 1_f64;
    *g.offset(25_isize) = 1_f64;
    *g.offset(26_isize) = 1_f64;
    *g.offset(27_isize) = 1_f64;
    *g.offset(28_isize) = *c0y.offset(0_isize);
    *g.offset(29_isize) = *c0y.offset(1_isize);
    *g.offset(30_isize) = *c0y.offset(2_isize);
    *g.offset(31_isize) = *c0y.offset(3_isize);
    *g.offset(32_isize) = *cpy.offset(0_isize);
    *g.offset(33_isize) = *cpy.offset(1_isize);
    *g.offset(34_isize) = *cpy.offset(2_isize);
    *g.offset(35_isize) = *cpy.offset(3_isize);
    *g.offset(36_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(37_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(38_isize) =
        *cpy.offset(2_isize) * *c0y.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(39_isize) =
        *cpy.offset(3_isize) * *c0y.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(40_isize) =
        *cpy.offset(0_isize) * *cpy.offset(0_isize) + *b01.offset(0_isize);
    *g.offset(41_isize) =
        *cpy.offset(1_isize) * *cpy.offset(1_isize) + *b01.offset(1_isize);
    *g.offset(42_isize) =
        *cpy.offset(2_isize) * *cpy.offset(2_isize) + *b01.offset(2_isize);
    *g.offset(43_isize) =
        *cpy.offset(3_isize) * *cpy.offset(3_isize) + *b01.offset(3_isize);
    *g.offset(44_isize) = *cpy.offset(0_isize)
        * (*g.offset(36_isize) + *b00.offset(0_isize))
        + *b01.offset(0_isize) * *c0y.offset(0_isize);
    *g.offset(45_isize) = *cpy.offset(1_isize)
        * (*g.offset(37_isize) + *b00.offset(1_isize))
        + *b01.offset(1_isize) * *c0y.offset(1_isize);
    *g.offset(46_isize) = *cpy.offset(2_isize)
        * (*g.offset(38_isize) + *b00.offset(2_isize))
        + *b01.offset(2_isize) * *c0y.offset(2_isize);
    *g.offset(47_isize) = *cpy.offset(3_isize)
        * (*g.offset(39_isize) + *b00.offset(3_isize))
        + *b01.offset(3_isize) * *c0y.offset(3_isize);
    *g.offset(52_isize) = *c0z.offset(0_isize) * *g.offset(48_isize);
    *g.offset(53_isize) = *c0z.offset(1_isize) * *g.offset(49_isize);
    *g.offset(54_isize) = *c0z.offset(2_isize) * *g.offset(50_isize);
    *g.offset(55_isize) = *c0z.offset(3_isize) * *g.offset(51_isize);
    *g.offset(56_isize) = *cpz.offset(0_isize) * *g.offset(48_isize);
    *g.offset(57_isize) = *cpz.offset(1_isize) * *g.offset(49_isize);
    *g.offset(58_isize) = *cpz.offset(2_isize) * *g.offset(50_isize);
    *g.offset(59_isize) = *cpz.offset(3_isize) * *g.offset(51_isize);
    *g.offset(60_isize) = *cpz.offset(0_isize) * *g.offset(52_isize)
        + *b00.offset(0_isize) * *g.offset(48_isize);
    *g.offset(61_isize) = *cpz.offset(1_isize) * *g.offset(53_isize)
        + *b00.offset(1_isize) * *g.offset(49_isize);
    *g.offset(62_isize) = *cpz.offset(2_isize) * *g.offset(54_isize)
        + *b00.offset(2_isize) * *g.offset(50_isize);
    *g.offset(63_isize) = *cpz.offset(3_isize) * *g.offset(55_isize)
        + *b00.offset(3_isize) * *g.offset(51_isize);
    *g.offset(64_isize) = *cpz.offset(0_isize) * *g.offset(56_isize)
        + *b01.offset(0_isize) * *g.offset(48_isize);
    *g.offset(65_isize) = *cpz.offset(1_isize) * *g.offset(57_isize)
        + *b01.offset(1_isize) * *g.offset(49_isize);
    *g.offset(66_isize) = *cpz.offset(2_isize) * *g.offset(58_isize)
        + *b01.offset(2_isize) * *g.offset(50_isize);
    *g.offset(67_isize) = *cpz.offset(3_isize) * *g.offset(59_isize)
        + *b01.offset(3_isize) * *g.offset(51_isize);
    *g.offset(68_isize) = *cpz.offset(0_isize) * *g.offset(60_isize)
        + *b01.offset(0_isize) * *g.offset(52_isize)
        + *b00.offset(0_isize) * *g.offset(56_isize);
    *g.offset(69_isize) = *cpz.offset(1_isize) * *g.offset(61_isize)
        + *b01.offset(1_isize) * *g.offset(53_isize)
        + *b00.offset(1_isize) * *g.offset(57_isize);
    *g.offset(70_isize) = *cpz.offset(2_isize) * *g.offset(62_isize)
        + *b01.offset(2_isize) * *g.offset(54_isize)
        + *b00.offset(2_isize) * *g.offset(58_isize);
    *g.offset(71_isize) = *cpz.offset(3_isize) * *g.offset(63_isize)
        + *b01.offset(3_isize) * *g.offset(55_isize)
        + *b00.offset(3_isize) * *g.offset(59_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1100(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    let xixj: f64 = (*envs).rirj[0_usize];
    let yiyj: f64 = (*envs).rirj[1_usize];
    let zizj: f64 = (*envs).rirj[2_usize];
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(8_isize) = *c0x.offset(0_isize);
    *g.offset(9_isize) = *c0x.offset(1_isize);
    *g.offset(10_isize) = *c0x.offset(2_isize);
    *g.offset(11_isize) = *c0x.offset(3_isize);
    *g.offset(12_isize) =
        *c0x.offset(0_isize) * (xixj + *c0x.offset(0_isize)) + *b10.offset(0_isize);
    *g.offset(13_isize) =
        *c0x.offset(1_isize) * (xixj + *c0x.offset(1_isize)) + *b10.offset(1_isize);
    *g.offset(14_isize) =
        *c0x.offset(2_isize) * (xixj + *c0x.offset(2_isize)) + *b10.offset(2_isize);
    *g.offset(15_isize) =
        *c0x.offset(3_isize) * (xixj + *c0x.offset(3_isize)) + *b10.offset(3_isize);
    *g.offset(4_isize) = xixj + *c0x.offset(0_isize);
    *g.offset(5_isize) = xixj + *c0x.offset(1_isize);
    *g.offset(6_isize) = xixj + *c0x.offset(2_isize);
    *g.offset(7_isize) = xixj + *c0x.offset(3_isize);
    *g.offset(24_isize) = 1_f64;
    *g.offset(25_isize) = 1_f64;
    *g.offset(26_isize) = 1_f64;
    *g.offset(27_isize) = 1_f64;
    *g.offset(32_isize) = *c0y.offset(0_isize);
    *g.offset(33_isize) = *c0y.offset(1_isize);
    *g.offset(34_isize) = *c0y.offset(2_isize);
    *g.offset(35_isize) = *c0y.offset(3_isize);
    *g.offset(36_isize) =
        *c0y.offset(0_isize) * (yiyj + *c0y.offset(0_isize)) + *b10.offset(0_isize);
    *g.offset(37_isize) =
        *c0y.offset(1_isize) * (yiyj + *c0y.offset(1_isize)) + *b10.offset(1_isize);
    *g.offset(38_isize) =
        *c0y.offset(2_isize) * (yiyj + *c0y.offset(2_isize)) + *b10.offset(2_isize);
    *g.offset(39_isize) =
        *c0y.offset(3_isize) * (yiyj + *c0y.offset(3_isize)) + *b10.offset(3_isize);
    *g.offset(28_isize) = yiyj + *c0y.offset(0_isize);
    *g.offset(29_isize) = yiyj + *c0y.offset(1_isize);
    *g.offset(30_isize) = yiyj + *c0y.offset(2_isize);
    *g.offset(31_isize) = yiyj + *c0y.offset(3_isize);
    *g.offset(56_isize) = *c0z.offset(0_isize) * *g.offset(48_isize);
    *g.offset(57_isize) = *c0z.offset(1_isize) * *g.offset(49_isize);
    *g.offset(58_isize) = *c0z.offset(2_isize) * *g.offset(50_isize);
    *g.offset(59_isize) = *c0z.offset(3_isize) * *g.offset(51_isize);
    *g.offset(60_isize) = *g.offset(56_isize) * (zizj + *c0z.offset(0_isize))
        + *b10.offset(0_isize) * *g.offset(48_isize);
    *g.offset(61_isize) = *g.offset(57_isize) * (zizj + *c0z.offset(1_isize))
        + *b10.offset(1_isize) * *g.offset(49_isize);
    *g.offset(62_isize) = *g.offset(58_isize) * (zizj + *c0z.offset(2_isize))
        + *b10.offset(2_isize) * *g.offset(50_isize);
    *g.offset(63_isize) = *g.offset(59_isize) * (zizj + *c0z.offset(3_isize))
        + *b10.offset(3_isize) * *g.offset(51_isize);
    *g.offset(52_isize) = *g.offset(48_isize) * (zizj + *c0z.offset(0_isize));
    *g.offset(53_isize) = *g.offset(49_isize) * (zizj + *c0z.offset(1_isize));
    *g.offset(54_isize) = *g.offset(50_isize) * (zizj + *c0z.offset(2_isize));
    *g.offset(55_isize) = *g.offset(51_isize) * (zizj + *c0z.offset(3_isize));
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1101(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    let xixj: f64 = (*envs).rirj[0_usize];
    let yiyj: f64 = (*envs).rirj[1_usize];
    let zizj: f64 = (*envs).rirj[2_usize];
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(16_isize) = *c0x.offset(0_isize);
    *g.offset(17_isize) = *c0x.offset(1_isize);
    *g.offset(18_isize) = *c0x.offset(2_isize);
    *g.offset(19_isize) = *c0x.offset(3_isize);
    *g.offset(8_isize) = *cpx.offset(0_isize);
    *g.offset(9_isize) = *cpx.offset(1_isize);
    *g.offset(10_isize) = *cpx.offset(2_isize);
    *g.offset(11_isize) = *cpx.offset(3_isize);
    *g.offset(24_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(25_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(26_isize) =
        *cpx.offset(2_isize) * *c0x.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(27_isize) =
        *cpx.offset(3_isize) * *c0x.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(20_isize) =
        *c0x.offset(0_isize) * (xixj + *c0x.offset(0_isize)) + *b10.offset(0_isize);
    *g.offset(21_isize) =
        *c0x.offset(1_isize) * (xixj + *c0x.offset(1_isize)) + *b10.offset(1_isize);
    *g.offset(22_isize) =
        *c0x.offset(2_isize) * (xixj + *c0x.offset(2_isize)) + *b10.offset(2_isize);
    *g.offset(23_isize) =
        *c0x.offset(3_isize) * (xixj + *c0x.offset(3_isize)) + *b10.offset(3_isize);
    *g.offset(4_isize) = xixj + *c0x.offset(0_isize);
    *g.offset(5_isize) = xixj + *c0x.offset(1_isize);
    *g.offset(6_isize) = xixj + *c0x.offset(2_isize);
    *g.offset(7_isize) = xixj + *c0x.offset(3_isize);
    *g.offset(28_isize) = *g.offset(24_isize) * (xixj + *c0x.offset(0_isize))
        + *c0x.offset(0_isize) * *b00.offset(0_isize)
        + *b10.offset(0_isize) * *cpx.offset(0_isize);
    *g.offset(29_isize) = *g.offset(25_isize) * (xixj + *c0x.offset(1_isize))
        + *c0x.offset(1_isize) * *b00.offset(1_isize)
        + *b10.offset(1_isize) * *cpx.offset(1_isize);
    *g.offset(30_isize) = *g.offset(26_isize) * (xixj + *c0x.offset(2_isize))
        + *c0x.offset(2_isize) * *b00.offset(2_isize)
        + *b10.offset(2_isize) * *cpx.offset(2_isize);
    *g.offset(31_isize) = *g.offset(27_isize) * (xixj + *c0x.offset(3_isize))
        + *c0x.offset(3_isize) * *b00.offset(3_isize)
        + *b10.offset(3_isize) * *cpx.offset(3_isize);
    *g.offset(12_isize) =
        *cpx.offset(0_isize) * (xixj + *c0x.offset(0_isize)) + *b00.offset(0_isize);
    *g.offset(13_isize) =
        *cpx.offset(1_isize) * (xixj + *c0x.offset(1_isize)) + *b00.offset(1_isize);
    *g.offset(14_isize) =
        *cpx.offset(2_isize) * (xixj + *c0x.offset(2_isize)) + *b00.offset(2_isize);
    *g.offset(15_isize) =
        *cpx.offset(3_isize) * (xixj + *c0x.offset(3_isize)) + *b00.offset(3_isize);
    *g.offset(48_isize) = 1_f64;
    *g.offset(49_isize) = 1_f64;
    *g.offset(50_isize) = 1_f64;
    *g.offset(51_isize) = 1_f64;
    *g.offset(64_isize) = *c0y.offset(0_isize);
    *g.offset(65_isize) = *c0y.offset(1_isize);
    *g.offset(66_isize) = *c0y.offset(2_isize);
    *g.offset(67_isize) = *c0y.offset(3_isize);
    *g.offset(56_isize) = *cpy.offset(0_isize);
    *g.offset(57_isize) = *cpy.offset(1_isize);
    *g.offset(58_isize) = *cpy.offset(2_isize);
    *g.offset(59_isize) = *cpy.offset(3_isize);
    *g.offset(72_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(73_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(74_isize) =
        *cpy.offset(2_isize) * *c0y.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(75_isize) =
        *cpy.offset(3_isize) * *c0y.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(68_isize) =
        *c0y.offset(0_isize) * (yiyj + *c0y.offset(0_isize)) + *b10.offset(0_isize);
    *g.offset(69_isize) =
        *c0y.offset(1_isize) * (yiyj + *c0y.offset(1_isize)) + *b10.offset(1_isize);
    *g.offset(70_isize) =
        *c0y.offset(2_isize) * (yiyj + *c0y.offset(2_isize)) + *b10.offset(2_isize);
    *g.offset(71_isize) =
        *c0y.offset(3_isize) * (yiyj + *c0y.offset(3_isize)) + *b10.offset(3_isize);
    *g.offset(52_isize) = yiyj + *c0y.offset(0_isize);
    *g.offset(53_isize) = yiyj + *c0y.offset(1_isize);
    *g.offset(54_isize) = yiyj + *c0y.offset(2_isize);
    *g.offset(55_isize) = yiyj + *c0y.offset(3_isize);
    *g.offset(76_isize) = *g.offset(72_isize) * (yiyj + *c0y.offset(0_isize))
        + *c0y.offset(0_isize) * *b00.offset(0_isize)
        + *b10.offset(0_isize) * *cpy.offset(0_isize);
    *g.offset(77_isize) = *g.offset(73_isize) * (yiyj + *c0y.offset(1_isize))
        + *c0y.offset(1_isize) * *b00.offset(1_isize)
        + *b10.offset(1_isize) * *cpy.offset(1_isize);
    *g.offset(78_isize) = *g.offset(74_isize) * (yiyj + *c0y.offset(2_isize))
        + *c0y.offset(2_isize) * *b00.offset(2_isize)
        + *b10.offset(2_isize) * *cpy.offset(2_isize);
    *g.offset(79_isize) = *g.offset(75_isize) * (yiyj + *c0y.offset(3_isize))
        + *c0y.offset(3_isize) * *b00.offset(3_isize)
        + *b10.offset(3_isize) * *cpy.offset(3_isize);
    *g.offset(60_isize) =
        *cpy.offset(0_isize) * (yiyj + *c0y.offset(0_isize)) + *b00.offset(0_isize);
    *g.offset(61_isize) =
        *cpy.offset(1_isize) * (yiyj + *c0y.offset(1_isize)) + *b00.offset(1_isize);
    *g.offset(62_isize) =
        *cpy.offset(2_isize) * (yiyj + *c0y.offset(2_isize)) + *b00.offset(2_isize);
    *g.offset(63_isize) =
        *cpy.offset(3_isize) * (yiyj + *c0y.offset(3_isize)) + *b00.offset(3_isize);
    *g.offset(112_isize) = *c0z.offset(0_isize) * *g.offset(96_isize);
    *g.offset(113_isize) = *c0z.offset(1_isize) * *g.offset(97_isize);
    *g.offset(114_isize) = *c0z.offset(2_isize) * *g.offset(98_isize);
    *g.offset(115_isize) = *c0z.offset(3_isize) * *g.offset(99_isize);
    *g.offset(104_isize) = *cpz.offset(0_isize) * *g.offset(96_isize);
    *g.offset(105_isize) = *cpz.offset(1_isize) * *g.offset(97_isize);
    *g.offset(106_isize) = *cpz.offset(2_isize) * *g.offset(98_isize);
    *g.offset(107_isize) = *cpz.offset(3_isize) * *g.offset(99_isize);
    *g.offset(120_isize) = *cpz.offset(0_isize) * *g.offset(112_isize)
        + *b00.offset(0_isize) * *g.offset(96_isize);
    *g.offset(121_isize) = *cpz.offset(1_isize) * *g.offset(113_isize)
        + *b00.offset(1_isize) * *g.offset(97_isize);
    *g.offset(122_isize) = *cpz.offset(2_isize) * *g.offset(114_isize)
        + *b00.offset(2_isize) * *g.offset(98_isize);
    *g.offset(123_isize) = *cpz.offset(3_isize) * *g.offset(115_isize)
        + *b00.offset(3_isize) * *g.offset(99_isize);
    *g.offset(116_isize) = *g.offset(112_isize) * (zizj + *c0z.offset(0_isize))
        + *b10.offset(0_isize) * *g.offset(96_isize);
    *g.offset(117_isize) = *g.offset(113_isize) * (zizj + *c0z.offset(1_isize))
        + *b10.offset(1_isize) * *g.offset(97_isize);
    *g.offset(118_isize) = *g.offset(114_isize) * (zizj + *c0z.offset(2_isize))
        + *b10.offset(2_isize) * *g.offset(98_isize);
    *g.offset(119_isize) = *g.offset(115_isize) * (zizj + *c0z.offset(3_isize))
        + *b10.offset(3_isize) * *g.offset(99_isize);
    *g.offset(100_isize) = *g.offset(96_isize) * (zizj + *c0z.offset(0_isize));
    *g.offset(101_isize) = *g.offset(97_isize) * (zizj + *c0z.offset(1_isize));
    *g.offset(102_isize) = *g.offset(98_isize) * (zizj + *c0z.offset(2_isize));
    *g.offset(103_isize) = *g.offset(99_isize) * (zizj + *c0z.offset(3_isize));
    *g.offset(124_isize) = *g.offset(120_isize) * (zizj + *c0z.offset(0_isize))
        + *b10.offset(0_isize) * *g.offset(104_isize)
        + *b00.offset(0_isize) * *g.offset(112_isize);
    *g.offset(125_isize) = *g.offset(121_isize) * (zizj + *c0z.offset(1_isize))
        + *b10.offset(1_isize) * *g.offset(105_isize)
        + *b00.offset(1_isize) * *g.offset(113_isize);
    *g.offset(126_isize) = *g.offset(122_isize) * (zizj + *c0z.offset(2_isize))
        + *b10.offset(2_isize) * *g.offset(106_isize)
        + *b00.offset(2_isize) * *g.offset(114_isize);
    *g.offset(127_isize) = *g.offset(123_isize) * (zizj + *c0z.offset(3_isize))
        + *b10.offset(3_isize) * *g.offset(107_isize)
        + *b00.offset(3_isize) * *g.offset(115_isize);
    *g.offset(108_isize) = zizj * *g.offset(104_isize)
        + *cpz.offset(0_isize) * *g.offset(112_isize)
        + *b00.offset(0_isize) * *g.offset(96_isize);
    *g.offset(109_isize) = zizj * *g.offset(105_isize)
        + *cpz.offset(1_isize) * *g.offset(113_isize)
        + *b00.offset(1_isize) * *g.offset(97_isize);
    *g.offset(110_isize) = zizj * *g.offset(106_isize)
        + *cpz.offset(2_isize) * *g.offset(114_isize)
        + *b00.offset(2_isize) * *g.offset(98_isize);
    *g.offset(111_isize) = zizj * *g.offset(107_isize)
        + *cpz.offset(3_isize) * *g.offset(115_isize)
        + *b00.offset(3_isize) * *g.offset(99_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1110(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    let xixj: f64 = (*envs).rirj[0_usize];
    let yiyj: f64 = (*envs).rirj[1_usize];
    let zizj: f64 = (*envs).rirj[2_usize];
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(16_isize) = *c0x.offset(0_isize);
    *g.offset(17_isize) = *c0x.offset(1_isize);
    *g.offset(18_isize) = *c0x.offset(2_isize);
    *g.offset(19_isize) = *c0x.offset(3_isize);
    *g.offset(8_isize) = *cpx.offset(0_isize);
    *g.offset(9_isize) = *cpx.offset(1_isize);
    *g.offset(10_isize) = *cpx.offset(2_isize);
    *g.offset(11_isize) = *cpx.offset(3_isize);
    *g.offset(24_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(25_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(26_isize) =
        *cpx.offset(2_isize) * *c0x.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(27_isize) =
        *cpx.offset(3_isize) * *c0x.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(20_isize) =
        *c0x.offset(0_isize) * (xixj + *c0x.offset(0_isize)) + *b10.offset(0_isize);
    *g.offset(21_isize) =
        *c0x.offset(1_isize) * (xixj + *c0x.offset(1_isize)) + *b10.offset(1_isize);
    *g.offset(22_isize) =
        *c0x.offset(2_isize) * (xixj + *c0x.offset(2_isize)) + *b10.offset(2_isize);
    *g.offset(23_isize) =
        *c0x.offset(3_isize) * (xixj + *c0x.offset(3_isize)) + *b10.offset(3_isize);
    *g.offset(4_isize) = xixj + *c0x.offset(0_isize);
    *g.offset(5_isize) = xixj + *c0x.offset(1_isize);
    *g.offset(6_isize) = xixj + *c0x.offset(2_isize);
    *g.offset(7_isize) = xixj + *c0x.offset(3_isize);
    *g.offset(28_isize) = *g.offset(24_isize) * (xixj + *c0x.offset(0_isize))
        + *c0x.offset(0_isize) * *b00.offset(0_isize)
        + *b10.offset(0_isize) * *cpx.offset(0_isize);
    *g.offset(29_isize) = *g.offset(25_isize) * (xixj + *c0x.offset(1_isize))
        + *c0x.offset(1_isize) * *b00.offset(1_isize)
        + *b10.offset(1_isize) * *cpx.offset(1_isize);
    *g.offset(30_isize) = *g.offset(26_isize) * (xixj + *c0x.offset(2_isize))
        + *c0x.offset(2_isize) * *b00.offset(2_isize)
        + *b10.offset(2_isize) * *cpx.offset(2_isize);
    *g.offset(31_isize) = *g.offset(27_isize) * (xixj + *c0x.offset(3_isize))
        + *c0x.offset(3_isize) * *b00.offset(3_isize)
        + *b10.offset(3_isize) * *cpx.offset(3_isize);
    *g.offset(12_isize) =
        *cpx.offset(0_isize) * (xixj + *c0x.offset(0_isize)) + *b00.offset(0_isize);
    *g.offset(13_isize) =
        *cpx.offset(1_isize) * (xixj + *c0x.offset(1_isize)) + *b00.offset(1_isize);
    *g.offset(14_isize) =
        *cpx.offset(2_isize) * (xixj + *c0x.offset(2_isize)) + *b00.offset(2_isize);
    *g.offset(15_isize) =
        *cpx.offset(3_isize) * (xixj + *c0x.offset(3_isize)) + *b00.offset(3_isize);
    *g.offset(48_isize) = 1_f64;
    *g.offset(49_isize) = 1_f64;
    *g.offset(50_isize) = 1_f64;
    *g.offset(51_isize) = 1_f64;
    *g.offset(64_isize) = *c0y.offset(0_isize);
    *g.offset(65_isize) = *c0y.offset(1_isize);
    *g.offset(66_isize) = *c0y.offset(2_isize);
    *g.offset(67_isize) = *c0y.offset(3_isize);
    *g.offset(56_isize) = *cpy.offset(0_isize);
    *g.offset(57_isize) = *cpy.offset(1_isize);
    *g.offset(58_isize) = *cpy.offset(2_isize);
    *g.offset(59_isize) = *cpy.offset(3_isize);
    *g.offset(72_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(73_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(74_isize) =
        *cpy.offset(2_isize) * *c0y.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(75_isize) =
        *cpy.offset(3_isize) * *c0y.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(68_isize) =
        *c0y.offset(0_isize) * (yiyj + *c0y.offset(0_isize)) + *b10.offset(0_isize);
    *g.offset(69_isize) =
        *c0y.offset(1_isize) * (yiyj + *c0y.offset(1_isize)) + *b10.offset(1_isize);
    *g.offset(70_isize) =
        *c0y.offset(2_isize) * (yiyj + *c0y.offset(2_isize)) + *b10.offset(2_isize);
    *g.offset(71_isize) =
        *c0y.offset(3_isize) * (yiyj + *c0y.offset(3_isize)) + *b10.offset(3_isize);
    *g.offset(52_isize) = yiyj + *c0y.offset(0_isize);
    *g.offset(53_isize) = yiyj + *c0y.offset(1_isize);
    *g.offset(54_isize) = yiyj + *c0y.offset(2_isize);
    *g.offset(55_isize) = yiyj + *c0y.offset(3_isize);
    *g.offset(76_isize) = *g.offset(72_isize) * (yiyj + *c0y.offset(0_isize))
        + *c0y.offset(0_isize) * *b00.offset(0_isize)
        + *b10.offset(0_isize) * *cpy.offset(0_isize);
    *g.offset(77_isize) = *g.offset(73_isize) * (yiyj + *c0y.offset(1_isize))
        + *c0y.offset(1_isize) * *b00.offset(1_isize)
        + *b10.offset(1_isize) * *cpy.offset(1_isize);
    *g.offset(78_isize) = *g.offset(74_isize) * (yiyj + *c0y.offset(2_isize))
        + *c0y.offset(2_isize) * *b00.offset(2_isize)
        + *b10.offset(2_isize) * *cpy.offset(2_isize);
    *g.offset(79_isize) = *g.offset(75_isize) * (yiyj + *c0y.offset(3_isize))
        + *c0y.offset(3_isize) * *b00.offset(3_isize)
        + *b10.offset(3_isize) * *cpy.offset(3_isize);
    *g.offset(60_isize) =
        *cpy.offset(0_isize) * (yiyj + *c0y.offset(0_isize)) + *b00.offset(0_isize);
    *g.offset(61_isize) =
        *cpy.offset(1_isize) * (yiyj + *c0y.offset(1_isize)) + *b00.offset(1_isize);
    *g.offset(62_isize) =
        *cpy.offset(2_isize) * (yiyj + *c0y.offset(2_isize)) + *b00.offset(2_isize);
    *g.offset(63_isize) =
        *cpy.offset(3_isize) * (yiyj + *c0y.offset(3_isize)) + *b00.offset(3_isize);
    *g.offset(112_isize) = *c0z.offset(0_isize) * *g.offset(96_isize);
    *g.offset(113_isize) = *c0z.offset(1_isize) * *g.offset(97_isize);
    *g.offset(114_isize) = *c0z.offset(2_isize) * *g.offset(98_isize);
    *g.offset(115_isize) = *c0z.offset(3_isize) * *g.offset(99_isize);
    *g.offset(104_isize) = *cpz.offset(0_isize) * *g.offset(96_isize);
    *g.offset(105_isize) = *cpz.offset(1_isize) * *g.offset(97_isize);
    *g.offset(106_isize) = *cpz.offset(2_isize) * *g.offset(98_isize);
    *g.offset(107_isize) = *cpz.offset(3_isize) * *g.offset(99_isize);
    *g.offset(120_isize) = *cpz.offset(0_isize) * *g.offset(112_isize)
        + *b00.offset(0_isize) * *g.offset(96_isize);
    *g.offset(121_isize) = *cpz.offset(1_isize) * *g.offset(113_isize)
        + *b00.offset(1_isize) * *g.offset(97_isize);
    *g.offset(122_isize) = *cpz.offset(2_isize) * *g.offset(114_isize)
        + *b00.offset(2_isize) * *g.offset(98_isize);
    *g.offset(123_isize) = *cpz.offset(3_isize) * *g.offset(115_isize)
        + *b00.offset(3_isize) * *g.offset(99_isize);
    *g.offset(116_isize) = *g.offset(112_isize) * (zizj + *c0z.offset(0_isize))
        + *b10.offset(0_isize) * *g.offset(96_isize);
    *g.offset(117_isize) = *g.offset(113_isize) * (zizj + *c0z.offset(1_isize))
        + *b10.offset(1_isize) * *g.offset(97_isize);
    *g.offset(118_isize) = *g.offset(114_isize) * (zizj + *c0z.offset(2_isize))
        + *b10.offset(2_isize) * *g.offset(98_isize);
    *g.offset(119_isize) = *g.offset(115_isize) * (zizj + *c0z.offset(3_isize))
        + *b10.offset(3_isize) * *g.offset(99_isize);
    *g.offset(100_isize) = *g.offset(96_isize) * (zizj + *c0z.offset(0_isize));
    *g.offset(101_isize) = *g.offset(97_isize) * (zizj + *c0z.offset(1_isize));
    *g.offset(102_isize) = *g.offset(98_isize) * (zizj + *c0z.offset(2_isize));
    *g.offset(103_isize) = *g.offset(99_isize) * (zizj + *c0z.offset(3_isize));
    *g.offset(124_isize) = *g.offset(120_isize) * (zizj + *c0z.offset(0_isize))
        + *b10.offset(0_isize) * *g.offset(104_isize)
        + *b00.offset(0_isize) * *g.offset(112_isize);
    *g.offset(125_isize) = *g.offset(121_isize) * (zizj + *c0z.offset(1_isize))
        + *b10.offset(1_isize) * *g.offset(105_isize)
        + *b00.offset(1_isize) * *g.offset(113_isize);
    *g.offset(126_isize) = *g.offset(122_isize) * (zizj + *c0z.offset(2_isize))
        + *b10.offset(2_isize) * *g.offset(106_isize)
        + *b00.offset(2_isize) * *g.offset(114_isize);
    *g.offset(127_isize) = *g.offset(123_isize) * (zizj + *c0z.offset(3_isize))
        + *b10.offset(3_isize) * *g.offset(107_isize)
        + *b00.offset(3_isize) * *g.offset(115_isize);
    *g.offset(108_isize) = zizj * *g.offset(104_isize)
        + *cpz.offset(0_isize) * *g.offset(112_isize)
        + *b00.offset(0_isize) * *g.offset(96_isize);
    *g.offset(109_isize) = zizj * *g.offset(105_isize)
        + *cpz.offset(1_isize) * *g.offset(113_isize)
        + *b00.offset(1_isize) * *g.offset(97_isize);
    *g.offset(110_isize) = zizj * *g.offset(106_isize)
        + *cpz.offset(2_isize) * *g.offset(114_isize)
        + *b00.offset(2_isize) * *g.offset(98_isize);
    *g.offset(111_isize) = zizj * *g.offset(107_isize)
        + *cpz.offset(3_isize) * *g.offset(115_isize)
        + *b00.offset(3_isize) * *g.offset(99_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1200(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    let xixj: f64 = (*envs).rirj[0_usize];
    let yiyj: f64 = (*envs).rirj[1_usize];
    let zizj: f64 = (*envs).rirj[2_usize];
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(8_isize) = *c0x.offset(0_isize);
    *g.offset(9_isize) = *c0x.offset(1_isize);
    *g.offset(10_isize) = *c0x.offset(2_isize);
    *g.offset(11_isize) = *c0x.offset(3_isize);
    *g.offset(16_isize) =
        *c0x.offset(0_isize) * *c0x.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(17_isize) =
        *c0x.offset(1_isize) * *c0x.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(18_isize) =
        *c0x.offset(2_isize) * *c0x.offset(2_isize) + *b10.offset(2_isize);
    *g.offset(19_isize) =
        *c0x.offset(3_isize) * *c0x.offset(3_isize) + *b10.offset(3_isize);
    *g.offset(20_isize) = *g.offset(16_isize) * (xixj + *c0x.offset(0_isize))
        + *c0x.offset(0_isize) * 2_f64 * *b10.offset(0_isize);
    *g.offset(21_isize) = *g.offset(17_isize) * (xixj + *c0x.offset(1_isize))
        + *c0x.offset(1_isize) * 2_f64 * *b10.offset(1_isize);
    *g.offset(22_isize) = *g.offset(18_isize) * (xixj + *c0x.offset(2_isize))
        + *c0x.offset(2_isize) * 2_f64 * *b10.offset(2_isize);
    *g.offset(23_isize) = *g.offset(19_isize) * (xixj + *c0x.offset(3_isize))
        + *c0x.offset(3_isize) * 2_f64 * *b10.offset(3_isize);
    *g.offset(12_isize) =
        *c0x.offset(0_isize) * (xixj + *c0x.offset(0_isize)) + *b10.offset(0_isize);
    *g.offset(13_isize) =
        *c0x.offset(1_isize) * (xixj + *c0x.offset(1_isize)) + *b10.offset(1_isize);
    *g.offset(14_isize) =
        *c0x.offset(2_isize) * (xixj + *c0x.offset(2_isize)) + *b10.offset(2_isize);
    *g.offset(15_isize) =
        *c0x.offset(3_isize) * (xixj + *c0x.offset(3_isize)) + *b10.offset(3_isize);
    *g.offset(4_isize) = xixj + *c0x.offset(0_isize);
    *g.offset(5_isize) = xixj + *c0x.offset(1_isize);
    *g.offset(6_isize) = xixj + *c0x.offset(2_isize);
    *g.offset(7_isize) = xixj + *c0x.offset(3_isize);
    *g.offset(32_isize) = 1_f64;
    *g.offset(33_isize) = 1_f64;
    *g.offset(34_isize) = 1_f64;
    *g.offset(35_isize) = 1_f64;
    *g.offset(40_isize) = *c0y.offset(0_isize);
    *g.offset(41_isize) = *c0y.offset(1_isize);
    *g.offset(42_isize) = *c0y.offset(2_isize);
    *g.offset(43_isize) = *c0y.offset(3_isize);
    *g.offset(48_isize) =
        *c0y.offset(0_isize) * *c0y.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(49_isize) =
        *c0y.offset(1_isize) * *c0y.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(50_isize) =
        *c0y.offset(2_isize) * *c0y.offset(2_isize) + *b10.offset(2_isize);
    *g.offset(51_isize) =
        *c0y.offset(3_isize) * *c0y.offset(3_isize) + *b10.offset(3_isize);
    *g.offset(52_isize) = *g.offset(48_isize) * (yiyj + *c0y.offset(0_isize))
        + *c0y.offset(0_isize) * 2_f64 * *b10.offset(0_isize);
    *g.offset(53_isize) = *g.offset(49_isize) * (yiyj + *c0y.offset(1_isize))
        + *c0y.offset(1_isize) * 2_f64 * *b10.offset(1_isize);
    *g.offset(54_isize) = *g.offset(50_isize) * (yiyj + *c0y.offset(2_isize))
        + *c0y.offset(2_isize) * 2_f64 * *b10.offset(2_isize);
    *g.offset(55_isize) = *g.offset(51_isize) * (yiyj + *c0y.offset(3_isize))
        + *c0y.offset(3_isize) * 2_f64 * *b10.offset(3_isize);
    *g.offset(44_isize) =
        *c0y.offset(0_isize) * (yiyj + *c0y.offset(0_isize)) + *b10.offset(0_isize);
    *g.offset(45_isize) =
        *c0y.offset(1_isize) * (yiyj + *c0y.offset(1_isize)) + *b10.offset(1_isize);
    *g.offset(46_isize) =
        *c0y.offset(2_isize) * (yiyj + *c0y.offset(2_isize)) + *b10.offset(2_isize);
    *g.offset(47_isize) =
        *c0y.offset(3_isize) * (yiyj + *c0y.offset(3_isize)) + *b10.offset(3_isize);
    *g.offset(36_isize) = yiyj + *c0y.offset(0_isize);
    *g.offset(37_isize) = yiyj + *c0y.offset(1_isize);
    *g.offset(38_isize) = yiyj + *c0y.offset(2_isize);
    *g.offset(39_isize) = yiyj + *c0y.offset(3_isize);
    *g.offset(72_isize) = *c0z.offset(0_isize) * *g.offset(64_isize);
    *g.offset(73_isize) = *c0z.offset(1_isize) * *g.offset(65_isize);
    *g.offset(74_isize) = *c0z.offset(2_isize) * *g.offset(66_isize);
    *g.offset(75_isize) = *c0z.offset(3_isize) * *g.offset(67_isize);
    *g.offset(80_isize) = *c0z.offset(0_isize) * *g.offset(72_isize)
        + *b10.offset(0_isize) * *g.offset(64_isize);
    *g.offset(81_isize) = *c0z.offset(1_isize) * *g.offset(73_isize)
        + *b10.offset(1_isize) * *g.offset(65_isize);
    *g.offset(82_isize) = *c0z.offset(2_isize) * *g.offset(74_isize)
        + *b10.offset(2_isize) * *g.offset(66_isize);
    *g.offset(83_isize) = *c0z.offset(3_isize) * *g.offset(75_isize)
        + *b10.offset(3_isize) * *g.offset(67_isize);
    *g.offset(84_isize) = *g.offset(80_isize) * (zizj + *c0z.offset(0_isize))
        + 2_f64 * *b10.offset(0_isize) * *g.offset(72_isize);
    *g.offset(85_isize) = *g.offset(81_isize) * (zizj + *c0z.offset(1_isize))
        + 2_f64 * *b10.offset(1_isize) * *g.offset(73_isize);
    *g.offset(86_isize) = *g.offset(82_isize) * (zizj + *c0z.offset(2_isize))
        + 2_f64 * *b10.offset(2_isize) * *g.offset(74_isize);
    *g.offset(87_isize) = *g.offset(83_isize) * (zizj + *c0z.offset(3_isize))
        + 2_f64 * *b10.offset(3_isize) * *g.offset(75_isize);
    *g.offset(76_isize) = *g.offset(72_isize) * (zizj + *c0z.offset(0_isize))
        + *b10.offset(0_isize) * *g.offset(64_isize);
    *g.offset(77_isize) = *g.offset(73_isize) * (zizj + *c0z.offset(1_isize))
        + *b10.offset(1_isize) * *g.offset(65_isize);
    *g.offset(78_isize) = *g.offset(74_isize) * (zizj + *c0z.offset(2_isize))
        + *b10.offset(2_isize) * *g.offset(66_isize);
    *g.offset(79_isize) = *g.offset(75_isize) * (zizj + *c0z.offset(3_isize))
        + *b10.offset(3_isize) * *g.offset(67_isize);
    *g.offset(68_isize) = *g.offset(64_isize) * (zizj + *c0z.offset(0_isize));
    *g.offset(69_isize) = *g.offset(65_isize) * (zizj + *c0z.offset(1_isize));
    *g.offset(70_isize) = *g.offset(66_isize) * (zizj + *c0z.offset(2_isize));
    *g.offset(71_isize) = *g.offset(67_isize) * (zizj + *c0z.offset(3_isize));
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_2000(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(4_isize) = *c0x.offset(0_isize);
    *g.offset(5_isize) = *c0x.offset(1_isize);
    *g.offset(6_isize) = *c0x.offset(2_isize);
    *g.offset(7_isize) = *c0x.offset(3_isize);
    *g.offset(8_isize) =
        *c0x.offset(0_isize) * *c0x.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(9_isize) =
        *c0x.offset(1_isize) * *c0x.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(10_isize) =
        *c0x.offset(2_isize) * *c0x.offset(2_isize) + *b10.offset(2_isize);
    *g.offset(11_isize) =
        *c0x.offset(3_isize) * *c0x.offset(3_isize) + *b10.offset(3_isize);
    *g.offset(12_isize) = 1_f64;
    *g.offset(13_isize) = 1_f64;
    *g.offset(14_isize) = 1_f64;
    *g.offset(15_isize) = 1_f64;
    *g.offset(16_isize) = *c0y.offset(0_isize);
    *g.offset(17_isize) = *c0y.offset(1_isize);
    *g.offset(18_isize) = *c0y.offset(2_isize);
    *g.offset(19_isize) = *c0y.offset(3_isize);
    *g.offset(20_isize) =
        *c0y.offset(0_isize) * *c0y.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(21_isize) =
        *c0y.offset(1_isize) * *c0y.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(22_isize) =
        *c0y.offset(2_isize) * *c0y.offset(2_isize) + *b10.offset(2_isize);
    *g.offset(23_isize) =
        *c0y.offset(3_isize) * *c0y.offset(3_isize) + *b10.offset(3_isize);
    *g.offset(28_isize) = *c0z.offset(0_isize) * *g.offset(24_isize);
    *g.offset(29_isize) = *c0z.offset(1_isize) * *g.offset(25_isize);
    *g.offset(30_isize) = *c0z.offset(2_isize) * *g.offset(26_isize);
    *g.offset(31_isize) = *c0z.offset(3_isize) * *g.offset(27_isize);
    *g.offset(32_isize) = *c0z.offset(0_isize) * *g.offset(28_isize)
        + *b10.offset(0_isize) * *g.offset(24_isize);
    *g.offset(33_isize) = *c0z.offset(1_isize) * *g.offset(29_isize)
        + *b10.offset(1_isize) * *g.offset(25_isize);
    *g.offset(34_isize) = *c0z.offset(2_isize) * *g.offset(30_isize)
        + *b10.offset(2_isize) * *g.offset(26_isize);
    *g.offset(35_isize) = *c0z.offset(3_isize) * *g.offset(31_isize)
        + *b10.offset(3_isize) * *g.offset(27_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_2001(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(4_isize) = *c0x.offset(0_isize);
    *g.offset(5_isize) = *c0x.offset(1_isize);
    *g.offset(6_isize) = *c0x.offset(2_isize);
    *g.offset(7_isize) = *c0x.offset(3_isize);
    *g.offset(8_isize) =
        *c0x.offset(0_isize) * *c0x.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(9_isize) =
        *c0x.offset(1_isize) * *c0x.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(10_isize) =
        *c0x.offset(2_isize) * *c0x.offset(2_isize) + *b10.offset(2_isize);
    *g.offset(11_isize) =
        *c0x.offset(3_isize) * *c0x.offset(3_isize) + *b10.offset(3_isize);
    *g.offset(12_isize) = *cpx.offset(0_isize);
    *g.offset(13_isize) = *cpx.offset(1_isize);
    *g.offset(14_isize) = *cpx.offset(2_isize);
    *g.offset(15_isize) = *cpx.offset(3_isize);
    *g.offset(16_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(17_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(18_isize) =
        *cpx.offset(2_isize) * *c0x.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(19_isize) =
        *cpx.offset(3_isize) * *c0x.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(20_isize) = *c0x.offset(0_isize)
        * (*g.offset(16_isize) + *b00.offset(0_isize))
        + *b10.offset(0_isize) * *cpx.offset(0_isize);
    *g.offset(21_isize) = *c0x.offset(1_isize)
        * (*g.offset(17_isize) + *b00.offset(1_isize))
        + *b10.offset(1_isize) * *cpx.offset(1_isize);
    *g.offset(22_isize) = *c0x.offset(2_isize)
        * (*g.offset(18_isize) + *b00.offset(2_isize))
        + *b10.offset(2_isize) * *cpx.offset(2_isize);
    *g.offset(23_isize) = *c0x.offset(3_isize)
        * (*g.offset(19_isize) + *b00.offset(3_isize))
        + *b10.offset(3_isize) * *cpx.offset(3_isize);
    *g.offset(24_isize) = 1_f64;
    *g.offset(25_isize) = 1_f64;
    *g.offset(26_isize) = 1_f64;
    *g.offset(27_isize) = 1_f64;
    *g.offset(28_isize) = *c0y.offset(0_isize);
    *g.offset(29_isize) = *c0y.offset(1_isize);
    *g.offset(30_isize) = *c0y.offset(2_isize);
    *g.offset(31_isize) = *c0y.offset(3_isize);
    *g.offset(32_isize) =
        *c0y.offset(0_isize) * *c0y.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(33_isize) =
        *c0y.offset(1_isize) * *c0y.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(34_isize) =
        *c0y.offset(2_isize) * *c0y.offset(2_isize) + *b10.offset(2_isize);
    *g.offset(35_isize) =
        *c0y.offset(3_isize) * *c0y.offset(3_isize) + *b10.offset(3_isize);
    *g.offset(36_isize) = *cpy.offset(0_isize);
    *g.offset(37_isize) = *cpy.offset(1_isize);
    *g.offset(38_isize) = *cpy.offset(2_isize);
    *g.offset(39_isize) = *cpy.offset(3_isize);
    *g.offset(40_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(41_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(42_isize) =
        *cpy.offset(2_isize) * *c0y.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(43_isize) =
        *cpy.offset(3_isize) * *c0y.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(44_isize) = *c0y.offset(0_isize)
        * (*g.offset(40_isize) + *b00.offset(0_isize))
        + *b10.offset(0_isize) * *cpy.offset(0_isize);
    *g.offset(45_isize) = *c0y.offset(1_isize)
        * (*g.offset(41_isize) + *b00.offset(1_isize))
        + *b10.offset(1_isize) * *cpy.offset(1_isize);
    *g.offset(46_isize) = *c0y.offset(2_isize)
        * (*g.offset(42_isize) + *b00.offset(2_isize))
        + *b10.offset(2_isize) * *cpy.offset(2_isize);
    *g.offset(47_isize) = *c0y.offset(3_isize)
        * (*g.offset(43_isize) + *b00.offset(3_isize))
        + *b10.offset(3_isize) * *cpy.offset(3_isize);
    *g.offset(52_isize) = *c0z.offset(0_isize) * *g.offset(48_isize);
    *g.offset(53_isize) = *c0z.offset(1_isize) * *g.offset(49_isize);
    *g.offset(54_isize) = *c0z.offset(2_isize) * *g.offset(50_isize);
    *g.offset(55_isize) = *c0z.offset(3_isize) * *g.offset(51_isize);
    *g.offset(56_isize) = *c0z.offset(0_isize) * *g.offset(52_isize)
        + *b10.offset(0_isize) * *g.offset(48_isize);
    *g.offset(57_isize) = *c0z.offset(1_isize) * *g.offset(53_isize)
        + *b10.offset(1_isize) * *g.offset(49_isize);
    *g.offset(58_isize) = *c0z.offset(2_isize) * *g.offset(54_isize)
        + *b10.offset(2_isize) * *g.offset(50_isize);
    *g.offset(59_isize) = *c0z.offset(3_isize) * *g.offset(55_isize)
        + *b10.offset(3_isize) * *g.offset(51_isize);
    *g.offset(60_isize) = *cpz.offset(0_isize) * *g.offset(48_isize);
    *g.offset(61_isize) = *cpz.offset(1_isize) * *g.offset(49_isize);
    *g.offset(62_isize) = *cpz.offset(2_isize) * *g.offset(50_isize);
    *g.offset(63_isize) = *cpz.offset(3_isize) * *g.offset(51_isize);
    *g.offset(64_isize) = *cpz.offset(0_isize) * *g.offset(52_isize)
        + *b00.offset(0_isize) * *g.offset(48_isize);
    *g.offset(65_isize) = *cpz.offset(1_isize) * *g.offset(53_isize)
        + *b00.offset(1_isize) * *g.offset(49_isize);
    *g.offset(66_isize) = *cpz.offset(2_isize) * *g.offset(54_isize)
        + *b00.offset(2_isize) * *g.offset(50_isize);
    *g.offset(67_isize) = *cpz.offset(3_isize) * *g.offset(55_isize)
        + *b00.offset(3_isize) * *g.offset(51_isize);
    *g.offset(68_isize) = *c0z.offset(0_isize) * *g.offset(64_isize)
        + *b10.offset(0_isize) * *g.offset(60_isize)
        + *b00.offset(0_isize) * *g.offset(52_isize);
    *g.offset(69_isize) = *c0z.offset(1_isize) * *g.offset(65_isize)
        + *b10.offset(1_isize) * *g.offset(61_isize)
        + *b00.offset(1_isize) * *g.offset(53_isize);
    *g.offset(70_isize) = *c0z.offset(2_isize) * *g.offset(66_isize)
        + *b10.offset(2_isize) * *g.offset(62_isize)
        + *b00.offset(2_isize) * *g.offset(54_isize);
    *g.offset(71_isize) = *c0z.offset(3_isize) * *g.offset(67_isize)
        + *b10.offset(3_isize) * *g.offset(63_isize)
        + *b00.offset(3_isize) * *g.offset(55_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_2010(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(4_isize) = *c0x.offset(0_isize);
    *g.offset(5_isize) = *c0x.offset(1_isize);
    *g.offset(6_isize) = *c0x.offset(2_isize);
    *g.offset(7_isize) = *c0x.offset(3_isize);
    *g.offset(8_isize) =
        *c0x.offset(0_isize) * *c0x.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(9_isize) =
        *c0x.offset(1_isize) * *c0x.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(10_isize) =
        *c0x.offset(2_isize) * *c0x.offset(2_isize) + *b10.offset(2_isize);
    *g.offset(11_isize) =
        *c0x.offset(3_isize) * *c0x.offset(3_isize) + *b10.offset(3_isize);
    *g.offset(12_isize) = *cpx.offset(0_isize);
    *g.offset(13_isize) = *cpx.offset(1_isize);
    *g.offset(14_isize) = *cpx.offset(2_isize);
    *g.offset(15_isize) = *cpx.offset(3_isize);
    *g.offset(16_isize) =
        *cpx.offset(0_isize) * *c0x.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(17_isize) =
        *cpx.offset(1_isize) * *c0x.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(18_isize) =
        *cpx.offset(2_isize) * *c0x.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(19_isize) =
        *cpx.offset(3_isize) * *c0x.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(20_isize) = *c0x.offset(0_isize)
        * (*g.offset(16_isize) + *b00.offset(0_isize))
        + *b10.offset(0_isize) * *cpx.offset(0_isize);
    *g.offset(21_isize) = *c0x.offset(1_isize)
        * (*g.offset(17_isize) + *b00.offset(1_isize))
        + *b10.offset(1_isize) * *cpx.offset(1_isize);
    *g.offset(22_isize) = *c0x.offset(2_isize)
        * (*g.offset(18_isize) + *b00.offset(2_isize))
        + *b10.offset(2_isize) * *cpx.offset(2_isize);
    *g.offset(23_isize) = *c0x.offset(3_isize)
        * (*g.offset(19_isize) + *b00.offset(3_isize))
        + *b10.offset(3_isize) * *cpx.offset(3_isize);
    *g.offset(24_isize) = 1_f64;
    *g.offset(25_isize) = 1_f64;
    *g.offset(26_isize) = 1_f64;
    *g.offset(27_isize) = 1_f64;
    *g.offset(28_isize) = *c0y.offset(0_isize);
    *g.offset(29_isize) = *c0y.offset(1_isize);
    *g.offset(30_isize) = *c0y.offset(2_isize);
    *g.offset(31_isize) = *c0y.offset(3_isize);
    *g.offset(32_isize) =
        *c0y.offset(0_isize) * *c0y.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(33_isize) =
        *c0y.offset(1_isize) * *c0y.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(34_isize) =
        *c0y.offset(2_isize) * *c0y.offset(2_isize) + *b10.offset(2_isize);
    *g.offset(35_isize) =
        *c0y.offset(3_isize) * *c0y.offset(3_isize) + *b10.offset(3_isize);
    *g.offset(36_isize) = *cpy.offset(0_isize);
    *g.offset(37_isize) = *cpy.offset(1_isize);
    *g.offset(38_isize) = *cpy.offset(2_isize);
    *g.offset(39_isize) = *cpy.offset(3_isize);
    *g.offset(40_isize) =
        *cpy.offset(0_isize) * *c0y.offset(0_isize) + *b00.offset(0_isize);
    *g.offset(41_isize) =
        *cpy.offset(1_isize) * *c0y.offset(1_isize) + *b00.offset(1_isize);
    *g.offset(42_isize) =
        *cpy.offset(2_isize) * *c0y.offset(2_isize) + *b00.offset(2_isize);
    *g.offset(43_isize) =
        *cpy.offset(3_isize) * *c0y.offset(3_isize) + *b00.offset(3_isize);
    *g.offset(44_isize) = *c0y.offset(0_isize)
        * (*g.offset(40_isize) + *b00.offset(0_isize))
        + *b10.offset(0_isize) * *cpy.offset(0_isize);
    *g.offset(45_isize) = *c0y.offset(1_isize)
        * (*g.offset(41_isize) + *b00.offset(1_isize))
        + *b10.offset(1_isize) * *cpy.offset(1_isize);
    *g.offset(46_isize) = *c0y.offset(2_isize)
        * (*g.offset(42_isize) + *b00.offset(2_isize))
        + *b10.offset(2_isize) * *cpy.offset(2_isize);
    *g.offset(47_isize) = *c0y.offset(3_isize)
        * (*g.offset(43_isize) + *b00.offset(3_isize))
        + *b10.offset(3_isize) * *cpy.offset(3_isize);
    *g.offset(52_isize) = *c0z.offset(0_isize) * *g.offset(48_isize);
    *g.offset(53_isize) = *c0z.offset(1_isize) * *g.offset(49_isize);
    *g.offset(54_isize) = *c0z.offset(2_isize) * *g.offset(50_isize);
    *g.offset(55_isize) = *c0z.offset(3_isize) * *g.offset(51_isize);
    *g.offset(56_isize) = *c0z.offset(0_isize) * *g.offset(52_isize)
        + *b10.offset(0_isize) * *g.offset(48_isize);
    *g.offset(57_isize) = *c0z.offset(1_isize) * *g.offset(53_isize)
        + *b10.offset(1_isize) * *g.offset(49_isize);
    *g.offset(58_isize) = *c0z.offset(2_isize) * *g.offset(54_isize)
        + *b10.offset(2_isize) * *g.offset(50_isize);
    *g.offset(59_isize) = *c0z.offset(3_isize) * *g.offset(55_isize)
        + *b10.offset(3_isize) * *g.offset(51_isize);
    *g.offset(60_isize) = *cpz.offset(0_isize) * *g.offset(48_isize);
    *g.offset(61_isize) = *cpz.offset(1_isize) * *g.offset(49_isize);
    *g.offset(62_isize) = *cpz.offset(2_isize) * *g.offset(50_isize);
    *g.offset(63_isize) = *cpz.offset(3_isize) * *g.offset(51_isize);
    *g.offset(64_isize) = *cpz.offset(0_isize) * *g.offset(52_isize)
        + *b00.offset(0_isize) * *g.offset(48_isize);
    *g.offset(65_isize) = *cpz.offset(1_isize) * *g.offset(53_isize)
        + *b00.offset(1_isize) * *g.offset(49_isize);
    *g.offset(66_isize) = *cpz.offset(2_isize) * *g.offset(54_isize)
        + *b00.offset(2_isize) * *g.offset(50_isize);
    *g.offset(67_isize) = *cpz.offset(3_isize) * *g.offset(55_isize)
        + *b00.offset(3_isize) * *g.offset(51_isize);
    *g.offset(68_isize) = *c0z.offset(0_isize) * *g.offset(64_isize)
        + *b10.offset(0_isize) * *g.offset(60_isize)
        + *b00.offset(0_isize) * *g.offset(52_isize);
    *g.offset(69_isize) = *c0z.offset(1_isize) * *g.offset(65_isize)
        + *b10.offset(1_isize) * *g.offset(61_isize)
        + *b00.offset(1_isize) * *g.offset(53_isize);
    *g.offset(70_isize) = *c0z.offset(2_isize) * *g.offset(66_isize)
        + *b10.offset(2_isize) * *g.offset(62_isize)
        + *b00.offset(2_isize) * *g.offset(54_isize);
    *g.offset(71_isize) = *c0z.offset(3_isize) * *g.offset(67_isize)
        + *b10.offset(3_isize) * *g.offset(63_isize)
        + *b00.offset(3_isize) * *g.offset(55_isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_2100(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    let xixj: f64 = (*envs).rirj[0_usize];
    let yiyj: f64 = (*envs).rirj[1_usize];
    let zizj: f64 = (*envs).rirj[2_usize];
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(4_isize) = *c0x.offset(0_isize);
    *g.offset(5_isize) = *c0x.offset(1_isize);
    *g.offset(6_isize) = *c0x.offset(2_isize);
    *g.offset(7_isize) = *c0x.offset(3_isize);
    *g.offset(8_isize) =
        *c0x.offset(0_isize) * *c0x.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(9_isize) =
        *c0x.offset(1_isize) * *c0x.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(10_isize) =
        *c0x.offset(2_isize) * *c0x.offset(2_isize) + *b10.offset(2_isize);
    *g.offset(11_isize) =
        *c0x.offset(3_isize) * *c0x.offset(3_isize) + *b10.offset(3_isize);
    *g.offset(24_isize) = *g.offset(8_isize) * (xixj + *c0x.offset(0_isize))
        + *c0x.offset(0_isize) * 2_f64 * *b10.offset(0_isize);
    *g.offset(25_isize) = *g.offset(9_isize) * (xixj + *c0x.offset(1_isize))
        + *c0x.offset(1_isize) * 2_f64 * *b10.offset(1_isize);
    *g.offset(26_isize) = *g.offset(10_isize) * (xixj + *c0x.offset(2_isize))
        + *c0x.offset(2_isize) * 2_f64 * *b10.offset(2_isize);
    *g.offset(27_isize) = *g.offset(11_isize) * (xixj + *c0x.offset(3_isize))
        + *c0x.offset(3_isize) * 2_f64 * *b10.offset(3_isize);
    *g.offset(20_isize) =
        *c0x.offset(0_isize) * (xixj + *c0x.offset(0_isize)) + *b10.offset(0_isize);
    *g.offset(21_isize) =
        *c0x.offset(1_isize) * (xixj + *c0x.offset(1_isize)) + *b10.offset(1_isize);
    *g.offset(22_isize) =
        *c0x.offset(2_isize) * (xixj + *c0x.offset(2_isize)) + *b10.offset(2_isize);
    *g.offset(23_isize) =
        *c0x.offset(3_isize) * (xixj + *c0x.offset(3_isize)) + *b10.offset(3_isize);
    *g.offset(16_isize) = xixj + *c0x.offset(0_isize);
    *g.offset(17_isize) = xixj + *c0x.offset(1_isize);
    *g.offset(18_isize) = xixj + *c0x.offset(2_isize);
    *g.offset(19_isize) = xixj + *c0x.offset(3_isize);
    *g.offset(32_isize) = 1_f64;
    *g.offset(33_isize) = 1_f64;
    *g.offset(34_isize) = 1_f64;
    *g.offset(35_isize) = 1_f64;
    *g.offset(36_isize) = *c0y.offset(0_isize);
    *g.offset(37_isize) = *c0y.offset(1_isize);
    *g.offset(38_isize) = *c0y.offset(2_isize);
    *g.offset(39_isize) = *c0y.offset(3_isize);
    *g.offset(40_isize) =
        *c0y.offset(0_isize) * *c0y.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(41_isize) =
        *c0y.offset(1_isize) * *c0y.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(42_isize) =
        *c0y.offset(2_isize) * *c0y.offset(2_isize) + *b10.offset(2_isize);
    *g.offset(43_isize) =
        *c0y.offset(3_isize) * *c0y.offset(3_isize) + *b10.offset(3_isize);
    *g.offset(56_isize) = *g.offset(40_isize) * (yiyj + *c0y.offset(0_isize))
        + *c0y.offset(0_isize) * 2_f64 * *b10.offset(0_isize);
    *g.offset(57_isize) = *g.offset(41_isize) * (yiyj + *c0y.offset(1_isize))
        + *c0y.offset(1_isize) * 2_f64 * *b10.offset(1_isize);
    *g.offset(58_isize) = *g.offset(42_isize) * (yiyj + *c0y.offset(2_isize))
        + *c0y.offset(2_isize) * 2_f64 * *b10.offset(2_isize);
    *g.offset(59_isize) = *g.offset(43_isize) * (yiyj + *c0y.offset(3_isize))
        + *c0y.offset(3_isize) * 2_f64 * *b10.offset(3_isize);
    *g.offset(52_isize) =
        *c0y.offset(0_isize) * (yiyj + *c0y.offset(0_isize)) + *b10.offset(0_isize);
    *g.offset(53_isize) =
        *c0y.offset(1_isize) * (yiyj + *c0y.offset(1_isize)) + *b10.offset(1_isize);
    *g.offset(54_isize) =
        *c0y.offset(2_isize) * (yiyj + *c0y.offset(2_isize)) + *b10.offset(2_isize);
    *g.offset(55_isize) =
        *c0y.offset(3_isize) * (yiyj + *c0y.offset(3_isize)) + *b10.offset(3_isize);
    *g.offset(48_isize) = yiyj + *c0y.offset(0_isize);
    *g.offset(49_isize) = yiyj + *c0y.offset(1_isize);
    *g.offset(50_isize) = yiyj + *c0y.offset(2_isize);
    *g.offset(51_isize) = yiyj + *c0y.offset(3_isize);
    *g.offset(68_isize) = *c0z.offset(0_isize) * *g.offset(64_isize);
    *g.offset(69_isize) = *c0z.offset(1_isize) * *g.offset(65_isize);
    *g.offset(70_isize) = *c0z.offset(2_isize) * *g.offset(66_isize);
    *g.offset(71_isize) = *c0z.offset(3_isize) * *g.offset(67_isize);
    *g.offset(72_isize) = *c0z.offset(0_isize) * *g.offset(68_isize)
        + *b10.offset(0_isize) * *g.offset(64_isize);
    *g.offset(73_isize) = *c0z.offset(1_isize) * *g.offset(69_isize)
        + *b10.offset(1_isize) * *g.offset(65_isize);
    *g.offset(74_isize) = *c0z.offset(2_isize) * *g.offset(70_isize)
        + *b10.offset(2_isize) * *g.offset(66_isize);
    *g.offset(75_isize) = *c0z.offset(3_isize) * *g.offset(71_isize)
        + *b10.offset(3_isize) * *g.offset(67_isize);
    *g.offset(88_isize) = *g.offset(72_isize) * (zizj + *c0z.offset(0_isize))
        + 2_f64 * *b10.offset(0_isize) * *g.offset(68_isize);
    *g.offset(89_isize) = *g.offset(73_isize) * (zizj + *c0z.offset(1_isize))
        + 2_f64 * *b10.offset(1_isize) * *g.offset(69_isize);
    *g.offset(90_isize) = *g.offset(74_isize) * (zizj + *c0z.offset(2_isize))
        + 2_f64 * *b10.offset(2_isize) * *g.offset(70_isize);
    *g.offset(91_isize) = *g.offset(75_isize) * (zizj + *c0z.offset(3_isize))
        + 2_f64 * *b10.offset(3_isize) * *g.offset(71_isize);
    *g.offset(84_isize) = *g.offset(68_isize) * (zizj + *c0z.offset(0_isize))
        + *b10.offset(0_isize) * *g.offset(64_isize);
    *g.offset(85_isize) = *g.offset(69_isize) * (zizj + *c0z.offset(1_isize))
        + *b10.offset(1_isize) * *g.offset(65_isize);
    *g.offset(86_isize) = *g.offset(70_isize) * (zizj + *c0z.offset(2_isize))
        + *b10.offset(2_isize) * *g.offset(66_isize);
    *g.offset(87_isize) = *g.offset(71_isize) * (zizj + *c0z.offset(3_isize))
        + *b10.offset(3_isize) * *g.offset(67_isize);
    *g.offset(80_isize) = *g.offset(64_isize) * (zizj + *c0z.offset(0_isize));
    *g.offset(81_isize) = *g.offset(65_isize) * (zizj + *c0z.offset(1_isize));
    *g.offset(82_isize) = *g.offset(66_isize) * (zizj + *c0z.offset(2_isize));
    *g.offset(83_isize) = *g.offset(67_isize) * (zizj + *c0z.offset(3_isize));
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_3000(
    g: *mut f64,
    bc: *mut Rys2eT,
    _envs: *mut CINTEnvVars,
) {
    let c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0_isize) = 1_f64;
    *g.offset(1_isize) = 1_f64;
    *g.offset(2_isize) = 1_f64;
    *g.offset(3_isize) = 1_f64;
    *g.offset(4_isize) = *c0x.offset(0_isize);
    *g.offset(5_isize) = *c0x.offset(1_isize);
    *g.offset(6_isize) = *c0x.offset(2_isize);
    *g.offset(7_isize) = *c0x.offset(3_isize);
    *g.offset(8_isize) =
        *c0x.offset(0_isize) * *c0x.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(9_isize) =
        *c0x.offset(1_isize) * *c0x.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(10_isize) =
        *c0x.offset(2_isize) * *c0x.offset(2_isize) + *b10.offset(2_isize);
    *g.offset(11_isize) =
        *c0x.offset(3_isize) * *c0x.offset(3_isize) + *b10.offset(3_isize);
    *g.offset(12_isize) =
        *c0x.offset(0_isize) * (*g.offset(8_isize) + 2_f64 * *b10.offset(0_isize));
    *g.offset(13_isize) =
        *c0x.offset(1_isize) * (*g.offset(9_isize) + 2_f64 * *b10.offset(1_isize));
    *g.offset(14_isize) =
        *c0x.offset(2_isize) * (*g.offset(10_isize) + 2_f64 * *b10.offset(2_isize));
    *g.offset(15_isize) =
        *c0x.offset(3_isize) * (*g.offset(11_isize) + 2_f64 * *b10.offset(3_isize));
    *g.offset(16_isize) = 1_f64;
    *g.offset(17_isize) = 1_f64;
    *g.offset(18_isize) = 1_f64;
    *g.offset(19_isize) = 1_f64;
    *g.offset(20_isize) = *c0y.offset(0_isize);
    *g.offset(21_isize) = *c0y.offset(1_isize);
    *g.offset(22_isize) = *c0y.offset(2_isize);
    *g.offset(23_isize) = *c0y.offset(3_isize);
    *g.offset(24_isize) =
        *c0y.offset(0_isize) * *c0y.offset(0_isize) + *b10.offset(0_isize);
    *g.offset(25_isize) =
        *c0y.offset(1_isize) * *c0y.offset(1_isize) + *b10.offset(1_isize);
    *g.offset(26_isize) =
        *c0y.offset(2_isize) * *c0y.offset(2_isize) + *b10.offset(2_isize);
    *g.offset(27_isize) =
        *c0y.offset(3_isize) * *c0y.offset(3_isize) + *b10.offset(3_isize);
    *g.offset(28_isize) =
        *c0y.offset(0_isize) * (*g.offset(24_isize) + 2_f64 * *b10.offset(0_isize));
    *g.offset(29_isize) =
        *c0y.offset(1_isize) * (*g.offset(25_isize) + 2_f64 * *b10.offset(1_isize));
    *g.offset(30_isize) =
        *c0y.offset(2_isize) * (*g.offset(26_isize) + 2_f64 * *b10.offset(2_isize));
    *g.offset(31_isize) =
        *c0y.offset(3_isize) * (*g.offset(27_isize) + 2_f64 * *b10.offset(3_isize));
    *g.offset(36_isize) = *c0z.offset(0_isize) * *g.offset(32_isize);
    *g.offset(37_isize) = *c0z.offset(1_isize) * *g.offset(33_isize);
    *g.offset(38_isize) = *c0z.offset(2_isize) * *g.offset(34_isize);
    *g.offset(39_isize) = *c0z.offset(3_isize) * *g.offset(35_isize);
    *g.offset(40_isize) = *c0z.offset(0_isize) * *g.offset(36_isize)
        + *b10.offset(0_isize) * *g.offset(32_isize);
    *g.offset(41_isize) = *c0z.offset(1_isize) * *g.offset(37_isize)
        + *b10.offset(1_isize) * *g.offset(33_isize);
    *g.offset(42_isize) = *c0z.offset(2_isize) * *g.offset(38_isize)
        + *b10.offset(2_isize) * *g.offset(34_isize);
    *g.offset(43_isize) = *c0z.offset(3_isize) * *g.offset(39_isize)
        + *b10.offset(3_isize) * *g.offset(35_isize);
    *g.offset(44_isize) = *c0z.offset(0_isize) * *g.offset(40_isize)
        + 2_f64 * *b10.offset(0_isize) * *g.offset(36_isize);
    *g.offset(45_isize) = *c0z.offset(1_isize) * *g.offset(41_isize)
        + 2_f64 * *b10.offset(1_isize) * *g.offset(37_isize);
    *g.offset(46_isize) = *c0z.offset(2_isize) * *g.offset(42_isize)
        + 2_f64 * *b10.offset(2_isize) * *g.offset(38_isize);
    *g.offset(47_isize) = *c0z.offset(3_isize) * *g.offset(43_isize)
        + 2_f64 * *b10.offset(3_isize) * *g.offset(39_isize);
}
#[no_mangle]
pub unsafe extern "C" fn CINTsrg0_2e_2d4d_unrolled(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    let type_ijkl: i32 = (*envs).li_ceil << 6_i32
        | (*envs).lj_ceil << 4_i32
        | (*envs).lk_ceil << 2_i32
        | (*envs).ll_ceil;
    match type_ijkl {
        0 => {
            _srg0_2d4d_0000(g, bc, envs);
            return;
        }
        1 => {
            _srg0_2d4d_0001(g, bc, envs);
            return;
        }
        2 => {
            _srg0_2d4d_0002(g, bc, envs);
            return;
        }
        3 => {
            _srg0_2d4d_0003(g, bc, envs);
            return;
        }
        4 => {
            _srg0_2d4d_0010(g, bc, envs);
            return;
        }
        5 => {
            _srg0_2d4d_0011(g, bc, envs);
            return;
        }
        6 => {
            _srg0_2d4d_0012(g, bc, envs);
            return;
        }
        8 => {
            _srg0_2d4d_0020(g, bc, envs);
            return;
        }
        9 => {
            _srg0_2d4d_0021(g, bc, envs);
            return;
        }
        12 => {
            _srg0_2d4d_0030(g, bc, envs);
            return;
        }
        16 => {
            _srg0_2d4d_0100(g, bc, envs);
            return;
        }
        17 => {
            _srg0_2d4d_0101(g, bc, envs);
            return;
        }
        18 => {
            _srg0_2d4d_0102(g, bc, envs);
            return;
        }
        20 => {
            _srg0_2d4d_0110(g, bc, envs);
            return;
        }
        21 => {
            _srg0_2d4d_0111(g, bc, envs);
            return;
        }
        24 => {
            _srg0_2d4d_0120(g, bc, envs);
            return;
        }
        32 => {
            _srg0_2d4d_0200(g, bc, envs);
            return;
        }
        33 => {
            _srg0_2d4d_0201(g, bc, envs);
            return;
        }
        36 => {
            _srg0_2d4d_0210(g, bc, envs);
            return;
        }
        48 => {
            _srg0_2d4d_0300(g, bc, envs);
            return;
        }
        64 => {
            _srg0_2d4d_1000(g, bc, envs);
            return;
        }
        65 => {
            _srg0_2d4d_1001(g, bc, envs);
            return;
        }
        66 => {
            _srg0_2d4d_1002(g, bc, envs);
            return;
        }
        68 => {
            _srg0_2d4d_1010(g, bc, envs);
            return;
        }
        69 => {
            _srg0_2d4d_1011(g, bc, envs);
            return;
        }
        72 => {
            _srg0_2d4d_1020(g, bc, envs);
            return;
        }
        80 => {
            _srg0_2d4d_1100(g, bc, envs);
            return;
        }
        81 => {
            _srg0_2d4d_1101(g, bc, envs);
            return;
        }
        84 => {
            _srg0_2d4d_1110(g, bc, envs);
            return;
        }
        96 => {
            _srg0_2d4d_1200(g, bc, envs);
            return;
        }
        128 => {
            _srg0_2d4d_2000(g, bc, envs);
            return;
        }
        129 => {
            _srg0_2d4d_2001(g, bc, envs);
            return;
        }
        132 => {
            _srg0_2d4d_2010(g, bc, envs);
            return;
        }
        144 => {
            _srg0_2d4d_2100(g, bc, envs);
            return;
        }
        192 => {
            _srg0_2d4d_3000(g, bc, envs);
            return;
        }
        _ => {}
    }
    eprintln!(
        "Dimension error for CINTg0_2e_lj2d4d: iklj = {} {} {} {}",
        (*envs).li_ceil,
        (*envs).lk_ceil,
        (*envs).ll_ceil,
        (*envs).lj_ceil
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e_lj2d4d(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    CINTg0_2e_2d(g, bc, envs);
    CINTg0_lj2d_4d(g, envs);
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e_kj2d4d(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    CINTg0_2e_2d(g, bc, envs);
    CINTg0_kj2d_4d(g, envs);
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e_ik2d4d(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    CINTg0_2e_2d(g, bc, envs);
    CINTg0_ik2d_4d(g, envs);
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e_il2d4d(
    g: *mut f64,
    bc: *mut Rys2eT,
    envs: *mut CINTEnvVars,
) {
    CINTg0_2e_2d(g, bc, envs);
    CINTg0_il2d_4d(g, envs);
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e(
    g: *mut f64,
    rij: *mut f64,
    rkl: *mut f64,
    cutoff: f64,
    envs: *mut CINTEnvVars,
) -> i32 {
    let mut irys: i32 = 0;
    let nroots: i32 = (*envs).nrys_roots;
    let aij: f64 = (*envs).ai[0_usize] + (*envs).aj[0_usize];
    let akl: f64 = (*envs).ak[0_usize] + (*envs).al[0_usize];
    let mut a0: f64 = 0.;
    let mut a1: f64 = 0.;
    let mut fac1: f64 = 0.;
    let mut x: f64 = 0.;
    let mut u: [f64; 32] = [0.; 32];
    let w: *mut f64 = g.offset(((*envs).g_size * 2_i32) as isize);
    let xij_kl: f64 = *rij.offset(0_isize) - *rkl.offset(0_isize);
    let yij_kl: f64 = *rij.offset(1_isize) - *rkl.offset(1_isize);
    let zij_kl: f64 = *rij.offset(2_isize) - *rkl.offset(2_isize);
    let rr: f64 = xij_kl * xij_kl + yij_kl * yij_kl + zij_kl * zij_kl;
    a1 = aij * akl;
    a0 = a1 / (aij + akl);
    fac1 = (a0 / (a1 * a1 * a1)).sqrt() * (*envs).fac[0_usize];
    x = a0 * rr;
    let omega: f64 = *((*envs).env).offset(8_isize);
    let mut theta: f64 = 0 as f64;
    if omega == 0.0f64 {
        CINTrys_roots(nroots, x, u.as_mut_ptr(), w);
    } else if omega < 0.0f64 {
        theta = omega * omega / (omega * omega + a0);
        if theta * x > cutoff || theta * x > 40_f64 {
            return 0_i32;
        }
        let rorder: i32 = (*envs).rys_order;
        if rorder == nroots {
            CINTsr_rys_roots(nroots, x, (theta).sqrt(), u.as_mut_ptr(), w);
        } else {
            let sqrt_theta: f64 = -(theta).sqrt();
            CINTrys_roots(rorder, x, u.as_mut_ptr(), w);
            CINTrys_roots(
                rorder,
                theta * x,
                u.as_mut_ptr().offset(rorder as isize),
                w.offset(rorder as isize),
            );
            if (*envs).g_size == 2_i32 {
                *g.offset(0_isize) = 1_f64;
                *g.offset(1_isize) = 1_f64;
                *g.offset(2_isize) = 1_f64;
                *g.offset(3_isize) = 1_f64;
                *g.offset(4_isize) *= fac1;
                *g.offset(5_isize) *= fac1 * sqrt_theta;
                return 1_i32;
            }
            irys = rorder;
            while irys < nroots {
                let ut: f64 = u[irys as usize] * theta;
                u[irys as usize] = ut / (u[irys as usize] + 1.0f64 - ut);
                *w.offset(irys as isize) *= sqrt_theta;
                irys += 1;
                irys;
            }
        }
    } else {
        theta = omega * omega / (omega * omega + a0);
        x *= theta;
        fac1 *= (theta).sqrt();
        CINTrys_roots(nroots, x, u.as_mut_ptr(), w);
        irys = 0_i32;
        while irys < nroots {
            let ut_0: f64 = u[irys as usize] * theta;
            u[irys as usize] = ut_0 / (u[irys as usize] + 1.0f64 - ut_0);
            irys += 1;
            irys;
        }
    }
    if (*envs).g_size == 1_i32 {
        *g.offset(0_isize) = 1_f64;
        *g.offset(1_isize) = 1_f64;
        *g.offset(2_isize) *= fac1;
        return 1_i32;
    }
    let mut u2: f64 = 0.;
    let mut tmp1: f64 = 0.;
    let mut tmp2: f64 = 0.;
    let mut tmp3: f64 = 0.;
    let mut tmp4: f64 = 0.;
    let mut tmp5: f64 = 0.;
    let rijrx: f64 = *rij.offset(0_isize) - *((*envs).rx_in_rijrx).offset(0_isize);
    let rijry: f64 = *rij.offset(1_isize) - *((*envs).rx_in_rijrx).offset(1_isize);
    let rijrz: f64 = *rij.offset(2_isize) - *((*envs).rx_in_rijrx).offset(2_isize);
    let rklrx: f64 = *rkl.offset(0_isize) - *((*envs).rx_in_rklrx).offset(0_isize);
    let rklry: f64 = *rkl.offset(1_isize) - *((*envs).rx_in_rklrx).offset(1_isize);
    let rklrz: f64 = *rkl.offset(2_isize) - *((*envs).rx_in_rklrx).offset(2_isize);
    let mut bc: Rys2eT = Rys2eT {
        c00x: [0.; 32],
        c00y: [0.; 32],
        c00z: [0.; 32],
        c0px: [0.; 32],
        c0py: [0.; 32],
        c0pz: [0.; 32],
        b01: [0.; 32],
        b00: [0.; 32],
        b10: [0.; 32],
    };
    let b00: *mut f64 = (bc.b00).as_mut_ptr();
    let b10: *mut f64 = (bc.b10).as_mut_ptr();
    let b01: *mut f64 = (bc.b01).as_mut_ptr();
    let c00x: *mut f64 = (bc.c00x).as_mut_ptr();
    let c00y: *mut f64 = (bc.c00y).as_mut_ptr();
    let c00z: *mut f64 = (bc.c00z).as_mut_ptr();
    let c0px: *mut f64 = (bc.c0px).as_mut_ptr();
    let c0py: *mut f64 = (bc.c0py).as_mut_ptr();
    let c0pz: *mut f64 = (bc.c0pz).as_mut_ptr();
    irys = 0_i32;
    while irys < nroots {
        u2 = a0 * u[irys as usize];
        tmp4 = 0.5f64 / (u2 * (aij + akl) + a1);
        tmp5 = u2 * tmp4;
        tmp1 = 2.0f64 * tmp5;
        tmp2 = tmp1 * akl;
        tmp3 = tmp1 * aij;
        *b00.offset(irys as isize) = tmp5;
        *b10.offset(irys as isize) = tmp5 + tmp4 * akl;
        *b01.offset(irys as isize) = tmp5 + tmp4 * aij;
        *c00x.offset(irys as isize) = rijrx - tmp2 * xij_kl;
        *c00y.offset(irys as isize) = rijry - tmp2 * yij_kl;
        *c00z.offset(irys as isize) = rijrz - tmp2 * zij_kl;
        *c0px.offset(irys as isize) = rklrx + tmp3 * xij_kl;
        *c0py.offset(irys as isize) = rklry + tmp3 * yij_kl;
        *c0pz.offset(irys as isize) = rklrz + tmp3 * zij_kl;
        *w.offset(irys as isize) *= fac1;
        irys += 1;
        irys;
    }
    ::core::mem::transmute::<_, fn(_, _, _)>(
        (Some(((*envs).f_g0_2d4d).expect("non-null function pointer")))
            .expect("non-null function pointer"),
    )(g, &mut bc, envs);
    1_i32
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1i_2e(
    f: *mut f64,
    g: *const f64,
    li: i32,
    lj: i32,
    lk: i32,
    ll: i32,
    envs: *const CINTEnvVars,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut n: i32 = 0;
    let mut ptr: i32 = 0;
    let di: i32 = (*envs).g_stride_i;
    let dk: i32 = (*envs).g_stride_k;
    let dl: i32 = (*envs).g_stride_l;
    let dj: i32 = (*envs).g_stride_j;
    let nroots: i32 = (*envs).nrys_roots;
    let ai2: f64 = -2_f64 * (*envs).ai[0_usize];
    let gx: *const f64 = g;
    let gy: *const f64 = g.offset((*envs).g_size as isize);
    let gz: *const f64 = g.offset(((*envs).g_size * 2_i32) as isize);
    let fx: *mut f64 = f;
    let fy: *mut f64 = f.offset((*envs).g_size as isize);
    let fz: *mut f64 = f.offset(((*envs).g_size * 2_i32) as isize);
    let p1x: *const f64 = gx.offset(-(di as isize));
    let p1y: *const f64 = gy.offset(-(di as isize));
    let p1z: *const f64 = gz.offset(-(di as isize));
    let p2x: *const f64 = gx.offset(di as isize);
    let p2y: *const f64 = gy.offset(di as isize);
    let p2z: *const f64 = gz.offset(di as isize);
    j = 0_i32;
    while j <= lj {
        l = 0_i32;
        while l <= ll {
            k = 0_i32;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                n = ptr;
                while n < ptr + nroots {
                    *fx.offset(n as isize) = ai2 * *p2x.offset(n as isize);
                    *fy.offset(n as isize) = ai2 * *p2y.offset(n as isize);
                    *fz.offset(n as isize) = ai2 * *p2z.offset(n as isize);
                    n += 1;
                    n;
                }
                ptr += di;
                i = 1_i32;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx.offset(n as isize) =
                            i as f64 * *p1x.offset(n as isize) + ai2 * *p2x.offset(n as isize);
                        *fy.offset(n as isize) =
                            i as f64 * *p1y.offset(n as isize) + ai2 * *p2y.offset(n as isize);
                        *fz.offset(n as isize) =
                            i as f64 * *p1z.offset(n as isize) + ai2 * *p2z.offset(n as isize);
                        n += 1;
                        n;
                    }
                    ptr += di;
                    i += 1;
                    i;
                }
                k += 1;
                k;
            }
            l += 1;
            l;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1j_2e(
    f: *mut f64,
    g: *const f64,
    li: i32,
    lj: i32,
    lk: i32,
    ll: i32,
    envs: *const CINTEnvVars,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut n: i32 = 0;
    let mut ptr: i32 = 0;
    let di: i32 = (*envs).g_stride_i;
    let dk: i32 = (*envs).g_stride_k;
    let dl: i32 = (*envs).g_stride_l;
    let dj: i32 = (*envs).g_stride_j;
    let nroots: i32 = (*envs).nrys_roots;
    let aj2: f64 = -2_f64 * (*envs).aj[0_usize];
    let gx: *const f64 = g;
    let gy: *const f64 = g.offset((*envs).g_size as isize);
    let gz: *const f64 = g.offset(((*envs).g_size * 2_i32) as isize);
    let fx: *mut f64 = f;
    let fy: *mut f64 = f.offset((*envs).g_size as isize);
    let fz: *mut f64 = f.offset(((*envs).g_size * 2_i32) as isize);
    let p1x: *const f64 = gx.offset(-(dj as isize));
    let p1y: *const f64 = gy.offset(-(dj as isize));
    let p1z: *const f64 = gz.offset(-(dj as isize));
    let p2x: *const f64 = gx.offset(dj as isize);
    let p2y: *const f64 = gy.offset(dj as isize);
    let p2z: *const f64 = gz.offset(dj as isize);
    l = 0_i32;
    while l <= ll {
        k = 0_i32;
        while k <= lk {
            ptr = dl * l + dk * k;
            i = 0_i32;
            while i <= li {
                n = ptr;
                while n < ptr + nroots {
                    *fx.offset(n as isize) = aj2 * *p2x.offset(n as isize);
                    *fy.offset(n as isize) = aj2 * *p2y.offset(n as isize);
                    *fz.offset(n as isize) = aj2 * *p2z.offset(n as isize);
                    n += 1;
                    n;
                }
                ptr += di;
                i += 1;
                i;
            }
            k += 1;
            k;
        }
        l += 1;
        l;
    }
    j = 1_i32;
    while j <= lj {
        l = 0_i32;
        while l <= ll {
            k = 0_i32;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0_i32;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx.offset(n as isize) =
                            j as f64 * *p1x.offset(n as isize) + aj2 * *p2x.offset(n as isize);
                        *fy.offset(n as isize) =
                            j as f64 * *p1y.offset(n as isize) + aj2 * *p2y.offset(n as isize);
                        *fz.offset(n as isize) =
                            j as f64 * *p1z.offset(n as isize) + aj2 * *p2z.offset(n as isize);
                        n += 1;
                        n;
                    }
                    ptr += di;
                    i += 1;
                    i;
                }
                k += 1;
                k;
            }
            l += 1;
            l;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1k_2e(
    f: *mut f64,
    g: *const f64,
    li: i32,
    lj: i32,
    lk: i32,
    ll: i32,
    envs: *const CINTEnvVars,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut n: i32 = 0;
    let mut ptr: i32 = 0;
    let di: i32 = (*envs).g_stride_i;
    let dk: i32 = (*envs).g_stride_k;
    let dl: i32 = (*envs).g_stride_l;
    let dj: i32 = (*envs).g_stride_j;
    let nroots: i32 = (*envs).nrys_roots;
    let ak2: f64 = -2_f64 * (*envs).ak[0_usize];
    let gx: *const f64 = g;
    let gy: *const f64 = g.offset((*envs).g_size as isize);
    let gz: *const f64 = g.offset(((*envs).g_size * 2_i32) as isize);
    let fx: *mut f64 = f;
    let fy: *mut f64 = f.offset((*envs).g_size as isize);
    let fz: *mut f64 = f.offset(((*envs).g_size * 2_i32) as isize);
    let p1x: *const f64 = gx.offset(-(dk as isize));
    let p1y: *const f64 = gy.offset(-(dk as isize));
    let p1z: *const f64 = gz.offset(-(dk as isize));
    let p2x: *const f64 = gx.offset(dk as isize);
    let p2y: *const f64 = gy.offset(dk as isize);
    let p2z: *const f64 = gz.offset(dk as isize);
    j = 0_i32;
    while j <= lj {
        l = 0_i32;
        while l <= ll {
            ptr = dj * j + dl * l;
            i = 0_i32;
            while i <= li {
                n = ptr;
                while n < ptr + nroots {
                    *fx.offset(n as isize) = ak2 * *p2x.offset(n as isize);
                    *fy.offset(n as isize) = ak2 * *p2y.offset(n as isize);
                    *fz.offset(n as isize) = ak2 * *p2z.offset(n as isize);
                    n += 1;
                    n;
                }
                ptr += di;
                i += 1;
                i;
            }
            k = 1_i32;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0_i32;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx.offset(n as isize) =
                            k as f64 * *p1x.offset(n as isize) + ak2 * *p2x.offset(n as isize);
                        *fy.offset(n as isize) =
                            k as f64 * *p1y.offset(n as isize) + ak2 * *p2y.offset(n as isize);
                        *fz.offset(n as isize) =
                            k as f64 * *p1z.offset(n as isize) + ak2 * *p2z.offset(n as isize);
                        n += 1;
                        n;
                    }
                    ptr += di;
                    i += 1;
                    i;
                }
                k += 1;
                k;
            }
            l += 1;
            l;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1l_2e(
    f: *mut f64,
    g: *const f64,
    li: i32,
    lj: i32,
    lk: i32,
    ll: i32,
    envs: *const CINTEnvVars,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut n: i32 = 0;
    let mut ptr: i32 = 0;
    let di: i32 = (*envs).g_stride_i;
    let dk: i32 = (*envs).g_stride_k;
    let dl: i32 = (*envs).g_stride_l;
    let dj: i32 = (*envs).g_stride_j;
    let nroots: i32 = (*envs).nrys_roots;
    let al2: f64 = -2_f64 * (*envs).al[0_usize];
    let gx: *const f64 = g;
    let gy: *const f64 = g.offset((*envs).g_size as isize);
    let gz: *const f64 = g.offset(((*envs).g_size * 2_i32) as isize);
    let fx: *mut f64 = f;
    let fy: *mut f64 = f.offset((*envs).g_size as isize);
    let fz: *mut f64 = f.offset(((*envs).g_size * 2_i32) as isize);
    let p1x: *const f64 = gx.offset(-(dl as isize));
    let p1y: *const f64 = gy.offset(-(dl as isize));
    let p1z: *const f64 = gz.offset(-(dl as isize));
    let p2x: *const f64 = gx.offset(dl as isize);
    let p2y: *const f64 = gy.offset(dl as isize);
    let p2z: *const f64 = gz.offset(dl as isize);
    j = 0_i32;
    while j <= lj {
        k = 0_i32;
        while k <= lk {
            ptr = dj * j + dk * k;
            i = 0_i32;
            while i <= li {
                n = ptr;
                while n < ptr + nroots {
                    *fx.offset(n as isize) = al2 * *p2x.offset(n as isize);
                    *fy.offset(n as isize) = al2 * *p2y.offset(n as isize);
                    *fz.offset(n as isize) = al2 * *p2z.offset(n as isize);
                    n += 1;
                    n;
                }
                ptr += di;
                i += 1;
                i;
            }
            k += 1;
            k;
        }
        l = 1_i32;
        while l <= ll {
            k = 0_i32;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0_i32;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx.offset(n as isize) =
                            l as f64 * *p1x.offset(n as isize) + al2 * *p2x.offset(n as isize);
                        *fy.offset(n as isize) =
                            l as f64 * *p1y.offset(n as isize) + al2 * *p2y.offset(n as isize);
                        *fz.offset(n as isize) =
                            l as f64 * *p1z.offset(n as isize) + al2 * *p2z.offset(n as isize);
                        n += 1;
                        n;
                    }
                    i += 1;
                    i;
                    ptr += di;
                }
                k += 1;
                k;
            }
            l += 1;
            l;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1i_2e(
    f: *mut f64,
    g: *const f64,
    ri: *const f64,
    li: i32,
    lj: i32,
    lk: i32,
    ll: i32,
    envs: *const CINTEnvVars,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut n: i32 = 0;
    let mut ptr: i32 = 0;
    let di: i32 = (*envs).g_stride_i;
    let dk: i32 = (*envs).g_stride_k;
    let dl: i32 = (*envs).g_stride_l;
    let dj: i32 = (*envs).g_stride_j;
    let nroots: i32 = (*envs).nrys_roots;
    let gx: *const f64 = g;
    let gy: *const f64 = g.offset((*envs).g_size as isize);
    let gz: *const f64 = g.offset(((*envs).g_size * 2_i32) as isize);
    let fx: *mut f64 = f;
    let fy: *mut f64 = f.offset((*envs).g_size as isize);
    let fz: *mut f64 = f.offset(((*envs).g_size * 2_i32) as isize);
    let p1x: *const f64 = gx.offset(di as isize);
    let p1y: *const f64 = gy.offset(di as isize);
    let p1z: *const f64 = gz.offset(di as isize);
    j = 0_i32;
    while j <= lj {
        l = 0_i32;
        while l <= ll {
            k = 0_i32;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0_i32;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx.offset(n as isize) = *p1x.offset(n as isize)
                            + *ri.offset(0_isize) * *gx.offset(n as isize);
                        *fy.offset(n as isize) = *p1y.offset(n as isize)
                            + *ri.offset(1_isize) * *gy.offset(n as isize);
                        *fz.offset(n as isize) = *p1z.offset(n as isize)
                            + *ri.offset(2_isize) * *gz.offset(n as isize);
                        n += 1;
                        n;
                    }
                    ptr += di;
                    i += 1;
                    i;
                }
                k += 1;
                k;
            }
            l += 1;
            l;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1j_2e(
    f: *mut f64,
    g: *const f64,
    rj: *const f64,
    li: i32,
    lj: i32,
    lk: i32,
    ll: i32,
    envs: *const CINTEnvVars,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut n: i32 = 0;
    let mut ptr: i32 = 0;
    let di: i32 = (*envs).g_stride_i;
    let dk: i32 = (*envs).g_stride_k;
    let dl: i32 = (*envs).g_stride_l;
    let dj: i32 = (*envs).g_stride_j;
    let nroots: i32 = (*envs).nrys_roots;
    let gx: *const f64 = g;
    let gy: *const f64 = g.offset((*envs).g_size as isize);
    let gz: *const f64 = g.offset(((*envs).g_size * 2_i32) as isize);
    let fx: *mut f64 = f;
    let fy: *mut f64 = f.offset((*envs).g_size as isize);
    let fz: *mut f64 = f.offset(((*envs).g_size * 2_i32) as isize);
    let p1x: *const f64 = gx.offset(dj as isize);
    let p1y: *const f64 = gy.offset(dj as isize);
    let p1z: *const f64 = gz.offset(dj as isize);
    j = 0_i32;
    while j <= lj {
        l = 0_i32;
        while l <= ll {
            k = 0_i32;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0_i32;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx.offset(n as isize) = *p1x.offset(n as isize)
                            + *rj.offset(0_isize) * *gx.offset(n as isize);
                        *fy.offset(n as isize) = *p1y.offset(n as isize)
                            + *rj.offset(1_isize) * *gy.offset(n as isize);
                        *fz.offset(n as isize) = *p1z.offset(n as isize)
                            + *rj.offset(2_isize) * *gz.offset(n as isize);
                        n += 1;
                        n;
                    }
                    ptr += di;
                    i += 1;
                    i;
                }
                k += 1;
                k;
            }
            l += 1;
            l;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1k_2e(
    f: *mut f64,
    g: *const f64,
    rk: *const f64,
    li: i32,
    lj: i32,
    lk: i32,
    ll: i32,
    envs: *const CINTEnvVars,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut n: i32 = 0;
    let mut ptr: i32 = 0;
    let di: i32 = (*envs).g_stride_i;
    let dk: i32 = (*envs).g_stride_k;
    let dl: i32 = (*envs).g_stride_l;
    let dj: i32 = (*envs).g_stride_j;
    let nroots: i32 = (*envs).nrys_roots;
    let gx: *const f64 = g;
    let gy: *const f64 = g.offset((*envs).g_size as isize);
    let gz: *const f64 = g.offset(((*envs).g_size * 2_i32) as isize);
    let fx: *mut f64 = f;
    let fy: *mut f64 = f.offset((*envs).g_size as isize);
    let fz: *mut f64 = f.offset(((*envs).g_size * 2_i32) as isize);
    let p1x: *const f64 = gx.offset(dk as isize);
    let p1y: *const f64 = gy.offset(dk as isize);
    let p1z: *const f64 = gz.offset(dk as isize);
    j = 0_i32;
    while j <= lj {
        l = 0_i32;
        while l <= ll {
            k = 0_i32;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0_i32;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx.offset(n as isize) = *p1x.offset(n as isize)
                            + *rk.offset(0_isize) * *gx.offset(n as isize);
                        *fy.offset(n as isize) = *p1y.offset(n as isize)
                            + *rk.offset(1_isize) * *gy.offset(n as isize);
                        *fz.offset(n as isize) = *p1z.offset(n as isize)
                            + *rk.offset(2_isize) * *gz.offset(n as isize);
                        n += 1;
                        n;
                    }
                    ptr += di;
                    i += 1;
                    i;
                }
                k += 1;
                k;
            }
            l += 1;
            l;
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1l_2e(
    f: *mut f64,
    g: *const f64,
    rl: *const f64,
    li: i32,
    lj: i32,
    lk: i32,
    ll: i32,
    envs: *const CINTEnvVars,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut n: i32 = 0;
    let mut ptr: i32 = 0;
    let di: i32 = (*envs).g_stride_i;
    let dk: i32 = (*envs).g_stride_k;
    let dl: i32 = (*envs).g_stride_l;
    let dj: i32 = (*envs).g_stride_j;
    let nroots: i32 = (*envs).nrys_roots;
    let gx: *const f64 = g;
    let gy: *const f64 = g.offset((*envs).g_size as isize);
    let gz: *const f64 = g.offset(((*envs).g_size * 2_i32) as isize);
    let fx: *mut f64 = f;
    let fy: *mut f64 = f.offset((*envs).g_size as isize);
    let fz: *mut f64 = f.offset(((*envs).g_size * 2_i32) as isize);
    let p1x: *const f64 = gx.offset(dl as isize);
    let p1y: *const f64 = gy.offset(dl as isize);
    let p1z: *const f64 = gz.offset(dl as isize);
    j = 0_i32;
    while j <= lj {
        l = 0_i32;
        while l <= ll {
            k = 0_i32;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0_i32;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx.offset(n as isize) = *p1x.offset(n as isize)
                            + *rl.offset(0_isize) * *gx.offset(n as isize);
                        *fy.offset(n as isize) = *p1y.offset(n as isize)
                            + *rl.offset(1_isize) * *gy.offset(n as isize);
                        *fz.offset(n as isize) = *p1z.offset(n as isize)
                            + *rl.offset(2_isize) * *gz.offset(n as isize);
                        n += 1;
                        n;
                    }
                    ptr += di;
                    i += 1;
                    i;
                }
                k += 1;
                k;
            }
            l += 1;
            l;
        }
        j += 1;
        j;
    }
}
