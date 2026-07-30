#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments
)]

use crate::g1e::CINTg1e_index_xyz;
use crate::g1e::CINTinit_int1e_EnvVars;
use crate::g2e::CINTg2e_index_xyz;
use crate::g2e::CINTinit_int2e_EnvVars;
// use crate::g1e_grids::CINTinit_int1e_grids_EnvVars;
// use crate::g2c2e::CINTinit_int2c2e_EnvVars;
// use crate::g3c1e::CINTinit_int3c1e_EnvVars;
// use crate::g3c1e::CINTg3c1e_index_xyz;
// use crate::g3c2e::CINTinit_int3c2e_EnvVars;

use crate::cint::CINTEnvVars;
use crate::cint::CINTOpt;
use crate::cint::PairData;

extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}

#[no_mangle]
pub unsafe extern "C" fn CINTinit_2e_optimizer(
    opt: *mut *mut CINTOpt,
    _atm: *mut i32,
    _natm: i32,
    _bas: *mut i32,
    nbas: i32,
    _env: *mut f64,
) {
    let opt0: *mut CINTOpt =
        malloc(::core::mem::size_of::<CINTOpt>() as libc::c_ulong) as *mut CINTOpt;
    (*opt0).index_xyz_array = std::ptr::null_mut::<*mut i32>();
    (*opt0).non0ctr = std::ptr::null_mut::<*mut i32>();
    (*opt0).sortedidx = std::ptr::null_mut::<*mut i32>();
    (*opt0).nbas = nbas;
    (*opt0).log_max_coeff = std::ptr::null_mut::<*mut f64>();
    (*opt0).pairdata = std::ptr::null_mut::<*mut PairData>();
    *opt = opt0;
}
#[no_mangle]
pub unsafe extern "C" fn CINTinit_optimizer(
    opt: *mut *mut CINTOpt,
    atm: *mut i32,
    natm: i32,
    bas: *mut i32,
    nbas: i32,
    env: *mut f64,
) {
    CINTinit_2e_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn CINTdel_2e_optimizer(opt: *mut *mut CINTOpt) {
    let opt0: *mut CINTOpt = *opt;
    if opt0.is_null() {
        return;
    }
    if !((*opt0).index_xyz_array).is_null() {
        free(*((*opt0).index_xyz_array).offset(0_isize) as *mut libc::c_void);
        free((*opt0).index_xyz_array as *mut libc::c_void);
    }
    if !((*opt0).non0ctr).is_null() {
        free(*((*opt0).sortedidx).offset(0_isize) as *mut libc::c_void);
        free((*opt0).sortedidx as *mut libc::c_void);
        free(*((*opt0).non0ctr).offset(0_isize) as *mut libc::c_void);
        free((*opt0).non0ctr as *mut libc::c_void);
    }
    if !((*opt0).log_max_coeff).is_null() {
        free(*((*opt0).log_max_coeff).offset(0_isize) as *mut libc::c_void);
        free((*opt0).log_max_coeff as *mut libc::c_void);
    }
    CINTdel_pairdata_optimizer(opt0);
    free(opt0 as *mut libc::c_void);
    *opt = std::ptr::null_mut::<CINTOpt>();
}
#[no_mangle]
pub unsafe extern "C" fn CINTdel_optimizer(opt: *mut *mut CINTOpt) {
    CINTdel_2e_optimizer(opt);
}
#[no_mangle]
pub unsafe extern "C" fn CINTno_optimizer(
    opt: *mut *mut CINTOpt,
    _atm: *mut i32,
    _natm: i32,
    _bas: *mut i32,
    _nbas: i32,
    _env: *mut f64,
) {
    *opt = std::ptr::null_mut::<CINTOpt>();
}
unsafe extern "C" fn _make_fakebas(
    fakebas: *mut i32,
    bas: *mut i32,
    nbas: i32,
    _env: *mut f64,
) -> i32 {
    let mut i: i32 = 0;
    let mut max_l: i32 = 0_i32;
    i = 0_i32;
    while i < nbas {
        max_l = if max_l > *bas.offset((8_i32 * i + 1_i32) as isize) {
            max_l
        } else {
            *bas.offset((8_i32 * i + 1_i32) as isize)
        };
        i += 1;
        i;
    }
    let fakenbas: i32 = max_l + 1_i32;
    i = 0_i32;
    while i < 8_i32 * fakenbas {
        *fakebas.offset(i as isize) = 0_i32;
        i += 1;
        i;
    }
    i = 0_i32;
    while i <= max_l {
        *fakebas.offset((8_i32 * i + 1_i32) as isize) = i;
        i += 1;
        i;
    }
    max_l
}
unsafe extern "C" fn _allocate_index_xyz(
    opt: *mut CINTOpt,
    max_l: i32,
    l_allow: i32,
    order: i32,
) -> *mut i32 {
    let mut i: i32 = 0;
    let cumcart: i32 =
        (l_allow + 1_i32) * (l_allow + 2_i32) * (l_allow + 3_i32) / 6_i32;
    let mut ll: u64 = (max_l + 1_i32) as u64;
    let mut cc: u64 = cumcart as u64;
    i = 1_i32;
    while i < order {
        ll = (ll as libc::c_ulong).wrapping_mul(16 as libc::c_ulong);
        cc = (cc as libc::c_ulong).wrapping_mul(cumcart as libc::c_ulong);
        i += 1;
        i;
    }
    let buf: *mut i32 = malloc(
        (::core::mem::size_of::<i32>() as libc::c_ulong)
            .wrapping_mul(cc)
            .wrapping_mul(3 as libc::c_ulong),
    ) as *mut i32;
    let ppbuf: *mut *mut i32 =
        malloc((::core::mem::size_of::<*mut i32>() as libc::c_ulong).wrapping_mul(ll))
            as *mut *mut i32;
    let fresh0 = &mut *ppbuf.offset(0_isize);
    *fresh0 = buf;
    i = 1_i32;
    while (i as libc::c_ulong) < ll {
        let fresh1 = &mut *ppbuf.offset(i as isize);
        *fresh1 = std::ptr::null_mut::<i32>();
        i += 1;
        i;
    }
    (*opt).index_xyz_array = ppbuf;
    buf
}
unsafe extern "C" fn gen_idx(
    opt: *mut CINTOpt,
    finit: Option<unsafe extern "C" fn() -> ()>,
    findex_xyz: Option<unsafe extern "C" fn() -> ()>,
    order: i32,
    mut l_allow: i32,
    ng: &[i32; 8],
    atm: *mut i32,
    natm: i32,
    bas: *mut i32,
    nbas: i32,
    env: *mut f64,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut ptr: i32 = 0;
    let mut fakebas: [i32; 128] = [0; 128];
    let max_l: i32 = _make_fakebas(fakebas.as_mut_ptr(), bas, nbas, env);
    let fakenbas: i32 = max_l + 1_i32;
    l_allow = if max_l < l_allow { max_l } else { l_allow };
    let mut buf: *mut i32 = _allocate_index_xyz(opt, max_l, l_allow, order);
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    let mut shls: [i32; 4] = [0_i32, 0, 0, 0];
    if order == 2_i32 {
        i = 0_i32;
        while i <= l_allow {
            j = 0_i32;
            while j <= l_allow {
                shls[0_usize] = i;
                shls[1_usize] = j;
                ::core::mem::transmute::<_, fn(_, _, _, _, _, _, _, _)>(
                    finit.expect("non-null function pointer"),
                )(
                    &mut envs,
                    ng,
                    shls.as_mut_ptr(),
                    atm,
                    natm,
                    fakebas.as_mut_ptr(),
                    fakenbas,
                    env,
                );
                ptr = i * 16_i32 + j;
                let fresh2 = &mut *((*opt).index_xyz_array).offset(ptr as isize);
                *fresh2 = buf;
                ::core::mem::transmute::<_, fn(_, _)>(
                    findex_xyz.expect("non-null function pointer"),
                )(buf, &mut envs);
                buf = buf.offset((envs.nf * 3_i32) as isize);
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if order == 3_i32 {
        i = 0_i32;
        while i <= l_allow {
            j = 0_i32;
            while j <= l_allow {
                k = 0_i32;
                while k <= l_allow {
                    shls[0_usize] = i;
                    shls[1_usize] = j;
                    shls[2_usize] = k;
                    ::core::mem::transmute::<_, fn(_, _, _, _, _, _, _, _)>(
                        finit.expect("non-null function pointer"),
                    )(
                        &mut envs,
                        ng,
                        shls.as_mut_ptr(),
                        atm,
                        natm,
                        fakebas.as_mut_ptr(),
                        fakenbas,
                        env,
                    );
                    ptr = i * 16_i32 * 16_i32 + j * 16_i32 + k;
                    let fresh3 = &mut *((*opt).index_xyz_array).offset(ptr as isize);
                    *fresh3 = buf;
                    ::core::mem::transmute::<_, fn(_, _)>(
                        findex_xyz.expect("non-null function pointer"),
                    )(buf, &mut envs);
                    buf = buf.offset((envs.nf * 3_i32) as isize);
                    k += 1;
                    k;
                }
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else {
        i = 0_i32;
        while i <= l_allow {
            j = 0_i32;
            while j <= l_allow {
                k = 0_i32;
                while k <= l_allow {
                    l = 0_i32;
                    while l <= l_allow {
                        shls[0_usize] = i;
                        shls[1_usize] = j;
                        shls[2_usize] = k;
                        shls[3_usize] = l;
                        ::core::mem::transmute::<_, fn(_, _, _, _, _, _, _, _)>(
                            finit.expect("non-null function pointer"),
                        )(
                            &mut envs,
                            ng,
                            shls.as_mut_ptr(),
                            atm,
                            natm,
                            fakebas.as_mut_ptr(),
                            fakenbas,
                            env,
                        );
                        ptr = i * 16_i32 * 16_i32 * 16_i32
                            + j * 16_i32 * 16_i32
                            + k * 16_i32
                            + l;
                        let fresh4 = &mut *((*opt).index_xyz_array).offset(ptr as isize);
                        *fresh4 = buf;
                        ::core::mem::transmute::<_, fn(_, _)>(
                            findex_xyz.expect("non-null function pointer"),
                        )(buf, &mut envs);
                        buf = buf.offset((envs.nf * 3_i32) as isize);
                        l += 1;
                        l;
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
    };
}
#[no_mangle]
pub unsafe extern "C" fn CINTall_1e_optimizer(
    opt: *mut *mut CINTOpt,
    ng: &[i32; 8],
    atm: *mut i32,
    natm: i32,
    bas: *mut i32,
    nbas: i32,
    env: *mut f64,
) {
    CINTinit_2e_optimizer(opt, atm, natm, bas, nbas, env);
    CINTOpt_set_log_maxc(*opt, atm, natm, bas, nbas, env);
    CINTOpt_set_non0coeff(*opt, atm, natm, bas, nbas, env);
    gen_idx(
        *opt,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut CINTEnvVars,
                    &[i32; 8],
                    *mut i32,
                    *mut i32,
                    i32,
                    *mut i32,
                    i32,
                    *mut f64,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            CINTinit_int1e_EnvVars
                as unsafe extern "C" fn(
                    *mut CINTEnvVars,
                    &[i32; 8],
                    *mut i32,
                    *mut i32,
                    i32,
                    *mut i32,
                    i32,
                    *mut f64,
                ) -> (),
        )),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut i32, *mut CINTEnvVars) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            CINTg1e_index_xyz as unsafe extern "C" fn(*mut i32, *mut CINTEnvVars) -> (),
        )),
        2_i32,
        15_i32,
        ng,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTall_2e_optimizer(
    opt: *mut *mut CINTOpt,
    ng: &[i32; 8],
    atm: *mut i32,
    natm: i32,
    bas: *mut i32,
    nbas: i32,
    env: *mut f64,
) {
    CINTinit_2e_optimizer(opt, atm, natm, bas, nbas, env);
    CINTOpt_setij(*opt, ng, atm, natm, bas, nbas, env);
    CINTOpt_set_non0coeff(*opt, atm, natm, bas, nbas, env);
    gen_idx(
        *opt,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut CINTEnvVars,
                    &[i32; 8],
                    *mut i32,
                    *mut i32,
                    i32,
                    *mut i32,
                    i32,
                    *mut f64,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            CINTinit_int2e_EnvVars
                as unsafe extern "C" fn(
                    *mut CINTEnvVars,
                    &[i32; 8],
                    *mut i32,
                    *mut i32,
                    i32,
                    *mut i32,
                    i32,
                    *mut f64,
                ) -> (),
        )),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut i32, *const CINTEnvVars) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            CINTg2e_index_xyz as unsafe extern "C" fn(*mut i32, *const CINTEnvVars) -> (),
        )),
        4_i32,
        6_i32,
        ng,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
// #[no_mangle]
// pub unsafe extern "C" fn CINTall_3c2e_optimizer(
//     mut opt: *mut *mut CINTOpt,
//     mut ng: *mut i32,
//     mut atm: *mut i32,
//     mut natm: i32,
//     mut bas: *mut i32,
//     mut nbas: i32,
//     mut env: *mut f64,
// ) {
//     CINTinit_2e_optimizer(opt, atm, natm, bas, nbas, env);
//     CINTOpt_setij(*opt, ng, atm, natm, bas, nbas, env);
//     CINTOpt_set_non0coeff(*opt, atm, natm, bas, nbas, env);
//     gen_idx(
//         *opt,
//         ::core::mem::transmute::<
//             Option::<
//                 unsafe extern "C" fn(
//                     *mut CINTEnvVars,
//                     *mut i32,
//                     *mut i32,
//                     *mut i32,
//                     i32,
//                     *mut i32,
//                     i32,
//                     *mut f64,
//                 ) -> (),
//             >,
//             Option::<unsafe extern "C" fn() -> ()>,
//         >(
//             Some(
//                 CINTinit_int3c2e_EnvVars
//                     as unsafe extern "C" fn(
//                         *mut CINTEnvVars,
//                         *mut i32,
//                         *mut i32,
//                         *mut i32,
//                         i32,
//                         *mut i32,
//                         i32,
//                         *mut f64,
//                     ) -> (),
//             ),
//         ),
//         ::core::mem::transmute::<
//             Option::<unsafe extern "C" fn(*mut i32, *const CINTEnvVars) -> ()>,
//             Option::<unsafe extern "C" fn() -> ()>,
//         >(
//             Some(
//                 CINTg2e_index_xyz
//                     as unsafe extern "C" fn(*mut i32, *const CINTEnvVars) -> (),
//             ),
//         ),
//         3 as i32,
//         12 as i32,
//         ng,
//         atm,
//         natm,
//         bas,
//         nbas,
//         env,
//     );
// }
// #[no_mangle]
// pub unsafe extern "C" fn CINTall_2c2e_optimizer(
//     mut opt: *mut *mut CINTOpt,
//     mut ng: *mut i32,
//     mut atm: *mut i32,
//     mut natm: i32,
//     mut bas: *mut i32,
//     mut nbas: i32,
//     mut env: *mut f64,
// ) {
//     CINTinit_2e_optimizer(opt, atm, natm, bas, nbas, env);
//     CINTOpt_set_log_maxc(*opt, atm, natm, bas, nbas, env);
//     CINTOpt_set_non0coeff(*opt, atm, natm, bas, nbas, env);
//     gen_idx(
//         *opt,
//         ::core::mem::transmute::<
//             Option::<
//                 unsafe extern "C" fn(
//                     *mut CINTEnvVars,
//                     *mut i32,
//                     *mut i32,
//                     *mut i32,
//                     i32,
//                     *mut i32,
//                     i32,
//                     *mut f64,
//                 ) -> (),
//             >,
//             Option::<unsafe extern "C" fn() -> ()>,
//         >(
//             Some(
//                 CINTinit_int2c2e_EnvVars
//                     as unsafe extern "C" fn(
//                         *mut CINTEnvVars,
//                         *mut i32,
//                         *mut i32,
//                         *mut i32,
//                         i32,
//                         *mut i32,
//                         i32,
//                         *mut f64,
//                     ) -> (),
//             ),
//         ),
//         ::core::mem::transmute::<
//             Option::<unsafe extern "C" fn(*mut i32, *mut CINTEnvVars) -> ()>,
//             Option::<unsafe extern "C" fn() -> ()>,
//         >(
//             Some(
//                 CINTg1e_index_xyz
//                     as unsafe extern "C" fn(*mut i32, *mut CINTEnvVars) -> (),
//             ),
//         ),
//         2 as i32,
//         15 as i32,
//         ng,
//         atm,
//         natm,
//         bas,
//         nbas,
//         env,
//     );
// }
// #[no_mangle]
// pub unsafe extern "C" fn CINTall_3c1e_optimizer(
//     mut opt: *mut *mut CINTOpt,
//     mut ng: *mut i32,
//     mut atm: *mut i32,
//     mut natm: i32,
//     mut bas: *mut i32,
//     mut nbas: i32,
//     mut env: *mut f64,
// ) {
//     CINTinit_2e_optimizer(opt, atm, natm, bas, nbas, env);
//     CINTOpt_setij(*opt, ng, atm, natm, bas, nbas, env);
//     CINTOpt_set_non0coeff(*opt, atm, natm, bas, nbas, env);
//     gen_idx(
//         *opt,
//         ::core::mem::transmute::<
//             Option::<
//                 unsafe extern "C" fn(
//                     *mut CINTEnvVars,
//                     *mut i32,
//                     *mut i32,
//                     *mut i32,
//                     i32,
//                     *mut i32,
//                     i32,
//                     *mut f64,
//                 ) -> (),
//             >,
//             Option::<unsafe extern "C" fn() -> ()>,
//         >(
//             Some(
//                 CINTinit_int3c1e_EnvVars
//                     as unsafe extern "C" fn(
//                         *mut CINTEnvVars,
//                         *mut i32,
//                         *mut i32,
//                         *mut i32,
//                         i32,
//                         *mut i32,
//                         i32,
//                         *mut f64,
//                     ) -> (),
//             ),
//         ),
//         ::core::mem::transmute::<
//             Option::<unsafe extern "C" fn(*mut i32, *const CINTEnvVars) -> ()>,
//             Option::<unsafe extern "C" fn() -> ()>,
//         >(
//             Some(
//                 CINTg3c1e_index_xyz
//                     as unsafe extern "C" fn(*mut i32, *const CINTEnvVars) -> (),
//             ),
//         ),
//         3 as i32,
//         12 as i32,
//         ng,
//         atm,
//         natm,
//         bas,
//         nbas,
//         env,
//     );
// }
// #[no_mangle]
// pub unsafe extern "C" fn CINTall_1e_grids_optimizer(
//     mut opt: *mut *mut CINTOpt,
//     mut ng: *mut i32,
//     mut atm: *mut i32,
//     mut natm: i32,
//     mut bas: *mut i32,
//     mut nbas: i32,
//     mut env: *mut f64,
// ) {
//     CINTinit_2e_optimizer(opt, atm, natm, bas, nbas, env);
//     CINTOpt_set_log_maxc(*opt, atm, natm, bas, nbas, env);
//     CINTOpt_set_non0coeff(*opt, atm, natm, bas, nbas, env);
//     gen_idx(
//         *opt,
//         ::core::mem::transmute::<
//             Option::<
//                 unsafe extern "C" fn(
//                     *mut CINTEnvVars,
//                     *mut i32,
//                     *mut i32,
//                     *mut i32,
//                     i32,
//                     *mut i32,
//                     i32,
//                     *mut f64,
//                 ) -> (),
//             >,
//             Option::<unsafe extern "C" fn() -> ()>,
//         >(
//             Some(
//                 CINTinit_int1e_grids_EnvVars
//                     as unsafe extern "C" fn(
//                         *mut CINTEnvVars,
//                         *mut i32,
//                         *mut i32,
//                         *mut i32,
//                         i32,
//                         *mut i32,
//                         i32,
//                         *mut f64,
//                     ) -> (),
//             ),
//         ),
//         ::core::mem::transmute::<
//             Option::<unsafe extern "C" fn(*mut i32, *mut CINTEnvVars) -> ()>,
//             Option::<unsafe extern "C" fn() -> ()>,
//         >(
//             Some(
//                 CINTg1e_index_xyz
//                     as unsafe extern "C" fn(*mut i32, *mut CINTEnvVars) -> (),
//             ),
//         ),
//         2 as i32,
//         15 as i32,
//         ng,
//         atm,
//         natm,
//         bas,
//         nbas,
//         env,
//     );
// }
#[no_mangle]
pub unsafe extern "C" fn CINTOpt_log_max_pgto_coeff(
    log_maxc: *mut f64,
    coeff: *mut f64,
    nprim: i32,
    nctr: i32,
) {
    let mut i: i32 = 0;
    let mut ip: i32 = 0;
    let mut maxc: f64 = 0.;
    ip = 0_i32;
    while ip < nprim {
        maxc = 0 as f64;
        i = 0_i32;
        while i < nctr {
            maxc = if maxc > (*coeff.offset((i * nprim + ip) as isize)).abs() {
                maxc
            } else {
                (*coeff.offset((i * nprim + ip) as isize)).abs()
            };
            i += 1;
            i;
        }
        *log_maxc.offset(ip as isize) = (maxc).ln();
        ip += 1;
        ip;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTOpt_set_log_maxc(
    opt: *mut CINTOpt,
    _atm: *mut i32,
    _natm: i32,
    bas: *mut i32,
    nbas: i32,
    env: *mut f64,
) {
    let mut i: i32 = 0;
    let mut iprim: i32 = 0;
    let mut ictr: i32 = 0;
    let mut ci: *mut f64 = std::ptr::null_mut::<f64>();
    let mut tot_prim: u64 = 0_u64;
    i = 0_i32;
    while i < nbas {
        tot_prim = (tot_prim as libc::c_ulong)
            .wrapping_add(*bas.offset((8_i32 * i + 2_i32) as isize) as libc::c_ulong);
        i += 1;
        i;
    }
    if tot_prim == 0 as libc::c_ulong {
        return;
    }
    (*opt).log_max_coeff = malloc(
        (::core::mem::size_of::<*mut f64>() as libc::c_ulong)
            .wrapping_mul((if nbas > 1_i32 { nbas } else { 1_i32 }) as libc::c_ulong),
    ) as *mut *mut f64;
    let mut plog_maxc: *mut f64 =
        malloc((::core::mem::size_of::<f64>() as libc::c_ulong).wrapping_mul(tot_prim)) as *mut f64;
    let fresh5 = &mut *((*opt).log_max_coeff).offset(0_isize);
    *fresh5 = plog_maxc;
    i = 0_i32;
    while i < nbas {
        iprim = *bas.offset((8_i32 * i + 2_i32) as isize);
        ictr = *bas.offset((8_i32 * i + 3_i32) as isize);
        ci = env.offset(*bas.offset((8_i32 * i + 6_i32) as isize) as isize);
        let fresh6 = &mut *((*opt).log_max_coeff).offset(i as isize);
        *fresh6 = plog_maxc;
        CINTOpt_log_max_pgto_coeff(plog_maxc, ci, iprim, ictr);
        plog_maxc = plog_maxc.offset(iprim as isize);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTset_pairdata(
    pairdata: *mut PairData,
    ai: *mut f64,
    aj: *mut f64,
    ri: *mut f64,
    rj: *mut f64,
    log_maxci: *mut f64,
    log_maxcj: *mut f64,
    li_ceil: i32,
    lj_ceil: i32,
    iprim: i32,
    jprim: i32,
    rr_ij: f64,
    expcutoff: f64,
    env: *mut f64,
) -> i32 {
    let mut ip: i32 = 0;
    let mut jp: i32 = 0;
    let mut n: i32 = 0;
    let mut aij: f64 = 0.;
    let mut eij: f64 = 0.;
    let mut cceij: f64 = 0.;
    let mut wj: f64 = 0.;
    aij = *ai.offset((iprim - 1_i32) as isize) + *aj.offset((jprim - 1_i32) as isize);
    let mut log_rr_ij: f64 = 1.7f64 - 1.5f64 * (aij).ln();
    let lij: i32 = li_ceil + lj_ceil;
    if lij > 0_i32 {
        let dist_ij: f64 = (rr_ij).sqrt();
        let omega: f64 = *env.offset(8_isize);
        if omega < 0 as f64 {
            let r_guess: f64 = 8.0f64;
            let omega2: f64 = omega * omega;
            let theta: f64 = omega2 / (omega2 + aij);
            log_rr_ij += lij as f64 * (dist_ij + theta * r_guess + 1.0f64).ln();
        } else {
            log_rr_ij += lij as f64 * (dist_ij + 1.0f64).ln();
        }
    }
    let mut pdata: *mut PairData = std::ptr::null_mut::<PairData>();
    let mut empty: i32 = 1_i32;
    n = 0_i32;
    jp = 0_i32;
    while jp < jprim {
        ip = 0_i32;
        while ip < iprim {
            aij = 1_f64 / (*ai.offset(ip as isize) + *aj.offset(jp as isize));
            eij = rr_ij * *ai.offset(ip as isize) * *aj.offset(jp as isize) * aij;
            cceij =
                eij - log_rr_ij - *log_maxci.offset(ip as isize) - *log_maxcj.offset(jp as isize);
            pdata = pairdata.offset(n as isize);
            (*pdata).cceij = cceij;
            if cceij < expcutoff {
                empty = 0_i32;
                wj = *aj.offset(jp as isize) * aij;
                (*pdata).rij[0_i32 as usize] =
                    *ri.offset(0_isize) + wj * (*rj.offset(0_isize) - *ri.offset(0_isize));
                (*pdata).rij[1_i32 as usize] =
                    *ri.offset(1_isize) + wj * (*rj.offset(1_isize) - *ri.offset(1_isize));
                (*pdata).rij[2_i32 as usize] =
                    *ri.offset(2_isize) + wj * (*rj.offset(2_isize) - *ri.offset(2_isize));
                (*pdata).eij = (-eij).exp();
            } else {
                (*pdata).rij[0_usize] = 1e18f64;
                (*pdata).rij[1_usize] = 1e18f64;
                (*pdata).rij[2_usize] = 1e18f64;
                (*pdata).eij = 0 as f64;
            }
            ip += 1;
            ip;
            n += 1;
            n;
        }
        jp += 1;
        jp;
    }
    empty
}
#[no_mangle]
pub unsafe extern "C" fn CINTOpt_setij(
    opt: *mut CINTOpt,
    ng: &[i32; 8],
    atm: *mut i32,
    natm: i32,
    bas: *mut i32,
    nbas: i32,
    env: *mut f64,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut ip: i32 = 0;
    let mut jp: i32 = 0;
    let mut iprim: i32 = 0;
    let mut jprim: i32 = 0;
    let mut li: i32 = 0;
    let mut lj: i32 = 0;
    let mut ai: *mut f64 = std::ptr::null_mut::<f64>();
    let mut aj: *mut f64 = std::ptr::null_mut::<f64>();
    let mut ri: *mut f64 = std::ptr::null_mut::<f64>();
    let mut rj: *mut f64 = std::ptr::null_mut::<f64>();
    let mut expcutoff: f64 = 0.;
    if *env.offset(0_isize) == 0 as f64 {
        expcutoff = 60_f64;
    } else {
        expcutoff = if 40_f64 > *env.offset(0_isize) {
            40_f64
        } else {
            *env.offset(0_isize)
        };
    }
    if ((*opt).log_max_coeff).is_null() {
        CINTOpt_set_log_maxc(opt, atm, natm, bas, nbas, env);
    }
    let log_max_coeff: *mut *mut f64 = (*opt).log_max_coeff;
    let mut log_maxci: *mut f64 = std::ptr::null_mut::<f64>();
    let mut log_maxcj: *mut f64 = std::ptr::null_mut::<f64>();
    let mut tot_prim: u64 = 0_u64;
    i = 0_i32;
    while i < nbas {
        tot_prim = (tot_prim as libc::c_ulong)
            .wrapping_add(*bas.offset((8_i32 * i + 2_i32) as isize) as libc::c_ulong);
        i += 1;
        i;
    }
    if tot_prim == 0 as libc::c_ulong || tot_prim > 2048 as libc::c_ulong {
        return;
    }
    (*opt).pairdata = malloc(
        (::core::mem::size_of::<*mut PairData>() as libc::c_ulong).wrapping_mul(
            (if nbas * nbas > 1_i32 {
                nbas * nbas
            } else {
                1_i32
            }) as libc::c_ulong,
        ),
    ) as *mut *mut PairData;
    let mut pdata: *mut PairData = malloc(
        (::core::mem::size_of::<PairData>() as libc::c_ulong)
            .wrapping_mul(tot_prim)
            .wrapping_mul(tot_prim),
    ) as *mut PairData;
    let fresh7 = &mut *((*opt).pairdata).offset(0_isize);
    *fresh7 = pdata;
    let mut ijkl_inc: i32 = 0;
    if ng[0] + ng[1] > ng[2] + ng[3] {
        ijkl_inc = ng[0] + ng[1];
    } else {
        ijkl_inc = ng[2] + ng[3];
    }
    let mut empty: i32 = 0;
    let mut rr: f64 = 0.;
    let mut pdata0: *mut PairData = std::ptr::null_mut::<PairData>();
    i = 0_i32;
    while i < nbas {
        ri = env.offset(*atm.offset(
            (6_i32 * *bas.offset((8_i32 * i) as isize) + 1_i32) as isize,
        ) as isize);
        ai = env.offset(*bas.offset((8_i32 * i + 5_i32) as isize) as isize);
        iprim = *bas.offset((8_i32 * i + 2_i32) as isize);
        li = *bas.offset((8_i32 * i + 1_i32) as isize);
        log_maxci = *log_max_coeff.offset(i as isize);
        j = 0_i32;
        while j <= i {
            rj = env.offset(*atm.offset(
                (6_i32 * *bas.offset((8_i32 * j) as isize) + 1_i32) as isize,
            ) as isize);
            aj = env.offset(*bas.offset((8_i32 * j + 5_i32) as isize) as isize);
            jprim = *bas.offset((8_i32 * j + 2_i32) as isize);
            lj = *bas.offset((8_i32 * j + 1_i32) as isize);
            log_maxcj = *log_max_coeff.offset(j as isize);
            rr = (*ri.offset(0_isize) - *rj.offset(0_isize))
                * (*ri.offset(0_isize) - *rj.offset(0_isize))
                + (*ri.offset(1_isize) - *rj.offset(1_isize))
                    * (*ri.offset(1_isize) - *rj.offset(1_isize))
                + (*ri.offset(2_isize) - *rj.offset(2_isize))
                    * (*ri.offset(2_isize) - *rj.offset(2_isize));
            empty = CINTset_pairdata(
                pdata,
                ai,
                aj,
                ri,
                rj,
                log_maxci,
                log_maxcj,
                li + ijkl_inc,
                lj,
                iprim,
                jprim,
                rr,
                expcutoff,
                env,
            );
            if i == 0_i32 && j == 0_i32 {
                let fresh8 = &mut *((*opt).pairdata).offset(0_isize);
                *fresh8 = pdata;
                pdata = pdata.offset((iprim * jprim) as isize);
            } else if empty == 0 {
                let fresh9 = &mut *((*opt).pairdata).offset((i * nbas + j) as isize);
                *fresh9 = pdata;
                pdata = pdata.offset((iprim * jprim) as isize);
                if i != j {
                    let fresh10 = &mut *((*opt).pairdata).offset((j * nbas + i) as isize);
                    *fresh10 = pdata;
                    pdata0 = *((*opt).pairdata).offset((i * nbas + j) as isize);
                    ip = 0_i32;
                    while ip < iprim {
                        jp = 0_i32;
                        while jp < jprim {
                            memcpy(
                                pdata as *mut libc::c_void,
                                pdata0.offset((jp * iprim) as isize).offset(ip as isize)
                                    as *const libc::c_void,
                                ::core::mem::size_of::<PairData>() as libc::c_ulong,
                            );
                            jp += 1;
                            jp;
                            pdata = pdata.offset(1);
                            pdata;
                        }
                        ip += 1;
                        ip;
                    }
                }
            } else {
                let fresh11 = &mut *((*opt).pairdata).offset((i * nbas + j) as isize);
                *fresh11 =
                    0xffffffffffffffff as libc::c_ulong as *mut libc::c_void as *mut PairData;
                let fresh12 = &mut *((*opt).pairdata).offset((j * nbas + i) as isize);
                *fresh12 =
                    0xffffffffffffffff as libc::c_ulong as *mut libc::c_void as *mut PairData;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTdel_pairdata_optimizer(cintopt: *mut CINTOpt) {
    if !cintopt.is_null() && !((*cintopt).pairdata).is_null() {
        free(*((*cintopt).pairdata).offset(0_isize) as *mut libc::c_void);
        free((*cintopt).pairdata as *mut libc::c_void);
        (*cintopt).pairdata = std::ptr::null_mut::<*mut PairData>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTOpt_non0coeff_byshell(
    mut sortedidx: *mut i32,
    non0ctr: *mut i32,
    ci: *mut f64,
    iprim: i32,
    ictr: i32,
) {
    let mut ip: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut kp: i32 = 0;
    let vla = ictr as usize;
    let mut zeroidx: Vec<i32> = ::std::vec::from_elem(0, vla);
    ip = 0_i32;
    while ip < iprim {
        j = 0_i32;
        k = 0_i32;
        kp = 0_i32;
        while j < ictr {
            if *ci.offset((iprim * j + ip) as isize) != 0 as f64 {
                *sortedidx.offset(k as isize) = j;
                k += 1;
                k;
            } else {
                *zeroidx.as_mut_ptr().offset(kp as isize) = j;
                kp += 1;
                kp;
            }
            j += 1;
            j;
        }
        j = 0_i32;
        while j < kp {
            *sortedidx.offset((k + j) as isize) = *zeroidx.as_mut_ptr().offset(j as isize);
            j += 1;
            j;
        }
        *non0ctr.offset(ip as isize) = k;
        sortedidx = sortedidx.offset(ictr as isize);
        ip += 1;
        ip;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTOpt_set_non0coeff(
    opt: *mut CINTOpt,
    _atm: *mut i32,
    _natm: i32,
    bas: *mut i32,
    nbas: i32,
    env: *mut f64,
) {
    let mut i: i32 = 0;
    let mut iprim: i32 = 0;
    let mut ictr: i32 = 0;
    let mut ci: *mut f64 = std::ptr::null_mut::<f64>();
    let mut tot_prim: u64 = 0_u64;
    let mut tot_prim_ctr: u64 = 0_u64;
    i = 0_i32;
    while i < nbas {
        tot_prim = (tot_prim as libc::c_ulong)
            .wrapping_add(*bas.offset((8_i32 * i + 2_i32) as isize) as libc::c_ulong);
        tot_prim_ctr = (tot_prim_ctr as libc::c_ulong).wrapping_add(
            (*bas.offset((8_i32 * i + 2_i32) as isize)
                * *bas.offset((8_i32 * i + 3_i32) as isize)) as libc::c_ulong,
        );
        i += 1;
        i;
    }
    if tot_prim == 0 as libc::c_ulong {
        return;
    }
    (*opt).non0ctr = malloc(
        (::core::mem::size_of::<*mut i32>() as libc::c_ulong)
            .wrapping_mul((if nbas > 1_i32 { nbas } else { 1_i32 }) as libc::c_ulong),
    ) as *mut *mut i32;
    (*opt).sortedidx = malloc(
        (::core::mem::size_of::<*mut i32>() as libc::c_ulong)
            .wrapping_mul((if nbas > 1_i32 { nbas } else { 1_i32 }) as libc::c_ulong),
    ) as *mut *mut i32;
    let mut pnon0ctr: *mut i32 =
        malloc((::core::mem::size_of::<i32>() as libc::c_ulong).wrapping_mul(tot_prim)) as *mut i32;
    let mut psortedidx: *mut i32 =
        malloc((::core::mem::size_of::<i32>() as libc::c_ulong).wrapping_mul(tot_prim_ctr))
            as *mut i32;
    let fresh13 = &mut *((*opt).non0ctr).offset(0_isize);
    *fresh13 = pnon0ctr;
    let fresh14 = &mut *((*opt).sortedidx).offset(0_isize);
    *fresh14 = psortedidx;
    i = 0_i32;
    while i < nbas {
        iprim = *bas.offset((8_i32 * i + 2_i32) as isize);
        ictr = *bas.offset((8_i32 * i + 3_i32) as isize);
        ci = env.offset(*bas.offset((8_i32 * i + 6_i32) as isize) as isize);
        let fresh15 = &mut *((*opt).non0ctr).offset(i as isize);
        *fresh15 = pnon0ctr;
        let fresh16 = &mut *((*opt).sortedidx).offset(i as isize);
        *fresh16 = psortedidx;
        CINTOpt_non0coeff_byshell(psortedidx, pnon0ctr, ci, iprim, ictr);
        pnon0ctr = pnon0ctr.offset(iprim as isize);
        psortedidx = psortedidx.offset((iprim * ictr) as isize);
        i += 1;
        i;
    }
}
