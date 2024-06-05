#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

extern "C" {
    fn exp(_: f64) -> f64;
    fn pow(_: f64, _: f64) -> f64;
    fn sqrt(_: f64) -> f64;
    fn lgamma(_: f64) -> f64;
}

#[no_mangle]
pub unsafe extern "C" fn CINTsquare_dist(
    mut r1: *const f64,
    mut r2: *const f64,
) -> f64 {
    let mut r12: [f64; 3] = [0.; 3];
    r12[0 as i32
        as usize] = *r1.offset(0 as i32 as isize)
        - *r2.offset(0 as i32 as isize);
    r12[1 as i32
        as usize] = *r1.offset(1 as i32 as isize)
        - *r2.offset(1 as i32 as isize);
    r12[2 as i32
        as usize] = *r1.offset(2 as i32 as isize)
        - *r2.offset(2 as i32 as isize);
    return r12[0 as i32 as usize] * r12[0 as i32 as usize]
        + r12[1 as i32 as usize] * r12[1 as i32 as usize]
        + r12[2 as i32 as usize] * r12[2 as i32 as usize];
}
unsafe extern "C" fn _gaussian_int(
    mut n: i32,
    mut alpha: f64,
) -> f64 {
    let mut n1: f64 = (n + 1 as i32) as f64 * 0.5f64;
    return exp(lgamma(n1)) / (2.0f64 * pow(alpha, n1));
}
#[no_mangle]
pub unsafe extern "C" fn CINTgto_norm(
    mut n: i32,
    mut a: f64,
) -> f64 {
    return 1.0f64
        / sqrt(
            _gaussian_int(
                n * 2 as i32 + 2 as i32,
                2 as i32 as f64 * a,
            ),
        );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgto_norm_(
    mut n: *mut i32,
    mut a: *mut f64,
) -> f64 {
    return CINTgto_norm(*n, *a);
}
