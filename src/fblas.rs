#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn CINTdset0(mut n: i32, mut x: *mut f64) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < n {
        *x.offset(i as isize) = 0 as i32 as f64;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTdaxpy2v(
    mut n: i32,
    mut a: f64,
    mut x: *mut f64,
    mut y: *mut f64,
    mut v: *mut f64,
) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < n {
        *v.offset(i as isize) = a * *x.offset(i as isize) + *y.offset(i as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTdmat_transpose(
    mut a_t: *mut f64,
    mut a: *mut f64,
    mut m: i32,
    mut n: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    j = 0 as i32;
    while j < n - 3 as i32 {
        i = 0 as i32;
        while i < m {
            *a_t
                .offset(
                    ((j + 0 as i32) * m + i) as isize,
                ) = *a.offset((i * n + j + 0 as i32) as isize);
            *a_t
                .offset(
                    ((j + 1 as i32) * m + i) as isize,
                ) = *a.offset((i * n + j + 1 as i32) as isize);
            *a_t
                .offset(
                    ((j + 2 as i32) * m + i) as isize,
                ) = *a.offset((i * n + j + 2 as i32) as isize);
            *a_t
                .offset(
                    ((j + 3 as i32) * m + i) as isize,
                ) = *a.offset((i * n + j + 3 as i32) as isize);
            i += 1;
        }
        j += 4 as i32;
    }
    match n - j {
        1 => {
            i = 0 as i32;
            while i < m {
                *a_t.offset((j * m + i) as isize) = *a.offset((i * n + j) as isize);
                i += 1;
            }
        }
        2 => {
            i = 0 as i32;
            while i < m {
                *a_t
                    .offset(
                        ((j + 0 as i32) * m + i) as isize,
                    ) = *a.offset((i * n + j + 0 as i32) as isize);
                *a_t
                    .offset(
                        ((j + 1 as i32) * m + i) as isize,
                    ) = *a.offset((i * n + j + 1 as i32) as isize);
                i += 1;
            }
        }
        3 => {
            i = 0 as i32;
            while i < m {
                *a_t
                    .offset(
                        ((j + 0 as i32) * m + i) as isize,
                    ) = *a.offset((i * n + j + 0 as i32) as isize);
                *a_t
                    .offset(
                        ((j + 1 as i32) * m + i) as isize,
                    ) = *a.offset((i * n + j + 1 as i32) as isize);
                *a_t
                    .offset(
                        ((j + 2 as i32) * m + i) as isize,
                    ) = *a.offset((i * n + j + 2 as i32) as isize);
                i += 1;
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn CINTdplus_transpose(
    mut a_t: *mut f64,
    mut a: *mut f64,
    mut m: i32,
    mut n: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    j = 0 as i32;
    while j < n - 3 as i32 {
        i = 0 as i32;
        while i < m {
            *a_t.offset(((j + 0 as i32) * m + i) as isize)
                += *a.offset((i * n + j + 0 as i32) as isize);
            *a_t.offset(((j + 1 as i32) * m + i) as isize)
                += *a.offset((i * n + j + 1 as i32) as isize);
            *a_t.offset(((j + 2 as i32) * m + i) as isize)
                += *a.offset((i * n + j + 2 as i32) as isize);
            *a_t.offset(((j + 3 as i32) * m + i) as isize)
                += *a.offset((i * n + j + 3 as i32) as isize);
            i += 1;
        }
        j += 4 as i32;
    }
    match n - j {
        1 => {
            i = 0 as i32;
            while i < m {
                *a_t.offset((j * m + i) as isize) += *a.offset((i * n + j) as isize);
                i += 1;
            }
        }
        2 => {
            i = 0 as i32;
            while i < m {
                *a_t.offset(((j + 0 as i32) * m + i) as isize)
                    += *a.offset((i * n + j + 0 as i32) as isize);
                *a_t.offset(((j + 1 as i32) * m + i) as isize)
                    += *a.offset((i * n + j + 1 as i32) as isize);
                i += 1;
            }
        }
        3 => {
            i = 0 as i32;
            while i < m {
                *a_t.offset(((j + 0 as i32) * m + i) as isize)
                    += *a.offset((i * n + j + 0 as i32) as isize);
                *a_t.offset(((j + 1 as i32) * m + i) as isize)
                    += *a.offset((i * n + j + 1 as i32) as isize);
                *a_t.offset(((j + 2 as i32) * m + i) as isize)
                    += *a.offset((i * n + j + 2 as i32) as isize);
                i += 1;
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn CINTdgemm_NN1(
    mut m: i32,
    mut n: i32,
    mut k: i32,
    mut a: *mut f64,
    mut b: *mut f64,
    mut c: *mut f64,
    mut ldc: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut kp: i32 = 0;
    let mut bi: f64 = 0.;
    j = 0 as i32;
    while j < n {
        i = 0 as i32;
        while i < m {
            *c.offset((i + ldc * j) as isize) = 0 as i32 as f64;
            i += 1;
        }
        kp = 0 as i32;
        while kp < k {
            bi = *b.offset((kp + k * j) as isize);
            i = 0 as i32;
            while i < m {
                *c.offset((i + ldc * j) as isize)
                    += *a.offset((i + m * kp) as isize) * bi;
                i += 1;
            }
            kp += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTdgemm_NN(
    mut m: i32,
    mut n: i32,
    mut k: i32,
    mut a: *mut f64,
    mut b: *mut f64,
    mut c: *mut f64,
) {
    CINTdgemm_NN1(m, n, k, a, b, c, m);
}
#[no_mangle]
pub unsafe extern "C" fn CINTdgemm_TN(
    mut m: i32,
    mut n: i32,
    mut k: i32,
    mut a: *mut f64,
    mut b: *mut f64,
    mut c: *mut f64,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut kp: i32 = 0;
    let mut ci: f64 = 0.;
    j = 0 as i32;
    while j < n {
        i = 0 as i32;
        while i < m {
            ci = 0 as i32 as f64;
            kp = 0 as i32;
            while kp < k {
                ci
                    += *a.offset((kp + k * i) as isize)
                        * *b.offset((kp + k * j) as isize);
                kp += 1;
            }
            *c.offset((i + m * j) as isize) = ci;
            i += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTdgemm_NT(
    mut m: i32,
    mut n: i32,
    mut k: i32,
    mut a: *mut f64,
    mut b: *mut f64,
    mut c: *mut f64,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut kp: i32 = 0;
    let mut bi: f64 = 0.;
    j = 0 as i32;
    while j < n {
        i = 0 as i32;
        while i < m {
            *c.offset((i + m * j) as isize) = 0 as i32 as f64;
            i += 1;
        }
        kp = 0 as i32;
        while kp < k {
            bi = *b.offset((j + n * kp) as isize);
            i = 0 as i32;
            while i < m {
                *c.offset((i + m * j) as isize) += *a.offset((i + m * kp) as isize) * bi;
                i += 1;
            }
            kp += 1;
        }
        j += 1;
    }
}
