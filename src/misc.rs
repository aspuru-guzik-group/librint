#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn lgamma(_: libc::c_double) -> libc::c_double;
}

#[no_mangle]
pub unsafe extern "C" fn CINTsquare_dist(
    mut r1: *const libc::c_double,
    mut r2: *const libc::c_double,
) -> libc::c_double {
    let mut r12: [libc::c_double; 3] = [0.; 3];
    r12[0 as libc::c_int
        as usize] = *r1.offset(0 as libc::c_int as isize)
        - *r2.offset(0 as libc::c_int as isize);
    r12[1 as libc::c_int
        as usize] = *r1.offset(1 as libc::c_int as isize)
        - *r2.offset(1 as libc::c_int as isize);
    r12[2 as libc::c_int
        as usize] = *r1.offset(2 as libc::c_int as isize)
        - *r2.offset(2 as libc::c_int as isize);
    return r12[0 as libc::c_int as usize] * r12[0 as libc::c_int as usize]
        + r12[1 as libc::c_int as usize] * r12[1 as libc::c_int as usize]
        + r12[2 as libc::c_int as usize] * r12[2 as libc::c_int as usize];
}
unsafe extern "C" fn _gaussian_int(
    mut n: libc::c_int,
    mut alpha: libc::c_double,
) -> libc::c_double {
    let mut n1: libc::c_double = (n + 1 as libc::c_int) as libc::c_double * 0.5f64;
    return exp(lgamma(n1)) / (2.0f64 * pow(alpha, n1));
}
#[no_mangle]
pub unsafe extern "C" fn CINTgto_norm(
    mut n: libc::c_int,
    mut a: libc::c_double,
) -> libc::c_double {
    return 1.0f64
        / sqrt(
            _gaussian_int(
                n * 2 as libc::c_int + 2 as libc::c_int,
                2 as libc::c_int as libc::c_double * a,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgto_norm_(
    mut n: *mut libc::c_int,
    mut a: *mut libc::c_double,
) -> libc::c_double {
    return CINTgto_norm(*n, *a);
}
