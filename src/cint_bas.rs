#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn CINTlen_cart(l: i32) -> i32 {
    return (l + 1 as i32) * (l + 2 as i32) / 2 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn CINTlen_spinor(
    bas_id: i32,
    mut bas: *const i32,
) -> i32 {
    if 0 as i32
        == *bas.offset((8 as i32 * bas_id + 4 as i32) as isize)
    {
        return 4 as i32
            * *bas.offset((8 as i32 * bas_id + 1 as i32) as isize)
            + 2 as i32
    } else if *bas.offset((8 as i32 * bas_id + 4 as i32) as isize)
        < 0 as i32
    {
        return 2 as i32
            * *bas.offset((8 as i32 * bas_id + 1 as i32) as isize)
            + 2 as i32
    } else {
        return 2 as i32
            * *bas.offset((8 as i32 * bas_id + 1 as i32) as isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn CINTcgtos_cart(
    bas_id: i32,
    mut bas: *const i32,
) -> i32 {
    let mut l: i32 = *bas
        .offset((8 as i32 * bas_id + 1 as i32) as isize);
    return (l + 1 as i32) * (l + 2 as i32) / 2 as i32
        * *bas.offset((8 as i32 * bas_id + 3 as i32) as isize);
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
    bas_id: i32,
    mut bas: *const i32,
) -> i32 {
    return (*bas.offset((8 as i32 * bas_id + 1 as i32) as isize)
        * 2 as i32 + 1 as i32)
        * *bas.offset((8 as i32 * bas_id + 3 as i32) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CINTcgto_spheric(
    bas_id: i32,
    mut bas: *const i32,
) -> i32 {
    return (*bas.offset((8 as i32 * bas_id + 1 as i32) as isize)
        * 2 as i32 + 1 as i32)
        * *bas.offset((8 as i32 * bas_id + 3 as i32) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CINTcgtos_spinor(
    bas_id: i32,
    mut bas: *const i32,
) -> i32 {
    return CINTlen_spinor(bas_id, bas)
        * *bas.offset((8 as i32 * bas_id + 3 as i32) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CINTcgto_spinor(
    bas_id: i32,
    mut bas: *const i32,
) -> i32 {
    return CINTlen_spinor(bas_id, bas)
        * *bas.offset((8 as i32 * bas_id + 3 as i32) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CINTtot_pgto_spheric(
    mut bas: *const i32,
    nbas: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut s: i32 = 0 as i32;
    i = 0 as i32;
    while i < nbas {
        s
            += (*bas.offset((8 as i32 * i + 1 as i32) as isize)
                * 2 as i32 + 1 as i32)
                * *bas.offset((8 as i32 * i + 2 as i32) as isize);
        i += 1;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn CINTtot_pgto_spinor(
    mut bas: *const i32,
    nbas: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut s: i32 = 0 as i32;
    i = 0 as i32;
    while i < nbas {
        s
            += CINTlen_spinor(i, bas)
                * *bas.offset((8 as i32 * i + 2 as i32) as isize);
        i += 1;
    }
    return s;
}
unsafe extern "C" fn tot_cgto_accum(
    mut f: Option::<unsafe extern "C" fn() -> i32>,
    mut bas: *const i32,
    nbas: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut s: i32 = 0 as i32;
    i = 0 as i32;
    while i < nbas {
        s
            += ::core::mem::transmute::<
                _,
                fn(_, _) -> i32,
            >(
                (Some(f.expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )(i, bas);
        i += 1;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn CINTtot_cgto_spheric(
    mut bas: *const i32,
    nbas: i32,
) -> i32 {
    return tot_cgto_accum(
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(i32, *const i32) -> i32,
            >,
            Option::<unsafe extern "C" fn() -> i32>,
        >(
            Some(
                CINTcgto_spheric
                    as unsafe extern "C" fn(
                        i32,
                        *const i32,
                    ) -> i32,
            ),
        ),
        bas,
        nbas,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTtot_cgto_spinor(
    mut bas: *const i32,
    nbas: i32,
) -> i32 {
    return tot_cgto_accum(
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(i32, *const i32) -> i32,
            >,
            Option::<unsafe extern "C" fn() -> i32>,
        >(
            Some(
                CINTcgto_spinor
                    as unsafe extern "C" fn(
                        i32,
                        *const i32,
                    ) -> i32,
            ),
        ),
        bas,
        nbas,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTtot_cgto_cart(
    mut bas: *const i32,
    nbas: i32,
) -> i32 {
    todo!();
    //return tot_cgto_accum(
    //    ::core::mem::transmute::<
    //        Option::<
    //            unsafe extern "C" fn(i32, *const i32) -> i32,
    //        >,
    //        Option::<unsafe extern "C" fn() -> i32>,
    //    >(
    //        Some(
    //            CINTcgto_cart
    //                as unsafe extern "C" fn(
    //                    i32,
    //                    *const i32,
    //                ) -> i32,
    //        ),
    //    ),
    //    bas,
    //    nbas,
    //);
}
unsafe extern "C" fn shells_cgto_offset(
    mut f: Option::<unsafe extern "C" fn() -> i32>,
    mut ao_loc: *mut i32,
    mut bas: *const i32,
    nbas: i32,
) {
    let mut i: i32 = 0;
    *ao_loc.offset(0 as i32 as isize) = 0 as i32;
    i = 1 as i32;
    while i < nbas {
        *ao_loc
            .offset(
                i as isize,
            ) = *ao_loc.offset((i - 1 as i32) as isize)
            + ::core::mem::transmute::<
                _,
                fn(_, _) -> i32,
            >(
                (Some(f.expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )(i - 1 as i32, bas);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTshells_cart_offset(
    mut ao_loc: *mut i32,
    mut bas: *const i32,
    nbas: i32,
) {
    todo!();
    //shells_cgto_offset(
    //    ::core::mem::transmute::<
    //        Option::<
    //            unsafe extern "C" fn(i32, *const i32) -> i32,
    //        >,
    //        Option::<unsafe extern "C" fn() -> i32>,
    //    >(
    //        Some(
    //            CINTcgto_cart
    //                as unsafe extern "C" fn(
    //                    i32,
    //                    *const i32,
    //                ) -> i32,
    //        ),
    //    ),
    //    ao_loc,
    //    bas,
    //    nbas,
    //);
}
#[no_mangle]
pub unsafe extern "C" fn CINTshells_spheric_offset(
    mut ao_loc: *mut i32,
    mut bas: *const i32,
    nbas: i32,
) {
    shells_cgto_offset(
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(i32, *const i32) -> i32,
            >,
            Option::<unsafe extern "C" fn() -> i32>,
        >(
            Some(
                CINTcgto_spheric
                    as unsafe extern "C" fn(
                        i32,
                        *const i32,
                    ) -> i32,
            ),
        ),
        ao_loc,
        bas,
        nbas,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTshells_spinor_offset(
    mut ao_loc: *mut i32,
    mut bas: *const i32,
    nbas: i32,
) {
    shells_cgto_offset(
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(i32, *const i32) -> i32,
            >,
            Option::<unsafe extern "C" fn() -> i32>,
        >(
            Some(
                CINTcgto_spinor
                    as unsafe extern "C" fn(
                        i32,
                        *const i32,
                    ) -> i32,
            ),
        ),
        ao_loc,
        bas,
        nbas,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTcart_comp(
    mut nx: *mut i32,
    mut ny: *mut i32,
    mut nz: *mut i32,
    lmax: i32,
) {
    let mut inc: i32 = 0 as i32;
    let mut lx: i32 = 0;
    let mut ly: i32 = 0;
    let mut lz: i32 = 0;
    lx = lmax;
    while lx >= 0 as i32 {
        ly = lmax - lx;
        while ly >= 0 as i32 {
            lz = lmax - lx - ly;
            *nx.offset(inc as isize) = lx;
            *ny.offset(inc as isize) = ly;
            *nz.offset(inc as isize) = lz;
            inc += 1;
            ly -= 1;
        }
        lx -= 1;
    }
}
