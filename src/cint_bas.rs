#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn CINTlen_cart(l: libc::c_int) -> libc::c_int {
    return (l + 1 as libc::c_int) * (l + 2 as libc::c_int) / 2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINTlen_spinor(
    bas_id: libc::c_int,
    mut bas: *const libc::c_int,
) -> libc::c_int {
    if 0 as libc::c_int
        == *bas.offset((8 as libc::c_int * bas_id + 4 as libc::c_int) as isize)
    {
        return 4 as libc::c_int
            * *bas.offset((8 as libc::c_int * bas_id + 1 as libc::c_int) as isize)
            + 2 as libc::c_int
    } else if *bas.offset((8 as libc::c_int * bas_id + 4 as libc::c_int) as isize)
        < 0 as libc::c_int
    {
        return 2 as libc::c_int
            * *bas.offset((8 as libc::c_int * bas_id + 1 as libc::c_int) as isize)
            + 2 as libc::c_int
    } else {
        return 2 as libc::c_int
            * *bas.offset((8 as libc::c_int * bas_id + 1 as libc::c_int) as isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn CINTcgtos_cart(
    bas_id: libc::c_int,
    mut bas: *const libc::c_int,
) -> libc::c_int {
    let mut l: libc::c_int = *bas
        .offset((8 as libc::c_int * bas_id + 1 as libc::c_int) as isize);
    return (l + 1 as libc::c_int) * (l + 2 as libc::c_int) / 2 as libc::c_int
        * *bas.offset((8 as libc::c_int * bas_id + 3 as libc::c_int) as isize);
}
#[no_mangle]
pub fn CINTcgto_cart(
    bas_id: usize,
    bas: &[i32],
) -> i32 {
    let l = bas[8 * bas_id + 1];
    return (l + 1) * (l + 2) / 2 * bas[8 * bas_id + 3];
}
#[no_mangle]
pub unsafe extern "C" fn CINTcgtos_spheric(
    bas_id: libc::c_int,
    mut bas: *const libc::c_int,
) -> libc::c_int {
    return (*bas.offset((8 as libc::c_int * bas_id + 1 as libc::c_int) as isize)
        * 2 as libc::c_int + 1 as libc::c_int)
        * *bas.offset((8 as libc::c_int * bas_id + 3 as libc::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CINTcgto_spheric(
    bas_id: libc::c_int,
    mut bas: *const libc::c_int,
) -> libc::c_int {
    return (*bas.offset((8 as libc::c_int * bas_id + 1 as libc::c_int) as isize)
        * 2 as libc::c_int + 1 as libc::c_int)
        * *bas.offset((8 as libc::c_int * bas_id + 3 as libc::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CINTcgtos_spinor(
    bas_id: libc::c_int,
    mut bas: *const libc::c_int,
) -> libc::c_int {
    return CINTlen_spinor(bas_id, bas)
        * *bas.offset((8 as libc::c_int * bas_id + 3 as libc::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CINTcgto_spinor(
    bas_id: libc::c_int,
    mut bas: *const libc::c_int,
) -> libc::c_int {
    return CINTlen_spinor(bas_id, bas)
        * *bas.offset((8 as libc::c_int * bas_id + 3 as libc::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CINTtot_pgto_spheric(
    mut bas: *const libc::c_int,
    nbas: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut s: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nbas {
        s
            += (*bas.offset((8 as libc::c_int * i + 1 as libc::c_int) as isize)
                * 2 as libc::c_int + 1 as libc::c_int)
                * *bas.offset((8 as libc::c_int * i + 2 as libc::c_int) as isize);
        i += 1;
        i;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn CINTtot_pgto_spinor(
    mut bas: *const libc::c_int,
    nbas: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut s: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nbas {
        s
            += CINTlen_spinor(i, bas)
                * *bas.offset((8 as libc::c_int * i + 2 as libc::c_int) as isize);
        i += 1;
        i;
    }
    return s;
}
unsafe extern "C" fn tot_cgto_accum(
    mut f: Option::<unsafe extern "C" fn() -> libc::c_int>,
    mut bas: *const libc::c_int,
    nbas: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut s: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nbas {
        s
            += ::core::mem::transmute::<
                _,
                fn(_, _) -> libc::c_int,
            >(
                (Some(f.expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )(i, bas);
        i += 1;
        i;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn CINTtot_cgto_spheric(
    mut bas: *const libc::c_int,
    nbas: libc::c_int,
) -> libc::c_int {
    return tot_cgto_accum(
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(libc::c_int, *const libc::c_int) -> libc::c_int,
            >,
            Option::<unsafe extern "C" fn() -> libc::c_int>,
        >(
            Some(
                CINTcgto_spheric
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *const libc::c_int,
                    ) -> libc::c_int,
            ),
        ),
        bas,
        nbas,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTtot_cgto_spinor(
    mut bas: *const libc::c_int,
    nbas: libc::c_int,
) -> libc::c_int {
    return tot_cgto_accum(
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(libc::c_int, *const libc::c_int) -> libc::c_int,
            >,
            Option::<unsafe extern "C" fn() -> libc::c_int>,
        >(
            Some(
                CINTcgto_spinor
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *const libc::c_int,
                    ) -> libc::c_int,
            ),
        ),
        bas,
        nbas,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTtot_cgto_cart(
    mut bas: *const libc::c_int,
    nbas: libc::c_int,
) -> libc::c_int {
    todo!();
    //return tot_cgto_accum(
    //    ::core::mem::transmute::<
    //        Option::<
    //            unsafe extern "C" fn(libc::c_int, *const libc::c_int) -> libc::c_int,
    //        >,
    //        Option::<unsafe extern "C" fn() -> libc::c_int>,
    //    >(
    //        Some(
    //            CINTcgto_cart
    //                as unsafe extern "C" fn(
    //                    libc::c_int,
    //                    *const libc::c_int,
    //                ) -> libc::c_int,
    //        ),
    //    ),
    //    bas,
    //    nbas,
    //);
}
unsafe extern "C" fn shells_cgto_offset(
    mut f: Option::<unsafe extern "C" fn() -> libc::c_int>,
    mut ao_loc: *mut libc::c_int,
    mut bas: *const libc::c_int,
    nbas: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    *ao_loc.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < nbas {
        *ao_loc
            .offset(
                i as isize,
            ) = *ao_loc.offset((i - 1 as libc::c_int) as isize)
            + ::core::mem::transmute::<
                _,
                fn(_, _) -> libc::c_int,
            >(
                (Some(f.expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )(i - 1 as libc::c_int, bas);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTshells_cart_offset(
    mut ao_loc: *mut libc::c_int,
    mut bas: *const libc::c_int,
    nbas: libc::c_int,
) {
    todo!();
    //shells_cgto_offset(
    //    ::core::mem::transmute::<
    //        Option::<
    //            unsafe extern "C" fn(libc::c_int, *const libc::c_int) -> libc::c_int,
    //        >,
    //        Option::<unsafe extern "C" fn() -> libc::c_int>,
    //    >(
    //        Some(
    //            CINTcgto_cart
    //                as unsafe extern "C" fn(
    //                    libc::c_int,
    //                    *const libc::c_int,
    //                ) -> libc::c_int,
    //        ),
    //    ),
    //    ao_loc,
    //    bas,
    //    nbas,
    //);
}
#[no_mangle]
pub unsafe extern "C" fn CINTshells_spheric_offset(
    mut ao_loc: *mut libc::c_int,
    mut bas: *const libc::c_int,
    nbas: libc::c_int,
) {
    shells_cgto_offset(
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(libc::c_int, *const libc::c_int) -> libc::c_int,
            >,
            Option::<unsafe extern "C" fn() -> libc::c_int>,
        >(
            Some(
                CINTcgto_spheric
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *const libc::c_int,
                    ) -> libc::c_int,
            ),
        ),
        ao_loc,
        bas,
        nbas,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTshells_spinor_offset(
    mut ao_loc: *mut libc::c_int,
    mut bas: *const libc::c_int,
    nbas: libc::c_int,
) {
    shells_cgto_offset(
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(libc::c_int, *const libc::c_int) -> libc::c_int,
            >,
            Option::<unsafe extern "C" fn() -> libc::c_int>,
        >(
            Some(
                CINTcgto_spinor
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *const libc::c_int,
                    ) -> libc::c_int,
            ),
        ),
        ao_loc,
        bas,
        nbas,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTcart_comp(
    mut nx: *mut libc::c_int,
    mut ny: *mut libc::c_int,
    mut nz: *mut libc::c_int,
    lmax: libc::c_int,
) {
    let mut inc: libc::c_int = 0 as libc::c_int;
    let mut lx: libc::c_int = 0;
    let mut ly: libc::c_int = 0;
    let mut lz: libc::c_int = 0;
    lx = lmax;
    while lx >= 0 as libc::c_int {
        ly = lmax - lx;
        while ly >= 0 as libc::c_int {
            lz = lmax - lx - ly;
            *nx.offset(inc as isize) = lx;
            *ny.offset(inc as isize) = ly;
            *nz.offset(inc as isize) = lz;
            inc += 1;
            inc;
            ly -= 1;
            ly;
        }
        lx -= 1;
        lx;
    }
}
