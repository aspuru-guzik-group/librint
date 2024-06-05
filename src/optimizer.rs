#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::g1e::CINTg1e_index_xyz;
use crate::g1e::CINTinit_int1e_EnvVars;
use crate::g1e_grids::CINTinit_int1e_grids_EnvVars;
use crate::g2e::CINTg2e_index_xyz;
use crate::g2e::CINTinit_int2e_EnvVars;
use crate::g2c2e::CINTinit_int2c2e_EnvVars;
use crate::g3c1e::CINTinit_int3c1e_EnvVars;
use crate::g3c1e::CINTg3c1e_index_xyz;
use crate::g3c2e::CINTinit_int3c2e_EnvVars;

use crate::cint::PairData;
use crate::cint::CINTOpt;
use crate::cint::CINTEnvVars;

extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exp(_: f64) -> f64;
    fn log(_: f64) -> f64;
    fn sqrt(_: f64) -> f64;
    fn fabs(_: f64) -> f64;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;

#[no_mangle]
pub unsafe extern "C" fn CINTinit_2e_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut f64,
) {
    let mut opt0: *mut CINTOpt = malloc(
        ::core::mem::size_of::<CINTOpt>() as libc::c_ulong,
    ) as *mut CINTOpt;
    (*opt0).index_xyz_array = 0 as *mut *mut libc::c_int;
    (*opt0).non0ctr = 0 as *mut *mut libc::c_int;
    (*opt0).sortedidx = 0 as *mut *mut libc::c_int;
    (*opt0).nbas = nbas;
    (*opt0).log_max_coeff = 0 as *mut *mut f64;
    (*opt0).pairdata = 0 as *mut *mut PairData;
    *opt = opt0;
}
#[no_mangle]
pub unsafe extern "C" fn CINTinit_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut f64,
) {
    CINTinit_2e_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn CINTdel_2e_optimizer(mut opt: *mut *mut CINTOpt) {
    let mut opt0: *mut CINTOpt = *opt;
    if opt0.is_null() {
        return;
    }
    if !((*opt0).index_xyz_array).is_null() {
        free(
            *((*opt0).index_xyz_array).offset(0 as libc::c_int as isize)
                as *mut libc::c_void,
        );
        free((*opt0).index_xyz_array as *mut libc::c_void);
    }
    if !((*opt0).non0ctr).is_null() {
        free(
            *((*opt0).sortedidx).offset(0 as libc::c_int as isize) as *mut libc::c_void,
        );
        free((*opt0).sortedidx as *mut libc::c_void);
        free(*((*opt0).non0ctr).offset(0 as libc::c_int as isize) as *mut libc::c_void);
        free((*opt0).non0ctr as *mut libc::c_void);
    }
    if !((*opt0).log_max_coeff).is_null() {
        free(
            *((*opt0).log_max_coeff).offset(0 as libc::c_int as isize)
                as *mut libc::c_void,
        );
        free((*opt0).log_max_coeff as *mut libc::c_void);
    }
    CINTdel_pairdata_optimizer(opt0);
    free(opt0 as *mut libc::c_void);
    *opt = 0 as *mut CINTOpt;
}
#[no_mangle]
pub unsafe extern "C" fn CINTdel_optimizer(mut opt: *mut *mut CINTOpt) {
    CINTdel_2e_optimizer(opt);
}
#[no_mangle]
pub unsafe extern "C" fn CINTno_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut f64,
) {
    *opt = 0 as *mut CINTOpt;
}
unsafe extern "C" fn _make_fakebas(
    mut fakebas: *mut libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut f64,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut max_l: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nbas {
        max_l = if max_l
            > *bas.offset((8 as libc::c_int * i + 1 as libc::c_int) as isize)
        {
            max_l
        } else {
            *bas.offset((8 as libc::c_int * i + 1 as libc::c_int) as isize)
        };
        i += 1;
        i;
    }
    let mut fakenbas: libc::c_int = max_l + 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int * fakenbas {
        *fakebas.offset(i as isize) = 0 as libc::c_int;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i <= max_l {
        *fakebas.offset((8 as libc::c_int * i + 1 as libc::c_int) as isize) = i;
        i += 1;
        i;
    }
    return max_l;
}
unsafe extern "C" fn _allocate_index_xyz(
    mut opt: *mut CINTOpt,
    mut max_l: libc::c_int,
    mut l_allow: libc::c_int,
    mut order: libc::c_int,
) -> *mut libc::c_int {
    let mut i: libc::c_int = 0;
    let mut cumcart: libc::c_int = (l_allow + 1 as libc::c_int)
        * (l_allow + 2 as libc::c_int) * (l_allow + 3 as libc::c_int) / 6 as libc::c_int;
    let mut ll: size_t = (max_l + 1 as libc::c_int) as size_t;
    let mut cc: size_t = cumcart as size_t;
    i = 1 as libc::c_int;
    while i < order {
        ll = (ll as libc::c_ulong).wrapping_mul(16 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        cc = (cc as libc::c_ulong).wrapping_mul(cumcart as libc::c_ulong) as size_t
            as size_t;
        i += 1;
        i;
    }
    let mut buf: *mut libc::c_int = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(cc)
            .wrapping_mul(3 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut ppbuf: *mut *mut libc::c_int = malloc(
        (::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong).wrapping_mul(ll),
    ) as *mut *mut libc::c_int;
    let ref mut fresh0 = *ppbuf.offset(0 as libc::c_int as isize);
    *fresh0 = buf;
    i = 1 as libc::c_int;
    while (i as libc::c_ulong) < ll {
        let ref mut fresh1 = *ppbuf.offset(i as isize);
        *fresh1 = 0 as *mut libc::c_int;
        i += 1;
        i;
    }
    (*opt).index_xyz_array = ppbuf;
    return buf;
}
unsafe extern "C" fn gen_idx(
    mut opt: *mut CINTOpt,
    mut finit: Option::<unsafe extern "C" fn() -> ()>,
    mut findex_xyz: Option::<unsafe extern "C" fn() -> ()>,
    mut order: libc::c_int,
    mut l_allow: libc::c_int,
    mut ng: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut f64,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut fakebas: [libc::c_int; 128] = [0; 128];
    let mut max_l: libc::c_int = _make_fakebas(fakebas.as_mut_ptr(), bas, nbas, env);
    let mut fakenbas: libc::c_int = max_l + 1 as libc::c_int;
    l_allow = if max_l < l_allow { max_l } else { l_allow };
    let mut buf: *mut libc::c_int = _allocate_index_xyz(opt, max_l, l_allow, order);
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    let mut shls: [libc::c_int; 4] = [0 as libc::c_int, 0, 0, 0];
    if order == 2 as libc::c_int {
        i = 0 as libc::c_int;
        while i <= l_allow {
            j = 0 as libc::c_int;
            while j <= l_allow {
                shls[0 as libc::c_int as usize] = i;
                shls[1 as libc::c_int as usize] = j;
                ::core::mem::transmute::<
                    _,
                    fn(_, _, _, _, _, _, _, _),
                >(
                    (Some(finit.expect("non-null function pointer")))
                        .expect("non-null function pointer"),
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
                ptr = i * 16 as libc::c_int + j;
                let ref mut fresh2 = *((*opt).index_xyz_array).offset(ptr as isize);
                *fresh2 = buf;
                ::core::mem::transmute::<
                    _,
                    fn(_, _),
                >(
                    (Some(findex_xyz.expect("non-null function pointer")))
                        .expect("non-null function pointer"),
                )(buf, &mut envs);
                buf = buf.offset((envs.nf * 3 as libc::c_int) as isize);
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    } else if order == 3 as libc::c_int {
        i = 0 as libc::c_int;
        while i <= l_allow {
            j = 0 as libc::c_int;
            while j <= l_allow {
                k = 0 as libc::c_int;
                while k <= l_allow {
                    shls[0 as libc::c_int as usize] = i;
                    shls[1 as libc::c_int as usize] = j;
                    shls[2 as libc::c_int as usize] = k;
                    ::core::mem::transmute::<
                        _,
                        fn(_, _, _, _, _, _, _, _),
                    >(
                        (Some(finit.expect("non-null function pointer")))
                            .expect("non-null function pointer"),
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
                    ptr = i * 16 as libc::c_int * 16 as libc::c_int
                        + j * 16 as libc::c_int + k;
                    let ref mut fresh3 = *((*opt).index_xyz_array).offset(ptr as isize);
                    *fresh3 = buf;
                    ::core::mem::transmute::<
                        _,
                        fn(_, _),
                    >(
                        (Some(findex_xyz.expect("non-null function pointer")))
                            .expect("non-null function pointer"),
                    )(buf, &mut envs);
                    buf = buf.offset((envs.nf * 3 as libc::c_int) as isize);
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
        i = 0 as libc::c_int;
        while i <= l_allow {
            j = 0 as libc::c_int;
            while j <= l_allow {
                k = 0 as libc::c_int;
                while k <= l_allow {
                    l = 0 as libc::c_int;
                    while l <= l_allow {
                        shls[0 as libc::c_int as usize] = i;
                        shls[1 as libc::c_int as usize] = j;
                        shls[2 as libc::c_int as usize] = k;
                        shls[3 as libc::c_int as usize] = l;
                        ::core::mem::transmute::<
                            _,
                            fn(_, _, _, _, _, _, _, _),
                        >(
                            (Some(finit.expect("non-null function pointer")))
                                .expect("non-null function pointer"),
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
                        ptr = i * 16 as libc::c_int * 16 as libc::c_int
                            * 16 as libc::c_int
                            + j * 16 as libc::c_int * 16 as libc::c_int
                            + k * 16 as libc::c_int + l;
                        let ref mut fresh4 = *((*opt).index_xyz_array)
                            .offset(ptr as isize);
                        *fresh4 = buf;
                        ::core::mem::transmute::<
                            _,
                            fn(_, _),
                        >(
                            (Some(findex_xyz.expect("non-null function pointer")))
                                .expect("non-null function pointer"),
                        )(buf, &mut envs);
                        buf = buf.offset((envs.nf * 3 as libc::c_int) as isize);
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
    mut opt: *mut *mut CINTOpt,
    mut ng: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut f64,
) {
    CINTinit_2e_optimizer(opt, atm, natm, bas, nbas, env);
    CINTOpt_set_log_maxc(*opt, atm, natm, bas, nbas, env);
    CINTOpt_set_non0coeff(*opt, atm, natm, bas, nbas, env);
    gen_idx(
        *opt,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut CINTEnvVars,
                    *mut libc::c_int,
                    *mut libc::c_int,
                    *mut libc::c_int,
                    libc::c_int,
                    *mut libc::c_int,
                    libc::c_int,
                    *mut f64,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTinit_int1e_EnvVars
                    as unsafe extern "C" fn(
                        *mut CINTEnvVars,
                        *mut libc::c_int,
                        *mut libc::c_int,
                        *mut libc::c_int,
                        libc::c_int,
                        *mut libc::c_int,
                        libc::c_int,
                        *mut f64,
                    ) -> (),
            ),
        ),
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut libc::c_int, *mut CINTEnvVars) -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg1e_index_xyz
                    as unsafe extern "C" fn(*mut libc::c_int, *mut CINTEnvVars) -> (),
            ),
        ),
        2 as libc::c_int,
        15 as libc::c_int,
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
    mut opt: *mut *mut CINTOpt,
    mut ng: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut f64,
) {
    CINTinit_2e_optimizer(opt, atm, natm, bas, nbas, env);
    CINTOpt_setij(*opt, ng, atm, natm, bas, nbas, env);
    CINTOpt_set_non0coeff(*opt, atm, natm, bas, nbas, env);
    gen_idx(
        *opt,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut CINTEnvVars,
                    *mut libc::c_int,
                    *mut libc::c_int,
                    *mut libc::c_int,
                    libc::c_int,
                    *mut libc::c_int,
                    libc::c_int,
                    *mut f64,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTinit_int2e_EnvVars
                    as unsafe extern "C" fn(
                        *mut CINTEnvVars,
                        *mut libc::c_int,
                        *mut libc::c_int,
                        *mut libc::c_int,
                        libc::c_int,
                        *mut libc::c_int,
                        libc::c_int,
                        *mut f64,
                    ) -> (),
            ),
        ),
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut libc::c_int, *const CINTEnvVars) -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg2e_index_xyz
                    as unsafe extern "C" fn(*mut libc::c_int, *const CINTEnvVars) -> (),
            ),
        ),
        4 as libc::c_int,
        6 as libc::c_int,
        ng,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTall_3c2e_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut ng: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut f64,
) {
    CINTinit_2e_optimizer(opt, atm, natm, bas, nbas, env);
    CINTOpt_setij(*opt, ng, atm, natm, bas, nbas, env);
    CINTOpt_set_non0coeff(*opt, atm, natm, bas, nbas, env);
    gen_idx(
        *opt,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut CINTEnvVars,
                    *mut libc::c_int,
                    *mut libc::c_int,
                    *mut libc::c_int,
                    libc::c_int,
                    *mut libc::c_int,
                    libc::c_int,
                    *mut f64,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTinit_int3c2e_EnvVars
                    as unsafe extern "C" fn(
                        *mut CINTEnvVars,
                        *mut libc::c_int,
                        *mut libc::c_int,
                        *mut libc::c_int,
                        libc::c_int,
                        *mut libc::c_int,
                        libc::c_int,
                        *mut f64,
                    ) -> (),
            ),
        ),
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut libc::c_int, *const CINTEnvVars) -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg2e_index_xyz
                    as unsafe extern "C" fn(*mut libc::c_int, *const CINTEnvVars) -> (),
            ),
        ),
        3 as libc::c_int,
        12 as libc::c_int,
        ng,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTall_2c2e_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut ng: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut f64,
) {
    CINTinit_2e_optimizer(opt, atm, natm, bas, nbas, env);
    CINTOpt_set_log_maxc(*opt, atm, natm, bas, nbas, env);
    CINTOpt_set_non0coeff(*opt, atm, natm, bas, nbas, env);
    gen_idx(
        *opt,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut CINTEnvVars,
                    *mut libc::c_int,
                    *mut libc::c_int,
                    *mut libc::c_int,
                    libc::c_int,
                    *mut libc::c_int,
                    libc::c_int,
                    *mut f64,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTinit_int2c2e_EnvVars
                    as unsafe extern "C" fn(
                        *mut CINTEnvVars,
                        *mut libc::c_int,
                        *mut libc::c_int,
                        *mut libc::c_int,
                        libc::c_int,
                        *mut libc::c_int,
                        libc::c_int,
                        *mut f64,
                    ) -> (),
            ),
        ),
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut libc::c_int, *mut CINTEnvVars) -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg1e_index_xyz
                    as unsafe extern "C" fn(*mut libc::c_int, *mut CINTEnvVars) -> (),
            ),
        ),
        2 as libc::c_int,
        15 as libc::c_int,
        ng,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTall_3c1e_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut ng: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut f64,
) {
    CINTinit_2e_optimizer(opt, atm, natm, bas, nbas, env);
    CINTOpt_setij(*opt, ng, atm, natm, bas, nbas, env);
    CINTOpt_set_non0coeff(*opt, atm, natm, bas, nbas, env);
    gen_idx(
        *opt,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut CINTEnvVars,
                    *mut libc::c_int,
                    *mut libc::c_int,
                    *mut libc::c_int,
                    libc::c_int,
                    *mut libc::c_int,
                    libc::c_int,
                    *mut f64,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTinit_int3c1e_EnvVars
                    as unsafe extern "C" fn(
                        *mut CINTEnvVars,
                        *mut libc::c_int,
                        *mut libc::c_int,
                        *mut libc::c_int,
                        libc::c_int,
                        *mut libc::c_int,
                        libc::c_int,
                        *mut f64,
                    ) -> (),
            ),
        ),
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut libc::c_int, *const CINTEnvVars) -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg3c1e_index_xyz
                    as unsafe extern "C" fn(*mut libc::c_int, *const CINTEnvVars) -> (),
            ),
        ),
        3 as libc::c_int,
        12 as libc::c_int,
        ng,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTall_1e_grids_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut ng: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut f64,
) {
    CINTinit_2e_optimizer(opt, atm, natm, bas, nbas, env);
    CINTOpt_set_log_maxc(*opt, atm, natm, bas, nbas, env);
    CINTOpt_set_non0coeff(*opt, atm, natm, bas, nbas, env);
    gen_idx(
        *opt,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut CINTEnvVars,
                    *mut libc::c_int,
                    *mut libc::c_int,
                    *mut libc::c_int,
                    libc::c_int,
                    *mut libc::c_int,
                    libc::c_int,
                    *mut f64,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTinit_int1e_grids_EnvVars
                    as unsafe extern "C" fn(
                        *mut CINTEnvVars,
                        *mut libc::c_int,
                        *mut libc::c_int,
                        *mut libc::c_int,
                        libc::c_int,
                        *mut libc::c_int,
                        libc::c_int,
                        *mut f64,
                    ) -> (),
            ),
        ),
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut libc::c_int, *mut CINTEnvVars) -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg1e_index_xyz
                    as unsafe extern "C" fn(*mut libc::c_int, *mut CINTEnvVars) -> (),
            ),
        ),
        2 as libc::c_int,
        15 as libc::c_int,
        ng,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTOpt_log_max_pgto_coeff(
    mut log_maxc: *mut f64,
    mut coeff: *mut f64,
    mut nprim: libc::c_int,
    mut nctr: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut ip: libc::c_int = 0;
    let mut maxc: f64 = 0.;
    ip = 0 as libc::c_int;
    while ip < nprim {
        maxc = 0 as libc::c_int as f64;
        i = 0 as libc::c_int;
        while i < nctr {
            maxc = if maxc > fabs(*coeff.offset((i * nprim + ip) as isize)) {
                maxc
            } else {
                fabs(*coeff.offset((i * nprim + ip) as isize))
            };
            i += 1;
            i;
        }
        *log_maxc.offset(ip as isize) = log(maxc);
        ip += 1;
        ip;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTOpt_set_log_maxc(
    mut opt: *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut f64,
) {
    let mut i: libc::c_int = 0;
    let mut iprim: libc::c_int = 0;
    let mut ictr: libc::c_int = 0;
    let mut ci: *mut f64 = 0 as *mut f64;
    let mut tot_prim: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int;
    while i < nbas {
        tot_prim = (tot_prim as libc::c_ulong)
            .wrapping_add(
                *bas.offset((8 as libc::c_int * i + 2 as libc::c_int) as isize)
                    as libc::c_ulong,
            ) as size_t as size_t;
        i += 1;
        i;
    }
    if tot_prim == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    (*opt)
        .log_max_coeff = malloc(
        (::core::mem::size_of::<*mut f64>() as libc::c_ulong)
            .wrapping_mul(
                (if nbas > 1 as libc::c_int { nbas } else { 1 as libc::c_int })
                    as libc::c_ulong,
            ),
    ) as *mut *mut f64;
    let mut plog_maxc: *mut f64 = malloc(
        (::core::mem::size_of::<f64>() as libc::c_ulong)
            .wrapping_mul(tot_prim),
    ) as *mut f64;
    let ref mut fresh5 = *((*opt).log_max_coeff).offset(0 as libc::c_int as isize);
    *fresh5 = plog_maxc;
    i = 0 as libc::c_int;
    while i < nbas {
        iprim = *bas.offset((8 as libc::c_int * i + 2 as libc::c_int) as isize);
        ictr = *bas.offset((8 as libc::c_int * i + 3 as libc::c_int) as isize);
        ci = env
            .offset(
                *bas.offset((8 as libc::c_int * i + 6 as libc::c_int) as isize) as isize,
            );
        let ref mut fresh6 = *((*opt).log_max_coeff).offset(i as isize);
        *fresh6 = plog_maxc;
        CINTOpt_log_max_pgto_coeff(plog_maxc, ci, iprim, ictr);
        plog_maxc = plog_maxc.offset(iprim as isize);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTset_pairdata(
    mut pairdata: *mut PairData,
    mut ai: *mut f64,
    mut aj: *mut f64,
    mut ri: *mut f64,
    mut rj: *mut f64,
    mut log_maxci: *mut f64,
    mut log_maxcj: *mut f64,
    mut li_ceil: libc::c_int,
    mut lj_ceil: libc::c_int,
    mut iprim: libc::c_int,
    mut jprim: libc::c_int,
    mut rr_ij: f64,
    mut expcutoff: f64,
    mut env: *mut f64,
) -> libc::c_int {
    let mut ip: libc::c_int = 0;
    let mut jp: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut aij: f64 = 0.;
    let mut eij: f64 = 0.;
    let mut cceij: f64 = 0.;
    let mut wj: f64 = 0.;
    aij = *ai.offset((iprim - 1 as libc::c_int) as isize)
        + *aj.offset((jprim - 1 as libc::c_int) as isize);
    let mut log_rr_ij: f64 = 1.7f64 - 1.5f64 * log(aij);
    let mut lij: libc::c_int = li_ceil + lj_ceil;
    if lij > 0 as libc::c_int {
        let mut dist_ij: f64 = sqrt(rr_ij);
        let mut omega: f64 = *env.offset(8 as libc::c_int as isize);
        if omega < 0 as libc::c_int as f64 {
            let mut r_guess: f64 = 8.0f64;
            let mut omega2: f64 = omega * omega;
            let mut theta: f64 = omega2 / (omega2 + aij);
            log_rr_ij += lij as f64 * log(dist_ij + theta * r_guess + 1.0f64);
        } else {
            log_rr_ij += lij as f64 * log(dist_ij + 1.0f64);
        }
    }
    let mut pdata: *mut PairData = 0 as *mut PairData;
    let mut empty: libc::c_int = 1 as libc::c_int;
    n = 0 as libc::c_int;
    jp = 0 as libc::c_int;
    while jp < jprim {
        ip = 0 as libc::c_int;
        while ip < iprim {
            aij = 1 as libc::c_int as f64
                / (*ai.offset(ip as isize) + *aj.offset(jp as isize));
            eij = rr_ij * *ai.offset(ip as isize) * *aj.offset(jp as isize) * aij;
            cceij = eij - log_rr_ij - *log_maxci.offset(ip as isize)
                - *log_maxcj.offset(jp as isize);
            pdata = pairdata.offset(n as isize);
            (*pdata).cceij = cceij;
            if cceij < expcutoff {
                empty = 0 as libc::c_int;
                wj = *aj.offset(jp as isize) * aij;
                (*pdata)
                    .rij[0 as libc::c_int
                    as usize] = *ri.offset(0 as libc::c_int as isize)
                    + wj
                        * (*rj.offset(0 as libc::c_int as isize)
                            - *ri.offset(0 as libc::c_int as isize));
                (*pdata)
                    .rij[1 as libc::c_int
                    as usize] = *ri.offset(1 as libc::c_int as isize)
                    + wj
                        * (*rj.offset(1 as libc::c_int as isize)
                            - *ri.offset(1 as libc::c_int as isize));
                (*pdata)
                    .rij[2 as libc::c_int
                    as usize] = *ri.offset(2 as libc::c_int as isize)
                    + wj
                        * (*rj.offset(2 as libc::c_int as isize)
                            - *ri.offset(2 as libc::c_int as isize));
                (*pdata).eij = exp(-eij);
            } else {
                (*pdata).rij[0 as libc::c_int as usize] = 1e18f64;
                (*pdata).rij[1 as libc::c_int as usize] = 1e18f64;
                (*pdata).rij[2 as libc::c_int as usize] = 1e18f64;
                (*pdata).eij = 0 as libc::c_int as f64;
            }
            ip += 1;
            ip;
            n += 1;
            n;
        }
        jp += 1;
        jp;
    }
    return empty;
}
#[no_mangle]
pub unsafe extern "C" fn CINTOpt_setij(
    mut opt: *mut CINTOpt,
    mut ng: *mut libc::c_int,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut f64,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ip: libc::c_int = 0;
    let mut jp: libc::c_int = 0;
    let mut iprim: libc::c_int = 0;
    let mut jprim: libc::c_int = 0;
    let mut li: libc::c_int = 0;
    let mut lj: libc::c_int = 0;
    let mut ai: *mut f64 = 0 as *mut f64;
    let mut aj: *mut f64 = 0 as *mut f64;
    let mut ri: *mut f64 = 0 as *mut f64;
    let mut rj: *mut f64 = 0 as *mut f64;
    let mut expcutoff: f64 = 0.;
    if *env.offset(0 as libc::c_int as isize) == 0 as libc::c_int as f64 {
        expcutoff = 60 as libc::c_int as f64;
    } else {
        expcutoff = if 40 as libc::c_int as f64
            > *env.offset(0 as libc::c_int as isize)
        {
            40 as libc::c_int as f64
        } else {
            *env.offset(0 as libc::c_int as isize)
        };
    }
    if ((*opt).log_max_coeff).is_null() {
        CINTOpt_set_log_maxc(opt, atm, natm, bas, nbas, env);
    }
    let mut log_max_coeff: *mut *mut f64 = (*opt).log_max_coeff;
    let mut log_maxci: *mut f64 = 0 as *mut f64;
    let mut log_maxcj: *mut f64 = 0 as *mut f64;
    let mut tot_prim: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int;
    while i < nbas {
        tot_prim = (tot_prim as libc::c_ulong)
            .wrapping_add(
                *bas.offset((8 as libc::c_int * i + 2 as libc::c_int) as isize)
                    as libc::c_ulong,
            ) as size_t as size_t;
        i += 1;
        i;
    }
    if tot_prim == 0 as libc::c_int as libc::c_ulong
        || tot_prim > 2048 as libc::c_int as libc::c_ulong
    {
        return;
    }
    (*opt)
        .pairdata = malloc(
        (::core::mem::size_of::<*mut PairData>() as libc::c_ulong)
            .wrapping_mul(
                (if nbas * nbas > 1 as libc::c_int {
                    nbas * nbas
                } else {
                    1 as libc::c_int
                }) as libc::c_ulong,
            ),
    ) as *mut *mut PairData;
    let mut pdata: *mut PairData = malloc(
        (::core::mem::size_of::<PairData>() as libc::c_ulong)
            .wrapping_mul(tot_prim)
            .wrapping_mul(tot_prim),
    ) as *mut PairData;
    let ref mut fresh7 = *((*opt).pairdata).offset(0 as libc::c_int as isize);
    *fresh7 = pdata;
    let mut ijkl_inc: libc::c_int = 0;
    if *ng.offset(0 as libc::c_int as isize) + *ng.offset(1 as libc::c_int as isize)
        > *ng.offset(2 as libc::c_int as isize) + *ng.offset(3 as libc::c_int as isize)
    {
        ijkl_inc = *ng.offset(0 as libc::c_int as isize)
            + *ng.offset(1 as libc::c_int as isize);
    } else {
        ijkl_inc = *ng.offset(2 as libc::c_int as isize)
            + *ng.offset(3 as libc::c_int as isize);
    }
    let mut empty: libc::c_int = 0;
    let mut rr: f64 = 0.;
    let mut pdata0: *mut PairData = 0 as *mut PairData;
    i = 0 as libc::c_int;
    while i < nbas {
        ri = env
            .offset(
                *atm
                    .offset(
                        (6 as libc::c_int
                            * *bas
                                .offset((8 as libc::c_int * i + 0 as libc::c_int) as isize)
                            + 1 as libc::c_int) as isize,
                    ) as isize,
            );
        ai = env
            .offset(
                *bas.offset((8 as libc::c_int * i + 5 as libc::c_int) as isize) as isize,
            );
        iprim = *bas.offset((8 as libc::c_int * i + 2 as libc::c_int) as isize);
        li = *bas.offset((8 as libc::c_int * i + 1 as libc::c_int) as isize);
        log_maxci = *log_max_coeff.offset(i as isize);
        j = 0 as libc::c_int;
        while j <= i {
            rj = env
                .offset(
                    *atm
                        .offset(
                            (6 as libc::c_int
                                * *bas
                                    .offset((8 as libc::c_int * j + 0 as libc::c_int) as isize)
                                + 1 as libc::c_int) as isize,
                        ) as isize,
                );
            aj = env
                .offset(
                    *bas.offset((8 as libc::c_int * j + 5 as libc::c_int) as isize)
                        as isize,
                );
            jprim = *bas.offset((8 as libc::c_int * j + 2 as libc::c_int) as isize);
            lj = *bas.offset((8 as libc::c_int * j + 1 as libc::c_int) as isize);
            log_maxcj = *log_max_coeff.offset(j as isize);
            rr = (*ri.offset(0 as libc::c_int as isize)
                - *rj.offset(0 as libc::c_int as isize))
                * (*ri.offset(0 as libc::c_int as isize)
                    - *rj.offset(0 as libc::c_int as isize))
                + (*ri.offset(1 as libc::c_int as isize)
                    - *rj.offset(1 as libc::c_int as isize))
                    * (*ri.offset(1 as libc::c_int as isize)
                        - *rj.offset(1 as libc::c_int as isize))
                + (*ri.offset(2 as libc::c_int as isize)
                    - *rj.offset(2 as libc::c_int as isize))
                    * (*ri.offset(2 as libc::c_int as isize)
                        - *rj.offset(2 as libc::c_int as isize));
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
            if i == 0 as libc::c_int && j == 0 as libc::c_int {
                let ref mut fresh8 = *((*opt).pairdata)
                    .offset(0 as libc::c_int as isize);
                *fresh8 = pdata;
                pdata = pdata.offset((iprim * jprim) as isize);
            } else if empty == 0 {
                let ref mut fresh9 = *((*opt).pairdata).offset((i * nbas + j) as isize);
                *fresh9 = pdata;
                pdata = pdata.offset((iprim * jprim) as isize);
                if i != j {
                    let ref mut fresh10 = *((*opt).pairdata)
                        .offset((j * nbas + i) as isize);
                    *fresh10 = pdata;
                    pdata0 = *((*opt).pairdata).offset((i * nbas + j) as isize);
                    ip = 0 as libc::c_int;
                    while ip < iprim {
                        jp = 0 as libc::c_int;
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
                let ref mut fresh11 = *((*opt).pairdata).offset((i * nbas + j) as isize);
                *fresh11 = 0xffffffffffffffff as libc::c_ulong as *mut libc::c_void
                    as *mut PairData;
                let ref mut fresh12 = *((*opt).pairdata).offset((j * nbas + i) as isize);
                *fresh12 = 0xffffffffffffffff as libc::c_ulong as *mut libc::c_void
                    as *mut PairData;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTdel_pairdata_optimizer(mut cintopt: *mut CINTOpt) {
    if !cintopt.is_null() && !((*cintopt).pairdata).is_null() {
        free(
            *((*cintopt).pairdata).offset(0 as libc::c_int as isize) as *mut libc::c_void,
        );
        free((*cintopt).pairdata as *mut libc::c_void);
        (*cintopt).pairdata = 0 as *mut *mut PairData;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTOpt_non0coeff_byshell(
    mut sortedidx: *mut libc::c_int,
    mut non0ctr: *mut libc::c_int,
    mut ci: *mut f64,
    mut iprim: libc::c_int,
    mut ictr: libc::c_int,
) {
    let mut ip: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut kp: libc::c_int = 0;
    let vla = ictr as usize;
    let mut zeroidx: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla);
    ip = 0 as libc::c_int;
    while ip < iprim {
        j = 0 as libc::c_int;
        k = 0 as libc::c_int;
        kp = 0 as libc::c_int;
        while j < ictr {
            if *ci.offset((iprim * j + ip) as isize)
                != 0 as libc::c_int as f64
            {
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
        j = 0 as libc::c_int;
        while j < kp {
            *sortedidx
                .offset((k + j) as isize) = *zeroidx.as_mut_ptr().offset(j as isize);
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
    mut opt: *mut CINTOpt,
    mut atm: *mut libc::c_int,
    mut natm: libc::c_int,
    mut bas: *mut libc::c_int,
    mut nbas: libc::c_int,
    mut env: *mut f64,
) {
    let mut i: libc::c_int = 0;
    let mut iprim: libc::c_int = 0;
    let mut ictr: libc::c_int = 0;
    let mut ci: *mut f64 = 0 as *mut f64;
    let mut tot_prim: size_t = 0 as libc::c_int as size_t;
    let mut tot_prim_ctr: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int;
    while i < nbas {
        tot_prim = (tot_prim as libc::c_ulong)
            .wrapping_add(
                *bas.offset((8 as libc::c_int * i + 2 as libc::c_int) as isize)
                    as libc::c_ulong,
            ) as size_t as size_t;
        tot_prim_ctr = (tot_prim_ctr as libc::c_ulong)
            .wrapping_add(
                (*bas.offset((8 as libc::c_int * i + 2 as libc::c_int) as isize)
                    * *bas.offset((8 as libc::c_int * i + 3 as libc::c_int) as isize))
                    as libc::c_ulong,
            ) as size_t as size_t;
        i += 1;
        i;
    }
    if tot_prim == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    (*opt)
        .non0ctr = malloc(
        (::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
            .wrapping_mul(
                (if nbas > 1 as libc::c_int { nbas } else { 1 as libc::c_int })
                    as libc::c_ulong,
            ),
    ) as *mut *mut libc::c_int;
    (*opt)
        .sortedidx = malloc(
        (::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
            .wrapping_mul(
                (if nbas > 1 as libc::c_int { nbas } else { 1 as libc::c_int })
                    as libc::c_ulong,
            ),
    ) as *mut *mut libc::c_int;
    let mut pnon0ctr: *mut libc::c_int = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(tot_prim),
    ) as *mut libc::c_int;
    let mut psortedidx: *mut libc::c_int = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(tot_prim_ctr),
    ) as *mut libc::c_int;
    let ref mut fresh13 = *((*opt).non0ctr).offset(0 as libc::c_int as isize);
    *fresh13 = pnon0ctr;
    let ref mut fresh14 = *((*opt).sortedidx).offset(0 as libc::c_int as isize);
    *fresh14 = psortedidx;
    i = 0 as libc::c_int;
    while i < nbas {
        iprim = *bas.offset((8 as libc::c_int * i + 2 as libc::c_int) as isize);
        ictr = *bas.offset((8 as libc::c_int * i + 3 as libc::c_int) as isize);
        ci = env
            .offset(
                *bas.offset((8 as libc::c_int * i + 6 as libc::c_int) as isize) as isize,
            );
        let ref mut fresh15 = *((*opt).non0ctr).offset(i as isize);
        *fresh15 = pnon0ctr;
        let ref mut fresh16 = *((*opt).sortedidx).offset(i as isize);
        *fresh16 = psortedidx;
        CINTOpt_non0coeff_byshell(psortedidx, pnon0ctr, ci, iprim, ictr);
        pnon0ctr = pnon0ctr.offset(iprim as isize);
        psortedidx = psortedidx.offset((iprim * ictr) as isize);
        i += 1;
        i;
    }
}
