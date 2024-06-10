#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

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
    let l_sh: i32 = *shls.offset(3 as i32 as isize);
    (*envs).i_l = *bas.offset((8 as i32 * i_sh + 1 as i32) as isize);
    (*envs).j_l = *bas.offset((8 as i32 * j_sh + 1 as i32) as isize);
    (*envs).k_l = *bas.offset((8 as i32 * k_sh + 1 as i32) as isize);
    (*envs).l_l = *bas.offset((8 as i32 * l_sh + 1 as i32) as isize);
    (*envs)
        .x_ctr[0 as i32
        as usize] = *bas.offset((8 as i32 * i_sh + 3 as i32) as isize);
    (*envs)
        .x_ctr[1 as i32
        as usize] = *bas.offset((8 as i32 * j_sh + 3 as i32) as isize);
    (*envs)
        .x_ctr[2 as i32
        as usize] = *bas.offset((8 as i32 * k_sh + 3 as i32) as isize);
    (*envs)
        .x_ctr[3 as i32
        as usize] = *bas.offset((8 as i32 * l_sh + 3 as i32) as isize);
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
    (*envs)
        .c2rust_unnamed_0
        .nfl = ((*envs).l_l + 1 as i32) * ((*envs).l_l + 2 as i32)
        / 2 as i32;
    (*envs)
        .nf = (*envs).nfi * (*envs).c2rust_unnamed.nfk * (*envs).c2rust_unnamed_0.nfl
        * (*envs).nfj;
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
    (*envs)
        .c2rust_unnamed_1
        .rl = env
        .offset(
            *atm
                .offset(
                    (6 as i32
                        * *bas
                            .offset(
                                (8 as i32 * l_sh + 0 as i32) as isize,
                            ) + 1 as i32) as isize,
                ) as isize,
        );
    (*envs)
        .common_factor = 3.14159265358979323846f64 * 3.14159265358979323846f64
        * 3.14159265358979323846f64 * 2 as i32 as f64
        / 1.7724538509055160272981674833411451f64 * CINTcommon_fac_sp((*envs).i_l)
        * CINTcommon_fac_sp((*envs).j_l) * CINTcommon_fac_sp((*envs).k_l)
        * CINTcommon_fac_sp((*envs).l_l);
    if *env.offset(0 as i32 as isize) == 0 as i32 as f64 {
        (*envs).expcutoff = 60 as i32 as f64;
    } else {
        (*envs)
            .expcutoff = (if 40 as i32 as f64
            > *env.offset(0 as i32 as isize)
        {
            40 as i32 as f64
        } else {
            *env.offset(0 as i32 as isize)
        }) + 1 as i32 as f64;
    }
    (*envs).gbits = *ng.offset(4 as i32 as isize);
    (*envs).ncomp_e1 = *ng.offset(5 as i32 as isize);
    (*envs).ncomp_e2 = *ng.offset(6 as i32 as isize);
    (*envs).ncomp_tensor = *ng.offset(7 as i32 as isize);
    (*envs).li_ceil = (*envs).i_l + *ng.offset(0 as i32 as isize);
    (*envs).lj_ceil = (*envs).j_l + *ng.offset(1 as i32 as isize);
    (*envs).lk_ceil = (*envs).k_l + *ng.offset(2 as i32 as isize);
    (*envs).ll_ceil = (*envs).l_l + *ng.offset(3 as i32 as isize);
    let mut rys_order: i32 = ((*envs).li_ceil + (*envs).lj_ceil + (*envs).lk_ceil
        + (*envs).ll_ceil) / 2 as i32 + 1 as i32;
    let mut nrys_roots: i32 = rys_order;
    let mut omega: f64 = *env.offset(8 as i32 as isize);
    if omega < 0 as i32 as f64 && rys_order <= 3 as i32 {
        nrys_roots *= 2 as i32;
    }
    (*envs).rys_order = rys_order;
    (*envs).nrys_roots = nrys_roots;
    let mut dli: i32 = 0;
    let mut dlj: i32 = 0;
    let mut dlk: i32 = 0;
    let mut dll: i32 = 0;
    let mut ibase: i32 = ((*envs).li_ceil > (*envs).lj_ceil) as i32;
    let mut kbase: i32 = ((*envs).lk_ceil > (*envs).ll_ceil) as i32;
    if kbase != 0 {
        dlk = (*envs).lk_ceil + (*envs).ll_ceil + 1 as i32;
        dll = (*envs).ll_ceil + 1 as i32;
    } else {
        dlk = (*envs).lk_ceil + 1 as i32;
        dll = (*envs).lk_ceil + (*envs).ll_ceil + 1 as i32;
    }
    if ibase != 0 {
        dli = (*envs).li_ceil + (*envs).lj_ceil + 1 as i32;
        dlj = (*envs).lj_ceil + 1 as i32;
    } else {
        dli = (*envs).li_ceil + 1 as i32;
        dlj = (*envs).li_ceil + (*envs).lj_ceil + 1 as i32;
    }
    (*envs).g_stride_i = nrys_roots;
    (*envs).g_stride_k = nrys_roots * dli;
    (*envs).g_stride_l = nrys_roots * dli * dlk;
    (*envs).g_stride_j = nrys_roots * dli * dlk * dll;
    (*envs).g_size = nrys_roots * dli * dlk * dll * dlj;
    if kbase != 0 {
        (*envs).g2d_klmax = (*envs).g_stride_k;
        (*envs).rx_in_rklrx = (*envs).rk;
        (*envs)
            .rkrl[0 as i32
            as usize] = *((*envs).rk).offset(0 as i32 as isize)
            - *((*envs).c2rust_unnamed_1.rl).offset(0 as i32 as isize);
        (*envs)
            .rkrl[1 as i32
            as usize] = *((*envs).rk).offset(1 as i32 as isize)
            - *((*envs).c2rust_unnamed_1.rl).offset(1 as i32 as isize);
        (*envs)
            .rkrl[2 as i32
            as usize] = *((*envs).rk).offset(2 as i32 as isize)
            - *((*envs).c2rust_unnamed_1.rl).offset(2 as i32 as isize);
    } else {
        (*envs).g2d_klmax = (*envs).g_stride_l;
        (*envs).rx_in_rklrx = (*envs).c2rust_unnamed_1.rl;
        (*envs)
            .rkrl[0 as i32
            as usize] = *((*envs).c2rust_unnamed_1.rl).offset(0 as i32 as isize)
            - *((*envs).rk).offset(0 as i32 as isize);
        (*envs)
            .rkrl[1 as i32
            as usize] = *((*envs).c2rust_unnamed_1.rl).offset(1 as i32 as isize)
            - *((*envs).rk).offset(1 as i32 as isize);
        (*envs)
            .rkrl[2 as i32
            as usize] = *((*envs).c2rust_unnamed_1.rl).offset(2 as i32 as isize)
            - *((*envs).rk).offset(2 as i32 as isize);
    }
    if ibase != 0 {
        (*envs).g2d_ijmax = (*envs).g_stride_i;
        (*envs).rx_in_rijrx = (*envs).ri;
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
    } else {
        (*envs).g2d_ijmax = (*envs).g_stride_j;
        (*envs).rx_in_rijrx = (*envs).rj;
        (*envs)
            .rirj[0 as i32
            as usize] = *((*envs).rj).offset(0 as i32 as isize)
            - *((*envs).ri).offset(0 as i32 as isize);
        (*envs)
            .rirj[1 as i32
            as usize] = *((*envs).rj).offset(1 as i32 as isize)
            - *((*envs).ri).offset(1 as i32 as isize);
        (*envs)
            .rirj[2 as i32
            as usize] = *((*envs).rj).offset(2 as i32 as isize)
            - *((*envs).ri).offset(2 as i32 as isize);
    }
    if rys_order <= 2 as i32 {
        (*envs)
            .f_g0_2d4d = ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut f64,
                    *mut Rys2eT,
                    *mut CINTEnvVars,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg0_2e_2d4d_unrolled
                    as unsafe extern "C" fn(
                        *mut f64,
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
                        *mut f64,
                        *mut Rys2eT,
                        *mut CINTEnvVars,
                    ) -> (),
                >,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    CINTsrg0_2e_2d4d_unrolled
                        as unsafe extern "C" fn(
                            *mut f64,
                            *mut Rys2eT,
                            *mut CINTEnvVars,
                        ) -> (),
                ),
            );
        }
    } else if kbase != 0 {
        if ibase != 0 {
            (*envs)
                .f_g0_2d4d = ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut f64,
                        *mut Rys2eT,
                        *mut CINTEnvVars,
                    ) -> (),
                >,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    CINTg0_2e_ik2d4d
                        as unsafe extern "C" fn(
                            *mut f64,
                            *mut Rys2eT,
                            *mut CINTEnvVars,
                        ) -> (),
                ),
            );
        } else {
            (*envs)
                .f_g0_2d4d = ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut f64,
                        *mut Rys2eT,
                        *mut CINTEnvVars,
                    ) -> (),
                >,
                Option::<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    CINTg0_2e_kj2d4d
                        as unsafe extern "C" fn(
                            *mut f64,
                            *mut Rys2eT,
                            *mut CINTEnvVars,
                        ) -> (),
                ),
            );
        }
    } else if ibase != 0 {
        (*envs)
            .f_g0_2d4d = ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut f64,
                    *mut Rys2eT,
                    *mut CINTEnvVars,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg0_2e_il2d4d
                    as unsafe extern "C" fn(
                        *mut f64,
                        *mut Rys2eT,
                        *mut CINTEnvVars,
                    ) -> (),
            ),
        );
    } else {
        (*envs)
            .f_g0_2d4d = ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut f64,
                    *mut Rys2eT,
                    *mut CINTEnvVars,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg0_2e_lj2d4d
                    as unsafe extern "C" fn(
                        *mut f64,
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
                *mut f64,
                *mut f64,
                *mut f64,
                f64,
                *mut CINTEnvVars,
            ) -> i32,
        >,
        Option::<unsafe extern "C" fn() -> i32>,
    >(
        Some(
            CINTg0_2e
                as unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut f64,
                    f64,
                    *mut CINTEnvVars,
                ) -> i32,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTg2e_index_xyz(
    mut idx: *mut i32,
    mut envs: *const CINTEnvVars,
) {
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
    ofx = 0 as i32;
    ofy = (*envs).g_size;
    ofz = (*envs).g_size * 2 as i32;
    n = 0 as i32;
    j = 0 as i32;
    while j < nfj {
        l = 0 as i32;
        while l < nfl {
            oflx = ofx + dj * j_nx[j as usize] + dl * l_nx[l as usize];
            ofly = ofy + dj * j_ny[j as usize] + dl * l_ny[l as usize];
            oflz = ofz + dj * j_nz[j as usize] + dl * l_nz[l as usize];
            k = 0 as i32;
            while k < nfk {
                ofkx = oflx + dk * k_nx[k as usize];
                ofky = ofly + dk * k_ny[k as usize];
                ofkz = oflz + dk * k_nz[k as usize];
                match i_l {
                    0 => {
                        *idx.offset((n + 0 as i32) as isize) = ofkx;
                        *idx.offset((n + 1 as i32) as isize) = ofky;
                        *idx.offset((n + 2 as i32) as isize) = ofkz;
                        n += 3 as i32;
                    }
                    1 => {
                        *idx.offset((n + 0 as i32) as isize) = ofkx + di;
                        *idx.offset((n + 1 as i32) as isize) = ofky;
                        *idx.offset((n + 2 as i32) as isize) = ofkz;
                        *idx.offset((n + 3 as i32) as isize) = ofkx;
                        *idx.offset((n + 4 as i32) as isize) = ofky + di;
                        *idx.offset((n + 5 as i32) as isize) = ofkz;
                        *idx.offset((n + 6 as i32) as isize) = ofkx;
                        *idx.offset((n + 7 as i32) as isize) = ofky;
                        *idx.offset((n + 8 as i32) as isize) = ofkz + di;
                        n += 9 as i32;
                    }
                    2 => {
                        *idx
                            .offset(
                                (n + 0 as i32) as isize,
                            ) = ofkx + di * 2 as i32;
                        *idx.offset((n + 1 as i32) as isize) = ofky;
                        *idx.offset((n + 2 as i32) as isize) = ofkz;
                        *idx.offset((n + 3 as i32) as isize) = ofkx + di;
                        *idx.offset((n + 4 as i32) as isize) = ofky + di;
                        *idx.offset((n + 5 as i32) as isize) = ofkz;
                        *idx.offset((n + 6 as i32) as isize) = ofkx + di;
                        *idx.offset((n + 7 as i32) as isize) = ofky;
                        *idx.offset((n + 8 as i32) as isize) = ofkz + di;
                        *idx.offset((n + 9 as i32) as isize) = ofkx;
                        *idx
                            .offset(
                                (n + 10 as i32) as isize,
                            ) = ofky + di * 2 as i32;
                        *idx.offset((n + 11 as i32) as isize) = ofkz;
                        *idx.offset((n + 12 as i32) as isize) = ofkx;
                        *idx.offset((n + 13 as i32) as isize) = ofky + di;
                        *idx.offset((n + 14 as i32) as isize) = ofkz + di;
                        *idx.offset((n + 15 as i32) as isize) = ofkx;
                        *idx.offset((n + 16 as i32) as isize) = ofky;
                        *idx
                            .offset(
                                (n + 17 as i32) as isize,
                            ) = ofkz + di * 2 as i32;
                        n += 18 as i32;
                    }
                    _ => {
                        i = 0 as i32;
                        while i < nfi {
                            *idx
                                .offset(
                                    (n + 0 as i32) as isize,
                                ) = ofkx + di * i_nx[i as usize];
                            *idx
                                .offset(
                                    (n + 1 as i32) as isize,
                                ) = ofky + di * i_ny[i as usize];
                            *idx
                                .offset(
                                    (n + 2 as i32) as isize,
                                ) = ofkz + di * i_nz[i as usize];
                            n += 3 as i32;
                            i += 1;
                        }
                    }
                }
                k += 1;
            }
            l += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e_2d(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
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
    let mut gx: *mut f64 = g;
    let mut gy: *mut f64 = g.offset((*envs).g_size as isize);
    let mut gz: *mut f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut p0x: *mut f64 = 0 as *mut f64;
    let mut p0y: *mut f64 = 0 as *mut f64;
    let mut p0z: *mut f64 = 0 as *mut f64;
    let mut p1x: *mut f64 = 0 as *mut f64;
    let mut p1y: *mut f64 = 0 as *mut f64;
    let mut p1z: *mut f64 = 0 as *mut f64;
    let mut nb1: f64 = 0.;
    let mut mb0: f64 = 0.;
    i = 0 as i32;
    while i < nroots {
        *gx.offset(i as isize) = 1 as i32 as f64;
        *gy.offset(i as isize) = 1 as i32 as f64;
        i += 1;
    }
    let mut s0x: f64 = 0.;
    let mut s1x: f64 = 0.;
    let mut s2x: f64 = 0.;
    let mut t0x: f64 = 0.;
    let mut t1x: f64 = 0.;
    let mut s0y: f64 = 0.;
    let mut s1y: f64 = 0.;
    let mut s2y: f64 = 0.;
    let mut t0y: f64 = 0.;
    let mut t1y: f64 = 0.;
    let mut s0z: f64 = 0.;
    let mut s1z: f64 = 0.;
    let mut s2z: f64 = 0.;
    let mut t0z: f64 = 0.;
    let mut t1z: f64 = 0.;
    let mut c00x: f64 = 0.;
    let mut c00y: f64 = 0.;
    let mut c00z: f64 = 0.;
    let mut c0px: f64 = 0.;
    let mut c0py: f64 = 0.;
    let mut c0pz: f64 = 0.;
    let mut b10: f64 = 0.;
    let mut b01: f64 = 0.;
    let mut b00: f64 = 0.;
    i = 0 as i32;
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
        if nmax > 0 as i32 {
            s0x = *gx.offset(i as isize);
            s0y = *gy.offset(i as isize);
            s0z = *gz.offset(i as isize);
            s1x = c00x * s0x;
            s1y = c00y * s0y;
            s1z = c00z * s0z;
            *gx.offset((i + dn) as isize) = s1x;
            *gy.offset((i + dn) as isize) = s1y;
            *gz.offset((i + dn) as isize) = s1z;
            n = 1 as i32;
            while n < nmax {
                s2x = c00x * s1x + n as f64 * b10 * s0x;
                s2y = c00y * s1y + n as f64 * b10 * s0y;
                s2z = c00z * s1z + n as f64 * b10 * s0z;
                *gx.offset((i + (n + 1 as i32) * dn) as isize) = s2x;
                *gy.offset((i + (n + 1 as i32) * dn) as isize) = s2y;
                *gz.offset((i + (n + 1 as i32) * dn) as isize) = s2z;
                s0x = s1x;
                s0y = s1y;
                s0z = s1z;
                s1x = s2x;
                s1y = s2y;
                s1z = s2z;
                n += 1;
            }
        }
        if mmax > 0 as i32 {
            s0x = *gx.offset(i as isize);
            s0y = *gy.offset(i as isize);
            s0z = *gz.offset(i as isize);
            s1x = c0px * s0x;
            s1y = c0py * s0y;
            s1z = c0pz * s0z;
            *gx.offset((i + dm) as isize) = s1x;
            *gy.offset((i + dm) as isize) = s1y;
            *gz.offset((i + dm) as isize) = s1z;
            m = 1 as i32;
            while m < mmax {
                s2x = c0px * s1x + m as f64 * b01 * s0x;
                s2y = c0py * s1y + m as f64 * b01 * s0y;
                s2z = c0pz * s1z + m as f64 * b01 * s0z;
                *gx.offset((i + (m + 1 as i32) * dm) as isize) = s2x;
                *gy.offset((i + (m + 1 as i32) * dm) as isize) = s2y;
                *gz.offset((i + (m + 1 as i32) * dm) as isize) = s2z;
                s0x = s1x;
                s0y = s1y;
                s0z = s1z;
                s1x = s2x;
                s1y = s2y;
                s1z = s2z;
                m += 1;
            }
            if nmax > 0 as i32 {
                s0x = *gx.offset((i + dn) as isize);
                s0y = *gy.offset((i + dn) as isize);
                s0z = *gz.offset((i + dn) as isize);
                s1x = c0px * s0x + b00 * *gx.offset(i as isize);
                s1y = c0py * s0y + b00 * *gy.offset(i as isize);
                s1z = c0pz * s0z + b00 * *gz.offset(i as isize);
                *gx.offset((i + dn + dm) as isize) = s1x;
                *gy.offset((i + dn + dm) as isize) = s1y;
                *gz.offset((i + dn + dm) as isize) = s1z;
                m = 1 as i32;
                while m < mmax {
                    s2x = c0px * s1x + m as f64 * b01 * s0x
                        + b00 * *gx.offset((i + m * dm) as isize);
                    s2y = c0py * s1y + m as f64 * b01 * s0y
                        + b00 * *gy.offset((i + m * dm) as isize);
                    s2z = c0pz * s1z + m as f64 * b01 * s0z
                        + b00 * *gz.offset((i + m * dm) as isize);
                    *gx.offset((i + dn + (m + 1 as i32) * dm) as isize) = s2x;
                    *gy.offset((i + dn + (m + 1 as i32) * dm) as isize) = s2y;
                    *gz.offset((i + dn + (m + 1 as i32) * dm) as isize) = s2z;
                    s0x = s1x;
                    s0y = s1y;
                    s0z = s1z;
                    s1x = s2x;
                    s1y = s2y;
                    s1z = s2z;
                    m += 1;
                }
            }
        }
        m = 1 as i32;
        while m <= mmax {
            off = m * dm;
            j = off + i;
            s0x = *gx.offset(j as isize);
            s0y = *gy.offset(j as isize);
            s0z = *gz.offset(j as isize);
            s1x = *gx.offset((j + dn) as isize);
            s1y = *gy.offset((j + dn) as isize);
            s1z = *gz.offset((j + dn) as isize);
            n = 1 as i32;
            while n < nmax {
                s2x = c00x * s1x + n as f64 * b10 * s0x
                    + m as f64 * b00 * *gx.offset((j + n * dn - dm) as isize);
                s2y = c00y * s1y + n as f64 * b10 * s0y
                    + m as f64 * b00 * *gy.offset((j + n * dn - dm) as isize);
                s2z = c00z * s1z + n as f64 * b10 * s0z
                    + m as f64 * b00 * *gz.offset((j + n * dn - dm) as isize);
                *gx.offset((j + (n + 1 as i32) * dn) as isize) = s2x;
                *gy.offset((j + (n + 1 as i32) * dn) as isize) = s2y;
                *gz.offset((j + (n + 1 as i32) * dn) as isize) = s2z;
                s0x = s1x;
                s0y = s1y;
                s0z = s1z;
                s1x = s2x;
                s1y = s2y;
                s1z = s2z;
                n += 1;
            }
            m += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_lj2d_4d(
    mut g: *mut f64,
    mut envs: *mut CINTEnvVars,
) {
    let mut li: i32 = (*envs).li_ceil;
    let mut lk: i32 = (*envs).lk_ceil;
    if li == 0 as i32 && lk == 0 as i32 {
        return;
    }
    let mut nmax: i32 = (*envs).li_ceil + (*envs).lj_ceil;
    let mut mmax: i32 = (*envs).lk_ceil + (*envs).ll_ceil;
    let mut lj: i32 = (*envs).lj_ceil;
    let mut nroots: i32 = (*envs).nrys_roots;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut ptr: i32 = 0;
    let mut n: i32 = 0;
    let mut di: i32 = (*envs).g_stride_i;
    let mut dk: i32 = (*envs).g_stride_k;
    let mut dl: i32 = (*envs).g_stride_l;
    let mut dj: i32 = (*envs).g_stride_j;
    let mut rirj: *mut f64 = ((*envs).rirj).as_mut_ptr();
    let mut rkrl: *mut f64 = ((*envs).rkrl).as_mut_ptr();
    let mut gx: *mut f64 = g;
    let mut gy: *mut f64 = g.offset((*envs).g_size as isize);
    let mut gz: *mut f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut p1x: *mut f64 = 0 as *mut f64;
    let mut p1y: *mut f64 = 0 as *mut f64;
    let mut p1z: *mut f64 = 0 as *mut f64;
    let mut p2x: *mut f64 = 0 as *mut f64;
    let mut p2y: *mut f64 = 0 as *mut f64;
    let mut p2z: *mut f64 = 0 as *mut f64;
    let mut rx: f64 = 0.;
    let mut ry: f64 = 0.;
    let mut rz: f64 = 0.;
    rx = *rirj.offset(0 as i32 as isize);
    ry = *rirj.offset(1 as i32 as isize);
    rz = *rirj.offset(2 as i32 as isize);
    p1x = gx.offset(-(di as isize));
    p1y = gy.offset(-(di as isize));
    p1z = gz.offset(-(di as isize));
    p2x = gx.offset(-(di as isize)).offset(dj as isize);
    p2y = gy.offset(-(di as isize)).offset(dj as isize);
    p2z = gz.offset(-(di as isize)).offset(dj as isize);
    i = 1 as i32;
    while i <= li {
        j = 0 as i32;
        while j <= nmax - i {
            l = 0 as i32;
            while l <= mmax {
                ptr = j * dj + l * dl + i * di;
                n = ptr;
                while n < ptr + nroots {
                    *gx
                        .offset(
                            n as isize,
                        ) = rx * *p1x.offset(n as isize) + *p2x.offset(n as isize);
                    *gy
                        .offset(
                            n as isize,
                        ) = ry * *p1y.offset(n as isize) + *p2y.offset(n as isize);
                    *gz
                        .offset(
                            n as isize,
                        ) = rz * *p1z.offset(n as isize) + *p2z.offset(n as isize);
                    n += 1;
                }
                l += 1;
            }
            j += 1;
        }
        i += 1;
    }
    rx = *rkrl.offset(0 as i32 as isize);
    ry = *rkrl.offset(1 as i32 as isize);
    rz = *rkrl.offset(2 as i32 as isize);
    p1x = gx.offset(-(dk as isize));
    p1y = gy.offset(-(dk as isize));
    p1z = gz.offset(-(dk as isize));
    p2x = gx.offset(-(dk as isize)).offset(dl as isize);
    p2y = gy.offset(-(dk as isize)).offset(dl as isize);
    p2z = gz.offset(-(dk as isize)).offset(dl as isize);
    j = 0 as i32;
    while j <= lj {
        k = 1 as i32;
        while k <= lk {
            l = 0 as i32;
            while l <= mmax - k {
                ptr = j * dj + l * dl + k * dk;
                n = ptr;
                while n < ptr + dk {
                    *gx
                        .offset(
                            n as isize,
                        ) = rx * *p1x.offset(n as isize) + *p2x.offset(n as isize);
                    *gy
                        .offset(
                            n as isize,
                        ) = ry * *p1y.offset(n as isize) + *p2y.offset(n as isize);
                    *gz
                        .offset(
                            n as isize,
                        ) = rz * *p1z.offset(n as isize) + *p2z.offset(n as isize);
                    n += 1;
                }
                l += 1;
            }
            k += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_kj2d_4d(
    mut g: *mut f64,
    mut envs: *mut CINTEnvVars,
) {
    let mut li: i32 = (*envs).li_ceil;
    let mut ll: i32 = (*envs).ll_ceil;
    if li == 0 as i32 && ll == 0 as i32 {
        return;
    }
    let mut nmax: i32 = (*envs).li_ceil + (*envs).lj_ceil;
    let mut mmax: i32 = (*envs).lk_ceil + (*envs).ll_ceil;
    let mut lj: i32 = (*envs).lj_ceil;
    let mut nroots: i32 = (*envs).nrys_roots;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut ptr: i32 = 0;
    let mut n: i32 = 0;
    let mut di: i32 = (*envs).g_stride_i;
    let mut dk: i32 = (*envs).g_stride_k;
    let mut dl: i32 = (*envs).g_stride_l;
    let mut dj: i32 = (*envs).g_stride_j;
    let mut rirj: *mut f64 = ((*envs).rirj).as_mut_ptr();
    let mut rkrl: *mut f64 = ((*envs).rkrl).as_mut_ptr();
    let mut gx: *mut f64 = g;
    let mut gy: *mut f64 = g.offset((*envs).g_size as isize);
    let mut gz: *mut f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut p1x: *mut f64 = 0 as *mut f64;
    let mut p1y: *mut f64 = 0 as *mut f64;
    let mut p1z: *mut f64 = 0 as *mut f64;
    let mut p2x: *mut f64 = 0 as *mut f64;
    let mut p2y: *mut f64 = 0 as *mut f64;
    let mut p2z: *mut f64 = 0 as *mut f64;
    let mut rx: f64 = 0.;
    let mut ry: f64 = 0.;
    let mut rz: f64 = 0.;
    rx = *rirj.offset(0 as i32 as isize);
    ry = *rirj.offset(1 as i32 as isize);
    rz = *rirj.offset(2 as i32 as isize);
    p1x = gx.offset(-(di as isize));
    p1y = gy.offset(-(di as isize));
    p1z = gz.offset(-(di as isize));
    p2x = gx.offset(-(di as isize)).offset(dj as isize);
    p2y = gy.offset(-(di as isize)).offset(dj as isize);
    p2z = gz.offset(-(di as isize)).offset(dj as isize);
    i = 1 as i32;
    while i <= li {
        j = 0 as i32;
        while j <= nmax - i {
            k = 0 as i32;
            while k <= mmax {
                ptr = j * dj + k * dk + i * di;
                n = ptr;
                while n < ptr + nroots {
                    *gx
                        .offset(
                            n as isize,
                        ) = rx * *p1x.offset(n as isize) + *p2x.offset(n as isize);
                    *gy
                        .offset(
                            n as isize,
                        ) = ry * *p1y.offset(n as isize) + *p2y.offset(n as isize);
                    *gz
                        .offset(
                            n as isize,
                        ) = rz * *p1z.offset(n as isize) + *p2z.offset(n as isize);
                    n += 1;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    rx = *rkrl.offset(0 as i32 as isize);
    ry = *rkrl.offset(1 as i32 as isize);
    rz = *rkrl.offset(2 as i32 as isize);
    p1x = gx.offset(-(dl as isize));
    p1y = gy.offset(-(dl as isize));
    p1z = gz.offset(-(dl as isize));
    p2x = gx.offset(-(dl as isize)).offset(dk as isize);
    p2y = gy.offset(-(dl as isize)).offset(dk as isize);
    p2z = gz.offset(-(dl as isize)).offset(dk as isize);
    j = 0 as i32;
    while j <= lj {
        l = 1 as i32;
        while l <= ll {
            k = 0 as i32;
            while k <= mmax - l {
                ptr = j * dj + l * dl + k * dk;
                n = ptr;
                while n < ptr + dk {
                    *gx
                        .offset(
                            n as isize,
                        ) = rx * *p1x.offset(n as isize) + *p2x.offset(n as isize);
                    *gy
                        .offset(
                            n as isize,
                        ) = ry * *p1y.offset(n as isize) + *p2y.offset(n as isize);
                    *gz
                        .offset(
                            n as isize,
                        ) = rz * *p1z.offset(n as isize) + *p2z.offset(n as isize);
                    n += 1;
                }
                k += 1;
            }
            l += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_il2d_4d(
    mut g: *mut f64,
    mut envs: *mut CINTEnvVars,
) {
    let mut lk: i32 = (*envs).lk_ceil;
    let mut lj: i32 = (*envs).lj_ceil;
    if lj == 0 as i32 && lk == 0 as i32 {
        return;
    }
    let mut nmax: i32 = (*envs).li_ceil + (*envs).lj_ceil;
    let mut mmax: i32 = (*envs).lk_ceil + (*envs).ll_ceil;
    let mut ll: i32 = (*envs).ll_ceil;
    let mut nroots: i32 = (*envs).nrys_roots;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut ptr: i32 = 0;
    let mut n: i32 = 0;
    let mut di: i32 = (*envs).g_stride_i;
    let mut dk: i32 = (*envs).g_stride_k;
    let mut dl: i32 = (*envs).g_stride_l;
    let mut dj: i32 = (*envs).g_stride_j;
    let mut rirj: *mut f64 = ((*envs).rirj).as_mut_ptr();
    let mut rkrl: *mut f64 = ((*envs).rkrl).as_mut_ptr();
    let mut gx: *mut f64 = g;
    let mut gy: *mut f64 = g.offset((*envs).g_size as isize);
    let mut gz: *mut f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut p1x: *mut f64 = 0 as *mut f64;
    let mut p1y: *mut f64 = 0 as *mut f64;
    let mut p1z: *mut f64 = 0 as *mut f64;
    let mut p2x: *mut f64 = 0 as *mut f64;
    let mut p2y: *mut f64 = 0 as *mut f64;
    let mut p2z: *mut f64 = 0 as *mut f64;
    let mut rx: f64 = 0.;
    let mut ry: f64 = 0.;
    let mut rz: f64 = 0.;
    rx = *rkrl.offset(0 as i32 as isize);
    ry = *rkrl.offset(1 as i32 as isize);
    rz = *rkrl.offset(2 as i32 as isize);
    p1x = gx.offset(-(dk as isize));
    p1y = gy.offset(-(dk as isize));
    p1z = gz.offset(-(dk as isize));
    p2x = gx.offset(-(dk as isize)).offset(dl as isize);
    p2y = gy.offset(-(dk as isize)).offset(dl as isize);
    p2z = gz.offset(-(dk as isize)).offset(dl as isize);
    k = 1 as i32;
    while k <= lk {
        l = 0 as i32;
        while l <= mmax - k {
            i = 0 as i32;
            while i <= nmax {
                ptr = l * dl + k * dk + i * di;
                n = ptr;
                while n < ptr + nroots {
                    *gx
                        .offset(
                            n as isize,
                        ) = rx * *p1x.offset(n as isize) + *p2x.offset(n as isize);
                    *gy
                        .offset(
                            n as isize,
                        ) = ry * *p1y.offset(n as isize) + *p2y.offset(n as isize);
                    *gz
                        .offset(
                            n as isize,
                        ) = rz * *p1z.offset(n as isize) + *p2z.offset(n as isize);
                    n += 1;
                }
                i += 1;
            }
            l += 1;
        }
        k += 1;
    }
    rx = *rirj.offset(0 as i32 as isize);
    ry = *rirj.offset(1 as i32 as isize);
    rz = *rirj.offset(2 as i32 as isize);
    p1x = gx.offset(-(dj as isize));
    p1y = gy.offset(-(dj as isize));
    p1z = gz.offset(-(dj as isize));
    p2x = gx.offset(-(dj as isize)).offset(di as isize);
    p2y = gy.offset(-(dj as isize)).offset(di as isize);
    p2z = gz.offset(-(dj as isize)).offset(di as isize);
    j = 1 as i32;
    while j <= lj {
        l = 0 as i32;
        while l <= ll {
            k = 0 as i32;
            while k <= lk {
                ptr = j * dj + l * dl + k * dk;
                n = ptr;
                while n < ptr + dk - di * j {
                    *gx
                        .offset(
                            n as isize,
                        ) = rx * *p1x.offset(n as isize) + *p2x.offset(n as isize);
                    *gy
                        .offset(
                            n as isize,
                        ) = ry * *p1y.offset(n as isize) + *p2y.offset(n as isize);
                    *gz
                        .offset(
                            n as isize,
                        ) = rz * *p1z.offset(n as isize) + *p2z.offset(n as isize);
                    n += 1;
                }
                k += 1;
            }
            l += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_ik2d_4d(
    mut g: *mut f64,
    mut envs: *mut CINTEnvVars,
) {
    let mut lj: i32 = (*envs).lj_ceil;
    let mut ll: i32 = (*envs).ll_ceil;
    if lj == 0 as i32 && ll == 0 as i32 {
        return;
    }
    let mut nmax: i32 = (*envs).li_ceil + (*envs).lj_ceil;
    let mut mmax: i32 = (*envs).lk_ceil + (*envs).ll_ceil;
    let mut lk: i32 = (*envs).lk_ceil;
    let mut nroots: i32 = (*envs).nrys_roots;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut ptr: i32 = 0;
    let mut n: i32 = 0;
    let mut di: i32 = (*envs).g_stride_i;
    let mut dk: i32 = (*envs).g_stride_k;
    let mut dl: i32 = (*envs).g_stride_l;
    let mut dj: i32 = (*envs).g_stride_j;
    let mut rirj: *mut f64 = ((*envs).rirj).as_mut_ptr();
    let mut rkrl: *mut f64 = ((*envs).rkrl).as_mut_ptr();
    let mut gx: *mut f64 = g;
    let mut gy: *mut f64 = g.offset((*envs).g_size as isize);
    let mut gz: *mut f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut p1x: *mut f64 = 0 as *mut f64;
    let mut p1y: *mut f64 = 0 as *mut f64;
    let mut p1z: *mut f64 = 0 as *mut f64;
    let mut p2x: *mut f64 = 0 as *mut f64;
    let mut p2y: *mut f64 = 0 as *mut f64;
    let mut p2z: *mut f64 = 0 as *mut f64;
    let mut rx: f64 = 0.;
    let mut ry: f64 = 0.;
    let mut rz: f64 = 0.;
    rx = *rkrl.offset(0 as i32 as isize);
    ry = *rkrl.offset(1 as i32 as isize);
    rz = *rkrl.offset(2 as i32 as isize);
    p1x = gx.offset(-(dl as isize));
    p1y = gy.offset(-(dl as isize));
    p1z = gz.offset(-(dl as isize));
    p2x = gx.offset(-(dl as isize)).offset(dk as isize);
    p2y = gy.offset(-(dl as isize)).offset(dk as isize);
    p2z = gz.offset(-(dl as isize)).offset(dk as isize);
    l = 1 as i32;
    while l <= ll {
        k = 0 as i32;
        while k <= mmax - l {
            i = 0 as i32;
            while i <= nmax {
                ptr = l * dl + k * dk + i * di;
                n = ptr;
                while n < ptr + nroots {
                    *gx
                        .offset(
                            n as isize,
                        ) = rx * *p1x.offset(n as isize) + *p2x.offset(n as isize);
                    *gy
                        .offset(
                            n as isize,
                        ) = ry * *p1y.offset(n as isize) + *p2y.offset(n as isize);
                    *gz
                        .offset(
                            n as isize,
                        ) = rz * *p1z.offset(n as isize) + *p2z.offset(n as isize);
                    n += 1;
                }
                i += 1;
            }
            k += 1;
        }
        l += 1;
    }
    rx = *rirj.offset(0 as i32 as isize);
    ry = *rirj.offset(1 as i32 as isize);
    rz = *rirj.offset(2 as i32 as isize);
    p1x = gx.offset(-(dj as isize));
    p1y = gy.offset(-(dj as isize));
    p1z = gz.offset(-(dj as isize));
    p2x = gx.offset(-(dj as isize)).offset(di as isize);
    p2y = gy.offset(-(dj as isize)).offset(di as isize);
    p2z = gz.offset(-(dj as isize)).offset(di as isize);
    j = 1 as i32;
    while j <= lj {
        l = 0 as i32;
        while l <= ll {
            k = 0 as i32;
            while k <= lk {
                ptr = j * dj + l * dl + k * dk;
                n = ptr;
                while n < ptr + dk - di * j {
                    *gx
                        .offset(
                            n as isize,
                        ) = rx * *p1x.offset(n as isize) + *p2x.offset(n as isize);
                    *gy
                        .offset(
                            n as isize,
                        ) = ry * *p1y.offset(n as isize) + *p2y.offset(n as isize);
                    *gz
                        .offset(
                            n as isize,
                        ) = rz * *p1z.offset(n as isize) + *p2z.offset(n as isize);
                    n += 1;
                }
                k += 1;
            }
            l += 1;
        }
        j += 1;
    }
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0000(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0001(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g
        .offset(
            5 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(4 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0002(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            4 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *cpx.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            5 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *cpx.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = 1 as i32 as f64;
    *g.offset(7 as i32 as isize) = 1 as i32 as f64;
    *g.offset(8 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(9 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *cpy.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *cpy.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(12 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(13 as i32 as isize);
    *g
        .offset(
            16 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(14 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(12 as i32 as isize);
    *g
        .offset(
            17 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(15 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(13 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0003(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            4 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *cpx.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            5 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *cpx.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            6 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (*g.offset(4 as i32 as isize)
            + 2 as i32 as f64
                * *b01.offset(0 as i32 as isize));
    *g
        .offset(
            7 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (*g.offset(5 as i32 as isize)
            + 2 as i32 as f64
                * *b01.offset(1 as i32 as isize));
    *g.offset(8 as i32 as isize) = 1 as i32 as f64;
    *g.offset(9 as i32 as isize) = 1 as i32 as f64;
    *g.offset(10 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(11 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *cpy.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            13 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *cpy.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (*g.offset(12 as i32 as isize)
            + 2 as i32 as f64
                * *b01.offset(0 as i32 as isize));
    *g
        .offset(
            15 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (*g.offset(13 as i32 as isize)
            + 2 as i32 as f64
                * *b01.offset(1 as i32 as isize));
    *g
        .offset(
            18 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(16 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(17 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(18 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(16 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(19 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(17 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(20 as i32 as isize)
        + 2 as i32 as f64 * *b01.offset(0 as i32 as isize)
            * *g.offset(18 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(21 as i32 as isize)
        + 2 as i32 as f64 * *b01.offset(1 as i32 as isize)
            * *g.offset(19 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0010(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g
        .offset(
            5 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(4 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0011(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    let mut xkxl: f64 = (*envs).rkrl[0 as i32 as usize];
    let mut ykyl: f64 = (*envs).rkrl[1 as i32 as usize];
    let mut zkzl: f64 = (*envs).rkrl[2 as i32 as usize];
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            6 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (xkxl + *cpx.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            7 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (xkxl + *cpx.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize);
    *g.offset(2 as i32 as isize) = xkxl + *cpx.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = xkxl + *cpx.offset(1 as i32 as isize);
    *g.offset(12 as i32 as isize) = 1 as i32 as f64;
    *g.offset(13 as i32 as isize) = 1 as i32 as f64;
    *g.offset(16 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(17 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            18 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (ykyl + *cpy.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (ykyl + *cpy.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = ykyl + *cpy.offset(0 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = ykyl + *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(24 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(25 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *g.offset(28 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize) * *g.offset(24 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *g.offset(29 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize) * *g.offset(25 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = *g.offset(24 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize));
    *g
        .offset(
            27 as i32 as isize,
        ) = *g.offset(25 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize));
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0012(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    let mut xkxl: f64 = (*envs).rkrl[0 as i32 as usize];
    let mut ykyl: f64 = (*envs).rkrl[1 as i32 as usize];
    let mut zkzl: f64 = (*envs).rkrl[2 as i32 as usize];
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            8 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *cpx.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            9 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *cpx.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *g.offset(8 as i32 as isize)
        * (xkxl + *cpx.offset(0 as i32 as isize))
        + *cpx.offset(0 as i32 as isize) * 2 as i32 as f64
            * *b01.offset(0 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *g.offset(9 as i32 as isize)
        * (xkxl + *cpx.offset(1 as i32 as isize))
        + *cpx.offset(1 as i32 as isize) * 2 as i32 as f64
            * *b01.offset(1 as i32 as isize);
    *g
        .offset(
            6 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (xkxl + *cpx.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            7 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (xkxl + *cpx.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize);
    *g.offset(2 as i32 as isize) = xkxl + *cpx.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = xkxl + *cpx.offset(1 as i32 as isize);
    *g.offset(16 as i32 as isize) = 1 as i32 as f64;
    *g.offset(17 as i32 as isize) = 1 as i32 as f64;
    *g.offset(20 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(21 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            24 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *cpy.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            25 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *cpy.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = *g.offset(24 as i32 as isize)
        * (ykyl + *cpy.offset(0 as i32 as isize))
        + *cpy.offset(0 as i32 as isize) * 2 as i32 as f64
            * *b01.offset(0 as i32 as isize);
    *g
        .offset(
            27 as i32 as isize,
        ) = *g.offset(25 as i32 as isize)
        * (ykyl + *cpy.offset(1 as i32 as isize))
        + *cpy.offset(1 as i32 as isize) * 2 as i32 as f64
            * *b01.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (ykyl + *cpy.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (ykyl + *cpy.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            18 as i32 as isize,
        ) = ykyl + *cpy.offset(0 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = ykyl + *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            36 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(32 as i32 as isize);
    *g
        .offset(
            37 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(33 as i32 as isize);
    *g
        .offset(
            40 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(36 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(32 as i32 as isize);
    *g
        .offset(
            41 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(37 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(33 as i32 as isize);
    *g
        .offset(
            42 as i32 as isize,
        ) = *g.offset(40 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize))
        + 2 as i32 as f64 * *b01.offset(0 as i32 as isize)
            * *g.offset(36 as i32 as isize);
    *g
        .offset(
            43 as i32 as isize,
        ) = *g.offset(41 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize))
        + 2 as i32 as f64 * *b01.offset(1 as i32 as isize)
            * *g.offset(37 as i32 as isize);
    *g
        .offset(
            38 as i32 as isize,
        ) = *g.offset(36 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize) * *g.offset(32 as i32 as isize);
    *g
        .offset(
            39 as i32 as isize,
        ) = *g.offset(37 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize) * *g.offset(33 as i32 as isize);
    *g
        .offset(
            34 as i32 as isize,
        ) = *g.offset(32 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize));
    *g
        .offset(
            35 as i32 as isize,
        ) = *g.offset(33 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize));
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0020(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            4 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *cpx.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            5 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *cpx.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = 1 as i32 as f64;
    *g.offset(7 as i32 as isize) = 1 as i32 as f64;
    *g.offset(8 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(9 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *cpy.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *cpy.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(12 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(13 as i32 as isize);
    *g
        .offset(
            16 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(14 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(12 as i32 as isize);
    *g
        .offset(
            17 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(15 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(13 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0021(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    let mut xkxl: f64 = (*envs).rkrl[0 as i32 as usize];
    let mut ykyl: f64 = (*envs).rkrl[1 as i32 as usize];
    let mut zkzl: f64 = (*envs).rkrl[2 as i32 as usize];
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            4 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *cpx.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            5 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *cpx.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g.offset(8 as i32 as isize) = xkxl + *cpx.offset(0 as i32 as isize);
    *g.offset(9 as i32 as isize) = xkxl + *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (xkxl + *cpx.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (xkxl + *cpx.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *g.offset(4 as i32 as isize)
        * (xkxl + *cpx.offset(0 as i32 as isize))
        + *cpx.offset(0 as i32 as isize) * 2 as i32 as f64
            * *b01.offset(0 as i32 as isize);
    *g
        .offset(
            13 as i32 as isize,
        ) = *g.offset(5 as i32 as isize)
        * (xkxl + *cpx.offset(1 as i32 as isize))
        + *cpx.offset(1 as i32 as isize) * 2 as i32 as f64
            * *b01.offset(1 as i32 as isize);
    *g.offset(16 as i32 as isize) = 1 as i32 as f64;
    *g.offset(17 as i32 as isize) = 1 as i32 as f64;
    *g.offset(18 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(19 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *cpy.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *cpy.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            24 as i32 as isize,
        ) = ykyl + *cpy.offset(0 as i32 as isize);
    *g
        .offset(
            25 as i32 as isize,
        ) = ykyl + *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (ykyl + *cpy.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            27 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (ykyl + *cpy.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *g.offset(20 as i32 as isize)
        * (ykyl + *cpy.offset(0 as i32 as isize))
        + *cpy.offset(0 as i32 as isize) * 2 as i32 as f64
            * *b01.offset(0 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = *g.offset(21 as i32 as isize)
        * (ykyl + *cpy.offset(1 as i32 as isize))
        + *cpy.offset(1 as i32 as isize) * 2 as i32 as f64
            * *b01.offset(1 as i32 as isize);
    *g
        .offset(
            34 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(32 as i32 as isize);
    *g
        .offset(
            35 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(33 as i32 as isize);
    *g
        .offset(
            36 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(34 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(32 as i32 as isize);
    *g
        .offset(
            37 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(35 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(33 as i32 as isize);
    *g
        .offset(
            40 as i32 as isize,
        ) = *g.offset(32 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize));
    *g
        .offset(
            41 as i32 as isize,
        ) = *g.offset(33 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize));
    *g
        .offset(
            42 as i32 as isize,
        ) = *g.offset(34 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize) * *g.offset(32 as i32 as isize);
    *g
        .offset(
            43 as i32 as isize,
        ) = *g.offset(35 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize) * *g.offset(33 as i32 as isize);
    *g
        .offset(
            44 as i32 as isize,
        ) = *g.offset(36 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize))
        + 2 as i32 as f64 * *b01.offset(0 as i32 as isize)
            * *g.offset(34 as i32 as isize);
    *g
        .offset(
            45 as i32 as isize,
        ) = *g.offset(37 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize))
        + 2 as i32 as f64 * *b01.offset(1 as i32 as isize)
            * *g.offset(35 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0030(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            4 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *cpx.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            5 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *cpx.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            6 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (*g.offset(4 as i32 as isize)
            + 2 as i32 as f64
                * *b01.offset(0 as i32 as isize));
    *g
        .offset(
            7 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (*g.offset(5 as i32 as isize)
            + 2 as i32 as f64
                * *b01.offset(1 as i32 as isize));
    *g.offset(8 as i32 as isize) = 1 as i32 as f64;
    *g.offset(9 as i32 as isize) = 1 as i32 as f64;
    *g.offset(10 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(11 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *cpy.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            13 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *cpy.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (*g.offset(12 as i32 as isize)
            + 2 as i32 as f64
                * *b01.offset(0 as i32 as isize));
    *g
        .offset(
            15 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (*g.offset(13 as i32 as isize)
            + 2 as i32 as f64
                * *b01.offset(1 as i32 as isize));
    *g
        .offset(
            18 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(16 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(17 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(18 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(16 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(19 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(17 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(20 as i32 as isize)
        + 2 as i32 as f64 * *b01.offset(0 as i32 as isize)
            * *g.offset(18 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(21 as i32 as isize)
        + 2 as i32 as f64 * *b01.offset(1 as i32 as isize)
            * *g.offset(19 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0100(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g
        .offset(
            5 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(4 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0101(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(4 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g
        .offset(
            6 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            7 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g.offset(8 as i32 as isize) = 1 as i32 as f64;
    *g.offset(9 as i32 as isize) = 1 as i32 as f64;
    *g.offset(10 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(11 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(12 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(13 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            18 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(16 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(17 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(16 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(17 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(20 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(16 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(21 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(17 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0102(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(7 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g
        .offset(
            4 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *cpx.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            5 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *cpx.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            8 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            9 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (*g.offset(8 as i32 as isize) + *b00.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize)
            * *c0x.offset(0 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (*g.offset(9 as i32 as isize) + *b00.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize)
            * *c0x.offset(1 as i32 as isize);
    *g.offset(12 as i32 as isize) = 1 as i32 as f64;
    *g.offset(13 as i32 as isize) = 1 as i32 as f64;
    *g.offset(14 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(15 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(18 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(19 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            16 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *cpy.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            17 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *cpy.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (*g.offset(20 as i32 as isize)
            + *b00.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize)
            * *c0y.offset(0 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (*g.offset(21 as i32 as isize)
            + *b00.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize)
            * *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(24 as i32 as isize);
    *g
        .offset(
            27 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(25 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(24 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(25 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(26 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(24 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(27 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(25 as i32 as isize);
    *g
        .offset(
            32 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(30 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(24 as i32 as isize);
    *g
        .offset(
            33 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(31 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(25 as i32 as isize);
    *g
        .offset(
            34 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(32 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(30 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(26 as i32 as isize);
    *g
        .offset(
            35 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(33 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(31 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(27 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0110(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(4 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g
        .offset(
            6 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            7 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g.offset(8 as i32 as isize) = 1 as i32 as f64;
    *g.offset(9 as i32 as isize) = 1 as i32 as f64;
    *g.offset(10 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(11 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(12 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(13 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            18 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(16 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(17 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(16 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(17 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(20 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(16 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(21 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(17 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0111(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    let mut xkxl: f64 = (*envs).rkrl[0 as i32 as usize];
    let mut ykyl: f64 = (*envs).rkrl[1 as i32 as usize];
    let mut zkzl: f64 = (*envs).rkrl[2 as i32 as usize];
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(12 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(13 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(4 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            16 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            17 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            6 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (xkxl + *cpx.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            7 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (xkxl + *cpx.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            18 as i32 as isize,
        ) = *g.offset(16 as i32 as isize)
        * (xkxl + *cpx.offset(0 as i32 as isize))
        + *cpx.offset(0 as i32 as isize) * *b00.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize)
            * *c0x.offset(0 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = *g.offset(17 as i32 as isize)
        * (xkxl + *cpx.offset(1 as i32 as isize))
        + *cpx.offset(1 as i32 as isize) * *b00.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize)
            * *c0x.offset(1 as i32 as isize);
    *g.offset(2 as i32 as isize) = xkxl + *cpx.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = xkxl + *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (xkxl + *cpx.offset(0 as i32 as isize))
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (xkxl + *cpx.offset(1 as i32 as isize))
        + *b00.offset(1 as i32 as isize);
    *g.offset(24 as i32 as isize) = 1 as i32 as f64;
    *g.offset(25 as i32 as isize) = 1 as i32 as f64;
    *g.offset(36 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(37 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(28 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(29 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            40 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            41 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (ykyl + *cpy.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (ykyl + *cpy.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            42 as i32 as isize,
        ) = *g.offset(40 as i32 as isize)
        * (ykyl + *cpy.offset(0 as i32 as isize))
        + *cpy.offset(0 as i32 as isize) * *b00.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize)
            * *c0y.offset(0 as i32 as isize);
    *g
        .offset(
            43 as i32 as isize,
        ) = *g.offset(41 as i32 as isize)
        * (ykyl + *cpy.offset(1 as i32 as isize))
        + *cpy.offset(1 as i32 as isize) * *b00.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize)
            * *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = ykyl + *cpy.offset(0 as i32 as isize);
    *g
        .offset(
            27 as i32 as isize,
        ) = ykyl + *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            38 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (ykyl + *cpy.offset(0 as i32 as isize))
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            39 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (ykyl + *cpy.offset(1 as i32 as isize))
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            60 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            61 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            52 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            53 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            64 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(60 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            65 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(61 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            54 as i32 as isize,
        ) = *g.offset(52 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            55 as i32 as isize,
        ) = *g.offset(53 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            66 as i32 as isize,
        ) = *g.offset(64 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize) * *g.offset(60 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(52 as i32 as isize);
    *g
        .offset(
            67 as i32 as isize,
        ) = *g.offset(65 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize) * *g.offset(61 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(53 as i32 as isize);
    *g
        .offset(
            50 as i32 as isize,
        ) = *g.offset(48 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize));
    *g
        .offset(
            51 as i32 as isize,
        ) = *g.offset(49 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize));
    *g
        .offset(
            62 as i32 as isize,
        ) = *g.offset(60 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize))
        + *b00.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            63 as i32 as isize,
        ) = *g.offset(61 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize))
        + *b00.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0120(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(7 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g
        .offset(
            4 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *cpx.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            5 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *cpx.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            8 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            9 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (*g.offset(8 as i32 as isize) + *b00.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize)
            * *c0x.offset(0 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (*g.offset(9 as i32 as isize) + *b00.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize)
            * *c0x.offset(1 as i32 as isize);
    *g.offset(12 as i32 as isize) = 1 as i32 as f64;
    *g.offset(13 as i32 as isize) = 1 as i32 as f64;
    *g.offset(14 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(15 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(18 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(19 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            16 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *cpy.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            17 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *cpy.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (*g.offset(20 as i32 as isize)
            + *b00.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize)
            * *c0y.offset(0 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (*g.offset(21 as i32 as isize)
            + *b00.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize)
            * *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(24 as i32 as isize);
    *g
        .offset(
            27 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(25 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(24 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(25 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(26 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(24 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(27 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(25 as i32 as isize);
    *g
        .offset(
            32 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(30 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(24 as i32 as isize);
    *g
        .offset(
            33 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(31 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(25 as i32 as isize);
    *g
        .offset(
            34 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(32 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(30 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(26 as i32 as isize);
    *g
        .offset(
            35 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(33 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(31 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(27 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0200(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g
        .offset(
            4 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            5 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = 1 as i32 as f64;
    *g.offset(7 as i32 as isize) = 1 as i32 as f64;
    *g.offset(8 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(9 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(12 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(13 as i32 as isize);
    *g
        .offset(
            16 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(14 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(12 as i32 as isize);
    *g
        .offset(
            17 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(15 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(13 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0201(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g
        .offset(
            8 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            9 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g.offset(2 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            6 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            7 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (*g.offset(6 as i32 as isize) + *b00.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize)
            * *cpx.offset(0 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (*g.offset(7 as i32 as isize) + *b00.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize)
            * *cpx.offset(1 as i32 as isize);
    *g.offset(12 as i32 as isize) = 1 as i32 as f64;
    *g.offset(13 as i32 as isize) = 1 as i32 as f64;
    *g.offset(16 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(17 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g.offset(14 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(15 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            18 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (*g.offset(18 as i32 as isize)
            + *b00.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize)
            * *cpy.offset(0 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (*g.offset(19 as i32 as isize)
            + *b00.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize)
            * *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(24 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(25 as i32 as isize);
    *g
        .offset(
            32 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(28 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(24 as i32 as isize);
    *g
        .offset(
            33 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(29 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(25 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(24 as i32 as isize);
    *g
        .offset(
            27 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(25 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(28 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(24 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(29 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(25 as i32 as isize);
    *g
        .offset(
            34 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(30 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(26 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(28 as i32 as isize);
    *g
        .offset(
            35 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(31 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(27 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(29 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0210(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(4 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g
        .offset(
            6 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            7 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            8 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            9 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (*g.offset(6 as i32 as isize) + *b00.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize)
            * *cpx.offset(0 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (*g.offset(7 as i32 as isize) + *b00.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize)
            * *cpx.offset(1 as i32 as isize);
    *g.offset(12 as i32 as isize) = 1 as i32 as f64;
    *g.offset(13 as i32 as isize) = 1 as i32 as f64;
    *g.offset(14 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(15 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(16 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(17 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            18 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (*g.offset(18 as i32 as isize)
            + *b00.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize)
            * *cpy.offset(0 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (*g.offset(19 as i32 as isize)
            + *b00.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize)
            * *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(24 as i32 as isize);
    *g
        .offset(
            27 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(25 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(24 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(25 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(28 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(24 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(29 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(25 as i32 as isize);
    *g
        .offset(
            32 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(28 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(24 as i32 as isize);
    *g
        .offset(
            33 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(29 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(25 as i32 as isize);
    *g
        .offset(
            34 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(30 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(26 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(28 as i32 as isize);
    *g
        .offset(
            35 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(31 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(27 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(29 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_0300(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g
        .offset(
            4 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            5 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            6 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (*g.offset(4 as i32 as isize)
            + 2 as i32 as f64
                * *b10.offset(0 as i32 as isize));
    *g
        .offset(
            7 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (*g.offset(5 as i32 as isize)
            + 2 as i32 as f64
                * *b10.offset(1 as i32 as isize));
    *g.offset(8 as i32 as isize) = 1 as i32 as f64;
    *g.offset(9 as i32 as isize) = 1 as i32 as f64;
    *g.offset(10 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(11 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            13 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (*g.offset(12 as i32 as isize)
            + 2 as i32 as f64
                * *b10.offset(0 as i32 as isize));
    *g
        .offset(
            15 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (*g.offset(13 as i32 as isize)
            + 2 as i32 as f64
                * *b10.offset(1 as i32 as isize));
    *g
        .offset(
            18 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(16 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(17 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(18 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(16 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(19 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(17 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(20 as i32 as isize)
        + 2 as i32 as f64 * *b10.offset(0 as i32 as isize)
            * *g.offset(18 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(21 as i32 as isize)
        + 2 as i32 as f64 * *b10.offset(1 as i32 as isize)
            * *g.offset(19 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1000(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g
        .offset(
            5 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(4 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1001(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(4 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            6 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            7 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g.offset(8 as i32 as isize) = 1 as i32 as f64;
    *g.offset(9 as i32 as isize) = 1 as i32 as f64;
    *g.offset(10 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(11 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(12 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(13 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            18 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(16 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(17 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(16 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(17 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(18 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(16 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(19 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(17 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1002(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(4 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            6 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            7 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            8 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *cpx.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            9 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *cpx.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (*g.offset(6 as i32 as isize) + *b00.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize)
            * *c0x.offset(0 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (*g.offset(7 as i32 as isize) + *b00.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize)
            * *c0x.offset(1 as i32 as isize);
    *g.offset(12 as i32 as isize) = 1 as i32 as f64;
    *g.offset(13 as i32 as isize) = 1 as i32 as f64;
    *g.offset(14 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(15 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(16 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(17 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            18 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *cpy.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *cpy.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (*g.offset(18 as i32 as isize)
            + *b00.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize)
            * *c0y.offset(0 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (*g.offset(19 as i32 as isize)
            + *b00.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize)
            * *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(24 as i32 as isize);
    *g
        .offset(
            27 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(25 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(24 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(25 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(26 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(24 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(27 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(25 as i32 as isize);
    *g
        .offset(
            32 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(28 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(24 as i32 as isize);
    *g
        .offset(
            33 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(29 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(25 as i32 as isize);
    *g
        .offset(
            34 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(30 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(26 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(28 as i32 as isize);
    *g
        .offset(
            35 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(31 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(27 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(29 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1010(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(4 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            6 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            7 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g.offset(8 as i32 as isize) = 1 as i32 as f64;
    *g.offset(9 as i32 as isize) = 1 as i32 as f64;
    *g.offset(10 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(11 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(12 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(13 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            18 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(16 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(17 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(16 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(17 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(18 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(16 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(19 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(17 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1011(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    let mut xkxl: f64 = (*envs).rkrl[0 as i32 as usize];
    let mut ykyl: f64 = (*envs).rkrl[1 as i32 as usize];
    let mut zkzl: f64 = (*envs).rkrl[2 as i32 as usize];
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(8 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(9 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (xkxl + *cpx.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            13 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (xkxl + *cpx.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *g.offset(10 as i32 as isize)
        * (xkxl + *cpx.offset(0 as i32 as isize))
        + *cpx.offset(0 as i32 as isize) * *b00.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize)
            * *c0x.offset(0 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *g.offset(11 as i32 as isize)
        * (xkxl + *cpx.offset(1 as i32 as isize))
        + *cpx.offset(1 as i32 as isize) * *b00.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize)
            * *c0x.offset(1 as i32 as isize);
    *g.offset(4 as i32 as isize) = xkxl + *cpx.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = xkxl + *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            6 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (xkxl + *cpx.offset(0 as i32 as isize))
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            7 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (xkxl + *cpx.offset(1 as i32 as isize))
        + *b00.offset(1 as i32 as isize);
    *g.offset(24 as i32 as isize) = 1 as i32 as f64;
    *g.offset(25 as i32 as isize) = 1 as i32 as f64;
    *g.offset(26 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(27 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(32 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(33 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            34 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            35 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            36 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (ykyl + *cpy.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            37 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (ykyl + *cpy.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            38 as i32 as isize,
        ) = *g.offset(34 as i32 as isize)
        * (ykyl + *cpy.offset(0 as i32 as isize))
        + *cpy.offset(0 as i32 as isize) * *b00.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize)
            * *c0y.offset(0 as i32 as isize);
    *g
        .offset(
            39 as i32 as isize,
        ) = *g.offset(35 as i32 as isize)
        * (ykyl + *cpy.offset(1 as i32 as isize))
        + *cpy.offset(1 as i32 as isize) * *b00.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize)
            * *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = ykyl + *cpy.offset(0 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = ykyl + *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (ykyl + *cpy.offset(0 as i32 as isize))
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (ykyl + *cpy.offset(1 as i32 as isize))
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            50 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            51 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            56 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            57 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            58 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(50 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            59 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(51 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            60 as i32 as isize,
        ) = *g.offset(56 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            61 as i32 as isize,
        ) = *g.offset(57 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            62 as i32 as isize,
        ) = *g.offset(58 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize) * *g.offset(50 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(56 as i32 as isize);
    *g
        .offset(
            63 as i32 as isize,
        ) = *g.offset(59 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize) * *g.offset(51 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(57 as i32 as isize);
    *g
        .offset(
            52 as i32 as isize,
        ) = *g.offset(48 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize));
    *g
        .offset(
            53 as i32 as isize,
        ) = *g.offset(49 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize));
    *g
        .offset(
            54 as i32 as isize,
        ) = *g.offset(50 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize))
        + *b00.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            55 as i32 as isize,
        ) = *g.offset(51 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize))
        + *b00.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1020(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(4 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            6 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            7 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            8 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *cpx.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            9 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *cpx.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (*g.offset(6 as i32 as isize) + *b00.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize)
            * *c0x.offset(0 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (*g.offset(7 as i32 as isize) + *b00.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize)
            * *c0x.offset(1 as i32 as isize);
    *g.offset(12 as i32 as isize) = 1 as i32 as f64;
    *g.offset(13 as i32 as isize) = 1 as i32 as f64;
    *g.offset(14 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(15 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(16 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(17 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            18 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *cpy.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *cpy.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (*g.offset(18 as i32 as isize)
            + *b00.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize)
            * *c0y.offset(0 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (*g.offset(19 as i32 as isize)
            + *b00.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize)
            * *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(24 as i32 as isize);
    *g
        .offset(
            27 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(25 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(24 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(25 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(26 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(24 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(27 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(25 as i32 as isize);
    *g
        .offset(
            32 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(28 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(24 as i32 as isize);
    *g
        .offset(
            33 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(29 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(25 as i32 as isize);
    *g
        .offset(
            34 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(30 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(26 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(28 as i32 as isize);
    *g
        .offset(
            35 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(31 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(27 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(29 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1100(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    let mut xixj: f64 = (*envs).rirj[0 as i32 as usize];
    let mut yiyj: f64 = (*envs).rirj[1 as i32 as usize];
    let mut zizj: f64 = (*envs).rirj[2 as i32 as usize];
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g
        .offset(
            6 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (xixj + *c0x.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            7 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (xixj + *c0x.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize);
    *g.offset(2 as i32 as isize) = xixj + *c0x.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = xixj + *c0x.offset(1 as i32 as isize);
    *g.offset(12 as i32 as isize) = 1 as i32 as f64;
    *g.offset(13 as i32 as isize) = 1 as i32 as f64;
    *g.offset(16 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(17 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            18 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (yiyj + *c0y.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (yiyj + *c0y.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = yiyj + *c0y.offset(0 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = yiyj + *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(24 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(25 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *g.offset(28 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize) * *g.offset(24 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *g.offset(29 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize) * *g.offset(25 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = *g.offset(24 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize));
    *g
        .offset(
            27 as i32 as isize,
        ) = *g.offset(25 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize));
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1101(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    let mut xixj: f64 = (*envs).rirj[0 as i32 as usize];
    let mut yiyj: f64 = (*envs).rirj[1 as i32 as usize];
    let mut zizj: f64 = (*envs).rirj[2 as i32 as usize];
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(8 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(9 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(4 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            13 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (xixj + *c0x.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (xixj + *c0x.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize);
    *g.offset(2 as i32 as isize) = xixj + *c0x.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = xixj + *c0x.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *g.offset(12 as i32 as isize)
        * (xixj + *c0x.offset(0 as i32 as isize))
        + *c0x.offset(0 as i32 as isize) * *b00.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize)
            * *cpx.offset(0 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *g.offset(13 as i32 as isize)
        * (xixj + *c0x.offset(1 as i32 as isize))
        + *c0x.offset(1 as i32 as isize) * *b00.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize)
            * *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            6 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (xixj + *c0x.offset(0 as i32 as isize))
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            7 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (xixj + *c0x.offset(1 as i32 as isize))
        + *b00.offset(1 as i32 as isize);
    *g.offset(24 as i32 as isize) = 1 as i32 as f64;
    *g.offset(25 as i32 as isize) = 1 as i32 as f64;
    *g.offset(32 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(33 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(28 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(29 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            36 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            37 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            34 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (yiyj + *c0y.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            35 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (yiyj + *c0y.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = yiyj + *c0y.offset(0 as i32 as isize);
    *g
        .offset(
            27 as i32 as isize,
        ) = yiyj + *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            38 as i32 as isize,
        ) = *g.offset(36 as i32 as isize)
        * (yiyj + *c0y.offset(0 as i32 as isize))
        + *c0y.offset(0 as i32 as isize) * *b00.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize)
            * *cpy.offset(0 as i32 as isize);
    *g
        .offset(
            39 as i32 as isize,
        ) = *g.offset(37 as i32 as isize)
        * (yiyj + *c0y.offset(1 as i32 as isize))
        + *c0y.offset(1 as i32 as isize) * *b00.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize)
            * *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (yiyj + *c0y.offset(0 as i32 as isize))
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (yiyj + *c0y.offset(1 as i32 as isize))
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            56 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            57 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            52 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            53 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            60 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(56 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            61 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(57 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            58 as i32 as isize,
        ) = *g.offset(56 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            59 as i32 as isize,
        ) = *g.offset(57 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            50 as i32 as isize,
        ) = *g.offset(48 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize));
    *g
        .offset(
            51 as i32 as isize,
        ) = *g.offset(49 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize));
    *g
        .offset(
            62 as i32 as isize,
        ) = *g.offset(60 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize) * *g.offset(52 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(56 as i32 as isize);
    *g
        .offset(
            63 as i32 as isize,
        ) = *g.offset(61 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize) * *g.offset(53 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(57 as i32 as isize);
    *g
        .offset(
            54 as i32 as isize,
        ) = zizj * *g.offset(52 as i32 as isize)
        + *cpz.offset(0 as i32 as isize) * *g.offset(56 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            55 as i32 as isize,
        ) = zizj * *g.offset(53 as i32 as isize)
        + *cpz.offset(1 as i32 as isize) * *g.offset(57 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1110(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    let mut xixj: f64 = (*envs).rirj[0 as i32 as usize];
    let mut yiyj: f64 = (*envs).rirj[1 as i32 as usize];
    let mut zizj: f64 = (*envs).rirj[2 as i32 as usize];
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(8 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(9 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(4 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            13 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (xixj + *c0x.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (xixj + *c0x.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize);
    *g.offset(2 as i32 as isize) = xixj + *c0x.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = xixj + *c0x.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *g.offset(12 as i32 as isize)
        * (xixj + *c0x.offset(0 as i32 as isize))
        + *c0x.offset(0 as i32 as isize) * *b00.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize)
            * *cpx.offset(0 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *g.offset(13 as i32 as isize)
        * (xixj + *c0x.offset(1 as i32 as isize))
        + *c0x.offset(1 as i32 as isize) * *b00.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize)
            * *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            6 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (xixj + *c0x.offset(0 as i32 as isize))
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            7 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (xixj + *c0x.offset(1 as i32 as isize))
        + *b00.offset(1 as i32 as isize);
    *g.offset(24 as i32 as isize) = 1 as i32 as f64;
    *g.offset(25 as i32 as isize) = 1 as i32 as f64;
    *g.offset(32 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(33 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(28 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(29 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            36 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            37 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            34 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (yiyj + *c0y.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            35 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (yiyj + *c0y.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = yiyj + *c0y.offset(0 as i32 as isize);
    *g
        .offset(
            27 as i32 as isize,
        ) = yiyj + *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            38 as i32 as isize,
        ) = *g.offset(36 as i32 as isize)
        * (yiyj + *c0y.offset(0 as i32 as isize))
        + *c0y.offset(0 as i32 as isize) * *b00.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize)
            * *cpy.offset(0 as i32 as isize);
    *g
        .offset(
            39 as i32 as isize,
        ) = *g.offset(37 as i32 as isize)
        * (yiyj + *c0y.offset(1 as i32 as isize))
        + *c0y.offset(1 as i32 as isize) * *b00.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize)
            * *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (yiyj + *c0y.offset(0 as i32 as isize))
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (yiyj + *c0y.offset(1 as i32 as isize))
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            56 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            57 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            52 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            53 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            60 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(56 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            61 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(57 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            58 as i32 as isize,
        ) = *g.offset(56 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            59 as i32 as isize,
        ) = *g.offset(57 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            50 as i32 as isize,
        ) = *g.offset(48 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize));
    *g
        .offset(
            51 as i32 as isize,
        ) = *g.offset(49 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize));
    *g
        .offset(
            62 as i32 as isize,
        ) = *g.offset(60 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize) * *g.offset(52 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(56 as i32 as isize);
    *g
        .offset(
            63 as i32 as isize,
        ) = *g.offset(61 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize) * *g.offset(53 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(57 as i32 as isize);
    *g
        .offset(
            54 as i32 as isize,
        ) = zizj * *g.offset(52 as i32 as isize)
        + *cpz.offset(0 as i32 as isize) * *g.offset(56 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            55 as i32 as isize,
        ) = zizj * *g.offset(53 as i32 as isize)
        + *cpz.offset(1 as i32 as isize) * *g.offset(57 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_1200(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    let mut xixj: f64 = (*envs).rirj[0 as i32 as usize];
    let mut yiyj: f64 = (*envs).rirj[1 as i32 as usize];
    let mut zizj: f64 = (*envs).rirj[2 as i32 as usize];
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g
        .offset(
            8 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            9 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *g.offset(8 as i32 as isize)
        * (xixj + *c0x.offset(0 as i32 as isize))
        + *c0x.offset(0 as i32 as isize) * 2 as i32 as f64
            * *b10.offset(0 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *g.offset(9 as i32 as isize)
        * (xixj + *c0x.offset(1 as i32 as isize))
        + *c0x.offset(1 as i32 as isize) * 2 as i32 as f64
            * *b10.offset(1 as i32 as isize);
    *g
        .offset(
            6 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (xixj + *c0x.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            7 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (xixj + *c0x.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize);
    *g.offset(2 as i32 as isize) = xixj + *c0x.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = xixj + *c0x.offset(1 as i32 as isize);
    *g.offset(16 as i32 as isize) = 1 as i32 as f64;
    *g.offset(17 as i32 as isize) = 1 as i32 as f64;
    *g.offset(20 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(21 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            24 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            25 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = *g.offset(24 as i32 as isize)
        * (yiyj + *c0y.offset(0 as i32 as isize))
        + *c0y.offset(0 as i32 as isize) * 2 as i32 as f64
            * *b10.offset(0 as i32 as isize);
    *g
        .offset(
            27 as i32 as isize,
        ) = *g.offset(25 as i32 as isize)
        * (yiyj + *c0y.offset(1 as i32 as isize))
        + *c0y.offset(1 as i32 as isize) * 2 as i32 as f64
            * *b10.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (yiyj + *c0y.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (yiyj + *c0y.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            18 as i32 as isize,
        ) = yiyj + *c0y.offset(0 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = yiyj + *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            36 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(32 as i32 as isize);
    *g
        .offset(
            37 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(33 as i32 as isize);
    *g
        .offset(
            40 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(36 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(32 as i32 as isize);
    *g
        .offset(
            41 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(37 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(33 as i32 as isize);
    *g
        .offset(
            42 as i32 as isize,
        ) = *g.offset(40 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize))
        + 2 as i32 as f64 * *b10.offset(0 as i32 as isize)
            * *g.offset(36 as i32 as isize);
    *g
        .offset(
            43 as i32 as isize,
        ) = *g.offset(41 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize))
        + 2 as i32 as f64 * *b10.offset(1 as i32 as isize)
            * *g.offset(37 as i32 as isize);
    *g
        .offset(
            38 as i32 as isize,
        ) = *g.offset(36 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize) * *g.offset(32 as i32 as isize);
    *g
        .offset(
            39 as i32 as isize,
        ) = *g.offset(37 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize) * *g.offset(33 as i32 as isize);
    *g
        .offset(
            34 as i32 as isize,
        ) = *g.offset(32 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize));
    *g
        .offset(
            35 as i32 as isize,
        ) = *g.offset(33 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize));
}
#[inline]
unsafe extern "C" fn _g0_2d4d_2000(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g
        .offset(
            4 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            5 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = 1 as i32 as f64;
    *g.offset(7 as i32 as isize) = 1 as i32 as f64;
    *g.offset(8 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(9 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(12 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(13 as i32 as isize);
    *g
        .offset(
            16 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(14 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(12 as i32 as isize);
    *g
        .offset(
            17 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(15 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(13 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_2001(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g
        .offset(
            4 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            5 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(7 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            8 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            9 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (*g.offset(8 as i32 as isize) + *b00.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize)
            * *cpx.offset(0 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (*g.offset(9 as i32 as isize) + *b00.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize)
            * *cpx.offset(1 as i32 as isize);
    *g.offset(12 as i32 as isize) = 1 as i32 as f64;
    *g.offset(13 as i32 as isize) = 1 as i32 as f64;
    *g.offset(14 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(15 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            16 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            17 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g.offset(18 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(19 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (*g.offset(20 as i32 as isize)
            + *b00.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize)
            * *cpy.offset(0 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (*g.offset(21 as i32 as isize)
            + *b00.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize)
            * *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(24 as i32 as isize);
    *g
        .offset(
            27 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(25 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(26 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(24 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(27 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(25 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(24 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(25 as i32 as isize);
    *g
        .offset(
            32 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(26 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(24 as i32 as isize);
    *g
        .offset(
            33 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(27 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(25 as i32 as isize);
    *g
        .offset(
            34 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(32 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(30 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(26 as i32 as isize);
    *g
        .offset(
            35 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(33 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(31 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(27 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_2010(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g
        .offset(
            4 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            5 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(7 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            8 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            9 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (*g.offset(8 as i32 as isize) + *b00.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize)
            * *cpx.offset(0 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (*g.offset(9 as i32 as isize) + *b00.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize)
            * *cpx.offset(1 as i32 as isize);
    *g.offset(12 as i32 as isize) = 1 as i32 as f64;
    *g.offset(13 as i32 as isize) = 1 as i32 as f64;
    *g.offset(14 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(15 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            16 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            17 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g.offset(18 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(19 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (*g.offset(20 as i32 as isize)
            + *b00.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize)
            * *cpy.offset(0 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (*g.offset(21 as i32 as isize)
            + *b00.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize)
            * *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(24 as i32 as isize);
    *g
        .offset(
            27 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(25 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(26 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(24 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(27 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(25 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(24 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(25 as i32 as isize);
    *g
        .offset(
            32 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(26 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(24 as i32 as isize);
    *g
        .offset(
            33 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(27 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(25 as i32 as isize);
    *g
        .offset(
            34 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(32 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(30 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(26 as i32 as isize);
    *g
        .offset(
            35 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(33 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(31 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(27 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _g0_2d4d_2100(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    let mut xixj: f64 = (*envs).rirj[0 as i32 as usize];
    let mut yiyj: f64 = (*envs).rirj[1 as i32 as usize];
    let mut zizj: f64 = (*envs).rirj[2 as i32 as usize];
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g
        .offset(
            4 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            5 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *g.offset(4 as i32 as isize)
        * (xixj + *c0x.offset(0 as i32 as isize))
        + *c0x.offset(0 as i32 as isize) * 2 as i32 as f64
            * *b10.offset(0 as i32 as isize);
    *g
        .offset(
            13 as i32 as isize,
        ) = *g.offset(5 as i32 as isize)
        * (xixj + *c0x.offset(1 as i32 as isize))
        + *c0x.offset(1 as i32 as isize) * 2 as i32 as f64
            * *b10.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (xixj + *c0x.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (xixj + *c0x.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize);
    *g.offset(8 as i32 as isize) = xixj + *c0x.offset(0 as i32 as isize);
    *g.offset(9 as i32 as isize) = xixj + *c0x.offset(1 as i32 as isize);
    *g.offset(16 as i32 as isize) = 1 as i32 as f64;
    *g.offset(17 as i32 as isize) = 1 as i32 as f64;
    *g.offset(18 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(19 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *g.offset(20 as i32 as isize)
        * (yiyj + *c0y.offset(0 as i32 as isize))
        + *c0y.offset(0 as i32 as isize) * 2 as i32 as f64
            * *b10.offset(0 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = *g.offset(21 as i32 as isize)
        * (yiyj + *c0y.offset(1 as i32 as isize))
        + *c0y.offset(1 as i32 as isize) * 2 as i32 as f64
            * *b10.offset(1 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (yiyj + *c0y.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            27 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (yiyj + *c0y.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            24 as i32 as isize,
        ) = yiyj + *c0y.offset(0 as i32 as isize);
    *g
        .offset(
            25 as i32 as isize,
        ) = yiyj + *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            34 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(32 as i32 as isize);
    *g
        .offset(
            35 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(33 as i32 as isize);
    *g
        .offset(
            36 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(34 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(32 as i32 as isize);
    *g
        .offset(
            37 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(35 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(33 as i32 as isize);
    *g
        .offset(
            44 as i32 as isize,
        ) = *g.offset(36 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize))
        + 2 as i32 as f64 * *b10.offset(0 as i32 as isize)
            * *g.offset(34 as i32 as isize);
    *g
        .offset(
            45 as i32 as isize,
        ) = *g.offset(37 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize))
        + 2 as i32 as f64 * *b10.offset(1 as i32 as isize)
            * *g.offset(35 as i32 as isize);
    *g
        .offset(
            42 as i32 as isize,
        ) = *g.offset(34 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize) * *g.offset(32 as i32 as isize);
    *g
        .offset(
            43 as i32 as isize,
        ) = *g.offset(35 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize) * *g.offset(33 as i32 as isize);
    *g
        .offset(
            40 as i32 as isize,
        ) = *g.offset(32 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize));
    *g
        .offset(
            41 as i32 as isize,
        ) = *g.offset(33 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize));
}
#[inline]
unsafe extern "C" fn _g0_2d4d_3000(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g
        .offset(
            4 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            5 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            6 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (*g.offset(4 as i32 as isize)
            + 2 as i32 as f64
                * *b10.offset(0 as i32 as isize));
    *g
        .offset(
            7 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (*g.offset(5 as i32 as isize)
            + 2 as i32 as f64
                * *b10.offset(1 as i32 as isize));
    *g.offset(8 as i32 as isize) = 1 as i32 as f64;
    *g.offset(9 as i32 as isize) = 1 as i32 as f64;
    *g.offset(10 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(11 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            13 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (*g.offset(12 as i32 as isize)
            + 2 as i32 as f64
                * *b10.offset(0 as i32 as isize));
    *g
        .offset(
            15 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (*g.offset(13 as i32 as isize)
            + 2 as i32 as f64
                * *b10.offset(1 as i32 as isize));
    *g
        .offset(
            18 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(16 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(17 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(18 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(16 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(19 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(17 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(20 as i32 as isize)
        + 2 as i32 as f64 * *b10.offset(0 as i32 as isize)
            * *g.offset(18 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(21 as i32 as isize)
        + 2 as i32 as f64 * *b10.offset(1 as i32 as isize)
            * *g.offset(19 as i32 as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e_2d4d_unrolled(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut type_ijkl: i32 = (*envs).li_ceil << 6 as i32
        | (*envs).lj_ceil << 4 as i32 | (*envs).lk_ceil << 2 as i32
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
    eprintln!("Dimension error for CINTg0_2e_lj2d4d: iklj = {} {} {} {}", (*envs).li_ceil, (*envs).lk_ceil, (*envs).ll_ceil, (*envs).lj_ceil);
    // fprintf(
    //     stderr,
    //     b"Dimension error for CINTg0_2e_lj2d4d: iklj = %d %d %d %d\0" as *const u8
    //         as *const libc::c_char,
    //     (*envs).li_ceil,
    //     (*envs).lk_ceil,
    //     (*envs).ll_ceil,
    //     (*envs).lj_ceil,
    // );
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0000(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0001(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(4 as i32 as isize) = 1 as i32 as f64;
    *g.offset(5 as i32 as isize) = 1 as i32 as f64;
    *g.offset(6 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(7 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(8 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(9 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0002(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *cpx.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = *cpx.offset(3 as i32 as isize);
    *g
        .offset(
            8 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *cpx.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            9 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *cpx.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *cpx.offset(2 as i32 as isize)
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *cpx.offset(3 as i32 as isize)
        + *b01.offset(3 as i32 as isize);
    *g.offset(12 as i32 as isize) = 1 as i32 as f64;
    *g.offset(13 as i32 as isize) = 1 as i32 as f64;
    *g.offset(14 as i32 as isize) = 1 as i32 as f64;
    *g.offset(15 as i32 as isize) = 1 as i32 as f64;
    *g.offset(16 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(17 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(18 as i32 as isize) = *cpy.offset(2 as i32 as isize);
    *g.offset(19 as i32 as isize) = *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *cpy.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *cpy.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *cpy.offset(2 as i32 as isize)
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *cpy.offset(3 as i32 as isize)
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(24 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(25 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(26 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(27 as i32 as isize);
    *g
        .offset(
            32 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(28 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(24 as i32 as isize);
    *g
        .offset(
            33 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(29 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(25 as i32 as isize);
    *g
        .offset(
            34 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(30 as i32 as isize)
        + *b01.offset(2 as i32 as isize) * *g.offset(26 as i32 as isize);
    *g
        .offset(
            35 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(31 as i32 as isize)
        + *b01.offset(3 as i32 as isize) * *g.offset(27 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0003(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *cpx.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = *cpx.offset(3 as i32 as isize);
    *g
        .offset(
            8 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *cpx.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            9 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *cpx.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *cpx.offset(2 as i32 as isize)
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *cpx.offset(3 as i32 as isize)
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (*g.offset(8 as i32 as isize)
            + 2 as i32 as f64
                * *b01.offset(0 as i32 as isize));
    *g
        .offset(
            13 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (*g.offset(9 as i32 as isize)
            + 2 as i32 as f64
                * *b01.offset(1 as i32 as isize));
    *g
        .offset(
            14 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * (*g.offset(10 as i32 as isize)
            + 2 as i32 as f64
                * *b01.offset(2 as i32 as isize));
    *g
        .offset(
            15 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * (*g.offset(11 as i32 as isize)
            + 2 as i32 as f64
                * *b01.offset(3 as i32 as isize));
    *g.offset(16 as i32 as isize) = 1 as i32 as f64;
    *g.offset(17 as i32 as isize) = 1 as i32 as f64;
    *g.offset(18 as i32 as isize) = 1 as i32 as f64;
    *g.offset(19 as i32 as isize) = 1 as i32 as f64;
    *g.offset(20 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(21 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(22 as i32 as isize) = *cpy.offset(2 as i32 as isize);
    *g.offset(23 as i32 as isize) = *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            24 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *cpy.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            25 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *cpy.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *cpy.offset(2 as i32 as isize)
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            27 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *cpy.offset(3 as i32 as isize)
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (*g.offset(24 as i32 as isize)
            + 2 as i32 as f64
                * *b01.offset(0 as i32 as isize));
    *g
        .offset(
            29 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (*g.offset(25 as i32 as isize)
            + 2 as i32 as f64
                * *b01.offset(1 as i32 as isize));
    *g
        .offset(
            30 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * (*g.offset(26 as i32 as isize)
            + 2 as i32 as f64
                * *b01.offset(2 as i32 as isize));
    *g
        .offset(
            31 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * (*g.offset(27 as i32 as isize)
            + 2 as i32 as f64
                * *b01.offset(3 as i32 as isize));
    *g
        .offset(
            36 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(32 as i32 as isize);
    *g
        .offset(
            37 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(33 as i32 as isize);
    *g
        .offset(
            38 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(34 as i32 as isize);
    *g
        .offset(
            39 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(35 as i32 as isize);
    *g
        .offset(
            40 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(36 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(32 as i32 as isize);
    *g
        .offset(
            41 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(37 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(33 as i32 as isize);
    *g
        .offset(
            42 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(38 as i32 as isize)
        + *b01.offset(2 as i32 as isize) * *g.offset(34 as i32 as isize);
    *g
        .offset(
            43 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(39 as i32 as isize)
        + *b01.offset(3 as i32 as isize) * *g.offset(35 as i32 as isize);
    *g
        .offset(
            44 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(40 as i32 as isize)
        + 2 as i32 as f64 * *b01.offset(0 as i32 as isize)
            * *g.offset(36 as i32 as isize);
    *g
        .offset(
            45 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(41 as i32 as isize)
        + 2 as i32 as f64 * *b01.offset(1 as i32 as isize)
            * *g.offset(37 as i32 as isize);
    *g
        .offset(
            46 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(42 as i32 as isize)
        + 2 as i32 as f64 * *b01.offset(2 as i32 as isize)
            * *g.offset(38 as i32 as isize);
    *g
        .offset(
            47 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(43 as i32 as isize)
        + 2 as i32 as f64 * *b01.offset(3 as i32 as isize)
            * *g.offset(39 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0010(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(4 as i32 as isize) = 1 as i32 as f64;
    *g.offset(5 as i32 as isize) = 1 as i32 as f64;
    *g.offset(6 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(7 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(8 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(9 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0011(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    let mut xkxl: f64 = (*envs).rkrl[0 as i32 as usize];
    let mut ykyl: f64 = (*envs).rkrl[1 as i32 as usize];
    let mut zkzl: f64 = (*envs).rkrl[2 as i32 as usize];
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(8 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(9 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(10 as i32 as isize) = *cpx.offset(2 as i32 as isize);
    *g.offset(11 as i32 as isize) = *cpx.offset(3 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (xkxl + *cpx.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            13 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (xkxl + *cpx.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * (xkxl + *cpx.offset(2 as i32 as isize))
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * (xkxl + *cpx.offset(3 as i32 as isize))
        + *b01.offset(3 as i32 as isize);
    *g.offset(4 as i32 as isize) = xkxl + *cpx.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = xkxl + *cpx.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = xkxl + *cpx.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = xkxl + *cpx.offset(3 as i32 as isize);
    *g.offset(24 as i32 as isize) = 1 as i32 as f64;
    *g.offset(25 as i32 as isize) = 1 as i32 as f64;
    *g.offset(26 as i32 as isize) = 1 as i32 as f64;
    *g.offset(27 as i32 as isize) = 1 as i32 as f64;
    *g.offset(32 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(33 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(34 as i32 as isize) = *cpy.offset(2 as i32 as isize);
    *g.offset(35 as i32 as isize) = *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            36 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (ykyl + *cpy.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            37 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (ykyl + *cpy.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            38 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * (ykyl + *cpy.offset(2 as i32 as isize))
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            39 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * (ykyl + *cpy.offset(3 as i32 as isize))
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = ykyl + *cpy.offset(0 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = ykyl + *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = ykyl + *cpy.offset(2 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = ykyl + *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            56 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            57 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            58 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(50 as i32 as isize);
    *g
        .offset(
            59 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(51 as i32 as isize);
    *g
        .offset(
            60 as i32 as isize,
        ) = *g.offset(56 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            61 as i32 as isize,
        ) = *g.offset(57 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            62 as i32 as isize,
        ) = *g.offset(58 as i32 as isize)
        * (zkzl + *cpz.offset(2 as i32 as isize))
        + *b01.offset(2 as i32 as isize) * *g.offset(50 as i32 as isize);
    *g
        .offset(
            63 as i32 as isize,
        ) = *g.offset(59 as i32 as isize)
        * (zkzl + *cpz.offset(3 as i32 as isize))
        + *b01.offset(3 as i32 as isize) * *g.offset(51 as i32 as isize);
    *g
        .offset(
            52 as i32 as isize,
        ) = *g.offset(48 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize));
    *g
        .offset(
            53 as i32 as isize,
        ) = *g.offset(49 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize));
    *g
        .offset(
            54 as i32 as isize,
        ) = *g.offset(50 as i32 as isize)
        * (zkzl + *cpz.offset(2 as i32 as isize));
    *g
        .offset(
            55 as i32 as isize,
        ) = *g.offset(51 as i32 as isize)
        * (zkzl + *cpz.offset(3 as i32 as isize));
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0012(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    let mut xkxl: f64 = (*envs).rkrl[0 as i32 as usize];
    let mut ykyl: f64 = (*envs).rkrl[1 as i32 as usize];
    let mut zkzl: f64 = (*envs).rkrl[2 as i32 as usize];
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(8 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(9 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(10 as i32 as isize) = *cpx.offset(2 as i32 as isize);
    *g.offset(11 as i32 as isize) = *cpx.offset(3 as i32 as isize);
    *g
        .offset(
            16 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *cpx.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            17 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *cpx.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            18 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *cpx.offset(2 as i32 as isize)
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *cpx.offset(3 as i32 as isize)
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *g.offset(16 as i32 as isize)
        * (xkxl + *cpx.offset(0 as i32 as isize))
        + *cpx.offset(0 as i32 as isize) * 2 as i32 as f64
            * *b01.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *g.offset(17 as i32 as isize)
        * (xkxl + *cpx.offset(1 as i32 as isize))
        + *cpx.offset(1 as i32 as isize) * 2 as i32 as f64
            * *b01.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *g.offset(18 as i32 as isize)
        * (xkxl + *cpx.offset(2 as i32 as isize))
        + *cpx.offset(2 as i32 as isize) * 2 as i32 as f64
            * *b01.offset(2 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *g.offset(19 as i32 as isize)
        * (xkxl + *cpx.offset(3 as i32 as isize))
        + *cpx.offset(3 as i32 as isize) * 2 as i32 as f64
            * *b01.offset(3 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (xkxl + *cpx.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            13 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (xkxl + *cpx.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * (xkxl + *cpx.offset(2 as i32 as isize))
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * (xkxl + *cpx.offset(3 as i32 as isize))
        + *b01.offset(3 as i32 as isize);
    *g.offset(4 as i32 as isize) = xkxl + *cpx.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = xkxl + *cpx.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = xkxl + *cpx.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = xkxl + *cpx.offset(3 as i32 as isize);
    *g.offset(32 as i32 as isize) = 1 as i32 as f64;
    *g.offset(33 as i32 as isize) = 1 as i32 as f64;
    *g.offset(34 as i32 as isize) = 1 as i32 as f64;
    *g.offset(35 as i32 as isize) = 1 as i32 as f64;
    *g.offset(40 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(41 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(42 as i32 as isize) = *cpy.offset(2 as i32 as isize);
    *g.offset(43 as i32 as isize) = *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            48 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *cpy.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            49 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *cpy.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            50 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *cpy.offset(2 as i32 as isize)
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            51 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *cpy.offset(3 as i32 as isize)
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            52 as i32 as isize,
        ) = *g.offset(48 as i32 as isize)
        * (ykyl + *cpy.offset(0 as i32 as isize))
        + *cpy.offset(0 as i32 as isize) * 2 as i32 as f64
            * *b01.offset(0 as i32 as isize);
    *g
        .offset(
            53 as i32 as isize,
        ) = *g.offset(49 as i32 as isize)
        * (ykyl + *cpy.offset(1 as i32 as isize))
        + *cpy.offset(1 as i32 as isize) * 2 as i32 as f64
            * *b01.offset(1 as i32 as isize);
    *g
        .offset(
            54 as i32 as isize,
        ) = *g.offset(50 as i32 as isize)
        * (ykyl + *cpy.offset(2 as i32 as isize))
        + *cpy.offset(2 as i32 as isize) * 2 as i32 as f64
            * *b01.offset(2 as i32 as isize);
    *g
        .offset(
            55 as i32 as isize,
        ) = *g.offset(51 as i32 as isize)
        * (ykyl + *cpy.offset(3 as i32 as isize))
        + *cpy.offset(3 as i32 as isize) * 2 as i32 as f64
            * *b01.offset(3 as i32 as isize);
    *g
        .offset(
            44 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (ykyl + *cpy.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            45 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (ykyl + *cpy.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            46 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * (ykyl + *cpy.offset(2 as i32 as isize))
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            47 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * (ykyl + *cpy.offset(3 as i32 as isize))
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            36 as i32 as isize,
        ) = ykyl + *cpy.offset(0 as i32 as isize);
    *g
        .offset(
            37 as i32 as isize,
        ) = ykyl + *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            38 as i32 as isize,
        ) = ykyl + *cpy.offset(2 as i32 as isize);
    *g
        .offset(
            39 as i32 as isize,
        ) = ykyl + *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            72 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(64 as i32 as isize);
    *g
        .offset(
            73 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(65 as i32 as isize);
    *g
        .offset(
            74 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(66 as i32 as isize);
    *g
        .offset(
            75 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(67 as i32 as isize);
    *g
        .offset(
            80 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(72 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(64 as i32 as isize);
    *g
        .offset(
            81 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(73 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(65 as i32 as isize);
    *g
        .offset(
            82 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(74 as i32 as isize)
        + *b01.offset(2 as i32 as isize) * *g.offset(66 as i32 as isize);
    *g
        .offset(
            83 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(75 as i32 as isize)
        + *b01.offset(3 as i32 as isize) * *g.offset(67 as i32 as isize);
    *g
        .offset(
            84 as i32 as isize,
        ) = *g.offset(80 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize))
        + 2 as i32 as f64 * *b01.offset(0 as i32 as isize)
            * *g.offset(72 as i32 as isize);
    *g
        .offset(
            85 as i32 as isize,
        ) = *g.offset(81 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize))
        + 2 as i32 as f64 * *b01.offset(1 as i32 as isize)
            * *g.offset(73 as i32 as isize);
    *g
        .offset(
            86 as i32 as isize,
        ) = *g.offset(82 as i32 as isize)
        * (zkzl + *cpz.offset(2 as i32 as isize))
        + 2 as i32 as f64 * *b01.offset(2 as i32 as isize)
            * *g.offset(74 as i32 as isize);
    *g
        .offset(
            87 as i32 as isize,
        ) = *g.offset(83 as i32 as isize)
        * (zkzl + *cpz.offset(3 as i32 as isize))
        + 2 as i32 as f64 * *b01.offset(3 as i32 as isize)
            * *g.offset(75 as i32 as isize);
    *g
        .offset(
            76 as i32 as isize,
        ) = *g.offset(72 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize) * *g.offset(64 as i32 as isize);
    *g
        .offset(
            77 as i32 as isize,
        ) = *g.offset(73 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize) * *g.offset(65 as i32 as isize);
    *g
        .offset(
            78 as i32 as isize,
        ) = *g.offset(74 as i32 as isize)
        * (zkzl + *cpz.offset(2 as i32 as isize))
        + *b01.offset(2 as i32 as isize) * *g.offset(66 as i32 as isize);
    *g
        .offset(
            79 as i32 as isize,
        ) = *g.offset(75 as i32 as isize)
        * (zkzl + *cpz.offset(3 as i32 as isize))
        + *b01.offset(3 as i32 as isize) * *g.offset(67 as i32 as isize);
    *g
        .offset(
            68 as i32 as isize,
        ) = *g.offset(64 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize));
    *g
        .offset(
            69 as i32 as isize,
        ) = *g.offset(65 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize));
    *g
        .offset(
            70 as i32 as isize,
        ) = *g.offset(66 as i32 as isize)
        * (zkzl + *cpz.offset(2 as i32 as isize));
    *g
        .offset(
            71 as i32 as isize,
        ) = *g.offset(67 as i32 as isize)
        * (zkzl + *cpz.offset(3 as i32 as isize));
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0020(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *cpx.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = *cpx.offset(3 as i32 as isize);
    *g
        .offset(
            8 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *cpx.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            9 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *cpx.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *cpx.offset(2 as i32 as isize)
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *cpx.offset(3 as i32 as isize)
        + *b01.offset(3 as i32 as isize);
    *g.offset(12 as i32 as isize) = 1 as i32 as f64;
    *g.offset(13 as i32 as isize) = 1 as i32 as f64;
    *g.offset(14 as i32 as isize) = 1 as i32 as f64;
    *g.offset(15 as i32 as isize) = 1 as i32 as f64;
    *g.offset(16 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(17 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(18 as i32 as isize) = *cpy.offset(2 as i32 as isize);
    *g.offset(19 as i32 as isize) = *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *cpy.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *cpy.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *cpy.offset(2 as i32 as isize)
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *cpy.offset(3 as i32 as isize)
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(24 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(25 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(26 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(27 as i32 as isize);
    *g
        .offset(
            32 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(28 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(24 as i32 as isize);
    *g
        .offset(
            33 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(29 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(25 as i32 as isize);
    *g
        .offset(
            34 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(30 as i32 as isize)
        + *b01.offset(2 as i32 as isize) * *g.offset(26 as i32 as isize);
    *g
        .offset(
            35 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(31 as i32 as isize)
        + *b01.offset(3 as i32 as isize) * *g.offset(27 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0021(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    let mut xkxl: f64 = (*envs).rkrl[0 as i32 as usize];
    let mut ykyl: f64 = (*envs).rkrl[1 as i32 as usize];
    let mut zkzl: f64 = (*envs).rkrl[2 as i32 as usize];
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *cpx.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = *cpx.offset(3 as i32 as isize);
    *g
        .offset(
            8 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *cpx.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            9 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *cpx.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *cpx.offset(2 as i32 as isize)
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *cpx.offset(3 as i32 as isize)
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            16 as i32 as isize,
        ) = xkxl + *cpx.offset(0 as i32 as isize);
    *g
        .offset(
            17 as i32 as isize,
        ) = xkxl + *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            18 as i32 as isize,
        ) = xkxl + *cpx.offset(2 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = xkxl + *cpx.offset(3 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (xkxl + *cpx.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (xkxl + *cpx.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * (xkxl + *cpx.offset(2 as i32 as isize))
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * (xkxl + *cpx.offset(3 as i32 as isize))
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            24 as i32 as isize,
        ) = *g.offset(8 as i32 as isize)
        * (xkxl + *cpx.offset(0 as i32 as isize))
        + *cpx.offset(0 as i32 as isize) * 2 as i32 as f64
            * *b01.offset(0 as i32 as isize);
    *g
        .offset(
            25 as i32 as isize,
        ) = *g.offset(9 as i32 as isize)
        * (xkxl + *cpx.offset(1 as i32 as isize))
        + *cpx.offset(1 as i32 as isize) * 2 as i32 as f64
            * *b01.offset(1 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = *g.offset(10 as i32 as isize)
        * (xkxl + *cpx.offset(2 as i32 as isize))
        + *cpx.offset(2 as i32 as isize) * 2 as i32 as f64
            * *b01.offset(2 as i32 as isize);
    *g
        .offset(
            27 as i32 as isize,
        ) = *g.offset(11 as i32 as isize)
        * (xkxl + *cpx.offset(3 as i32 as isize))
        + *cpx.offset(3 as i32 as isize) * 2 as i32 as f64
            * *b01.offset(3 as i32 as isize);
    *g.offset(32 as i32 as isize) = 1 as i32 as f64;
    *g.offset(33 as i32 as isize) = 1 as i32 as f64;
    *g.offset(34 as i32 as isize) = 1 as i32 as f64;
    *g.offset(35 as i32 as isize) = 1 as i32 as f64;
    *g.offset(36 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(37 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(38 as i32 as isize) = *cpy.offset(2 as i32 as isize);
    *g.offset(39 as i32 as isize) = *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            40 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *cpy.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            41 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *cpy.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            42 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *cpy.offset(2 as i32 as isize)
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            43 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *cpy.offset(3 as i32 as isize)
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            48 as i32 as isize,
        ) = ykyl + *cpy.offset(0 as i32 as isize);
    *g
        .offset(
            49 as i32 as isize,
        ) = ykyl + *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            50 as i32 as isize,
        ) = ykyl + *cpy.offset(2 as i32 as isize);
    *g
        .offset(
            51 as i32 as isize,
        ) = ykyl + *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            52 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (ykyl + *cpy.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            53 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (ykyl + *cpy.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            54 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * (ykyl + *cpy.offset(2 as i32 as isize))
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            55 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * (ykyl + *cpy.offset(3 as i32 as isize))
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            56 as i32 as isize,
        ) = *g.offset(40 as i32 as isize)
        * (ykyl + *cpy.offset(0 as i32 as isize))
        + *cpy.offset(0 as i32 as isize) * 2 as i32 as f64
            * *b01.offset(0 as i32 as isize);
    *g
        .offset(
            57 as i32 as isize,
        ) = *g.offset(41 as i32 as isize)
        * (ykyl + *cpy.offset(1 as i32 as isize))
        + *cpy.offset(1 as i32 as isize) * 2 as i32 as f64
            * *b01.offset(1 as i32 as isize);
    *g
        .offset(
            58 as i32 as isize,
        ) = *g.offset(42 as i32 as isize)
        * (ykyl + *cpy.offset(2 as i32 as isize))
        + *cpy.offset(2 as i32 as isize) * 2 as i32 as f64
            * *b01.offset(2 as i32 as isize);
    *g
        .offset(
            59 as i32 as isize,
        ) = *g.offset(43 as i32 as isize)
        * (ykyl + *cpy.offset(3 as i32 as isize))
        + *cpy.offset(3 as i32 as isize) * 2 as i32 as f64
            * *b01.offset(3 as i32 as isize);
    *g
        .offset(
            68 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(64 as i32 as isize);
    *g
        .offset(
            69 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(65 as i32 as isize);
    *g
        .offset(
            70 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(66 as i32 as isize);
    *g
        .offset(
            71 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(67 as i32 as isize);
    *g
        .offset(
            72 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(68 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(64 as i32 as isize);
    *g
        .offset(
            73 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(69 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(65 as i32 as isize);
    *g
        .offset(
            74 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(70 as i32 as isize)
        + *b01.offset(2 as i32 as isize) * *g.offset(66 as i32 as isize);
    *g
        .offset(
            75 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(71 as i32 as isize)
        + *b01.offset(3 as i32 as isize) * *g.offset(67 as i32 as isize);
    *g
        .offset(
            80 as i32 as isize,
        ) = *g.offset(64 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize));
    *g
        .offset(
            81 as i32 as isize,
        ) = *g.offset(65 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize));
    *g
        .offset(
            82 as i32 as isize,
        ) = *g.offset(66 as i32 as isize)
        * (zkzl + *cpz.offset(2 as i32 as isize));
    *g
        .offset(
            83 as i32 as isize,
        ) = *g.offset(67 as i32 as isize)
        * (zkzl + *cpz.offset(3 as i32 as isize));
    *g
        .offset(
            84 as i32 as isize,
        ) = *g.offset(68 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize) * *g.offset(64 as i32 as isize);
    *g
        .offset(
            85 as i32 as isize,
        ) = *g.offset(69 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize) * *g.offset(65 as i32 as isize);
    *g
        .offset(
            86 as i32 as isize,
        ) = *g.offset(70 as i32 as isize)
        * (zkzl + *cpz.offset(2 as i32 as isize))
        + *b01.offset(2 as i32 as isize) * *g.offset(66 as i32 as isize);
    *g
        .offset(
            87 as i32 as isize,
        ) = *g.offset(71 as i32 as isize)
        * (zkzl + *cpz.offset(3 as i32 as isize))
        + *b01.offset(3 as i32 as isize) * *g.offset(67 as i32 as isize);
    *g
        .offset(
            88 as i32 as isize,
        ) = *g.offset(72 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize))
        + 2 as i32 as f64 * *b01.offset(0 as i32 as isize)
            * *g.offset(68 as i32 as isize);
    *g
        .offset(
            89 as i32 as isize,
        ) = *g.offset(73 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize))
        + 2 as i32 as f64 * *b01.offset(1 as i32 as isize)
            * *g.offset(69 as i32 as isize);
    *g
        .offset(
            90 as i32 as isize,
        ) = *g.offset(74 as i32 as isize)
        * (zkzl + *cpz.offset(2 as i32 as isize))
        + 2 as i32 as f64 * *b01.offset(2 as i32 as isize)
            * *g.offset(70 as i32 as isize);
    *g
        .offset(
            91 as i32 as isize,
        ) = *g.offset(75 as i32 as isize)
        * (zkzl + *cpz.offset(3 as i32 as isize))
        + 2 as i32 as f64 * *b01.offset(3 as i32 as isize)
            * *g.offset(71 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0030(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *cpx.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = *cpx.offset(3 as i32 as isize);
    *g
        .offset(
            8 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *cpx.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            9 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *cpx.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *cpx.offset(2 as i32 as isize)
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *cpx.offset(3 as i32 as isize)
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (*g.offset(8 as i32 as isize)
            + 2 as i32 as f64
                * *b01.offset(0 as i32 as isize));
    *g
        .offset(
            13 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (*g.offset(9 as i32 as isize)
            + 2 as i32 as f64
                * *b01.offset(1 as i32 as isize));
    *g
        .offset(
            14 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * (*g.offset(10 as i32 as isize)
            + 2 as i32 as f64
                * *b01.offset(2 as i32 as isize));
    *g
        .offset(
            15 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * (*g.offset(11 as i32 as isize)
            + 2 as i32 as f64
                * *b01.offset(3 as i32 as isize));
    *g.offset(16 as i32 as isize) = 1 as i32 as f64;
    *g.offset(17 as i32 as isize) = 1 as i32 as f64;
    *g.offset(18 as i32 as isize) = 1 as i32 as f64;
    *g.offset(19 as i32 as isize) = 1 as i32 as f64;
    *g.offset(20 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(21 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(22 as i32 as isize) = *cpy.offset(2 as i32 as isize);
    *g.offset(23 as i32 as isize) = *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            24 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *cpy.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            25 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *cpy.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *cpy.offset(2 as i32 as isize)
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            27 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *cpy.offset(3 as i32 as isize)
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (*g.offset(24 as i32 as isize)
            + 2 as i32 as f64
                * *b01.offset(0 as i32 as isize));
    *g
        .offset(
            29 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (*g.offset(25 as i32 as isize)
            + 2 as i32 as f64
                * *b01.offset(1 as i32 as isize));
    *g
        .offset(
            30 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * (*g.offset(26 as i32 as isize)
            + 2 as i32 as f64
                * *b01.offset(2 as i32 as isize));
    *g
        .offset(
            31 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * (*g.offset(27 as i32 as isize)
            + 2 as i32 as f64
                * *b01.offset(3 as i32 as isize));
    *g
        .offset(
            36 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(32 as i32 as isize);
    *g
        .offset(
            37 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(33 as i32 as isize);
    *g
        .offset(
            38 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(34 as i32 as isize);
    *g
        .offset(
            39 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(35 as i32 as isize);
    *g
        .offset(
            40 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(36 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(32 as i32 as isize);
    *g
        .offset(
            41 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(37 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(33 as i32 as isize);
    *g
        .offset(
            42 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(38 as i32 as isize)
        + *b01.offset(2 as i32 as isize) * *g.offset(34 as i32 as isize);
    *g
        .offset(
            43 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(39 as i32 as isize)
        + *b01.offset(3 as i32 as isize) * *g.offset(35 as i32 as isize);
    *g
        .offset(
            44 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(40 as i32 as isize)
        + 2 as i32 as f64 * *b01.offset(0 as i32 as isize)
            * *g.offset(36 as i32 as isize);
    *g
        .offset(
            45 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(41 as i32 as isize)
        + 2 as i32 as f64 * *b01.offset(1 as i32 as isize)
            * *g.offset(37 as i32 as isize);
    *g
        .offset(
            46 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(42 as i32 as isize)
        + 2 as i32 as f64 * *b01.offset(2 as i32 as isize)
            * *g.offset(38 as i32 as isize);
    *g
        .offset(
            47 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(43 as i32 as isize)
        + 2 as i32 as f64 * *b01.offset(3 as i32 as isize)
            * *g.offset(39 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0100(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(4 as i32 as isize) = 1 as i32 as f64;
    *g.offset(5 as i32 as isize) = 1 as i32 as f64;
    *g.offset(6 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(7 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(8 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(9 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0101(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *cpx.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = *cpx.offset(3 as i32 as isize);
    *g.offset(8 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(9 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(10 as i32 as isize) = *c0x.offset(2 as i32 as isize);
    *g.offset(11 as i32 as isize) = *c0x.offset(3 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            13 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g.offset(16 as i32 as isize) = 1 as i32 as f64;
    *g.offset(17 as i32 as isize) = 1 as i32 as f64;
    *g.offset(18 as i32 as isize) = 1 as i32 as f64;
    *g.offset(19 as i32 as isize) = 1 as i32 as f64;
    *g.offset(20 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(21 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(22 as i32 as isize) = *cpy.offset(2 as i32 as isize);
    *g.offset(23 as i32 as isize) = *cpy.offset(3 as i32 as isize);
    *g.offset(24 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(25 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(26 as i32 as isize) = *c0y.offset(2 as i32 as isize);
    *g.offset(27 as i32 as isize) = *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            36 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(32 as i32 as isize);
    *g
        .offset(
            37 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(33 as i32 as isize);
    *g
        .offset(
            38 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(34 as i32 as isize);
    *g
        .offset(
            39 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(35 as i32 as isize);
    *g
        .offset(
            40 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(32 as i32 as isize);
    *g
        .offset(
            41 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(33 as i32 as isize);
    *g
        .offset(
            42 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(34 as i32 as isize);
    *g
        .offset(
            43 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(35 as i32 as isize);
    *g
        .offset(
            44 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(40 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(32 as i32 as isize);
    *g
        .offset(
            45 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(41 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(33 as i32 as isize);
    *g
        .offset(
            46 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(42 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(34 as i32 as isize);
    *g
        .offset(
            47 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(43 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(35 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0102(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *cpx.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = *cpx.offset(3 as i32 as isize);
    *g.offset(12 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(13 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(14 as i32 as isize) = *c0x.offset(2 as i32 as isize);
    *g.offset(15 as i32 as isize) = *c0x.offset(3 as i32 as isize);
    *g
        .offset(
            8 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *cpx.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            9 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *cpx.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *cpx.offset(2 as i32 as isize)
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *cpx.offset(3 as i32 as isize)
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            16 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            17 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            18 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (*g.offset(16 as i32 as isize)
            + *b00.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize)
            * *c0x.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (*g.offset(17 as i32 as isize)
            + *b00.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize)
            * *c0x.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * (*g.offset(18 as i32 as isize)
            + *b00.offset(2 as i32 as isize))
        + *b01.offset(2 as i32 as isize)
            * *c0x.offset(2 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * (*g.offset(19 as i32 as isize)
            + *b00.offset(3 as i32 as isize))
        + *b01.offset(3 as i32 as isize)
            * *c0x.offset(3 as i32 as isize);
    *g.offset(24 as i32 as isize) = 1 as i32 as f64;
    *g.offset(25 as i32 as isize) = 1 as i32 as f64;
    *g.offset(26 as i32 as isize) = 1 as i32 as f64;
    *g.offset(27 as i32 as isize) = 1 as i32 as f64;
    *g.offset(28 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(29 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(30 as i32 as isize) = *cpy.offset(2 as i32 as isize);
    *g.offset(31 as i32 as isize) = *cpy.offset(3 as i32 as isize);
    *g.offset(36 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(37 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(38 as i32 as isize) = *c0y.offset(2 as i32 as isize);
    *g.offset(39 as i32 as isize) = *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            32 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *cpy.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            33 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *cpy.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            34 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *cpy.offset(2 as i32 as isize)
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            35 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *cpy.offset(3 as i32 as isize)
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            40 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            41 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            42 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            43 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            44 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (*g.offset(40 as i32 as isize)
            + *b00.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize)
            * *c0y.offset(0 as i32 as isize);
    *g
        .offset(
            45 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (*g.offset(41 as i32 as isize)
            + *b00.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize)
            * *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            46 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * (*g.offset(42 as i32 as isize)
            + *b00.offset(2 as i32 as isize))
        + *b01.offset(2 as i32 as isize)
            * *c0y.offset(2 as i32 as isize);
    *g
        .offset(
            47 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * (*g.offset(43 as i32 as isize)
            + *b00.offset(3 as i32 as isize))
        + *b01.offset(3 as i32 as isize)
            * *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            52 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            53 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            54 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(50 as i32 as isize);
    *g
        .offset(
            55 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(51 as i32 as isize);
    *g
        .offset(
            60 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            61 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            62 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(50 as i32 as isize);
    *g
        .offset(
            63 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(51 as i32 as isize);
    *g
        .offset(
            56 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(52 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            57 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(53 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            58 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(54 as i32 as isize)
        + *b01.offset(2 as i32 as isize) * *g.offset(50 as i32 as isize);
    *g
        .offset(
            59 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(55 as i32 as isize)
        + *b01.offset(3 as i32 as isize) * *g.offset(51 as i32 as isize);
    *g
        .offset(
            64 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(60 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            65 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(61 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            66 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(62 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(50 as i32 as isize);
    *g
        .offset(
            67 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(63 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(51 as i32 as isize);
    *g
        .offset(
            68 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(64 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(60 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(52 as i32 as isize);
    *g
        .offset(
            69 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(65 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(61 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(53 as i32 as isize);
    *g
        .offset(
            70 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(66 as i32 as isize)
        + *b01.offset(2 as i32 as isize) * *g.offset(62 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(54 as i32 as isize);
    *g
        .offset(
            71 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(67 as i32 as isize)
        + *b01.offset(3 as i32 as isize) * *g.offset(63 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(55 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0110(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *cpx.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = *cpx.offset(3 as i32 as isize);
    *g.offset(8 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(9 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(10 as i32 as isize) = *c0x.offset(2 as i32 as isize);
    *g.offset(11 as i32 as isize) = *c0x.offset(3 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            13 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g.offset(16 as i32 as isize) = 1 as i32 as f64;
    *g.offset(17 as i32 as isize) = 1 as i32 as f64;
    *g.offset(18 as i32 as isize) = 1 as i32 as f64;
    *g.offset(19 as i32 as isize) = 1 as i32 as f64;
    *g.offset(20 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(21 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(22 as i32 as isize) = *cpy.offset(2 as i32 as isize);
    *g.offset(23 as i32 as isize) = *cpy.offset(3 as i32 as isize);
    *g.offset(24 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(25 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(26 as i32 as isize) = *c0y.offset(2 as i32 as isize);
    *g.offset(27 as i32 as isize) = *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            36 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(32 as i32 as isize);
    *g
        .offset(
            37 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(33 as i32 as isize);
    *g
        .offset(
            38 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(34 as i32 as isize);
    *g
        .offset(
            39 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(35 as i32 as isize);
    *g
        .offset(
            40 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(32 as i32 as isize);
    *g
        .offset(
            41 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(33 as i32 as isize);
    *g
        .offset(
            42 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(34 as i32 as isize);
    *g
        .offset(
            43 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(35 as i32 as isize);
    *g
        .offset(
            44 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(40 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(32 as i32 as isize);
    *g
        .offset(
            45 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(41 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(33 as i32 as isize);
    *g
        .offset(
            46 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(42 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(34 as i32 as isize);
    *g
        .offset(
            47 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(43 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(35 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0111(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    let mut xkxl: f64 = (*envs).rkrl[0 as i32 as usize];
    let mut ykyl: f64 = (*envs).rkrl[1 as i32 as usize];
    let mut zkzl: f64 = (*envs).rkrl[2 as i32 as usize];
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(24 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(25 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(26 as i32 as isize) = *c0x.offset(2 as i32 as isize);
    *g.offset(27 as i32 as isize) = *c0x.offset(3 as i32 as isize);
    *g.offset(8 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(9 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(10 as i32 as isize) = *cpx.offset(2 as i32 as isize);
    *g.offset(11 as i32 as isize) = *cpx.offset(3 as i32 as isize);
    *g
        .offset(
            32 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            33 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            34 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            35 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (xkxl + *cpx.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            13 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (xkxl + *cpx.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * (xkxl + *cpx.offset(2 as i32 as isize))
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * (xkxl + *cpx.offset(3 as i32 as isize))
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            36 as i32 as isize,
        ) = *g.offset(32 as i32 as isize)
        * (xkxl + *cpx.offset(0 as i32 as isize))
        + *cpx.offset(0 as i32 as isize) * *b00.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize)
            * *c0x.offset(0 as i32 as isize);
    *g
        .offset(
            37 as i32 as isize,
        ) = *g.offset(33 as i32 as isize)
        * (xkxl + *cpx.offset(1 as i32 as isize))
        + *cpx.offset(1 as i32 as isize) * *b00.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize)
            * *c0x.offset(1 as i32 as isize);
    *g
        .offset(
            38 as i32 as isize,
        ) = *g.offset(34 as i32 as isize)
        * (xkxl + *cpx.offset(2 as i32 as isize))
        + *cpx.offset(2 as i32 as isize) * *b00.offset(2 as i32 as isize)
        + *b01.offset(2 as i32 as isize)
            * *c0x.offset(2 as i32 as isize);
    *g
        .offset(
            39 as i32 as isize,
        ) = *g.offset(35 as i32 as isize)
        * (xkxl + *cpx.offset(3 as i32 as isize))
        + *cpx.offset(3 as i32 as isize) * *b00.offset(3 as i32 as isize)
        + *b01.offset(3 as i32 as isize)
            * *c0x.offset(3 as i32 as isize);
    *g.offset(4 as i32 as isize) = xkxl + *cpx.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = xkxl + *cpx.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = xkxl + *cpx.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = xkxl + *cpx.offset(3 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (xkxl + *cpx.offset(0 as i32 as isize))
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (xkxl + *cpx.offset(1 as i32 as isize))
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *c0x.offset(2 as i32 as isize)
        * (xkxl + *cpx.offset(2 as i32 as isize))
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *c0x.offset(3 as i32 as isize)
        * (xkxl + *cpx.offset(3 as i32 as isize))
        + *b00.offset(3 as i32 as isize);
    *g.offset(48 as i32 as isize) = 1 as i32 as f64;
    *g.offset(49 as i32 as isize) = 1 as i32 as f64;
    *g.offset(50 as i32 as isize) = 1 as i32 as f64;
    *g.offset(51 as i32 as isize) = 1 as i32 as f64;
    *g.offset(72 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(73 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(74 as i32 as isize) = *c0y.offset(2 as i32 as isize);
    *g.offset(75 as i32 as isize) = *c0y.offset(3 as i32 as isize);
    *g.offset(56 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(57 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(58 as i32 as isize) = *cpy.offset(2 as i32 as isize);
    *g.offset(59 as i32 as isize) = *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            80 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            81 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            82 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            83 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            60 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (ykyl + *cpy.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            61 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (ykyl + *cpy.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            62 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * (ykyl + *cpy.offset(2 as i32 as isize))
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            63 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * (ykyl + *cpy.offset(3 as i32 as isize))
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            84 as i32 as isize,
        ) = *g.offset(80 as i32 as isize)
        * (ykyl + *cpy.offset(0 as i32 as isize))
        + *cpy.offset(0 as i32 as isize) * *b00.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize)
            * *c0y.offset(0 as i32 as isize);
    *g
        .offset(
            85 as i32 as isize,
        ) = *g.offset(81 as i32 as isize)
        * (ykyl + *cpy.offset(1 as i32 as isize))
        + *cpy.offset(1 as i32 as isize) * *b00.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize)
            * *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            86 as i32 as isize,
        ) = *g.offset(82 as i32 as isize)
        * (ykyl + *cpy.offset(2 as i32 as isize))
        + *cpy.offset(2 as i32 as isize) * *b00.offset(2 as i32 as isize)
        + *b01.offset(2 as i32 as isize)
            * *c0y.offset(2 as i32 as isize);
    *g
        .offset(
            87 as i32 as isize,
        ) = *g.offset(83 as i32 as isize)
        * (ykyl + *cpy.offset(3 as i32 as isize))
        + *cpy.offset(3 as i32 as isize) * *b00.offset(3 as i32 as isize)
        + *b01.offset(3 as i32 as isize)
            * *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            52 as i32 as isize,
        ) = ykyl + *cpy.offset(0 as i32 as isize);
    *g
        .offset(
            53 as i32 as isize,
        ) = ykyl + *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            54 as i32 as isize,
        ) = ykyl + *cpy.offset(2 as i32 as isize);
    *g
        .offset(
            55 as i32 as isize,
        ) = ykyl + *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            76 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (ykyl + *cpy.offset(0 as i32 as isize))
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            77 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (ykyl + *cpy.offset(1 as i32 as isize))
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            78 as i32 as isize,
        ) = *c0y.offset(2 as i32 as isize)
        * (ykyl + *cpy.offset(2 as i32 as isize))
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            79 as i32 as isize,
        ) = *c0y.offset(3 as i32 as isize)
        * (ykyl + *cpy.offset(3 as i32 as isize))
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            120 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(96 as i32 as isize);
    *g
        .offset(
            121 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(97 as i32 as isize);
    *g
        .offset(
            122 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(98 as i32 as isize);
    *g
        .offset(
            123 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(99 as i32 as isize);
    *g
        .offset(
            104 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(96 as i32 as isize);
    *g
        .offset(
            105 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(97 as i32 as isize);
    *g
        .offset(
            106 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(98 as i32 as isize);
    *g
        .offset(
            107 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(99 as i32 as isize);
    *g
        .offset(
            128 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(120 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(96 as i32 as isize);
    *g
        .offset(
            129 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(121 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(97 as i32 as isize);
    *g
        .offset(
            130 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(122 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(98 as i32 as isize);
    *g
        .offset(
            131 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(123 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(99 as i32 as isize);
    *g
        .offset(
            108 as i32 as isize,
        ) = *g.offset(104 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize) * *g.offset(96 as i32 as isize);
    *g
        .offset(
            109 as i32 as isize,
        ) = *g.offset(105 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize) * *g.offset(97 as i32 as isize);
    *g
        .offset(
            110 as i32 as isize,
        ) = *g.offset(106 as i32 as isize)
        * (zkzl + *cpz.offset(2 as i32 as isize))
        + *b01.offset(2 as i32 as isize) * *g.offset(98 as i32 as isize);
    *g
        .offset(
            111 as i32 as isize,
        ) = *g.offset(107 as i32 as isize)
        * (zkzl + *cpz.offset(3 as i32 as isize))
        + *b01.offset(3 as i32 as isize) * *g.offset(99 as i32 as isize);
    *g
        .offset(
            132 as i32 as isize,
        ) = *g.offset(128 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize) * *g.offset(120 as i32 as isize)
        + *b00.offset(0 as i32 as isize)
            * *g.offset(104 as i32 as isize);
    *g
        .offset(
            133 as i32 as isize,
        ) = *g.offset(129 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize) * *g.offset(121 as i32 as isize)
        + *b00.offset(1 as i32 as isize)
            * *g.offset(105 as i32 as isize);
    *g
        .offset(
            134 as i32 as isize,
        ) = *g.offset(130 as i32 as isize)
        * (zkzl + *cpz.offset(2 as i32 as isize))
        + *b01.offset(2 as i32 as isize) * *g.offset(122 as i32 as isize)
        + *b00.offset(2 as i32 as isize)
            * *g.offset(106 as i32 as isize);
    *g
        .offset(
            135 as i32 as isize,
        ) = *g.offset(131 as i32 as isize)
        * (zkzl + *cpz.offset(3 as i32 as isize))
        + *b01.offset(3 as i32 as isize) * *g.offset(123 as i32 as isize)
        + *b00.offset(3 as i32 as isize)
            * *g.offset(107 as i32 as isize);
    *g
        .offset(
            100 as i32 as isize,
        ) = *g.offset(96 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize));
    *g
        .offset(
            101 as i32 as isize,
        ) = *g.offset(97 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize));
    *g
        .offset(
            102 as i32 as isize,
        ) = *g.offset(98 as i32 as isize)
        * (zkzl + *cpz.offset(2 as i32 as isize));
    *g
        .offset(
            103 as i32 as isize,
        ) = *g.offset(99 as i32 as isize)
        * (zkzl + *cpz.offset(3 as i32 as isize));
    *g
        .offset(
            124 as i32 as isize,
        ) = *g.offset(120 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize))
        + *b00.offset(0 as i32 as isize) * *g.offset(96 as i32 as isize);
    *g
        .offset(
            125 as i32 as isize,
        ) = *g.offset(121 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize))
        + *b00.offset(1 as i32 as isize) * *g.offset(97 as i32 as isize);
    *g
        .offset(
            126 as i32 as isize,
        ) = *g.offset(122 as i32 as isize)
        * (zkzl + *cpz.offset(2 as i32 as isize))
        + *b00.offset(2 as i32 as isize) * *g.offset(98 as i32 as isize);
    *g
        .offset(
            127 as i32 as isize,
        ) = *g.offset(123 as i32 as isize)
        * (zkzl + *cpz.offset(3 as i32 as isize))
        + *b00.offset(3 as i32 as isize) * *g.offset(99 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0120(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *cpx.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = *cpx.offset(3 as i32 as isize);
    *g.offset(12 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(13 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(14 as i32 as isize) = *c0x.offset(2 as i32 as isize);
    *g.offset(15 as i32 as isize) = *c0x.offset(3 as i32 as isize);
    *g
        .offset(
            8 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *cpx.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            9 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *cpx.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *cpx.offset(2 as i32 as isize)
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *cpx.offset(3 as i32 as isize)
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            16 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            17 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            18 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (*g.offset(16 as i32 as isize)
            + *b00.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize)
            * *c0x.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (*g.offset(17 as i32 as isize)
            + *b00.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize)
            * *c0x.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * (*g.offset(18 as i32 as isize)
            + *b00.offset(2 as i32 as isize))
        + *b01.offset(2 as i32 as isize)
            * *c0x.offset(2 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * (*g.offset(19 as i32 as isize)
            + *b00.offset(3 as i32 as isize))
        + *b01.offset(3 as i32 as isize)
            * *c0x.offset(3 as i32 as isize);
    *g.offset(24 as i32 as isize) = 1 as i32 as f64;
    *g.offset(25 as i32 as isize) = 1 as i32 as f64;
    *g.offset(26 as i32 as isize) = 1 as i32 as f64;
    *g.offset(27 as i32 as isize) = 1 as i32 as f64;
    *g.offset(28 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(29 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(30 as i32 as isize) = *cpy.offset(2 as i32 as isize);
    *g.offset(31 as i32 as isize) = *cpy.offset(3 as i32 as isize);
    *g.offset(36 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(37 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(38 as i32 as isize) = *c0y.offset(2 as i32 as isize);
    *g.offset(39 as i32 as isize) = *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            32 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *cpy.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            33 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *cpy.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            34 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *cpy.offset(2 as i32 as isize)
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            35 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *cpy.offset(3 as i32 as isize)
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            40 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            41 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            42 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            43 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            44 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (*g.offset(40 as i32 as isize)
            + *b00.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize)
            * *c0y.offset(0 as i32 as isize);
    *g
        .offset(
            45 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (*g.offset(41 as i32 as isize)
            + *b00.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize)
            * *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            46 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * (*g.offset(42 as i32 as isize)
            + *b00.offset(2 as i32 as isize))
        + *b01.offset(2 as i32 as isize)
            * *c0y.offset(2 as i32 as isize);
    *g
        .offset(
            47 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * (*g.offset(43 as i32 as isize)
            + *b00.offset(3 as i32 as isize))
        + *b01.offset(3 as i32 as isize)
            * *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            52 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            53 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            54 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(50 as i32 as isize);
    *g
        .offset(
            55 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(51 as i32 as isize);
    *g
        .offset(
            60 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            61 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            62 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(50 as i32 as isize);
    *g
        .offset(
            63 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(51 as i32 as isize);
    *g
        .offset(
            56 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(52 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            57 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(53 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            58 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(54 as i32 as isize)
        + *b01.offset(2 as i32 as isize) * *g.offset(50 as i32 as isize);
    *g
        .offset(
            59 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(55 as i32 as isize)
        + *b01.offset(3 as i32 as isize) * *g.offset(51 as i32 as isize);
    *g
        .offset(
            64 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(60 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            65 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(61 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            66 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(62 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(50 as i32 as isize);
    *g
        .offset(
            67 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(63 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(51 as i32 as isize);
    *g
        .offset(
            68 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(64 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(60 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(52 as i32 as isize);
    *g
        .offset(
            69 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(65 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(61 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(53 as i32 as isize);
    *g
        .offset(
            70 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(66 as i32 as isize)
        + *b01.offset(2 as i32 as isize) * *g.offset(62 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(54 as i32 as isize);
    *g
        .offset(
            71 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(67 as i32 as isize)
        + *b01.offset(3 as i32 as isize) * *g.offset(63 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(55 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0200(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *c0x.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = *c0x.offset(3 as i32 as isize);
    *g
        .offset(
            8 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            9 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *c0x.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *c0x.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b10.offset(3 as i32 as isize);
    *g.offset(12 as i32 as isize) = 1 as i32 as f64;
    *g.offset(13 as i32 as isize) = 1 as i32 as f64;
    *g.offset(14 as i32 as isize) = 1 as i32 as f64;
    *g.offset(15 as i32 as isize) = 1 as i32 as f64;
    *g.offset(16 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(17 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(18 as i32 as isize) = *c0y.offset(2 as i32 as isize);
    *g.offset(19 as i32 as isize) = *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *c0y.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *c0y.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b10.offset(3 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(24 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(25 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(26 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(27 as i32 as isize);
    *g
        .offset(
            32 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(28 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(24 as i32 as isize);
    *g
        .offset(
            33 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(29 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(25 as i32 as isize);
    *g
        .offset(
            34 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(30 as i32 as isize)
        + *b10.offset(2 as i32 as isize) * *g.offset(26 as i32 as isize);
    *g
        .offset(
            35 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(31 as i32 as isize)
        + *b10.offset(3 as i32 as isize) * *g.offset(27 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0201(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(8 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(9 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(10 as i32 as isize) = *c0x.offset(2 as i32 as isize);
    *g.offset(11 as i32 as isize) = *c0x.offset(3 as i32 as isize);
    *g
        .offset(
            16 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            17 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            18 as i32 as isize,
        ) = *c0x.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = *c0x.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b10.offset(3 as i32 as isize);
    *g.offset(4 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *cpx.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = *cpx.offset(3 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            13 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (*g.offset(12 as i32 as isize)
            + *b00.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize)
            * *cpx.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (*g.offset(13 as i32 as isize)
            + *b00.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize)
            * *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *c0x.offset(2 as i32 as isize)
        * (*g.offset(14 as i32 as isize)
            + *b00.offset(2 as i32 as isize))
        + *b10.offset(2 as i32 as isize)
            * *cpx.offset(2 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *c0x.offset(3 as i32 as isize)
        * (*g.offset(15 as i32 as isize)
            + *b00.offset(3 as i32 as isize))
        + *b10.offset(3 as i32 as isize)
            * *cpx.offset(3 as i32 as isize);
    *g.offset(24 as i32 as isize) = 1 as i32 as f64;
    *g.offset(25 as i32 as isize) = 1 as i32 as f64;
    *g.offset(26 as i32 as isize) = 1 as i32 as f64;
    *g.offset(27 as i32 as isize) = 1 as i32 as f64;
    *g.offset(32 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(33 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(34 as i32 as isize) = *c0y.offset(2 as i32 as isize);
    *g.offset(35 as i32 as isize) = *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            40 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            41 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            42 as i32 as isize,
        ) = *c0y.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            43 as i32 as isize,
        ) = *c0y.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b10.offset(3 as i32 as isize);
    *g.offset(28 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(29 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(30 as i32 as isize) = *cpy.offset(2 as i32 as isize);
    *g.offset(31 as i32 as isize) = *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            36 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            37 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            38 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            39 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            44 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (*g.offset(36 as i32 as isize)
            + *b00.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize)
            * *cpy.offset(0 as i32 as isize);
    *g
        .offset(
            45 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (*g.offset(37 as i32 as isize)
            + *b00.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize)
            * *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            46 as i32 as isize,
        ) = *c0y.offset(2 as i32 as isize)
        * (*g.offset(38 as i32 as isize)
            + *b00.offset(2 as i32 as isize))
        + *b10.offset(2 as i32 as isize)
            * *cpy.offset(2 as i32 as isize);
    *g
        .offset(
            47 as i32 as isize,
        ) = *c0y.offset(3 as i32 as isize)
        * (*g.offset(39 as i32 as isize)
            + *b00.offset(3 as i32 as isize))
        + *b10.offset(3 as i32 as isize)
            * *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            56 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            57 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            58 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(50 as i32 as isize);
    *g
        .offset(
            59 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(51 as i32 as isize);
    *g
        .offset(
            64 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(56 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            65 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(57 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            66 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(58 as i32 as isize)
        + *b10.offset(2 as i32 as isize) * *g.offset(50 as i32 as isize);
    *g
        .offset(
            67 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(59 as i32 as isize)
        + *b10.offset(3 as i32 as isize) * *g.offset(51 as i32 as isize);
    *g
        .offset(
            52 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            53 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            54 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(50 as i32 as isize);
    *g
        .offset(
            55 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(51 as i32 as isize);
    *g
        .offset(
            60 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(56 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            61 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(57 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            62 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(58 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(50 as i32 as isize);
    *g
        .offset(
            63 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(59 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(51 as i32 as isize);
    *g
        .offset(
            68 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(60 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(52 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(56 as i32 as isize);
    *g
        .offset(
            69 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(61 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(53 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(57 as i32 as isize);
    *g
        .offset(
            70 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(62 as i32 as isize)
        + *b10.offset(2 as i32 as isize) * *g.offset(54 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(58 as i32 as isize);
    *g
        .offset(
            71 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(63 as i32 as isize)
        + *b10.offset(3 as i32 as isize) * *g.offset(55 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(59 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0210(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *cpx.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = *cpx.offset(3 as i32 as isize);
    *g.offset(8 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(9 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(10 as i32 as isize) = *c0x.offset(2 as i32 as isize);
    *g.offset(11 as i32 as isize) = *c0x.offset(3 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            13 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            16 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            17 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            18 as i32 as isize,
        ) = *c0x.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = *c0x.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b10.offset(3 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (*g.offset(12 as i32 as isize)
            + *b00.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize)
            * *cpx.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (*g.offset(13 as i32 as isize)
            + *b00.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize)
            * *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *c0x.offset(2 as i32 as isize)
        * (*g.offset(14 as i32 as isize)
            + *b00.offset(2 as i32 as isize))
        + *b10.offset(2 as i32 as isize)
            * *cpx.offset(2 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *c0x.offset(3 as i32 as isize)
        * (*g.offset(15 as i32 as isize)
            + *b00.offset(3 as i32 as isize))
        + *b10.offset(3 as i32 as isize)
            * *cpx.offset(3 as i32 as isize);
    *g.offset(24 as i32 as isize) = 1 as i32 as f64;
    *g.offset(25 as i32 as isize) = 1 as i32 as f64;
    *g.offset(26 as i32 as isize) = 1 as i32 as f64;
    *g.offset(27 as i32 as isize) = 1 as i32 as f64;
    *g.offset(28 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(29 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(30 as i32 as isize) = *cpy.offset(2 as i32 as isize);
    *g.offset(31 as i32 as isize) = *cpy.offset(3 as i32 as isize);
    *g.offset(32 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(33 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(34 as i32 as isize) = *c0y.offset(2 as i32 as isize);
    *g.offset(35 as i32 as isize) = *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            36 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            37 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            38 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            39 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            40 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            41 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            42 as i32 as isize,
        ) = *c0y.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            43 as i32 as isize,
        ) = *c0y.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b10.offset(3 as i32 as isize);
    *g
        .offset(
            44 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (*g.offset(36 as i32 as isize)
            + *b00.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize)
            * *cpy.offset(0 as i32 as isize);
    *g
        .offset(
            45 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (*g.offset(37 as i32 as isize)
            + *b00.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize)
            * *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            46 as i32 as isize,
        ) = *c0y.offset(2 as i32 as isize)
        * (*g.offset(38 as i32 as isize)
            + *b00.offset(2 as i32 as isize))
        + *b10.offset(2 as i32 as isize)
            * *cpy.offset(2 as i32 as isize);
    *g
        .offset(
            47 as i32 as isize,
        ) = *c0y.offset(3 as i32 as isize)
        * (*g.offset(39 as i32 as isize)
            + *b00.offset(3 as i32 as isize))
        + *b10.offset(3 as i32 as isize)
            * *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            52 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            53 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            54 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(50 as i32 as isize);
    *g
        .offset(
            55 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(51 as i32 as isize);
    *g
        .offset(
            56 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            57 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            58 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(50 as i32 as isize);
    *g
        .offset(
            59 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(51 as i32 as isize);
    *g
        .offset(
            60 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(56 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            61 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(57 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            62 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(58 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(50 as i32 as isize);
    *g
        .offset(
            63 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(59 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(51 as i32 as isize);
    *g
        .offset(
            64 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(56 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            65 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(57 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            66 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(58 as i32 as isize)
        + *b10.offset(2 as i32 as isize) * *g.offset(50 as i32 as isize);
    *g
        .offset(
            67 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(59 as i32 as isize)
        + *b10.offset(3 as i32 as isize) * *g.offset(51 as i32 as isize);
    *g
        .offset(
            68 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(60 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(52 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(56 as i32 as isize);
    *g
        .offset(
            69 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(61 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(53 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(57 as i32 as isize);
    *g
        .offset(
            70 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(62 as i32 as isize)
        + *b10.offset(2 as i32 as isize) * *g.offset(54 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(58 as i32 as isize);
    *g
        .offset(
            71 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(63 as i32 as isize)
        + *b10.offset(3 as i32 as isize) * *g.offset(55 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(59 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_0300(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *c0x.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = *c0x.offset(3 as i32 as isize);
    *g
        .offset(
            8 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            9 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *c0x.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *c0x.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b10.offset(3 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (*g.offset(8 as i32 as isize)
            + 2 as i32 as f64
                * *b10.offset(0 as i32 as isize));
    *g
        .offset(
            13 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (*g.offset(9 as i32 as isize)
            + 2 as i32 as f64
                * *b10.offset(1 as i32 as isize));
    *g
        .offset(
            14 as i32 as isize,
        ) = *c0x.offset(2 as i32 as isize)
        * (*g.offset(10 as i32 as isize)
            + 2 as i32 as f64
                * *b10.offset(2 as i32 as isize));
    *g
        .offset(
            15 as i32 as isize,
        ) = *c0x.offset(3 as i32 as isize)
        * (*g.offset(11 as i32 as isize)
            + 2 as i32 as f64
                * *b10.offset(3 as i32 as isize));
    *g.offset(16 as i32 as isize) = 1 as i32 as f64;
    *g.offset(17 as i32 as isize) = 1 as i32 as f64;
    *g.offset(18 as i32 as isize) = 1 as i32 as f64;
    *g.offset(19 as i32 as isize) = 1 as i32 as f64;
    *g.offset(20 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(21 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(22 as i32 as isize) = *c0y.offset(2 as i32 as isize);
    *g.offset(23 as i32 as isize) = *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            24 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            25 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = *c0y.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            27 as i32 as isize,
        ) = *c0y.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b10.offset(3 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (*g.offset(24 as i32 as isize)
            + 2 as i32 as f64
                * *b10.offset(0 as i32 as isize));
    *g
        .offset(
            29 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (*g.offset(25 as i32 as isize)
            + 2 as i32 as f64
                * *b10.offset(1 as i32 as isize));
    *g
        .offset(
            30 as i32 as isize,
        ) = *c0y.offset(2 as i32 as isize)
        * (*g.offset(26 as i32 as isize)
            + 2 as i32 as f64
                * *b10.offset(2 as i32 as isize));
    *g
        .offset(
            31 as i32 as isize,
        ) = *c0y.offset(3 as i32 as isize)
        * (*g.offset(27 as i32 as isize)
            + 2 as i32 as f64
                * *b10.offset(3 as i32 as isize));
    *g
        .offset(
            36 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(32 as i32 as isize);
    *g
        .offset(
            37 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(33 as i32 as isize);
    *g
        .offset(
            38 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(34 as i32 as isize);
    *g
        .offset(
            39 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(35 as i32 as isize);
    *g
        .offset(
            40 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(36 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(32 as i32 as isize);
    *g
        .offset(
            41 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(37 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(33 as i32 as isize);
    *g
        .offset(
            42 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(38 as i32 as isize)
        + *b10.offset(2 as i32 as isize) * *g.offset(34 as i32 as isize);
    *g
        .offset(
            43 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(39 as i32 as isize)
        + *b10.offset(3 as i32 as isize) * *g.offset(35 as i32 as isize);
    *g
        .offset(
            44 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(40 as i32 as isize)
        + 2 as i32 as f64 * *b10.offset(0 as i32 as isize)
            * *g.offset(36 as i32 as isize);
    *g
        .offset(
            45 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(41 as i32 as isize)
        + 2 as i32 as f64 * *b10.offset(1 as i32 as isize)
            * *g.offset(37 as i32 as isize);
    *g
        .offset(
            46 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(42 as i32 as isize)
        + 2 as i32 as f64 * *b10.offset(2 as i32 as isize)
            * *g.offset(38 as i32 as isize);
    *g
        .offset(
            47 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(43 as i32 as isize)
        + 2 as i32 as f64 * *b10.offset(3 as i32 as isize)
            * *g.offset(39 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1000(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(3 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(4 as i32 as isize) = 1 as i32 as f64;
    *g.offset(5 as i32 as isize) = 1 as i32 as f64;
    *g.offset(6 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(7 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(8 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(9 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1001(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *c0x.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = *c0x.offset(3 as i32 as isize);
    *g.offset(8 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(9 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(10 as i32 as isize) = *cpx.offset(2 as i32 as isize);
    *g.offset(11 as i32 as isize) = *cpx.offset(3 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            13 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g.offset(16 as i32 as isize) = 1 as i32 as f64;
    *g.offset(17 as i32 as isize) = 1 as i32 as f64;
    *g.offset(18 as i32 as isize) = 1 as i32 as f64;
    *g.offset(19 as i32 as isize) = 1 as i32 as f64;
    *g.offset(20 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(21 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(22 as i32 as isize) = *c0y.offset(2 as i32 as isize);
    *g.offset(23 as i32 as isize) = *c0y.offset(3 as i32 as isize);
    *g.offset(24 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(25 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(26 as i32 as isize) = *cpy.offset(2 as i32 as isize);
    *g.offset(27 as i32 as isize) = *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            36 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(32 as i32 as isize);
    *g
        .offset(
            37 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(33 as i32 as isize);
    *g
        .offset(
            38 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(34 as i32 as isize);
    *g
        .offset(
            39 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(35 as i32 as isize);
    *g
        .offset(
            40 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(32 as i32 as isize);
    *g
        .offset(
            41 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(33 as i32 as isize);
    *g
        .offset(
            42 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(34 as i32 as isize);
    *g
        .offset(
            43 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(35 as i32 as isize);
    *g
        .offset(
            44 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(36 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(32 as i32 as isize);
    *g
        .offset(
            45 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(37 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(33 as i32 as isize);
    *g
        .offset(
            46 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(38 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(34 as i32 as isize);
    *g
        .offset(
            47 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(39 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(35 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1002(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *c0x.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = *c0x.offset(3 as i32 as isize);
    *g.offset(8 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(9 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(10 as i32 as isize) = *cpx.offset(2 as i32 as isize);
    *g.offset(11 as i32 as isize) = *cpx.offset(3 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            13 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            16 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *cpx.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            17 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *cpx.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            18 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *cpx.offset(2 as i32 as isize)
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *cpx.offset(3 as i32 as isize)
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (*g.offset(12 as i32 as isize)
            + *b00.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize)
            * *c0x.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (*g.offset(13 as i32 as isize)
            + *b00.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize)
            * *c0x.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * (*g.offset(14 as i32 as isize)
            + *b00.offset(2 as i32 as isize))
        + *b01.offset(2 as i32 as isize)
            * *c0x.offset(2 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * (*g.offset(15 as i32 as isize)
            + *b00.offset(3 as i32 as isize))
        + *b01.offset(3 as i32 as isize)
            * *c0x.offset(3 as i32 as isize);
    *g.offset(24 as i32 as isize) = 1 as i32 as f64;
    *g.offset(25 as i32 as isize) = 1 as i32 as f64;
    *g.offset(26 as i32 as isize) = 1 as i32 as f64;
    *g.offset(27 as i32 as isize) = 1 as i32 as f64;
    *g.offset(28 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(29 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(30 as i32 as isize) = *c0y.offset(2 as i32 as isize);
    *g.offset(31 as i32 as isize) = *c0y.offset(3 as i32 as isize);
    *g.offset(32 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(33 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(34 as i32 as isize) = *cpy.offset(2 as i32 as isize);
    *g.offset(35 as i32 as isize) = *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            36 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            37 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            38 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            39 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            40 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *cpy.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            41 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *cpy.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            42 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *cpy.offset(2 as i32 as isize)
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            43 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *cpy.offset(3 as i32 as isize)
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            44 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (*g.offset(36 as i32 as isize)
            + *b00.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize)
            * *c0y.offset(0 as i32 as isize);
    *g
        .offset(
            45 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (*g.offset(37 as i32 as isize)
            + *b00.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize)
            * *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            46 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * (*g.offset(38 as i32 as isize)
            + *b00.offset(2 as i32 as isize))
        + *b01.offset(2 as i32 as isize)
            * *c0y.offset(2 as i32 as isize);
    *g
        .offset(
            47 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * (*g.offset(39 as i32 as isize)
            + *b00.offset(3 as i32 as isize))
        + *b01.offset(3 as i32 as isize)
            * *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            52 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            53 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            54 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(50 as i32 as isize);
    *g
        .offset(
            55 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(51 as i32 as isize);
    *g
        .offset(
            56 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            57 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            58 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(50 as i32 as isize);
    *g
        .offset(
            59 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(51 as i32 as isize);
    *g
        .offset(
            60 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(52 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            61 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(53 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            62 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(54 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(50 as i32 as isize);
    *g
        .offset(
            63 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(55 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(51 as i32 as isize);
    *g
        .offset(
            64 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(56 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            65 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(57 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            66 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(58 as i32 as isize)
        + *b01.offset(2 as i32 as isize) * *g.offset(50 as i32 as isize);
    *g
        .offset(
            67 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(59 as i32 as isize)
        + *b01.offset(3 as i32 as isize) * *g.offset(51 as i32 as isize);
    *g
        .offset(
            68 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(60 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(52 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(56 as i32 as isize);
    *g
        .offset(
            69 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(61 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(53 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(57 as i32 as isize);
    *g
        .offset(
            70 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(62 as i32 as isize)
        + *b01.offset(2 as i32 as isize) * *g.offset(54 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(58 as i32 as isize);
    *g
        .offset(
            71 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(63 as i32 as isize)
        + *b01.offset(3 as i32 as isize) * *g.offset(55 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(59 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1010(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *c0x.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = *c0x.offset(3 as i32 as isize);
    *g.offset(8 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(9 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(10 as i32 as isize) = *cpx.offset(2 as i32 as isize);
    *g.offset(11 as i32 as isize) = *cpx.offset(3 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            13 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g.offset(16 as i32 as isize) = 1 as i32 as f64;
    *g.offset(17 as i32 as isize) = 1 as i32 as f64;
    *g.offset(18 as i32 as isize) = 1 as i32 as f64;
    *g.offset(19 as i32 as isize) = 1 as i32 as f64;
    *g.offset(20 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(21 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(22 as i32 as isize) = *c0y.offset(2 as i32 as isize);
    *g.offset(23 as i32 as isize) = *c0y.offset(3 as i32 as isize);
    *g.offset(24 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(25 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(26 as i32 as isize) = *cpy.offset(2 as i32 as isize);
    *g.offset(27 as i32 as isize) = *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            36 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(32 as i32 as isize);
    *g
        .offset(
            37 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(33 as i32 as isize);
    *g
        .offset(
            38 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(34 as i32 as isize);
    *g
        .offset(
            39 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(35 as i32 as isize);
    *g
        .offset(
            40 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(32 as i32 as isize);
    *g
        .offset(
            41 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(33 as i32 as isize);
    *g
        .offset(
            42 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(34 as i32 as isize);
    *g
        .offset(
            43 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(35 as i32 as isize);
    *g
        .offset(
            44 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(36 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(32 as i32 as isize);
    *g
        .offset(
            45 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(37 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(33 as i32 as isize);
    *g
        .offset(
            46 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(38 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(34 as i32 as isize);
    *g
        .offset(
            47 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(39 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(35 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1011(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    let mut xkxl: f64 = (*envs).rkrl[0 as i32 as usize];
    let mut ykyl: f64 = (*envs).rkrl[1 as i32 as usize];
    let mut zkzl: f64 = (*envs).rkrl[2 as i32 as usize];
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *c0x.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = *c0x.offset(3 as i32 as isize);
    *g.offset(16 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(17 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(18 as i32 as isize) = *cpx.offset(2 as i32 as isize);
    *g.offset(19 as i32 as isize) = *cpx.offset(3 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            24 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (xkxl + *cpx.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            25 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (xkxl + *cpx.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * (xkxl + *cpx.offset(2 as i32 as isize))
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            27 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * (xkxl + *cpx.offset(3 as i32 as isize))
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *g.offset(20 as i32 as isize)
        * (xkxl + *cpx.offset(0 as i32 as isize))
        + *cpx.offset(0 as i32 as isize) * *b00.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize)
            * *c0x.offset(0 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = *g.offset(21 as i32 as isize)
        * (xkxl + *cpx.offset(1 as i32 as isize))
        + *cpx.offset(1 as i32 as isize) * *b00.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize)
            * *c0x.offset(1 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *g.offset(22 as i32 as isize)
        * (xkxl + *cpx.offset(2 as i32 as isize))
        + *cpx.offset(2 as i32 as isize) * *b00.offset(2 as i32 as isize)
        + *b01.offset(2 as i32 as isize)
            * *c0x.offset(2 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *g.offset(23 as i32 as isize)
        * (xkxl + *cpx.offset(3 as i32 as isize))
        + *cpx.offset(3 as i32 as isize) * *b00.offset(3 as i32 as isize)
        + *b01.offset(3 as i32 as isize)
            * *c0x.offset(3 as i32 as isize);
    *g.offset(8 as i32 as isize) = xkxl + *cpx.offset(0 as i32 as isize);
    *g.offset(9 as i32 as isize) = xkxl + *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = xkxl + *cpx.offset(2 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = xkxl + *cpx.offset(3 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (xkxl + *cpx.offset(0 as i32 as isize))
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            13 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (xkxl + *cpx.offset(1 as i32 as isize))
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *c0x.offset(2 as i32 as isize)
        * (xkxl + *cpx.offset(2 as i32 as isize))
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *c0x.offset(3 as i32 as isize)
        * (xkxl + *cpx.offset(3 as i32 as isize))
        + *b00.offset(3 as i32 as isize);
    *g.offset(48 as i32 as isize) = 1 as i32 as f64;
    *g.offset(49 as i32 as isize) = 1 as i32 as f64;
    *g.offset(50 as i32 as isize) = 1 as i32 as f64;
    *g.offset(51 as i32 as isize) = 1 as i32 as f64;
    *g.offset(52 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(53 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(54 as i32 as isize) = *c0y.offset(2 as i32 as isize);
    *g.offset(55 as i32 as isize) = *c0y.offset(3 as i32 as isize);
    *g.offset(64 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(65 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(66 as i32 as isize) = *cpy.offset(2 as i32 as isize);
    *g.offset(67 as i32 as isize) = *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            68 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            69 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            70 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            71 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            72 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (ykyl + *cpy.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            73 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (ykyl + *cpy.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            74 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * (ykyl + *cpy.offset(2 as i32 as isize))
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            75 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * (ykyl + *cpy.offset(3 as i32 as isize))
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            76 as i32 as isize,
        ) = *g.offset(68 as i32 as isize)
        * (ykyl + *cpy.offset(0 as i32 as isize))
        + *cpy.offset(0 as i32 as isize) * *b00.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize)
            * *c0y.offset(0 as i32 as isize);
    *g
        .offset(
            77 as i32 as isize,
        ) = *g.offset(69 as i32 as isize)
        * (ykyl + *cpy.offset(1 as i32 as isize))
        + *cpy.offset(1 as i32 as isize) * *b00.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize)
            * *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            78 as i32 as isize,
        ) = *g.offset(70 as i32 as isize)
        * (ykyl + *cpy.offset(2 as i32 as isize))
        + *cpy.offset(2 as i32 as isize) * *b00.offset(2 as i32 as isize)
        + *b01.offset(2 as i32 as isize)
            * *c0y.offset(2 as i32 as isize);
    *g
        .offset(
            79 as i32 as isize,
        ) = *g.offset(71 as i32 as isize)
        * (ykyl + *cpy.offset(3 as i32 as isize))
        + *cpy.offset(3 as i32 as isize) * *b00.offset(3 as i32 as isize)
        + *b01.offset(3 as i32 as isize)
            * *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            56 as i32 as isize,
        ) = ykyl + *cpy.offset(0 as i32 as isize);
    *g
        .offset(
            57 as i32 as isize,
        ) = ykyl + *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            58 as i32 as isize,
        ) = ykyl + *cpy.offset(2 as i32 as isize);
    *g
        .offset(
            59 as i32 as isize,
        ) = ykyl + *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            60 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (ykyl + *cpy.offset(0 as i32 as isize))
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            61 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (ykyl + *cpy.offset(1 as i32 as isize))
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            62 as i32 as isize,
        ) = *c0y.offset(2 as i32 as isize)
        * (ykyl + *cpy.offset(2 as i32 as isize))
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            63 as i32 as isize,
        ) = *c0y.offset(3 as i32 as isize)
        * (ykyl + *cpy.offset(3 as i32 as isize))
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            100 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(96 as i32 as isize);
    *g
        .offset(
            101 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(97 as i32 as isize);
    *g
        .offset(
            102 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(98 as i32 as isize);
    *g
        .offset(
            103 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(99 as i32 as isize);
    *g
        .offset(
            112 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(96 as i32 as isize);
    *g
        .offset(
            113 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(97 as i32 as isize);
    *g
        .offset(
            114 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(98 as i32 as isize);
    *g
        .offset(
            115 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(99 as i32 as isize);
    *g
        .offset(
            116 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(100 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(96 as i32 as isize);
    *g
        .offset(
            117 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(101 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(97 as i32 as isize);
    *g
        .offset(
            118 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(102 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(98 as i32 as isize);
    *g
        .offset(
            119 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(103 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(99 as i32 as isize);
    *g
        .offset(
            120 as i32 as isize,
        ) = *g.offset(112 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize) * *g.offset(96 as i32 as isize);
    *g
        .offset(
            121 as i32 as isize,
        ) = *g.offset(113 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize) * *g.offset(97 as i32 as isize);
    *g
        .offset(
            122 as i32 as isize,
        ) = *g.offset(114 as i32 as isize)
        * (zkzl + *cpz.offset(2 as i32 as isize))
        + *b01.offset(2 as i32 as isize) * *g.offset(98 as i32 as isize);
    *g
        .offset(
            123 as i32 as isize,
        ) = *g.offset(115 as i32 as isize)
        * (zkzl + *cpz.offset(3 as i32 as isize))
        + *b01.offset(3 as i32 as isize) * *g.offset(99 as i32 as isize);
    *g
        .offset(
            124 as i32 as isize,
        ) = *g.offset(116 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize) * *g.offset(100 as i32 as isize)
        + *b00.offset(0 as i32 as isize)
            * *g.offset(112 as i32 as isize);
    *g
        .offset(
            125 as i32 as isize,
        ) = *g.offset(117 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize) * *g.offset(101 as i32 as isize)
        + *b00.offset(1 as i32 as isize)
            * *g.offset(113 as i32 as isize);
    *g
        .offset(
            126 as i32 as isize,
        ) = *g.offset(118 as i32 as isize)
        * (zkzl + *cpz.offset(2 as i32 as isize))
        + *b01.offset(2 as i32 as isize) * *g.offset(102 as i32 as isize)
        + *b00.offset(2 as i32 as isize)
            * *g.offset(114 as i32 as isize);
    *g
        .offset(
            127 as i32 as isize,
        ) = *g.offset(119 as i32 as isize)
        * (zkzl + *cpz.offset(3 as i32 as isize))
        + *b01.offset(3 as i32 as isize) * *g.offset(103 as i32 as isize)
        + *b00.offset(3 as i32 as isize)
            * *g.offset(115 as i32 as isize);
    *g
        .offset(
            104 as i32 as isize,
        ) = *g.offset(96 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize));
    *g
        .offset(
            105 as i32 as isize,
        ) = *g.offset(97 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize));
    *g
        .offset(
            106 as i32 as isize,
        ) = *g.offset(98 as i32 as isize)
        * (zkzl + *cpz.offset(2 as i32 as isize));
    *g
        .offset(
            107 as i32 as isize,
        ) = *g.offset(99 as i32 as isize)
        * (zkzl + *cpz.offset(3 as i32 as isize));
    *g
        .offset(
            108 as i32 as isize,
        ) = *g.offset(100 as i32 as isize)
        * (zkzl + *cpz.offset(0 as i32 as isize))
        + *b00.offset(0 as i32 as isize) * *g.offset(96 as i32 as isize);
    *g
        .offset(
            109 as i32 as isize,
        ) = *g.offset(101 as i32 as isize)
        * (zkzl + *cpz.offset(1 as i32 as isize))
        + *b00.offset(1 as i32 as isize) * *g.offset(97 as i32 as isize);
    *g
        .offset(
            110 as i32 as isize,
        ) = *g.offset(102 as i32 as isize)
        * (zkzl + *cpz.offset(2 as i32 as isize))
        + *b00.offset(2 as i32 as isize) * *g.offset(98 as i32 as isize);
    *g
        .offset(
            111 as i32 as isize,
        ) = *g.offset(103 as i32 as isize)
        * (zkzl + *cpz.offset(3 as i32 as isize))
        + *b00.offset(3 as i32 as isize) * *g.offset(99 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1020(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let mut b01: *mut f64 = ((*bc).b01).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *c0x.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = *c0x.offset(3 as i32 as isize);
    *g.offset(8 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(9 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(10 as i32 as isize) = *cpx.offset(2 as i32 as isize);
    *g.offset(11 as i32 as isize) = *cpx.offset(3 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            13 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            16 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *cpx.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            17 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *cpx.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            18 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *cpx.offset(2 as i32 as isize)
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *cpx.offset(3 as i32 as isize)
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (*g.offset(12 as i32 as isize)
            + *b00.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize)
            * *c0x.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (*g.offset(13 as i32 as isize)
            + *b00.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize)
            * *c0x.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * (*g.offset(14 as i32 as isize)
            + *b00.offset(2 as i32 as isize))
        + *b01.offset(2 as i32 as isize)
            * *c0x.offset(2 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * (*g.offset(15 as i32 as isize)
            + *b00.offset(3 as i32 as isize))
        + *b01.offset(3 as i32 as isize)
            * *c0x.offset(3 as i32 as isize);
    *g.offset(24 as i32 as isize) = 1 as i32 as f64;
    *g.offset(25 as i32 as isize) = 1 as i32 as f64;
    *g.offset(26 as i32 as isize) = 1 as i32 as f64;
    *g.offset(27 as i32 as isize) = 1 as i32 as f64;
    *g.offset(28 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(29 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(30 as i32 as isize) = *c0y.offset(2 as i32 as isize);
    *g.offset(31 as i32 as isize) = *c0y.offset(3 as i32 as isize);
    *g.offset(32 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(33 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(34 as i32 as isize) = *cpy.offset(2 as i32 as isize);
    *g.offset(35 as i32 as isize) = *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            36 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            37 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            38 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            39 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            40 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *cpy.offset(0 as i32 as isize)
        + *b01.offset(0 as i32 as isize);
    *g
        .offset(
            41 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *cpy.offset(1 as i32 as isize)
        + *b01.offset(1 as i32 as isize);
    *g
        .offset(
            42 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *cpy.offset(2 as i32 as isize)
        + *b01.offset(2 as i32 as isize);
    *g
        .offset(
            43 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *cpy.offset(3 as i32 as isize)
        + *b01.offset(3 as i32 as isize);
    *g
        .offset(
            44 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (*g.offset(36 as i32 as isize)
            + *b00.offset(0 as i32 as isize))
        + *b01.offset(0 as i32 as isize)
            * *c0y.offset(0 as i32 as isize);
    *g
        .offset(
            45 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (*g.offset(37 as i32 as isize)
            + *b00.offset(1 as i32 as isize))
        + *b01.offset(1 as i32 as isize)
            * *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            46 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * (*g.offset(38 as i32 as isize)
            + *b00.offset(2 as i32 as isize))
        + *b01.offset(2 as i32 as isize)
            * *c0y.offset(2 as i32 as isize);
    *g
        .offset(
            47 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * (*g.offset(39 as i32 as isize)
            + *b00.offset(3 as i32 as isize))
        + *b01.offset(3 as i32 as isize)
            * *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            52 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            53 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            54 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(50 as i32 as isize);
    *g
        .offset(
            55 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(51 as i32 as isize);
    *g
        .offset(
            56 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            57 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            58 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(50 as i32 as isize);
    *g
        .offset(
            59 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(51 as i32 as isize);
    *g
        .offset(
            60 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(52 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            61 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(53 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            62 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(54 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(50 as i32 as isize);
    *g
        .offset(
            63 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(55 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(51 as i32 as isize);
    *g
        .offset(
            64 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(56 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            65 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(57 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            66 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(58 as i32 as isize)
        + *b01.offset(2 as i32 as isize) * *g.offset(50 as i32 as isize);
    *g
        .offset(
            67 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(59 as i32 as isize)
        + *b01.offset(3 as i32 as isize) * *g.offset(51 as i32 as isize);
    *g
        .offset(
            68 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(60 as i32 as isize)
        + *b01.offset(0 as i32 as isize) * *g.offset(52 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(56 as i32 as isize);
    *g
        .offset(
            69 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(61 as i32 as isize)
        + *b01.offset(1 as i32 as isize) * *g.offset(53 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(57 as i32 as isize);
    *g
        .offset(
            70 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(62 as i32 as isize)
        + *b01.offset(2 as i32 as isize) * *g.offset(54 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(58 as i32 as isize);
    *g
        .offset(
            71 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(63 as i32 as isize)
        + *b01.offset(3 as i32 as isize) * *g.offset(55 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(59 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1100(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    let mut xixj: f64 = (*envs).rirj[0 as i32 as usize];
    let mut yiyj: f64 = (*envs).rirj[1 as i32 as usize];
    let mut zizj: f64 = (*envs).rirj[2 as i32 as usize];
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(8 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(9 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(10 as i32 as isize) = *c0x.offset(2 as i32 as isize);
    *g.offset(11 as i32 as isize) = *c0x.offset(3 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (xixj + *c0x.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            13 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (xixj + *c0x.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *c0x.offset(2 as i32 as isize)
        * (xixj + *c0x.offset(2 as i32 as isize))
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *c0x.offset(3 as i32 as isize)
        * (xixj + *c0x.offset(3 as i32 as isize))
        + *b10.offset(3 as i32 as isize);
    *g.offset(4 as i32 as isize) = xixj + *c0x.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = xixj + *c0x.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = xixj + *c0x.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = xixj + *c0x.offset(3 as i32 as isize);
    *g.offset(24 as i32 as isize) = 1 as i32 as f64;
    *g.offset(25 as i32 as isize) = 1 as i32 as f64;
    *g.offset(26 as i32 as isize) = 1 as i32 as f64;
    *g.offset(27 as i32 as isize) = 1 as i32 as f64;
    *g.offset(32 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(33 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(34 as i32 as isize) = *c0y.offset(2 as i32 as isize);
    *g.offset(35 as i32 as isize) = *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            36 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (yiyj + *c0y.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            37 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (yiyj + *c0y.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            38 as i32 as isize,
        ) = *c0y.offset(2 as i32 as isize)
        * (yiyj + *c0y.offset(2 as i32 as isize))
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            39 as i32 as isize,
        ) = *c0y.offset(3 as i32 as isize)
        * (yiyj + *c0y.offset(3 as i32 as isize))
        + *b10.offset(3 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = yiyj + *c0y.offset(0 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = yiyj + *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = yiyj + *c0y.offset(2 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = yiyj + *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            56 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            57 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            58 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(50 as i32 as isize);
    *g
        .offset(
            59 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(51 as i32 as isize);
    *g
        .offset(
            60 as i32 as isize,
        ) = *g.offset(56 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            61 as i32 as isize,
        ) = *g.offset(57 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            62 as i32 as isize,
        ) = *g.offset(58 as i32 as isize)
        * (zizj + *c0z.offset(2 as i32 as isize))
        + *b10.offset(2 as i32 as isize) * *g.offset(50 as i32 as isize);
    *g
        .offset(
            63 as i32 as isize,
        ) = *g.offset(59 as i32 as isize)
        * (zizj + *c0z.offset(3 as i32 as isize))
        + *b10.offset(3 as i32 as isize) * *g.offset(51 as i32 as isize);
    *g
        .offset(
            52 as i32 as isize,
        ) = *g.offset(48 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize));
    *g
        .offset(
            53 as i32 as isize,
        ) = *g.offset(49 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize));
    *g
        .offset(
            54 as i32 as isize,
        ) = *g.offset(50 as i32 as isize)
        * (zizj + *c0z.offset(2 as i32 as isize));
    *g
        .offset(
            55 as i32 as isize,
        ) = *g.offset(51 as i32 as isize)
        * (zizj + *c0z.offset(3 as i32 as isize));
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1101(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    let mut xixj: f64 = (*envs).rirj[0 as i32 as usize];
    let mut yiyj: f64 = (*envs).rirj[1 as i32 as usize];
    let mut zizj: f64 = (*envs).rirj[2 as i32 as usize];
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(16 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(17 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(18 as i32 as isize) = *c0x.offset(2 as i32 as isize);
    *g.offset(19 as i32 as isize) = *c0x.offset(3 as i32 as isize);
    *g.offset(8 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(9 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(10 as i32 as isize) = *cpx.offset(2 as i32 as isize);
    *g.offset(11 as i32 as isize) = *cpx.offset(3 as i32 as isize);
    *g
        .offset(
            24 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            25 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            27 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (xixj + *c0x.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (xixj + *c0x.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *c0x.offset(2 as i32 as isize)
        * (xixj + *c0x.offset(2 as i32 as isize))
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *c0x.offset(3 as i32 as isize)
        * (xixj + *c0x.offset(3 as i32 as isize))
        + *b10.offset(3 as i32 as isize);
    *g.offset(4 as i32 as isize) = xixj + *c0x.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = xixj + *c0x.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = xixj + *c0x.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = xixj + *c0x.offset(3 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *g.offset(24 as i32 as isize)
        * (xixj + *c0x.offset(0 as i32 as isize))
        + *c0x.offset(0 as i32 as isize) * *b00.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize)
            * *cpx.offset(0 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = *g.offset(25 as i32 as isize)
        * (xixj + *c0x.offset(1 as i32 as isize))
        + *c0x.offset(1 as i32 as isize) * *b00.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize)
            * *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *g.offset(26 as i32 as isize)
        * (xixj + *c0x.offset(2 as i32 as isize))
        + *c0x.offset(2 as i32 as isize) * *b00.offset(2 as i32 as isize)
        + *b10.offset(2 as i32 as isize)
            * *cpx.offset(2 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *g.offset(27 as i32 as isize)
        * (xixj + *c0x.offset(3 as i32 as isize))
        + *c0x.offset(3 as i32 as isize) * *b00.offset(3 as i32 as isize)
        + *b10.offset(3 as i32 as isize)
            * *cpx.offset(3 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (xixj + *c0x.offset(0 as i32 as isize))
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            13 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (xixj + *c0x.offset(1 as i32 as isize))
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * (xixj + *c0x.offset(2 as i32 as isize))
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * (xixj + *c0x.offset(3 as i32 as isize))
        + *b00.offset(3 as i32 as isize);
    *g.offset(48 as i32 as isize) = 1 as i32 as f64;
    *g.offset(49 as i32 as isize) = 1 as i32 as f64;
    *g.offset(50 as i32 as isize) = 1 as i32 as f64;
    *g.offset(51 as i32 as isize) = 1 as i32 as f64;
    *g.offset(64 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(65 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(66 as i32 as isize) = *c0y.offset(2 as i32 as isize);
    *g.offset(67 as i32 as isize) = *c0y.offset(3 as i32 as isize);
    *g.offset(56 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(57 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(58 as i32 as isize) = *cpy.offset(2 as i32 as isize);
    *g.offset(59 as i32 as isize) = *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            72 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            73 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            74 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            75 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            68 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (yiyj + *c0y.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            69 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (yiyj + *c0y.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            70 as i32 as isize,
        ) = *c0y.offset(2 as i32 as isize)
        * (yiyj + *c0y.offset(2 as i32 as isize))
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            71 as i32 as isize,
        ) = *c0y.offset(3 as i32 as isize)
        * (yiyj + *c0y.offset(3 as i32 as isize))
        + *b10.offset(3 as i32 as isize);
    *g
        .offset(
            52 as i32 as isize,
        ) = yiyj + *c0y.offset(0 as i32 as isize);
    *g
        .offset(
            53 as i32 as isize,
        ) = yiyj + *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            54 as i32 as isize,
        ) = yiyj + *c0y.offset(2 as i32 as isize);
    *g
        .offset(
            55 as i32 as isize,
        ) = yiyj + *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            76 as i32 as isize,
        ) = *g.offset(72 as i32 as isize)
        * (yiyj + *c0y.offset(0 as i32 as isize))
        + *c0y.offset(0 as i32 as isize) * *b00.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize)
            * *cpy.offset(0 as i32 as isize);
    *g
        .offset(
            77 as i32 as isize,
        ) = *g.offset(73 as i32 as isize)
        * (yiyj + *c0y.offset(1 as i32 as isize))
        + *c0y.offset(1 as i32 as isize) * *b00.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize)
            * *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            78 as i32 as isize,
        ) = *g.offset(74 as i32 as isize)
        * (yiyj + *c0y.offset(2 as i32 as isize))
        + *c0y.offset(2 as i32 as isize) * *b00.offset(2 as i32 as isize)
        + *b10.offset(2 as i32 as isize)
            * *cpy.offset(2 as i32 as isize);
    *g
        .offset(
            79 as i32 as isize,
        ) = *g.offset(75 as i32 as isize)
        * (yiyj + *c0y.offset(3 as i32 as isize))
        + *c0y.offset(3 as i32 as isize) * *b00.offset(3 as i32 as isize)
        + *b10.offset(3 as i32 as isize)
            * *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            60 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (yiyj + *c0y.offset(0 as i32 as isize))
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            61 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (yiyj + *c0y.offset(1 as i32 as isize))
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            62 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * (yiyj + *c0y.offset(2 as i32 as isize))
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            63 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * (yiyj + *c0y.offset(3 as i32 as isize))
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            112 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(96 as i32 as isize);
    *g
        .offset(
            113 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(97 as i32 as isize);
    *g
        .offset(
            114 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(98 as i32 as isize);
    *g
        .offset(
            115 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(99 as i32 as isize);
    *g
        .offset(
            104 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(96 as i32 as isize);
    *g
        .offset(
            105 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(97 as i32 as isize);
    *g
        .offset(
            106 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(98 as i32 as isize);
    *g
        .offset(
            107 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(99 as i32 as isize);
    *g
        .offset(
            120 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(112 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(96 as i32 as isize);
    *g
        .offset(
            121 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(113 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(97 as i32 as isize);
    *g
        .offset(
            122 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(114 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(98 as i32 as isize);
    *g
        .offset(
            123 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(115 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(99 as i32 as isize);
    *g
        .offset(
            116 as i32 as isize,
        ) = *g.offset(112 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize) * *g.offset(96 as i32 as isize);
    *g
        .offset(
            117 as i32 as isize,
        ) = *g.offset(113 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize) * *g.offset(97 as i32 as isize);
    *g
        .offset(
            118 as i32 as isize,
        ) = *g.offset(114 as i32 as isize)
        * (zizj + *c0z.offset(2 as i32 as isize))
        + *b10.offset(2 as i32 as isize) * *g.offset(98 as i32 as isize);
    *g
        .offset(
            119 as i32 as isize,
        ) = *g.offset(115 as i32 as isize)
        * (zizj + *c0z.offset(3 as i32 as isize))
        + *b10.offset(3 as i32 as isize) * *g.offset(99 as i32 as isize);
    *g
        .offset(
            100 as i32 as isize,
        ) = *g.offset(96 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize));
    *g
        .offset(
            101 as i32 as isize,
        ) = *g.offset(97 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize));
    *g
        .offset(
            102 as i32 as isize,
        ) = *g.offset(98 as i32 as isize)
        * (zizj + *c0z.offset(2 as i32 as isize));
    *g
        .offset(
            103 as i32 as isize,
        ) = *g.offset(99 as i32 as isize)
        * (zizj + *c0z.offset(3 as i32 as isize));
    *g
        .offset(
            124 as i32 as isize,
        ) = *g.offset(120 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize) * *g.offset(104 as i32 as isize)
        + *b00.offset(0 as i32 as isize)
            * *g.offset(112 as i32 as isize);
    *g
        .offset(
            125 as i32 as isize,
        ) = *g.offset(121 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize) * *g.offset(105 as i32 as isize)
        + *b00.offset(1 as i32 as isize)
            * *g.offset(113 as i32 as isize);
    *g
        .offset(
            126 as i32 as isize,
        ) = *g.offset(122 as i32 as isize)
        * (zizj + *c0z.offset(2 as i32 as isize))
        + *b10.offset(2 as i32 as isize) * *g.offset(106 as i32 as isize)
        + *b00.offset(2 as i32 as isize)
            * *g.offset(114 as i32 as isize);
    *g
        .offset(
            127 as i32 as isize,
        ) = *g.offset(123 as i32 as isize)
        * (zizj + *c0z.offset(3 as i32 as isize))
        + *b10.offset(3 as i32 as isize) * *g.offset(107 as i32 as isize)
        + *b00.offset(3 as i32 as isize)
            * *g.offset(115 as i32 as isize);
    *g
        .offset(
            108 as i32 as isize,
        ) = zizj * *g.offset(104 as i32 as isize)
        + *cpz.offset(0 as i32 as isize) * *g.offset(112 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(96 as i32 as isize);
    *g
        .offset(
            109 as i32 as isize,
        ) = zizj * *g.offset(105 as i32 as isize)
        + *cpz.offset(1 as i32 as isize) * *g.offset(113 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(97 as i32 as isize);
    *g
        .offset(
            110 as i32 as isize,
        ) = zizj * *g.offset(106 as i32 as isize)
        + *cpz.offset(2 as i32 as isize) * *g.offset(114 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(98 as i32 as isize);
    *g
        .offset(
            111 as i32 as isize,
        ) = zizj * *g.offset(107 as i32 as isize)
        + *cpz.offset(3 as i32 as isize) * *g.offset(115 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(99 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1110(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    let mut xixj: f64 = (*envs).rirj[0 as i32 as usize];
    let mut yiyj: f64 = (*envs).rirj[1 as i32 as usize];
    let mut zizj: f64 = (*envs).rirj[2 as i32 as usize];
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(16 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(17 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(18 as i32 as isize) = *c0x.offset(2 as i32 as isize);
    *g.offset(19 as i32 as isize) = *c0x.offset(3 as i32 as isize);
    *g.offset(8 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(9 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(10 as i32 as isize) = *cpx.offset(2 as i32 as isize);
    *g.offset(11 as i32 as isize) = *cpx.offset(3 as i32 as isize);
    *g
        .offset(
            24 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            25 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            27 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (xixj + *c0x.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (xixj + *c0x.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *c0x.offset(2 as i32 as isize)
        * (xixj + *c0x.offset(2 as i32 as isize))
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *c0x.offset(3 as i32 as isize)
        * (xixj + *c0x.offset(3 as i32 as isize))
        + *b10.offset(3 as i32 as isize);
    *g.offset(4 as i32 as isize) = xixj + *c0x.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = xixj + *c0x.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = xixj + *c0x.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = xixj + *c0x.offset(3 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *g.offset(24 as i32 as isize)
        * (xixj + *c0x.offset(0 as i32 as isize))
        + *c0x.offset(0 as i32 as isize) * *b00.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize)
            * *cpx.offset(0 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = *g.offset(25 as i32 as isize)
        * (xixj + *c0x.offset(1 as i32 as isize))
        + *c0x.offset(1 as i32 as isize) * *b00.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize)
            * *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *g.offset(26 as i32 as isize)
        * (xixj + *c0x.offset(2 as i32 as isize))
        + *c0x.offset(2 as i32 as isize) * *b00.offset(2 as i32 as isize)
        + *b10.offset(2 as i32 as isize)
            * *cpx.offset(2 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *g.offset(27 as i32 as isize)
        * (xixj + *c0x.offset(3 as i32 as isize))
        + *c0x.offset(3 as i32 as isize) * *b00.offset(3 as i32 as isize)
        + *b10.offset(3 as i32 as isize)
            * *cpx.offset(3 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * (xixj + *c0x.offset(0 as i32 as isize))
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            13 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * (xixj + *c0x.offset(1 as i32 as isize))
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * (xixj + *c0x.offset(2 as i32 as isize))
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * (xixj + *c0x.offset(3 as i32 as isize))
        + *b00.offset(3 as i32 as isize);
    *g.offset(48 as i32 as isize) = 1 as i32 as f64;
    *g.offset(49 as i32 as isize) = 1 as i32 as f64;
    *g.offset(50 as i32 as isize) = 1 as i32 as f64;
    *g.offset(51 as i32 as isize) = 1 as i32 as f64;
    *g.offset(64 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(65 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(66 as i32 as isize) = *c0y.offset(2 as i32 as isize);
    *g.offset(67 as i32 as isize) = *c0y.offset(3 as i32 as isize);
    *g.offset(56 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(57 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(58 as i32 as isize) = *cpy.offset(2 as i32 as isize);
    *g.offset(59 as i32 as isize) = *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            72 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            73 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            74 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            75 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            68 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (yiyj + *c0y.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            69 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (yiyj + *c0y.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            70 as i32 as isize,
        ) = *c0y.offset(2 as i32 as isize)
        * (yiyj + *c0y.offset(2 as i32 as isize))
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            71 as i32 as isize,
        ) = *c0y.offset(3 as i32 as isize)
        * (yiyj + *c0y.offset(3 as i32 as isize))
        + *b10.offset(3 as i32 as isize);
    *g
        .offset(
            52 as i32 as isize,
        ) = yiyj + *c0y.offset(0 as i32 as isize);
    *g
        .offset(
            53 as i32 as isize,
        ) = yiyj + *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            54 as i32 as isize,
        ) = yiyj + *c0y.offset(2 as i32 as isize);
    *g
        .offset(
            55 as i32 as isize,
        ) = yiyj + *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            76 as i32 as isize,
        ) = *g.offset(72 as i32 as isize)
        * (yiyj + *c0y.offset(0 as i32 as isize))
        + *c0y.offset(0 as i32 as isize) * *b00.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize)
            * *cpy.offset(0 as i32 as isize);
    *g
        .offset(
            77 as i32 as isize,
        ) = *g.offset(73 as i32 as isize)
        * (yiyj + *c0y.offset(1 as i32 as isize))
        + *c0y.offset(1 as i32 as isize) * *b00.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize)
            * *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            78 as i32 as isize,
        ) = *g.offset(74 as i32 as isize)
        * (yiyj + *c0y.offset(2 as i32 as isize))
        + *c0y.offset(2 as i32 as isize) * *b00.offset(2 as i32 as isize)
        + *b10.offset(2 as i32 as isize)
            * *cpy.offset(2 as i32 as isize);
    *g
        .offset(
            79 as i32 as isize,
        ) = *g.offset(75 as i32 as isize)
        * (yiyj + *c0y.offset(3 as i32 as isize))
        + *c0y.offset(3 as i32 as isize) * *b00.offset(3 as i32 as isize)
        + *b10.offset(3 as i32 as isize)
            * *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            60 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * (yiyj + *c0y.offset(0 as i32 as isize))
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            61 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * (yiyj + *c0y.offset(1 as i32 as isize))
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            62 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * (yiyj + *c0y.offset(2 as i32 as isize))
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            63 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * (yiyj + *c0y.offset(3 as i32 as isize))
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            112 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(96 as i32 as isize);
    *g
        .offset(
            113 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(97 as i32 as isize);
    *g
        .offset(
            114 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(98 as i32 as isize);
    *g
        .offset(
            115 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(99 as i32 as isize);
    *g
        .offset(
            104 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(96 as i32 as isize);
    *g
        .offset(
            105 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(97 as i32 as isize);
    *g
        .offset(
            106 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(98 as i32 as isize);
    *g
        .offset(
            107 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(99 as i32 as isize);
    *g
        .offset(
            120 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(112 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(96 as i32 as isize);
    *g
        .offset(
            121 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(113 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(97 as i32 as isize);
    *g
        .offset(
            122 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(114 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(98 as i32 as isize);
    *g
        .offset(
            123 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(115 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(99 as i32 as isize);
    *g
        .offset(
            116 as i32 as isize,
        ) = *g.offset(112 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize) * *g.offset(96 as i32 as isize);
    *g
        .offset(
            117 as i32 as isize,
        ) = *g.offset(113 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize) * *g.offset(97 as i32 as isize);
    *g
        .offset(
            118 as i32 as isize,
        ) = *g.offset(114 as i32 as isize)
        * (zizj + *c0z.offset(2 as i32 as isize))
        + *b10.offset(2 as i32 as isize) * *g.offset(98 as i32 as isize);
    *g
        .offset(
            119 as i32 as isize,
        ) = *g.offset(115 as i32 as isize)
        * (zizj + *c0z.offset(3 as i32 as isize))
        + *b10.offset(3 as i32 as isize) * *g.offset(99 as i32 as isize);
    *g
        .offset(
            100 as i32 as isize,
        ) = *g.offset(96 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize));
    *g
        .offset(
            101 as i32 as isize,
        ) = *g.offset(97 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize));
    *g
        .offset(
            102 as i32 as isize,
        ) = *g.offset(98 as i32 as isize)
        * (zizj + *c0z.offset(2 as i32 as isize));
    *g
        .offset(
            103 as i32 as isize,
        ) = *g.offset(99 as i32 as isize)
        * (zizj + *c0z.offset(3 as i32 as isize));
    *g
        .offset(
            124 as i32 as isize,
        ) = *g.offset(120 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize) * *g.offset(104 as i32 as isize)
        + *b00.offset(0 as i32 as isize)
            * *g.offset(112 as i32 as isize);
    *g
        .offset(
            125 as i32 as isize,
        ) = *g.offset(121 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize) * *g.offset(105 as i32 as isize)
        + *b00.offset(1 as i32 as isize)
            * *g.offset(113 as i32 as isize);
    *g
        .offset(
            126 as i32 as isize,
        ) = *g.offset(122 as i32 as isize)
        * (zizj + *c0z.offset(2 as i32 as isize))
        + *b10.offset(2 as i32 as isize) * *g.offset(106 as i32 as isize)
        + *b00.offset(2 as i32 as isize)
            * *g.offset(114 as i32 as isize);
    *g
        .offset(
            127 as i32 as isize,
        ) = *g.offset(123 as i32 as isize)
        * (zizj + *c0z.offset(3 as i32 as isize))
        + *b10.offset(3 as i32 as isize) * *g.offset(107 as i32 as isize)
        + *b00.offset(3 as i32 as isize)
            * *g.offset(115 as i32 as isize);
    *g
        .offset(
            108 as i32 as isize,
        ) = zizj * *g.offset(104 as i32 as isize)
        + *cpz.offset(0 as i32 as isize) * *g.offset(112 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(96 as i32 as isize);
    *g
        .offset(
            109 as i32 as isize,
        ) = zizj * *g.offset(105 as i32 as isize)
        + *cpz.offset(1 as i32 as isize) * *g.offset(113 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(97 as i32 as isize);
    *g
        .offset(
            110 as i32 as isize,
        ) = zizj * *g.offset(106 as i32 as isize)
        + *cpz.offset(2 as i32 as isize) * *g.offset(114 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(98 as i32 as isize);
    *g
        .offset(
            111 as i32 as isize,
        ) = zizj * *g.offset(107 as i32 as isize)
        + *cpz.offset(3 as i32 as isize) * *g.offset(115 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(99 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_1200(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    let mut xixj: f64 = (*envs).rirj[0 as i32 as usize];
    let mut yiyj: f64 = (*envs).rirj[1 as i32 as usize];
    let mut zizj: f64 = (*envs).rirj[2 as i32 as usize];
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(8 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(9 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(10 as i32 as isize) = *c0x.offset(2 as i32 as isize);
    *g.offset(11 as i32 as isize) = *c0x.offset(3 as i32 as isize);
    *g
        .offset(
            16 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            17 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            18 as i32 as isize,
        ) = *c0x.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = *c0x.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b10.offset(3 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *g.offset(16 as i32 as isize)
        * (xixj + *c0x.offset(0 as i32 as isize))
        + *c0x.offset(0 as i32 as isize) * 2 as i32 as f64
            * *b10.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *g.offset(17 as i32 as isize)
        * (xixj + *c0x.offset(1 as i32 as isize))
        + *c0x.offset(1 as i32 as isize) * 2 as i32 as f64
            * *b10.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *g.offset(18 as i32 as isize)
        * (xixj + *c0x.offset(2 as i32 as isize))
        + *c0x.offset(2 as i32 as isize) * 2 as i32 as f64
            * *b10.offset(2 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *g.offset(19 as i32 as isize)
        * (xixj + *c0x.offset(3 as i32 as isize))
        + *c0x.offset(3 as i32 as isize) * 2 as i32 as f64
            * *b10.offset(3 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (xixj + *c0x.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            13 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (xixj + *c0x.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            14 as i32 as isize,
        ) = *c0x.offset(2 as i32 as isize)
        * (xixj + *c0x.offset(2 as i32 as isize))
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            15 as i32 as isize,
        ) = *c0x.offset(3 as i32 as isize)
        * (xixj + *c0x.offset(3 as i32 as isize))
        + *b10.offset(3 as i32 as isize);
    *g.offset(4 as i32 as isize) = xixj + *c0x.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = xixj + *c0x.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = xixj + *c0x.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = xixj + *c0x.offset(3 as i32 as isize);
    *g.offset(32 as i32 as isize) = 1 as i32 as f64;
    *g.offset(33 as i32 as isize) = 1 as i32 as f64;
    *g.offset(34 as i32 as isize) = 1 as i32 as f64;
    *g.offset(35 as i32 as isize) = 1 as i32 as f64;
    *g.offset(40 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(41 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(42 as i32 as isize) = *c0y.offset(2 as i32 as isize);
    *g.offset(43 as i32 as isize) = *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            48 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            49 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            50 as i32 as isize,
        ) = *c0y.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            51 as i32 as isize,
        ) = *c0y.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b10.offset(3 as i32 as isize);
    *g
        .offset(
            52 as i32 as isize,
        ) = *g.offset(48 as i32 as isize)
        * (yiyj + *c0y.offset(0 as i32 as isize))
        + *c0y.offset(0 as i32 as isize) * 2 as i32 as f64
            * *b10.offset(0 as i32 as isize);
    *g
        .offset(
            53 as i32 as isize,
        ) = *g.offset(49 as i32 as isize)
        * (yiyj + *c0y.offset(1 as i32 as isize))
        + *c0y.offset(1 as i32 as isize) * 2 as i32 as f64
            * *b10.offset(1 as i32 as isize);
    *g
        .offset(
            54 as i32 as isize,
        ) = *g.offset(50 as i32 as isize)
        * (yiyj + *c0y.offset(2 as i32 as isize))
        + *c0y.offset(2 as i32 as isize) * 2 as i32 as f64
            * *b10.offset(2 as i32 as isize);
    *g
        .offset(
            55 as i32 as isize,
        ) = *g.offset(51 as i32 as isize)
        * (yiyj + *c0y.offset(3 as i32 as isize))
        + *c0y.offset(3 as i32 as isize) * 2 as i32 as f64
            * *b10.offset(3 as i32 as isize);
    *g
        .offset(
            44 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (yiyj + *c0y.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            45 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (yiyj + *c0y.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            46 as i32 as isize,
        ) = *c0y.offset(2 as i32 as isize)
        * (yiyj + *c0y.offset(2 as i32 as isize))
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            47 as i32 as isize,
        ) = *c0y.offset(3 as i32 as isize)
        * (yiyj + *c0y.offset(3 as i32 as isize))
        + *b10.offset(3 as i32 as isize);
    *g
        .offset(
            36 as i32 as isize,
        ) = yiyj + *c0y.offset(0 as i32 as isize);
    *g
        .offset(
            37 as i32 as isize,
        ) = yiyj + *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            38 as i32 as isize,
        ) = yiyj + *c0y.offset(2 as i32 as isize);
    *g
        .offset(
            39 as i32 as isize,
        ) = yiyj + *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            72 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(64 as i32 as isize);
    *g
        .offset(
            73 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(65 as i32 as isize);
    *g
        .offset(
            74 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(66 as i32 as isize);
    *g
        .offset(
            75 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(67 as i32 as isize);
    *g
        .offset(
            80 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(72 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(64 as i32 as isize);
    *g
        .offset(
            81 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(73 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(65 as i32 as isize);
    *g
        .offset(
            82 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(74 as i32 as isize)
        + *b10.offset(2 as i32 as isize) * *g.offset(66 as i32 as isize);
    *g
        .offset(
            83 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(75 as i32 as isize)
        + *b10.offset(3 as i32 as isize) * *g.offset(67 as i32 as isize);
    *g
        .offset(
            84 as i32 as isize,
        ) = *g.offset(80 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize))
        + 2 as i32 as f64 * *b10.offset(0 as i32 as isize)
            * *g.offset(72 as i32 as isize);
    *g
        .offset(
            85 as i32 as isize,
        ) = *g.offset(81 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize))
        + 2 as i32 as f64 * *b10.offset(1 as i32 as isize)
            * *g.offset(73 as i32 as isize);
    *g
        .offset(
            86 as i32 as isize,
        ) = *g.offset(82 as i32 as isize)
        * (zizj + *c0z.offset(2 as i32 as isize))
        + 2 as i32 as f64 * *b10.offset(2 as i32 as isize)
            * *g.offset(74 as i32 as isize);
    *g
        .offset(
            87 as i32 as isize,
        ) = *g.offset(83 as i32 as isize)
        * (zizj + *c0z.offset(3 as i32 as isize))
        + 2 as i32 as f64 * *b10.offset(3 as i32 as isize)
            * *g.offset(75 as i32 as isize);
    *g
        .offset(
            76 as i32 as isize,
        ) = *g.offset(72 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize) * *g.offset(64 as i32 as isize);
    *g
        .offset(
            77 as i32 as isize,
        ) = *g.offset(73 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize) * *g.offset(65 as i32 as isize);
    *g
        .offset(
            78 as i32 as isize,
        ) = *g.offset(74 as i32 as isize)
        * (zizj + *c0z.offset(2 as i32 as isize))
        + *b10.offset(2 as i32 as isize) * *g.offset(66 as i32 as isize);
    *g
        .offset(
            79 as i32 as isize,
        ) = *g.offset(75 as i32 as isize)
        * (zizj + *c0z.offset(3 as i32 as isize))
        + *b10.offset(3 as i32 as isize) * *g.offset(67 as i32 as isize);
    *g
        .offset(
            68 as i32 as isize,
        ) = *g.offset(64 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize));
    *g
        .offset(
            69 as i32 as isize,
        ) = *g.offset(65 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize));
    *g
        .offset(
            70 as i32 as isize,
        ) = *g.offset(66 as i32 as isize)
        * (zizj + *c0z.offset(2 as i32 as isize));
    *g
        .offset(
            71 as i32 as isize,
        ) = *g.offset(67 as i32 as isize)
        * (zizj + *c0z.offset(3 as i32 as isize));
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_2000(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *c0x.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = *c0x.offset(3 as i32 as isize);
    *g
        .offset(
            8 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            9 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *c0x.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *c0x.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b10.offset(3 as i32 as isize);
    *g.offset(12 as i32 as isize) = 1 as i32 as f64;
    *g.offset(13 as i32 as isize) = 1 as i32 as f64;
    *g.offset(14 as i32 as isize) = 1 as i32 as f64;
    *g.offset(15 as i32 as isize) = 1 as i32 as f64;
    *g.offset(16 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(17 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(18 as i32 as isize) = *c0y.offset(2 as i32 as isize);
    *g.offset(19 as i32 as isize) = *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *c0y.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *c0y.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b10.offset(3 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(24 as i32 as isize);
    *g
        .offset(
            29 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(25 as i32 as isize);
    *g
        .offset(
            30 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(26 as i32 as isize);
    *g
        .offset(
            31 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(27 as i32 as isize);
    *g
        .offset(
            32 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(28 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(24 as i32 as isize);
    *g
        .offset(
            33 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(29 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(25 as i32 as isize);
    *g
        .offset(
            34 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(30 as i32 as isize)
        + *b10.offset(2 as i32 as isize) * *g.offset(26 as i32 as isize);
    *g
        .offset(
            35 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(31 as i32 as isize)
        + *b10.offset(3 as i32 as isize) * *g.offset(27 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_2001(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *c0x.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = *c0x.offset(3 as i32 as isize);
    *g
        .offset(
            8 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            9 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *c0x.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *c0x.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b10.offset(3 as i32 as isize);
    *g.offset(12 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(13 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(14 as i32 as isize) = *cpx.offset(2 as i32 as isize);
    *g.offset(15 as i32 as isize) = *cpx.offset(3 as i32 as isize);
    *g
        .offset(
            16 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            17 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            18 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (*g.offset(16 as i32 as isize)
            + *b00.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize)
            * *cpx.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (*g.offset(17 as i32 as isize)
            + *b00.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize)
            * *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *c0x.offset(2 as i32 as isize)
        * (*g.offset(18 as i32 as isize)
            + *b00.offset(2 as i32 as isize))
        + *b10.offset(2 as i32 as isize)
            * *cpx.offset(2 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *c0x.offset(3 as i32 as isize)
        * (*g.offset(19 as i32 as isize)
            + *b00.offset(3 as i32 as isize))
        + *b10.offset(3 as i32 as isize)
            * *cpx.offset(3 as i32 as isize);
    *g.offset(24 as i32 as isize) = 1 as i32 as f64;
    *g.offset(25 as i32 as isize) = 1 as i32 as f64;
    *g.offset(26 as i32 as isize) = 1 as i32 as f64;
    *g.offset(27 as i32 as isize) = 1 as i32 as f64;
    *g.offset(28 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(29 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(30 as i32 as isize) = *c0y.offset(2 as i32 as isize);
    *g.offset(31 as i32 as isize) = *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            32 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            33 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            34 as i32 as isize,
        ) = *c0y.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            35 as i32 as isize,
        ) = *c0y.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b10.offset(3 as i32 as isize);
    *g.offset(36 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(37 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(38 as i32 as isize) = *cpy.offset(2 as i32 as isize);
    *g.offset(39 as i32 as isize) = *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            40 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            41 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            42 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            43 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            44 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (*g.offset(40 as i32 as isize)
            + *b00.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize)
            * *cpy.offset(0 as i32 as isize);
    *g
        .offset(
            45 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (*g.offset(41 as i32 as isize)
            + *b00.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize)
            * *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            46 as i32 as isize,
        ) = *c0y.offset(2 as i32 as isize)
        * (*g.offset(42 as i32 as isize)
            + *b00.offset(2 as i32 as isize))
        + *b10.offset(2 as i32 as isize)
            * *cpy.offset(2 as i32 as isize);
    *g
        .offset(
            47 as i32 as isize,
        ) = *c0y.offset(3 as i32 as isize)
        * (*g.offset(43 as i32 as isize)
            + *b00.offset(3 as i32 as isize))
        + *b10.offset(3 as i32 as isize)
            * *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            52 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            53 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            54 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(50 as i32 as isize);
    *g
        .offset(
            55 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(51 as i32 as isize);
    *g
        .offset(
            56 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(52 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            57 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(53 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            58 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(54 as i32 as isize)
        + *b10.offset(2 as i32 as isize) * *g.offset(50 as i32 as isize);
    *g
        .offset(
            59 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(55 as i32 as isize)
        + *b10.offset(3 as i32 as isize) * *g.offset(51 as i32 as isize);
    *g
        .offset(
            60 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            61 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            62 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(50 as i32 as isize);
    *g
        .offset(
            63 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(51 as i32 as isize);
    *g
        .offset(
            64 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(52 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            65 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(53 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            66 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(54 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(50 as i32 as isize);
    *g
        .offset(
            67 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(55 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(51 as i32 as isize);
    *g
        .offset(
            68 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(64 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(60 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(52 as i32 as isize);
    *g
        .offset(
            69 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(65 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(61 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(53 as i32 as isize);
    *g
        .offset(
            70 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(66 as i32 as isize)
        + *b10.offset(2 as i32 as isize) * *g.offset(62 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(54 as i32 as isize);
    *g
        .offset(
            71 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(67 as i32 as isize)
        + *b10.offset(3 as i32 as isize) * *g.offset(63 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(55 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_2010(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut cpx: *mut f64 = ((*bc).c0px).as_mut_ptr();
    let mut cpy: *mut f64 = ((*bc).c0py).as_mut_ptr();
    let mut cpz: *mut f64 = ((*bc).c0pz).as_mut_ptr();
    let mut b00: *mut f64 = ((*bc).b00).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *c0x.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = *c0x.offset(3 as i32 as isize);
    *g
        .offset(
            8 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            9 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *c0x.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *c0x.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b10.offset(3 as i32 as isize);
    *g.offset(12 as i32 as isize) = *cpx.offset(0 as i32 as isize);
    *g.offset(13 as i32 as isize) = *cpx.offset(1 as i32 as isize);
    *g.offset(14 as i32 as isize) = *cpx.offset(2 as i32 as isize);
    *g.offset(15 as i32 as isize) = *cpx.offset(3 as i32 as isize);
    *g
        .offset(
            16 as i32 as isize,
        ) = *cpx.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            17 as i32 as isize,
        ) = *cpx.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            18 as i32 as isize,
        ) = *cpx.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = *cpx.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (*g.offset(16 as i32 as isize)
            + *b00.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize)
            * *cpx.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (*g.offset(17 as i32 as isize)
            + *b00.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize)
            * *cpx.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *c0x.offset(2 as i32 as isize)
        * (*g.offset(18 as i32 as isize)
            + *b00.offset(2 as i32 as isize))
        + *b10.offset(2 as i32 as isize)
            * *cpx.offset(2 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *c0x.offset(3 as i32 as isize)
        * (*g.offset(19 as i32 as isize)
            + *b00.offset(3 as i32 as isize))
        + *b10.offset(3 as i32 as isize)
            * *cpx.offset(3 as i32 as isize);
    *g.offset(24 as i32 as isize) = 1 as i32 as f64;
    *g.offset(25 as i32 as isize) = 1 as i32 as f64;
    *g.offset(26 as i32 as isize) = 1 as i32 as f64;
    *g.offset(27 as i32 as isize) = 1 as i32 as f64;
    *g.offset(28 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(29 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(30 as i32 as isize) = *c0y.offset(2 as i32 as isize);
    *g.offset(31 as i32 as isize) = *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            32 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            33 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            34 as i32 as isize,
        ) = *c0y.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            35 as i32 as isize,
        ) = *c0y.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b10.offset(3 as i32 as isize);
    *g.offset(36 as i32 as isize) = *cpy.offset(0 as i32 as isize);
    *g.offset(37 as i32 as isize) = *cpy.offset(1 as i32 as isize);
    *g.offset(38 as i32 as isize) = *cpy.offset(2 as i32 as isize);
    *g.offset(39 as i32 as isize) = *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            40 as i32 as isize,
        ) = *cpy.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b00.offset(0 as i32 as isize);
    *g
        .offset(
            41 as i32 as isize,
        ) = *cpy.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b00.offset(1 as i32 as isize);
    *g
        .offset(
            42 as i32 as isize,
        ) = *cpy.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b00.offset(2 as i32 as isize);
    *g
        .offset(
            43 as i32 as isize,
        ) = *cpy.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b00.offset(3 as i32 as isize);
    *g
        .offset(
            44 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (*g.offset(40 as i32 as isize)
            + *b00.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize)
            * *cpy.offset(0 as i32 as isize);
    *g
        .offset(
            45 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (*g.offset(41 as i32 as isize)
            + *b00.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize)
            * *cpy.offset(1 as i32 as isize);
    *g
        .offset(
            46 as i32 as isize,
        ) = *c0y.offset(2 as i32 as isize)
        * (*g.offset(42 as i32 as isize)
            + *b00.offset(2 as i32 as isize))
        + *b10.offset(2 as i32 as isize)
            * *cpy.offset(2 as i32 as isize);
    *g
        .offset(
            47 as i32 as isize,
        ) = *c0y.offset(3 as i32 as isize)
        * (*g.offset(43 as i32 as isize)
            + *b00.offset(3 as i32 as isize))
        + *b10.offset(3 as i32 as isize)
            * *cpy.offset(3 as i32 as isize);
    *g
        .offset(
            52 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            53 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            54 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(50 as i32 as isize);
    *g
        .offset(
            55 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(51 as i32 as isize);
    *g
        .offset(
            56 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(52 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            57 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(53 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            58 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(54 as i32 as isize)
        + *b10.offset(2 as i32 as isize) * *g.offset(50 as i32 as isize);
    *g
        .offset(
            59 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(55 as i32 as isize)
        + *b10.offset(3 as i32 as isize) * *g.offset(51 as i32 as isize);
    *g
        .offset(
            60 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(48 as i32 as isize);
    *g
        .offset(
            61 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(49 as i32 as isize);
    *g
        .offset(
            62 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(50 as i32 as isize);
    *g
        .offset(
            63 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(51 as i32 as isize);
    *g
        .offset(
            64 as i32 as isize,
        ) = *cpz.offset(0 as i32 as isize)
        * *g.offset(52 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(48 as i32 as isize);
    *g
        .offset(
            65 as i32 as isize,
        ) = *cpz.offset(1 as i32 as isize)
        * *g.offset(53 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(49 as i32 as isize);
    *g
        .offset(
            66 as i32 as isize,
        ) = *cpz.offset(2 as i32 as isize)
        * *g.offset(54 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(50 as i32 as isize);
    *g
        .offset(
            67 as i32 as isize,
        ) = *cpz.offset(3 as i32 as isize)
        * *g.offset(55 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(51 as i32 as isize);
    *g
        .offset(
            68 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(64 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(60 as i32 as isize)
        + *b00.offset(0 as i32 as isize) * *g.offset(52 as i32 as isize);
    *g
        .offset(
            69 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(65 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(61 as i32 as isize)
        + *b00.offset(1 as i32 as isize) * *g.offset(53 as i32 as isize);
    *g
        .offset(
            70 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(66 as i32 as isize)
        + *b10.offset(2 as i32 as isize) * *g.offset(62 as i32 as isize)
        + *b00.offset(2 as i32 as isize) * *g.offset(54 as i32 as isize);
    *g
        .offset(
            71 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(67 as i32 as isize)
        + *b10.offset(3 as i32 as isize) * *g.offset(63 as i32 as isize)
        + *b00.offset(3 as i32 as isize) * *g.offset(55 as i32 as isize);
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_2100(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    let mut xixj: f64 = (*envs).rirj[0 as i32 as usize];
    let mut yiyj: f64 = (*envs).rirj[1 as i32 as usize];
    let mut zizj: f64 = (*envs).rirj[2 as i32 as usize];
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *c0x.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = *c0x.offset(3 as i32 as isize);
    *g
        .offset(
            8 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            9 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *c0x.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *c0x.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b10.offset(3 as i32 as isize);
    *g
        .offset(
            24 as i32 as isize,
        ) = *g.offset(8 as i32 as isize)
        * (xixj + *c0x.offset(0 as i32 as isize))
        + *c0x.offset(0 as i32 as isize) * 2 as i32 as f64
            * *b10.offset(0 as i32 as isize);
    *g
        .offset(
            25 as i32 as isize,
        ) = *g.offset(9 as i32 as isize)
        * (xixj + *c0x.offset(1 as i32 as isize))
        + *c0x.offset(1 as i32 as isize) * 2 as i32 as f64
            * *b10.offset(1 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = *g.offset(10 as i32 as isize)
        * (xixj + *c0x.offset(2 as i32 as isize))
        + *c0x.offset(2 as i32 as isize) * 2 as i32 as f64
            * *b10.offset(2 as i32 as isize);
    *g
        .offset(
            27 as i32 as isize,
        ) = *g.offset(11 as i32 as isize)
        * (xixj + *c0x.offset(3 as i32 as isize))
        + *c0x.offset(3 as i32 as isize) * 2 as i32 as f64
            * *b10.offset(3 as i32 as isize);
    *g
        .offset(
            20 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (xixj + *c0x.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            21 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (xixj + *c0x.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            22 as i32 as isize,
        ) = *c0x.offset(2 as i32 as isize)
        * (xixj + *c0x.offset(2 as i32 as isize))
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            23 as i32 as isize,
        ) = *c0x.offset(3 as i32 as isize)
        * (xixj + *c0x.offset(3 as i32 as isize))
        + *b10.offset(3 as i32 as isize);
    *g
        .offset(
            16 as i32 as isize,
        ) = xixj + *c0x.offset(0 as i32 as isize);
    *g
        .offset(
            17 as i32 as isize,
        ) = xixj + *c0x.offset(1 as i32 as isize);
    *g
        .offset(
            18 as i32 as isize,
        ) = xixj + *c0x.offset(2 as i32 as isize);
    *g
        .offset(
            19 as i32 as isize,
        ) = xixj + *c0x.offset(3 as i32 as isize);
    *g.offset(32 as i32 as isize) = 1 as i32 as f64;
    *g.offset(33 as i32 as isize) = 1 as i32 as f64;
    *g.offset(34 as i32 as isize) = 1 as i32 as f64;
    *g.offset(35 as i32 as isize) = 1 as i32 as f64;
    *g.offset(36 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(37 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(38 as i32 as isize) = *c0y.offset(2 as i32 as isize);
    *g.offset(39 as i32 as isize) = *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            40 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            41 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            42 as i32 as isize,
        ) = *c0y.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            43 as i32 as isize,
        ) = *c0y.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b10.offset(3 as i32 as isize);
    *g
        .offset(
            56 as i32 as isize,
        ) = *g.offset(40 as i32 as isize)
        * (yiyj + *c0y.offset(0 as i32 as isize))
        + *c0y.offset(0 as i32 as isize) * 2 as i32 as f64
            * *b10.offset(0 as i32 as isize);
    *g
        .offset(
            57 as i32 as isize,
        ) = *g.offset(41 as i32 as isize)
        * (yiyj + *c0y.offset(1 as i32 as isize))
        + *c0y.offset(1 as i32 as isize) * 2 as i32 as f64
            * *b10.offset(1 as i32 as isize);
    *g
        .offset(
            58 as i32 as isize,
        ) = *g.offset(42 as i32 as isize)
        * (yiyj + *c0y.offset(2 as i32 as isize))
        + *c0y.offset(2 as i32 as isize) * 2 as i32 as f64
            * *b10.offset(2 as i32 as isize);
    *g
        .offset(
            59 as i32 as isize,
        ) = *g.offset(43 as i32 as isize)
        * (yiyj + *c0y.offset(3 as i32 as isize))
        + *c0y.offset(3 as i32 as isize) * 2 as i32 as f64
            * *b10.offset(3 as i32 as isize);
    *g
        .offset(
            52 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (yiyj + *c0y.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            53 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (yiyj + *c0y.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            54 as i32 as isize,
        ) = *c0y.offset(2 as i32 as isize)
        * (yiyj + *c0y.offset(2 as i32 as isize))
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            55 as i32 as isize,
        ) = *c0y.offset(3 as i32 as isize)
        * (yiyj + *c0y.offset(3 as i32 as isize))
        + *b10.offset(3 as i32 as isize);
    *g
        .offset(
            48 as i32 as isize,
        ) = yiyj + *c0y.offset(0 as i32 as isize);
    *g
        .offset(
            49 as i32 as isize,
        ) = yiyj + *c0y.offset(1 as i32 as isize);
    *g
        .offset(
            50 as i32 as isize,
        ) = yiyj + *c0y.offset(2 as i32 as isize);
    *g
        .offset(
            51 as i32 as isize,
        ) = yiyj + *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            68 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(64 as i32 as isize);
    *g
        .offset(
            69 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(65 as i32 as isize);
    *g
        .offset(
            70 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(66 as i32 as isize);
    *g
        .offset(
            71 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(67 as i32 as isize);
    *g
        .offset(
            72 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(68 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(64 as i32 as isize);
    *g
        .offset(
            73 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(69 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(65 as i32 as isize);
    *g
        .offset(
            74 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(70 as i32 as isize)
        + *b10.offset(2 as i32 as isize) * *g.offset(66 as i32 as isize);
    *g
        .offset(
            75 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(71 as i32 as isize)
        + *b10.offset(3 as i32 as isize) * *g.offset(67 as i32 as isize);
    *g
        .offset(
            88 as i32 as isize,
        ) = *g.offset(72 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize))
        + 2 as i32 as f64 * *b10.offset(0 as i32 as isize)
            * *g.offset(68 as i32 as isize);
    *g
        .offset(
            89 as i32 as isize,
        ) = *g.offset(73 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize))
        + 2 as i32 as f64 * *b10.offset(1 as i32 as isize)
            * *g.offset(69 as i32 as isize);
    *g
        .offset(
            90 as i32 as isize,
        ) = *g.offset(74 as i32 as isize)
        * (zizj + *c0z.offset(2 as i32 as isize))
        + 2 as i32 as f64 * *b10.offset(2 as i32 as isize)
            * *g.offset(70 as i32 as isize);
    *g
        .offset(
            91 as i32 as isize,
        ) = *g.offset(75 as i32 as isize)
        * (zizj + *c0z.offset(3 as i32 as isize))
        + 2 as i32 as f64 * *b10.offset(3 as i32 as isize)
            * *g.offset(71 as i32 as isize);
    *g
        .offset(
            84 as i32 as isize,
        ) = *g.offset(68 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize))
        + *b10.offset(0 as i32 as isize) * *g.offset(64 as i32 as isize);
    *g
        .offset(
            85 as i32 as isize,
        ) = *g.offset(69 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize))
        + *b10.offset(1 as i32 as isize) * *g.offset(65 as i32 as isize);
    *g
        .offset(
            86 as i32 as isize,
        ) = *g.offset(70 as i32 as isize)
        * (zizj + *c0z.offset(2 as i32 as isize))
        + *b10.offset(2 as i32 as isize) * *g.offset(66 as i32 as isize);
    *g
        .offset(
            87 as i32 as isize,
        ) = *g.offset(71 as i32 as isize)
        * (zizj + *c0z.offset(3 as i32 as isize))
        + *b10.offset(3 as i32 as isize) * *g.offset(67 as i32 as isize);
    *g
        .offset(
            80 as i32 as isize,
        ) = *g.offset(64 as i32 as isize)
        * (zizj + *c0z.offset(0 as i32 as isize));
    *g
        .offset(
            81 as i32 as isize,
        ) = *g.offset(65 as i32 as isize)
        * (zizj + *c0z.offset(1 as i32 as isize));
    *g
        .offset(
            82 as i32 as isize,
        ) = *g.offset(66 as i32 as isize)
        * (zizj + *c0z.offset(2 as i32 as isize));
    *g
        .offset(
            83 as i32 as isize,
        ) = *g.offset(67 as i32 as isize)
        * (zizj + *c0z.offset(3 as i32 as isize));
}
#[inline]
unsafe extern "C" fn _srg0_2d4d_3000(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut c0x: *mut f64 = ((*bc).c00x).as_mut_ptr();
    let mut c0y: *mut f64 = ((*bc).c00y).as_mut_ptr();
    let mut c0z: *mut f64 = ((*bc).c00z).as_mut_ptr();
    let mut b10: *mut f64 = ((*bc).b10).as_mut_ptr();
    *g.offset(0 as i32 as isize) = 1 as i32 as f64;
    *g.offset(1 as i32 as isize) = 1 as i32 as f64;
    *g.offset(2 as i32 as isize) = 1 as i32 as f64;
    *g.offset(3 as i32 as isize) = 1 as i32 as f64;
    *g.offset(4 as i32 as isize) = *c0x.offset(0 as i32 as isize);
    *g.offset(5 as i32 as isize) = *c0x.offset(1 as i32 as isize);
    *g.offset(6 as i32 as isize) = *c0x.offset(2 as i32 as isize);
    *g.offset(7 as i32 as isize) = *c0x.offset(3 as i32 as isize);
    *g
        .offset(
            8 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * *c0x.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            9 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * *c0x.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            10 as i32 as isize,
        ) = *c0x.offset(2 as i32 as isize)
        * *c0x.offset(2 as i32 as isize)
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            11 as i32 as isize,
        ) = *c0x.offset(3 as i32 as isize)
        * *c0x.offset(3 as i32 as isize)
        + *b10.offset(3 as i32 as isize);
    *g
        .offset(
            12 as i32 as isize,
        ) = *c0x.offset(0 as i32 as isize)
        * (*g.offset(8 as i32 as isize)
            + 2 as i32 as f64
                * *b10.offset(0 as i32 as isize));
    *g
        .offset(
            13 as i32 as isize,
        ) = *c0x.offset(1 as i32 as isize)
        * (*g.offset(9 as i32 as isize)
            + 2 as i32 as f64
                * *b10.offset(1 as i32 as isize));
    *g
        .offset(
            14 as i32 as isize,
        ) = *c0x.offset(2 as i32 as isize)
        * (*g.offset(10 as i32 as isize)
            + 2 as i32 as f64
                * *b10.offset(2 as i32 as isize));
    *g
        .offset(
            15 as i32 as isize,
        ) = *c0x.offset(3 as i32 as isize)
        * (*g.offset(11 as i32 as isize)
            + 2 as i32 as f64
                * *b10.offset(3 as i32 as isize));
    *g.offset(16 as i32 as isize) = 1 as i32 as f64;
    *g.offset(17 as i32 as isize) = 1 as i32 as f64;
    *g.offset(18 as i32 as isize) = 1 as i32 as f64;
    *g.offset(19 as i32 as isize) = 1 as i32 as f64;
    *g.offset(20 as i32 as isize) = *c0y.offset(0 as i32 as isize);
    *g.offset(21 as i32 as isize) = *c0y.offset(1 as i32 as isize);
    *g.offset(22 as i32 as isize) = *c0y.offset(2 as i32 as isize);
    *g.offset(23 as i32 as isize) = *c0y.offset(3 as i32 as isize);
    *g
        .offset(
            24 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * *c0y.offset(0 as i32 as isize)
        + *b10.offset(0 as i32 as isize);
    *g
        .offset(
            25 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * *c0y.offset(1 as i32 as isize)
        + *b10.offset(1 as i32 as isize);
    *g
        .offset(
            26 as i32 as isize,
        ) = *c0y.offset(2 as i32 as isize)
        * *c0y.offset(2 as i32 as isize)
        + *b10.offset(2 as i32 as isize);
    *g
        .offset(
            27 as i32 as isize,
        ) = *c0y.offset(3 as i32 as isize)
        * *c0y.offset(3 as i32 as isize)
        + *b10.offset(3 as i32 as isize);
    *g
        .offset(
            28 as i32 as isize,
        ) = *c0y.offset(0 as i32 as isize)
        * (*g.offset(24 as i32 as isize)
            + 2 as i32 as f64
                * *b10.offset(0 as i32 as isize));
    *g
        .offset(
            29 as i32 as isize,
        ) = *c0y.offset(1 as i32 as isize)
        * (*g.offset(25 as i32 as isize)
            + 2 as i32 as f64
                * *b10.offset(1 as i32 as isize));
    *g
        .offset(
            30 as i32 as isize,
        ) = *c0y.offset(2 as i32 as isize)
        * (*g.offset(26 as i32 as isize)
            + 2 as i32 as f64
                * *b10.offset(2 as i32 as isize));
    *g
        .offset(
            31 as i32 as isize,
        ) = *c0y.offset(3 as i32 as isize)
        * (*g.offset(27 as i32 as isize)
            + 2 as i32 as f64
                * *b10.offset(3 as i32 as isize));
    *g
        .offset(
            36 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(32 as i32 as isize);
    *g
        .offset(
            37 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(33 as i32 as isize);
    *g
        .offset(
            38 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(34 as i32 as isize);
    *g
        .offset(
            39 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(35 as i32 as isize);
    *g
        .offset(
            40 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(36 as i32 as isize)
        + *b10.offset(0 as i32 as isize) * *g.offset(32 as i32 as isize);
    *g
        .offset(
            41 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(37 as i32 as isize)
        + *b10.offset(1 as i32 as isize) * *g.offset(33 as i32 as isize);
    *g
        .offset(
            42 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(38 as i32 as isize)
        + *b10.offset(2 as i32 as isize) * *g.offset(34 as i32 as isize);
    *g
        .offset(
            43 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(39 as i32 as isize)
        + *b10.offset(3 as i32 as isize) * *g.offset(35 as i32 as isize);
    *g
        .offset(
            44 as i32 as isize,
        ) = *c0z.offset(0 as i32 as isize)
        * *g.offset(40 as i32 as isize)
        + 2 as i32 as f64 * *b10.offset(0 as i32 as isize)
            * *g.offset(36 as i32 as isize);
    *g
        .offset(
            45 as i32 as isize,
        ) = *c0z.offset(1 as i32 as isize)
        * *g.offset(41 as i32 as isize)
        + 2 as i32 as f64 * *b10.offset(1 as i32 as isize)
            * *g.offset(37 as i32 as isize);
    *g
        .offset(
            46 as i32 as isize,
        ) = *c0z.offset(2 as i32 as isize)
        * *g.offset(42 as i32 as isize)
        + 2 as i32 as f64 * *b10.offset(2 as i32 as isize)
            * *g.offset(38 as i32 as isize);
    *g
        .offset(
            47 as i32 as isize,
        ) = *c0z.offset(3 as i32 as isize)
        * *g.offset(43 as i32 as isize)
        + 2 as i32 as f64 * *b10.offset(3 as i32 as isize)
            * *g.offset(39 as i32 as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CINTsrg0_2e_2d4d_unrolled(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    let mut type_ijkl: i32 = (*envs).li_ceil << 6 as i32
        | (*envs).lj_ceil << 4 as i32 | (*envs).lk_ceil << 2 as i32
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
    eprintln!("Dimension error for CINTg0_2e_lj2d4d: iklj = {} {} {} {}", (*envs).li_ceil, (*envs).lk_ceil, (*envs).ll_ceil, (*envs).lj_ceil);
    // fprintf(
    //     stderr,
    //     b"Dimension error for CINTg0_2e_lj2d4d: iklj = %d %d %d %d\0" as *const u8
    //         as *const libc::c_char,
    //     (*envs).li_ceil,
    //     (*envs).lk_ceil,
    //     (*envs).ll_ceil,
    //     (*envs).lj_ceil,
    // );
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e_lj2d4d(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    CINTg0_2e_2d(g, bc, envs);
    CINTg0_lj2d_4d(g, envs);
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e_kj2d4d(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    CINTg0_2e_2d(g, bc, envs);
    CINTg0_kj2d_4d(g, envs);
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e_ik2d4d(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    CINTg0_2e_2d(g, bc, envs);
    CINTg0_ik2d_4d(g, envs);
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e_il2d4d(
    mut g: *mut f64,
    mut bc: *mut Rys2eT,
    mut envs: *mut CINTEnvVars,
) {
    CINTg0_2e_2d(g, bc, envs);
    CINTg0_il2d_4d(g, envs);
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_2e(
    mut g: *mut f64,
    mut rij: *mut f64,
    mut rkl: *mut f64,
    mut cutoff: f64,
    mut envs: *mut CINTEnvVars,
) -> i32 {
    let mut irys: i32 = 0;
    let mut nroots: i32 = (*envs).nrys_roots;
    let mut aij: f64 = (*envs).ai[0 as i32 as usize]
        + (*envs).aj[0 as i32 as usize];
    let mut akl: f64 = (*envs).ak[0 as i32 as usize]
        + (*envs).al[0 as i32 as usize];
    let mut a0: f64 = 0.;
    let mut a1: f64 = 0.;
    let mut fac1: f64 = 0.;
    let mut x: f64 = 0.;
    let mut u: [f64; 32] = [0.; 32];
    let mut w: *mut f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut xij_kl: f64 = *rij.offset(0 as i32 as isize)
        - *rkl.offset(0 as i32 as isize);
    let mut yij_kl: f64 = *rij.offset(1 as i32 as isize)
        - *rkl.offset(1 as i32 as isize);
    let mut zij_kl: f64 = *rij.offset(2 as i32 as isize)
        - *rkl.offset(2 as i32 as isize);
    let mut rr: f64 = xij_kl * xij_kl + yij_kl * yij_kl + zij_kl * zij_kl;
    a1 = aij * akl;
    a0 = a1 / (aij + akl);
    fac1 = (a0 / (a1 * a1 * a1)).sqrt() * (*envs).fac[0 as i32 as usize];
    x = a0 * rr;
    let omega: f64 = *((*envs).env).offset(8 as i32 as isize);
    let mut theta: f64 = 0 as i32 as f64;
    if omega == 0.0f64 {
        CINTrys_roots(nroots, x, u.as_mut_ptr(), w);
    } else if omega < 0.0f64 {
        theta = omega * omega / (omega * omega + a0);
        if theta * x > cutoff || theta * x > 40 as i32 as f64 {
            return 0 as i32;
        }
        let mut rorder: i32 = (*envs).rys_order;
        if rorder == nroots {
            CINTsr_rys_roots(nroots, x, (theta).sqrt(), u.as_mut_ptr(), w);
        } else {
            let mut sqrt_theta: f64 = -(theta.sqrt());
            CINTrys_roots(rorder, x, u.as_mut_ptr(), w);
            CINTrys_roots(
                rorder,
                theta * x,
                u.as_mut_ptr().offset(rorder as isize),
                w.offset(rorder as isize),
            );
            if (*envs).g_size == 2 as i32 {
                *g
                    .offset(
                        0 as i32 as isize,
                    ) = 1 as i32 as f64;
                *g
                    .offset(
                        1 as i32 as isize,
                    ) = 1 as i32 as f64;
                *g
                    .offset(
                        2 as i32 as isize,
                    ) = 1 as i32 as f64;
                *g
                    .offset(
                        3 as i32 as isize,
                    ) = 1 as i32 as f64;
                *g.offset(4 as i32 as isize) *= fac1;
                *g.offset(5 as i32 as isize) *= fac1 * sqrt_theta;
                return 1 as i32;
            }
            irys = rorder;
            while irys < nroots {
                let mut ut: f64 = u[irys as usize] * theta;
                u[irys as usize] = ut / (u[irys as usize] + 1.0f64 - ut);
                *w.offset(irys as isize) *= sqrt_theta;
                irys += 1;
            }
        }
    } else {
        theta = omega * omega / (omega * omega + a0);
        x *= theta;
        fac1 *= theta.sqrt();
        CINTrys_roots(nroots, x, u.as_mut_ptr(), w);
        irys = 0 as i32;
        while irys < nroots {
            let mut ut_0: f64 = u[irys as usize] * theta;
            u[irys as usize] = ut_0 / (u[irys as usize] + 1.0f64 - ut_0);
            irys += 1;
        }
    }
    if (*envs).g_size == 1 as i32 {
        *g.offset(0 as i32 as isize) = 1 as i32 as f64;
        *g.offset(1 as i32 as isize) = 1 as i32 as f64;
        *g.offset(2 as i32 as isize) *= fac1;
        return 1 as i32;
    }
    let mut u2: f64 = 0.;
    let mut tmp1: f64 = 0.;
    let mut tmp2: f64 = 0.;
    let mut tmp3: f64 = 0.;
    let mut tmp4: f64 = 0.;
    let mut tmp5: f64 = 0.;
    let mut rijrx: f64 = *rij.offset(0 as i32 as isize)
        - *((*envs).rx_in_rijrx).offset(0 as i32 as isize);
    let mut rijry: f64 = *rij.offset(1 as i32 as isize)
        - *((*envs).rx_in_rijrx).offset(1 as i32 as isize);
    let mut rijrz: f64 = *rij.offset(2 as i32 as isize)
        - *((*envs).rx_in_rijrx).offset(2 as i32 as isize);
    let mut rklrx: f64 = *rkl.offset(0 as i32 as isize)
        - *((*envs).rx_in_rklrx).offset(0 as i32 as isize);
    let mut rklry: f64 = *rkl.offset(1 as i32 as isize)
        - *((*envs).rx_in_rklrx).offset(1 as i32 as isize);
    let mut rklrz: f64 = *rkl.offset(2 as i32 as isize)
        - *((*envs).rx_in_rklrx).offset(2 as i32 as isize);
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
    let mut b00: *mut f64 = (bc.b00).as_mut_ptr();
    let mut b10: *mut f64 = (bc.b10).as_mut_ptr();
    let mut b01: *mut f64 = (bc.b01).as_mut_ptr();
    let mut c00x: *mut f64 = (bc.c00x).as_mut_ptr();
    let mut c00y: *mut f64 = (bc.c00y).as_mut_ptr();
    let mut c00z: *mut f64 = (bc.c00z).as_mut_ptr();
    let mut c0px: *mut f64 = (bc.c0px).as_mut_ptr();
    let mut c0py: *mut f64 = (bc.c0py).as_mut_ptr();
    let mut c0pz: *mut f64 = (bc.c0pz).as_mut_ptr();
    irys = 0 as i32;
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
    }
    ::core::mem::transmute::<
        _,
        fn(_, _, _),
    >(
        (Some(((*envs).f_g0_2d4d).expect("non-null function pointer")))
            .expect("non-null function pointer"),
    )(g, &mut bc, envs);
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1i_2e(
    mut f: *mut f64,
    mut g: *const f64,
    li: i32,
    lj: i32,
    lk: i32,
    ll: i32,
    mut envs: *const CINTEnvVars,
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
    let ai2: f64 = -(2 as i32) as f64
        * (*envs).ai[0 as i32 as usize];
    let mut gx: *const f64 = g;
    let mut gy: *const f64 = g.offset((*envs).g_size as isize);
    let mut gz: *const f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut fx: *mut f64 = f;
    let mut fy: *mut f64 = f.offset((*envs).g_size as isize);
    let mut fz: *mut f64 = f
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut p1x: *const f64 = gx.offset(-(di as isize));
    let mut p1y: *const f64 = gy.offset(-(di as isize));
    let mut p1z: *const f64 = gz.offset(-(di as isize));
    let mut p2x: *const f64 = gx.offset(di as isize);
    let mut p2y: *const f64 = gy.offset(di as isize);
    let mut p2z: *const f64 = gz.offset(di as isize);
    j = 0 as i32;
    while j <= lj {
        l = 0 as i32;
        while l <= ll {
            k = 0 as i32;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                n = ptr;
                while n < ptr + nroots {
                    *fx.offset(n as isize) = ai2 * *p2x.offset(n as isize);
                    *fy.offset(n as isize) = ai2 * *p2y.offset(n as isize);
                    *fz.offset(n as isize) = ai2 * *p2z.offset(n as isize);
                    n += 1;
                }
                ptr += di;
                i = 1 as i32;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx
                            .offset(
                                n as isize,
                            ) = i as f64 * *p1x.offset(n as isize)
                            + ai2 * *p2x.offset(n as isize);
                        *fy
                            .offset(
                                n as isize,
                            ) = i as f64 * *p1y.offset(n as isize)
                            + ai2 * *p2y.offset(n as isize);
                        *fz
                            .offset(
                                n as isize,
                            ) = i as f64 * *p1z.offset(n as isize)
                            + ai2 * *p2z.offset(n as isize);
                        n += 1;
                    }
                    ptr += di;
                    i += 1;
                }
                k += 1;
            }
            l += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1j_2e(
    mut f: *mut f64,
    mut g: *const f64,
    li: i32,
    lj: i32,
    lk: i32,
    ll: i32,
    mut envs: *const CINTEnvVars,
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
    let aj2: f64 = -(2 as i32) as f64
        * (*envs).aj[0 as i32 as usize];
    let mut gx: *const f64 = g;
    let mut gy: *const f64 = g.offset((*envs).g_size as isize);
    let mut gz: *const f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut fx: *mut f64 = f;
    let mut fy: *mut f64 = f.offset((*envs).g_size as isize);
    let mut fz: *mut f64 = f
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut p1x: *const f64 = gx.offset(-(dj as isize));
    let mut p1y: *const f64 = gy.offset(-(dj as isize));
    let mut p1z: *const f64 = gz.offset(-(dj as isize));
    let mut p2x: *const f64 = gx.offset(dj as isize);
    let mut p2y: *const f64 = gy.offset(dj as isize);
    let mut p2z: *const f64 = gz.offset(dj as isize);
    l = 0 as i32;
    while l <= ll {
        k = 0 as i32;
        while k <= lk {
            ptr = dl * l + dk * k;
            i = 0 as i32;
            while i <= li {
                n = ptr;
                while n < ptr + nroots {
                    *fx.offset(n as isize) = aj2 * *p2x.offset(n as isize);
                    *fy.offset(n as isize) = aj2 * *p2y.offset(n as isize);
                    *fz.offset(n as isize) = aj2 * *p2z.offset(n as isize);
                    n += 1;
                }
                ptr += di;
                i += 1;
            }
            k += 1;
        }
        l += 1;
    }
    j = 1 as i32;
    while j <= lj {
        l = 0 as i32;
        while l <= ll {
            k = 0 as i32;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0 as i32;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx
                            .offset(
                                n as isize,
                            ) = j as f64 * *p1x.offset(n as isize)
                            + aj2 * *p2x.offset(n as isize);
                        *fy
                            .offset(
                                n as isize,
                            ) = j as f64 * *p1y.offset(n as isize)
                            + aj2 * *p2y.offset(n as isize);
                        *fz
                            .offset(
                                n as isize,
                            ) = j as f64 * *p1z.offset(n as isize)
                            + aj2 * *p2z.offset(n as isize);
                        n += 1;
                    }
                    ptr += di;
                    i += 1;
                }
                k += 1;
            }
            l += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1k_2e(
    mut f: *mut f64,
    mut g: *const f64,
    li: i32,
    lj: i32,
    lk: i32,
    ll: i32,
    mut envs: *const CINTEnvVars,
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
    let ak2: f64 = -(2 as i32) as f64
        * (*envs).ak[0 as i32 as usize];
    let mut gx: *const f64 = g;
    let mut gy: *const f64 = g.offset((*envs).g_size as isize);
    let mut gz: *const f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut fx: *mut f64 = f;
    let mut fy: *mut f64 = f.offset((*envs).g_size as isize);
    let mut fz: *mut f64 = f
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut p1x: *const f64 = gx.offset(-(dk as isize));
    let mut p1y: *const f64 = gy.offset(-(dk as isize));
    let mut p1z: *const f64 = gz.offset(-(dk as isize));
    let mut p2x: *const f64 = gx.offset(dk as isize);
    let mut p2y: *const f64 = gy.offset(dk as isize);
    let mut p2z: *const f64 = gz.offset(dk as isize);
    j = 0 as i32;
    while j <= lj {
        l = 0 as i32;
        while l <= ll {
            ptr = dj * j + dl * l;
            i = 0 as i32;
            while i <= li {
                n = ptr;
                while n < ptr + nroots {
                    *fx.offset(n as isize) = ak2 * *p2x.offset(n as isize);
                    *fy.offset(n as isize) = ak2 * *p2y.offset(n as isize);
                    *fz.offset(n as isize) = ak2 * *p2z.offset(n as isize);
                    n += 1;
                }
                ptr += di;
                i += 1;
            }
            k = 1 as i32;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0 as i32;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx
                            .offset(
                                n as isize,
                            ) = k as f64 * *p1x.offset(n as isize)
                            + ak2 * *p2x.offset(n as isize);
                        *fy
                            .offset(
                                n as isize,
                            ) = k as f64 * *p1y.offset(n as isize)
                            + ak2 * *p2y.offset(n as isize);
                        *fz
                            .offset(
                                n as isize,
                            ) = k as f64 * *p1z.offset(n as isize)
                            + ak2 * *p2z.offset(n as isize);
                        n += 1;
                    }
                    ptr += di;
                    i += 1;
                }
                k += 1;
            }
            l += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1l_2e(
    mut f: *mut f64,
    mut g: *const f64,
    li: i32,
    lj: i32,
    lk: i32,
    ll: i32,
    mut envs: *const CINTEnvVars,
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
    let al2: f64 = -(2 as i32) as f64
        * (*envs).al[0 as i32 as usize];
    let mut gx: *const f64 = g;
    let mut gy: *const f64 = g.offset((*envs).g_size as isize);
    let mut gz: *const f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut fx: *mut f64 = f;
    let mut fy: *mut f64 = f.offset((*envs).g_size as isize);
    let mut fz: *mut f64 = f
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut p1x: *const f64 = gx.offset(-(dl as isize));
    let mut p1y: *const f64 = gy.offset(-(dl as isize));
    let mut p1z: *const f64 = gz.offset(-(dl as isize));
    let mut p2x: *const f64 = gx.offset(dl as isize);
    let mut p2y: *const f64 = gy.offset(dl as isize);
    let mut p2z: *const f64 = gz.offset(dl as isize);
    j = 0 as i32;
    while j <= lj {
        k = 0 as i32;
        while k <= lk {
            ptr = dj * j + dk * k;
            i = 0 as i32;
            while i <= li {
                n = ptr;
                while n < ptr + nroots {
                    *fx.offset(n as isize) = al2 * *p2x.offset(n as isize);
                    *fy.offset(n as isize) = al2 * *p2y.offset(n as isize);
                    *fz.offset(n as isize) = al2 * *p2z.offset(n as isize);
                    n += 1;
                }
                ptr += di;
                i += 1;
            }
            k += 1;
        }
        l = 1 as i32;
        while l <= ll {
            k = 0 as i32;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0 as i32;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx
                            .offset(
                                n as isize,
                            ) = l as f64 * *p1x.offset(n as isize)
                            + al2 * *p2x.offset(n as isize);
                        *fy
                            .offset(
                                n as isize,
                            ) = l as f64 * *p1y.offset(n as isize)
                            + al2 * *p2y.offset(n as isize);
                        *fz
                            .offset(
                                n as isize,
                            ) = l as f64 * *p1z.offset(n as isize)
                            + al2 * *p2z.offset(n as isize);
                        n += 1;
                    }
                    i += 1;
                    ptr += di;
                }
                k += 1;
            }
            l += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1i_2e(
    mut f: *mut f64,
    mut g: *const f64,
    mut ri: *const f64,
    li: i32,
    lj: i32,
    lk: i32,
    ll: i32,
    mut envs: *const CINTEnvVars,
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
    let mut gx: *const f64 = g;
    let mut gy: *const f64 = g.offset((*envs).g_size as isize);
    let mut gz: *const f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut fx: *mut f64 = f;
    let mut fy: *mut f64 = f.offset((*envs).g_size as isize);
    let mut fz: *mut f64 = f
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut p1x: *const f64 = gx.offset(di as isize);
    let mut p1y: *const f64 = gy.offset(di as isize);
    let mut p1z: *const f64 = gz.offset(di as isize);
    j = 0 as i32;
    while j <= lj {
        l = 0 as i32;
        while l <= ll {
            k = 0 as i32;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0 as i32;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx
                            .offset(
                                n as isize,
                            ) = *p1x.offset(n as isize)
                            + *ri.offset(0 as i32 as isize)
                                * *gx.offset(n as isize);
                        *fy
                            .offset(
                                n as isize,
                            ) = *p1y.offset(n as isize)
                            + *ri.offset(1 as i32 as isize)
                                * *gy.offset(n as isize);
                        *fz
                            .offset(
                                n as isize,
                            ) = *p1z.offset(n as isize)
                            + *ri.offset(2 as i32 as isize)
                                * *gz.offset(n as isize);
                        n += 1;
                    }
                    ptr += di;
                    i += 1;
                }
                k += 1;
            }
            l += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1j_2e(
    mut f: *mut f64,
    mut g: *const f64,
    mut rj: *const f64,
    li: i32,
    lj: i32,
    lk: i32,
    ll: i32,
    mut envs: *const CINTEnvVars,
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
    let mut gx: *const f64 = g;
    let mut gy: *const f64 = g.offset((*envs).g_size as isize);
    let mut gz: *const f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut fx: *mut f64 = f;
    let mut fy: *mut f64 = f.offset((*envs).g_size as isize);
    let mut fz: *mut f64 = f
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut p1x: *const f64 = gx.offset(dj as isize);
    let mut p1y: *const f64 = gy.offset(dj as isize);
    let mut p1z: *const f64 = gz.offset(dj as isize);
    j = 0 as i32;
    while j <= lj {
        l = 0 as i32;
        while l <= ll {
            k = 0 as i32;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0 as i32;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx
                            .offset(
                                n as isize,
                            ) = *p1x.offset(n as isize)
                            + *rj.offset(0 as i32 as isize)
                                * *gx.offset(n as isize);
                        *fy
                            .offset(
                                n as isize,
                            ) = *p1y.offset(n as isize)
                            + *rj.offset(1 as i32 as isize)
                                * *gy.offset(n as isize);
                        *fz
                            .offset(
                                n as isize,
                            ) = *p1z.offset(n as isize)
                            + *rj.offset(2 as i32 as isize)
                                * *gz.offset(n as isize);
                        n += 1;
                    }
                    ptr += di;
                    i += 1;
                }
                k += 1;
            }
            l += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1k_2e(
    mut f: *mut f64,
    mut g: *const f64,
    mut rk: *const f64,
    li: i32,
    lj: i32,
    lk: i32,
    ll: i32,
    mut envs: *const CINTEnvVars,
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
    let mut gx: *const f64 = g;
    let mut gy: *const f64 = g.offset((*envs).g_size as isize);
    let mut gz: *const f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut fx: *mut f64 = f;
    let mut fy: *mut f64 = f.offset((*envs).g_size as isize);
    let mut fz: *mut f64 = f
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut p1x: *const f64 = gx.offset(dk as isize);
    let mut p1y: *const f64 = gy.offset(dk as isize);
    let mut p1z: *const f64 = gz.offset(dk as isize);
    j = 0 as i32;
    while j <= lj {
        l = 0 as i32;
        while l <= ll {
            k = 0 as i32;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0 as i32;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx
                            .offset(
                                n as isize,
                            ) = *p1x.offset(n as isize)
                            + *rk.offset(0 as i32 as isize)
                                * *gx.offset(n as isize);
                        *fy
                            .offset(
                                n as isize,
                            ) = *p1y.offset(n as isize)
                            + *rk.offset(1 as i32 as isize)
                                * *gy.offset(n as isize);
                        *fz
                            .offset(
                                n as isize,
                            ) = *p1z.offset(n as isize)
                            + *rk.offset(2 as i32 as isize)
                                * *gz.offset(n as isize);
                        n += 1;
                    }
                    ptr += di;
                    i += 1;
                }
                k += 1;
            }
            l += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1l_2e(
    mut f: *mut f64,
    mut g: *const f64,
    mut rl: *const f64,
    li: i32,
    lj: i32,
    lk: i32,
    ll: i32,
    mut envs: *const CINTEnvVars,
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
    let mut gx: *const f64 = g;
    let mut gy: *const f64 = g.offset((*envs).g_size as isize);
    let mut gz: *const f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut fx: *mut f64 = f;
    let mut fy: *mut f64 = f.offset((*envs).g_size as isize);
    let mut fz: *mut f64 = f
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut p1x: *const f64 = gx.offset(dl as isize);
    let mut p1y: *const f64 = gy.offset(dl as isize);
    let mut p1z: *const f64 = gz.offset(dl as isize);
    j = 0 as i32;
    while j <= lj {
        l = 0 as i32;
        while l <= ll {
            k = 0 as i32;
            while k <= lk {
                ptr = dj * j + dl * l + dk * k;
                i = 0 as i32;
                while i <= li {
                    n = ptr;
                    while n < ptr + nroots {
                        *fx
                            .offset(
                                n as isize,
                            ) = *p1x.offset(n as isize)
                            + *rl.offset(0 as i32 as isize)
                                * *gx.offset(n as isize);
                        *fy
                            .offset(
                                n as isize,
                            ) = *p1y.offset(n as isize)
                            + *rl.offset(1 as i32 as isize)
                                * *gy.offset(n as isize);
                        *fz
                            .offset(
                                n as isize,
                            ) = *p1z.offset(n as isize)
                            + *rl.offset(2 as i32 as isize)
                                * *gz.offset(n as isize);
                        n += 1;
                    }
                    ptr += di;
                    i += 1;
                }
                k += 1;
            }
            l += 1;
        }
        j += 1;
    }
}
