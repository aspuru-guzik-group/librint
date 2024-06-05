#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn expl(_: f64) -> f64;
    fn exp(_: f64) -> f64;
    fn sqrtl(_: f64) -> f64;
    fn sqrt(_: f64) -> f64;
    fn fabsl(_: f64) -> f64;
    fn fabs(_: f64) -> f64;
    fn erfl(_: f64) -> f64;
    fn erf(_: f64) -> f64;
    fn erfcl(_: f64) -> f64;
    fn erfc(_: f64) -> f64;
}
static mut TURNOVER_POINT: [f64; 40] = [
    0.0f64,
    0.0f64,
    0.866025403784f64,
    1.295010032056f64,
    1.705493613097f64,
    2.106432965305f64,
    2.501471934009f64,
    2.892473348218f64,
    3.280525047072f64,
    3.666320693281f64,
    4.05033123037f64,
    4.432891808508f64,
    4.814249856864f64,
    5.194593501454f64,
    5.574069276051f64,
    5.952793645111f64,
    6.330860773135f64,
    6.708347923415f64,
    7.08531930745f64,
    7.461828891625f64,
    7.837922483937f64,
    8.213639312398f64,
    8.589013237349f64,
    8.964073695432f64,
    9.338846443746f64,
    9.713354153046f64,
    10.08761688545f64,
    10.46165248270f64,
    10.83547688448f64,
    11.20910439128f64,
    11.58254788331f64,
    11.95581900374f64,
    12.32892831326f64,
    12.70188542111f64,
    13.07469909673f64,
    13.44737736550f64,
    13.81992759110f64,
    14.19235654675f64,
    14.56467047710f64,
    14.93687515212f64,
];
unsafe extern "C" fn fmt1_gamma_inc_like(
    mut f: *mut f64,
    mut t: f64,
    mut m: i32,
) {
    let mut i: i32 = 0;
    let mut b: f64 = m as f64 + 0.5f64;
    let mut bi: f64 = 0.;
    let mut e: f64 = 0.5f64 * exp(-t);
    let mut x: f64 = e;
    let mut s: f64 = e;
    let mut tol: f64 = 2.2204460492503131e-16f64 * 0.5f64 * e;
    bi = b + 1.0f64;
    while x > tol {
        x *= t / bi;
        s += x;
        bi += 1.0f64;
    }
    *f.offset(m as isize) = s / b;
    i = m;
    while i > 0 as i32 {
        b -= 1.0f64;
        *f.offset((i - 1 as i32) as isize) = (e + t * *f.offset(i as isize)) / b;
        i -= 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gamma_inc_like(
    mut f: *mut f64,
    mut t: f64,
    mut m: i32,
) {
    if t < TURNOVER_POINT[m as usize] {
        fmt1_gamma_inc_like(f, t, m);
    } else {
        let mut i: i32 = 0;
        let mut tt: f64 = sqrt(t);
        *f
            .offset(
                0 as i32 as isize,
            ) = 0.8862269254527580136490837416705725913987747280611935641069038949264f64
            / tt * erf(tt);
        if m > 0 as i32 {
            let mut e: f64 = exp(-t);
            let mut b: f64 = 0.5f64 / t;
            i = 1 as i32;
            while i <= m {
                *f
                    .offset(
                        i as isize,
                    ) = b
                    * ((2 as i32 * i - 1 as i32) as f64
                        * *f.offset((i - 1 as i32) as isize) - e);
                i += 1;
                i;
            }
        }
    };
}
unsafe extern "C" fn fmt1_lgamma_inc_like(
    mut f: *mut f64,
    mut t: f64,
    mut m: i32,
) {
    let mut b: f64 = m as f64 + 0.5f64;
    let mut bi: f64 = 0.0f64;
    let mut e: f64 = 0.5f64 * expl(-t);
    let mut x: f64 = e;
    let mut s: f64 = e;
    let mut tol: f64 = 2.0e-20f64 * e;
    let mut i: i32 = 0;
    bi = b + 1.0f64;
    while x > tol {
        x *= t / bi;
        s += x;
        bi += 1.0f64;
    }
    *f.offset(m as isize) = s / b;
    i = m;
    while i > 0 as i32 {
        b -= 1 as f64;
        *f.offset((i - 1 as i32) as isize) = (e + t * *f.offset(i as isize)) / b;
        i -= 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lgamma_inc_like(
    mut f: *mut f64,
    mut t: f64,
    mut m: i32,
) {
    if t < TURNOVER_POINT[m as usize] {
        fmt1_lgamma_inc_like(f, t, m);
    } else {
        let mut i: i32 = 0;
        let mut tt: f64 = sqrtl(t);
        *f
            .offset(
                0 as i32 as isize,
            ) = 
            0.8862269254527580136490837416705725913987747280611935641069038949264f64
         / tt * erfl(tt);
        if m > 0 as i32 {
            let mut e: f64 = expl(-t);
            let mut b: f64 = 0.5f64 / t;
            i = 1 as i32;
            while i <= m {
                *f
                    .offset(
                        i as isize,
                    ) = b
                    * ((2 as i32 * i - 1 as i32) as f64
                        * *f.offset((i - 1 as i32) as isize) - e);
                i += 1;
                i;
            }
        }
    };
}
#[inline]
unsafe extern "C" fn _pow(
    mut base: f64,
    mut exponent: i32,
) -> f64 {
    let mut i: i32 = 0;
    let mut result: f64 = 1 as i32 as f64;
    i = 1 as i32;
    while i <= exponent {
        if i & exponent != 0 {
            result *= base;
        }
        base *= base;
        i <<= 1 as i32;
    }
    return result;
}
#[inline]
unsafe extern "C" fn _powl(
    mut base: f64,
    mut exponent: i32,
) -> f64 {
    let mut i: i32 = 0;
    let mut result: f64 = 1.0f64;
    i = 1 as i32;
    while i <= exponent {
        if i & exponent != 0 {
            result *= base;
        }
        base *= base;
        i <<= 1 as i32;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn fmt1_erfc_like(
    mut f: *mut f64,
    mut t: f64,
    mut lower: f64,
    mut m: i32,
) {
    let mut i: i32 = 0;
    let mut lower2: f64 = lower * lower;
    let mut b: f64 = m as f64 + 0.5f64;
    let mut bi: f64 = 0.;
    let mut e: f64 = 0.5f64 * exp(-t);
    let mut e1: f64 = 0.5f64 * exp(-t * lower2) * lower;
    e1 *= _pow(lower2, m);
    let mut x: f64 = e;
    let mut x1: f64 = e1;
    let mut s: f64 = e - e1;
    let mut div: f64 = 1.0f64;
    let mut delta: f64 = s;
    let mut tol: f64 = 2.2204460492503131e-16f64 * 0.5f64 * fabs(delta);
    bi = b + 1.0f64;
    while fabs(delta) > tol {
        div *= t / bi;
        x1 *= lower2;
        delta = (x - x1) * div;
        s += delta;
        bi += 1.0f64;
    }
    let mut val: f64 = s / b;
    *f.offset(m as isize) = val;
    i = m;
    while i > 0 as i32 {
        b -= 1.0f64;
        e1 /= lower2;
        val = (e - e1 + t * val) / b;
        *f.offset((i - 1 as i32) as isize) = val;
        i -= 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn fmt_erfc_like(
    mut f: *mut f64,
    mut t: f64,
    mut lower: f64,
    mut m: i32,
) {
    if lower == 0 as i32 as f64 {
        return gamma_inc_like(f, t, m);
    }
    let mut i: i32 = 0;
    let mut lower2: f64 = lower * lower;
    if t * lower2 > 200 as i32 as f64 {
        i = 0 as i32;
        while i <= m {
            *f.offset(i as isize) = 0 as i32 as f64;
            i += 1;
            i;
        }
        return;
    }
    if t < TURNOVER_POINT[m as usize] {
        fmt1_erfc_like(f, t, lower, m);
    } else {
        let mut tt: f64 = sqrt(t);
        let mut val: f64 = 0.8862269254527580136490837416705725913987747280611935641069038949264f64
            / tt * (erfc(lower * tt) - erfc(tt));
        *f.offset(0 as i32 as isize) = val;
        if m > 0 as i32 {
            let mut e: f64 = exp(-t);
            let mut e1: f64 = exp(-t * lower2) * lower;
            let mut b: f64 = 0.5f64 / t;
            i = 0 as i32;
            while i < m {
                val = b
                    * ((2 as i32 * i + 1 as i32) as f64 * val
                        - e + e1);
                e1 *= lower2;
                *f.offset((i + 1 as i32) as isize) = val;
                i += 1;
                i;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn fmt_lerfc_like(
    mut f: *mut f64,
    mut t: f64,
    mut lower: f64,
    mut m: i32,
) {
    if lower == 0.0f64 {
        return lgamma_inc_like(f, t, m);
    }
    let mut i: i32 = 0;
    let mut lower2: f64 = lower * lower;
    if t * lower2 > 200.0f64 {
        i = 0 as i32;
        while i <= m {
            *f.offset(i as isize) = 0.0f64;
            i += 1;
            i;
        }
        return;
    }
    if t < TURNOVER_POINT[m as usize] {
        fmt1_lerfc_like(f, t, lower, m);
    } else {
        let mut tt: f64 = sqrtl(t);
        let mut val: f64 = 
            0.8862269254527580136490837416705725913987747280611935641069038949264f64
         / tt * (erfcl(lower * tt) - erfcl(tt));
        *f.offset(0 as i32 as isize) = val;
        if m > 0 as i32 {
            let mut e: f64 = expl(-t);
            let mut e1: f64 = expl(-t * lower2) * lower;
            let mut b: f64 = 0.5f64 / t;
            i = 0 as i32;
            while i < m {
                val = b
                    * ((2 as i32 * i + 1 as i32) as f64 * val - e
                        + e1);
                e1 *= lower2;
                *f.offset((i + 1 as i32) as isize) = val;
                i += 1;
                i;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn fmt1_lerfc_like(
    mut f: *mut f64,
    mut t: f64,
    mut lower: f64,
    mut m: i32,
) {
    let mut i: i32 = 0;
    let mut lower2: f64 = lower * lower;
    let mut b: f64 = m as f64 + 0.5f64;
    let mut bi: f64 = 0.0f64;
    let mut e: f64 = 0.5f64 * expl(-t);
    let mut e1: f64 = 0.5f64 * expl(-t * lower2) * lower;
    e1 *= _powl(lower2, m);
    let mut x: f64 = e;
    let mut x1: f64 = e1;
    let mut s: f64 = e - e1;
    let mut div: f64 = 1.0f64;
    let mut delta: f64 = s;
    let mut tol: f64 = 2.0e-20f64 * fabsl(delta);
    bi = b + 1.0f64;
    while fabsl(delta) > tol {
        div *= t / bi;
        x1 *= lower2;
        delta = (x - x1) * div;
        s += delta;
        bi += 1.0f64;
    }
    let mut val: f64 = s / b;
    *f.offset(m as isize) = val;
    i = m;
    while i > 0 as i32 {
        b -= 1.0f64;
        e1 /= lower2;
        val = (e - e1 + t * val) / b;
        *f.offset((i - 1 as i32) as isize) = val;
        i -= 1;
        i;
    }
}
