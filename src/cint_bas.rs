#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments
)]
pub unsafe fn CINTlen_cart(l: i32) -> i32 {
    (l + 1_i32) * (l + 2_i32) / 2_i32
}
pub unsafe fn CINTlen_spinor(bas_id: i32, bas: *const i32) -> i32 {
    if 0_i32 == *bas.offset((8_i32 * bas_id + 4_i32) as isize) {
        4_i32 * *bas.offset((8_i32 * bas_id + 1_i32) as isize) + 2_i32
    } else if *bas.offset((8_i32 * bas_id + 4_i32) as isize) < 0_i32 {
        2_i32 * *bas.offset((8_i32 * bas_id + 1_i32) as isize) + 2_i32
    } else {
        2_i32 * *bas.offset((8_i32 * bas_id + 1_i32) as isize)
    }
}
pub unsafe fn CINTcgtos_cart(bas_id: i32, bas: *const i32) -> i32 {
    let l: i32 = *bas.offset((8_i32 * bas_id + 1_i32) as isize);
    (l + 1_i32) * (l + 2_i32) / 2_i32 * *bas.offset((8_i32 * bas_id + 3_i32) as isize)
}

// #[no_mangle]
// pub unsafe extern "C" fn CINTcgto_cart(
//     bas_id: i32,
//     mut bas: *const i32,
// ) -> i32 {
//     let mut l: i32 = *bas
//         .offset((8 as i32 * bas_id + 1 as i32) as isize);
//     return (l + 1 as i32) * (l + 2 as i32) / 2 as i32
//         * *bas.offset((8 as i32 * bas_id + 3 as i32) as isize);
// }

pub fn CINTcgto_cart(bas_id: usize, bas: &[i32]) -> i32 {
    let l: i32 = bas[8 * bas_id + 1];
    (l + 1) * (l + 2) / 2 * bas[8 * bas_id + 3]
}

pub unsafe fn CINTcgtos_spheric(bas_id: i32, bas: *const i32) -> i32 {
    (*bas.offset((8_i32 * bas_id + 1_i32) as isize) * 2_i32 + 1_i32)
        * *bas.offset((8_i32 * bas_id + 3_i32) as isize)
}

// #[no_mangle]
// pub unsafe extern "C" fn CINTcgto_spheric(
//     bas_id: i32,
//     mut bas: *const i32,
// ) -> i32 {
//     return (*bas.offset((8 as i32 * bas_id + 1 as i32) as isize)
//         * 2 as i32 + 1 as i32)
//         * *bas.offset((8 as i32 * bas_id + 3 as i32) as isize);
// }

pub fn CINTcgto_spheric(bas_id: usize, bas: &[i32]) -> i32 {
    (bas[8 * bas_id + 1] * 2 + 1) * bas[8 * bas_id + 3]
}

pub unsafe fn CINTcgtos_spinor(bas_id: i32, bas: *const i32) -> i32 {
    CINTlen_spinor(bas_id, bas) * *bas.offset((8_i32 * bas_id + 3_i32) as isize)
}
pub unsafe fn CINTcgto_spinor(bas_id: i32, bas: *const i32) -> i32 {
    CINTlen_spinor(bas_id, bas) * *bas.offset((8_i32 * bas_id + 3_i32) as isize)
}
pub unsafe fn CINTtot_pgto_spheric(bas: *const i32, nbas: i32) -> i32 {
    let mut i: i32 = 0;
    let mut s: i32 = 0_i32;
    i = 0_i32;
    while i < nbas {
        s += (*bas.offset((8_i32 * i + 1_i32) as isize) * 2_i32 + 1_i32)
            * *bas.offset((8_i32 * i + 2_i32) as isize);
        i += 1;
        i;
    }
    s
}
pub unsafe fn CINTtot_pgto_spinor(bas: *const i32, nbas: i32) -> i32 {
    let mut i: i32 = 0;
    let mut s: i32 = 0_i32;
    i = 0_i32;
    while i < nbas {
        s += CINTlen_spinor(i, bas) * *bas.offset((8_i32 * i + 2_i32) as isize);
        i += 1;
        i;
    }
    s
}
/// Sum a per-shell count over all `nbas` shells.
///
/// Takes the counter as a generic callable rather than the erased
/// `Option<unsafe extern "C" fn() -> i32>` that c2rust produced: the C original
/// passed a real `int (*)(int, const int *)`, but the translation dropped the
/// parameter list, so every call site had to `transmute` the true signature back
/// before calling. The generic keeps the signature checked by the compiler and
/// gets monomorphised, so there is no indirect call left to inline through.
unsafe fn tot_cgto_accum(f: impl Fn(i32, *const i32) -> i32, bas: *const i32, nbas: i32) -> i32 {
    let mut i: i32 = 0;
    let mut s: i32 = 0_i32;
    i = 0_i32;
    while i < nbas {
        s += f(i, bas);
        i += 1;
        i;
    }
    s
}
// #[no_mangle]
// pub unsafe extern "C" fn CINTtot_cgto_spheric(
//     mut bas: *const i32,
//     nbas: i32,
// ) -> i32 {
//     return tot_cgto_accum(
//         ::core::mem::transmute::<
//             Option::<
//                 unsafe extern "C" fn(i32, *const i32) -> i32,
//             >,
//             Option::<unsafe extern "C" fn() -> i32>,
//         >(
//             Some(
//                 CINTcgto_spheric
//                     as unsafe extern "C" fn(
//                         i32,
//                         *const i32,
//                     ) -> i32,
//             ),
//         ),
//         bas,
//         nbas,
//     );
// }
pub unsafe fn CINTtot_cgto_spinor(bas: *const i32, nbas: i32) -> i32 {
    tot_cgto_accum(|id, b| CINTcgto_spinor(id, b), bas, nbas)
}
// #[no_mangle]
// pub unsafe extern "C" fn CINTtot_cgto_cart(
//     mut bas: *const i32,
//     nbas: i32,
// ) -> i32 {
//     return tot_cgto_accum(
//         ::core::mem::transmute::<
//             Option::<
//                 unsafe extern "C" fn(i32, *const i32) -> i32,
//             >,
//             Option::<unsafe extern "C" fn() -> i32>,
//         >(
//             Some(
//                 CINTcgto_cart
//                     as unsafe extern "C" fn(
//                         i32,
//                         *const i32,
//                     ) -> i32,
//             ),
//         ),
//         bas,
//         nbas,
//     );
// }
/// Prefix-sum the per-shell counts into `ao_loc`.
///
/// Same reasoning as [`tot_cgto_accum`]: the counter is a checked generic
/// callable instead of an erased function pointer plus a `transmute`.
unsafe fn shells_cgto_offset(
    f: impl Fn(i32, *const i32) -> i32,
    ao_loc: *mut i32,
    bas: *const i32,
    nbas: i32,
) {
    let mut i: i32 = 0;
    *ao_loc.offset(0_isize) = 0_i32;
    i = 1_i32;
    while i < nbas {
        *ao_loc.offset(i as isize) = *ao_loc.offset((i - 1_i32) as isize) + f(i - 1_i32, bas);
        i += 1;
        i;
    }
}
// #[no_mangle]
// pub unsafe extern "C" fn CINTshells_cart_offset(
//     mut ao_loc: *mut i32,
//     mut bas: *const i32,
//     nbas: i32,
// ) {
//     shells_cgto_offset(
//         ::core::mem::transmute::<
//             Option::<
//                 unsafe extern "C" fn(i32, *const i32) -> i32,
//             >,
//             Option::<unsafe extern "C" fn() -> i32>,
//         >(
//             Some(
//                 CINTcgto_cart
//                     as unsafe extern "C" fn(
//                         i32,
//                         *const i32,
//                     ) -> i32,
//             ),
//         ),
//         ao_loc,
//         bas,
//         nbas,
//     );
// }
// #[no_mangle]
// pub unsafe extern "C" fn CINTshells_spheric_offset(
//     mut ao_loc: *mut i32,
//     mut bas: *const i32,
//     nbas: i32,
// ) {
//     shells_cgto_offset(
//         ::core::mem::transmute::<
//             Option::<
//                 unsafe extern "C" fn(i32, *const i32) -> i32,
//             >,
//             Option::<unsafe extern "C" fn() -> i32>,
//         >(
//             Some(
//                 CINTcgto_spheric
//                     as unsafe extern "C" fn(
//                         i32,
//                         *const i32,
//                     ) -> i32,
//             ),
//         ),
//         ao_loc,
//         bas,
//         nbas,
//     );
// }
pub unsafe fn CINTshells_spinor_offset(ao_loc: *mut i32, bas: *const i32, nbas: i32) {
    shells_cgto_offset(|id, b| CINTcgto_spinor(id, b), ao_loc, bas, nbas);
}
pub unsafe fn CINTcart_comp(nx: *mut i32, ny: *mut i32, nz: *mut i32, lmax: i32) {
    let mut inc: i32 = 0_i32;
    let mut lx: i32 = 0;
    let mut ly: i32 = 0;
    let mut lz: i32 = 0;
    lx = lmax;
    while lx >= 0_i32 {
        ly = lmax - lx;
        while ly >= 0_i32 {
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
