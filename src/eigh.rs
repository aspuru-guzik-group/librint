#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

unsafe extern "C" fn _dlarrk(
    mut n: i32,
    mut iw: i32,
    mut gl: f64,
    mut gu: f64,
    mut diag: *mut f64,
    mut e2: *mut f64,
    mut reltol: f64,
    mut w: *mut f64,
    mut werr: *mut f64,
) -> i32 {
    let mut i: i32 = 0;
    let mut it: i32 = 0;
    let mut mid: f64 = 0.;
    let mut tmp1: f64 = 0.;
    let mut left: f64 = 0.;
    let mut right: f64 = 0.;
    let mut tnorm: f64 = 0.;
    let mut negcnt: i32 = 0;
    let mut info: i32 = 0;
    if n <= 0 as i32 {
        return 0 as i32;
    }
    tnorm = if gl.abs() > gu.abs() { gl.abs() } else { gu.abs() };
    info = -(1 as i32);
    left = gl - tnorm * 2.0f64 * 2.2204460492503131e-16f64 * n as f64;
    right = gu + tnorm * 2.0f64 * 2.2204460492503131e-16f64 * n as f64;
    it = 0 as i32;
    while it < 1000 as i32 {
        tmp1 = (right - left).abs();
        if tmp1 <= 0 as i32 as f64
            || tmp1
                < reltol
                    * (if right.abs() > left.abs() { right.abs() } else { left.abs() })
        {
            info = 0 as i32;
            break;
        } else {
            mid = (left + right) * 0.5f64;
            negcnt = 0 as i32;
            tmp1 = *diag.offset(0 as i32 as isize) - mid;
            if tmp1 <= 0.0f64 {
                negcnt += 1;
            }
            i = 1 as i32;
            while i < n {
                tmp1 = *diag.offset(i as isize)
                    - *e2.offset((i - 1 as i32) as isize) / tmp1 - mid;
                if tmp1 <= 0.0f64 {
                    negcnt += 1;
                }
                i += 1;
            }
            if negcnt >= iw {
                right = mid;
            } else {
                left = mid;
            }
            it += 1;
        }
    }
    *w = (left + right) * 0.5f64;
    *werr = (right - left).abs() * 0.5f64;
    return info;
}
unsafe extern "C" fn _dlarrc(
    mut n: i32,
    mut vl: f64,
    mut vu: f64,
    mut diag: *mut f64,
    mut e2: *mut f64,
    mut lcnt: *mut i32,
    mut rcnt: *mut i32,
) {
    let mut i: i32 = 0;
    let mut left_count: i32 = 0 as i32;
    let mut right_count: i32 = 0 as i32;
    let mut tmp: f64 = 0.;
    let mut lpivot: f64 = 0.;
    let mut rpivot: f64 = 0.;
    lpivot = *diag.offset(0 as i32 as isize) - vl;
    rpivot = *diag.offset(0 as i32 as isize) - vu;
    if lpivot <= 0.0f64 {
        left_count += 1;
    }
    if rpivot <= 0.0f64 {
        right_count += 1;
    }
    i = 0 as i32;
    while i < n - 1 as i32 {
        tmp = *e2.offset(i as isize);
        lpivot = *diag.offset((i + 1 as i32) as isize) - vl - tmp / lpivot;
        rpivot = *diag.offset((i + 1 as i32) as isize) - vu - tmp / rpivot;
        if lpivot <= 0.0f64 {
            left_count += 1;
        }
        if rpivot <= 0.0f64 {
            right_count += 1;
        }
        i += 1;
    }
    *lcnt = left_count;
    *rcnt = right_count;
}
unsafe extern "C" fn _dlasq4(
    mut i0: i32,
    mut n0: i32,
    mut n0init: i32,
    mut qvecp: *mut f64,
    mut qvec1p: *mut f64,
    mut evecp: *mut f64,
    mut evec1p: *mut f64,
    mut dmin: *mut f64,
    mut dn: *mut f64,
    mut tau: *mut f64,
) -> i32 {
    let mut a2: f64 = 0.;
    let mut b1: f64 = 0.;
    let mut b2: f64 = 0.;
    let mut gap1: f64 = 0.;
    let mut gap2: f64 = 0.;
    let mut s: f64 = 0.0f64;
    let mut i: i32 = 0;
    if n0init == n0 {
        if *dmin.offset(0 as i32 as isize)
            == *dn.offset(0 as i32 as isize)
        {
            if *dmin.offset(1 as i32 as isize)
                == *dn.offset(1 as i32 as isize)
            {
                b1 = (*qvecp.offset((n0 - 1 as i32) as isize)
                        * *evecp.offset((n0 - 2 as i32) as isize)
                ).sqrt();
                b2 = (*qvecp.offset((n0 - 2 as i32) as isize)
                        * *evecp.offset((n0 - 3 as i32) as isize)
                ).sqrt();
                a2 = *qvecp.offset((n0 - 2 as i32) as isize)
                    + *evecp.offset((n0 - 2 as i32) as isize);
                gap2 = *dmin.offset(2 as i32 as isize) - a2
                    - *dmin.offset(2 as i32 as isize) * 0.25f64;
                if gap2 > b2 {
                    gap1 = a2 - *dn.offset(0 as i32 as isize) - b2 / gap2 * b2;
                } else {
                    gap1 = a2 - *dn.offset(0 as i32 as isize) - (b1 + b2);
                }
                if gap1 > b1 {
                    s = if *dn.offset(0 as i32 as isize) - b1 / gap1 * b1
                        > *dmin.offset(0 as i32 as isize) * 0.5f64
                    {
                        *dn.offset(0 as i32 as isize) - b1 / gap1 * b1
                    } else {
                        *dmin.offset(0 as i32 as isize) * 0.5f64
                    };
                } else {
                    s = 0.0f64;
                    if *dn.offset(0 as i32 as isize) > b1 {
                        s = *dn.offset(0 as i32 as isize) - b1;
                    }
                    if a2 > b1 + b2 {
                        s = if s < a2 - (b1 + b2) { s } else { a2 - (b1 + b2) };
                    }
                    s = if s > *dmin.offset(0 as i32 as isize) * 0.333f64 {
                        s
                    } else {
                        *dmin.offset(0 as i32 as isize) * 0.333f64
                    };
                }
            } else {
                if *evecp.offset((n0 - 2 as i32) as isize)
                    > *qvecp.offset((n0 - 2 as i32) as isize)
                {
                    return 0 as i32;
                }
                b2 = *evecp.offset((n0 - 2 as i32) as isize)
                    / *qvecp.offset((n0 - 2 as i32) as isize);
                a2 = b2;
                i = n0 - 3 as i32;
                while i >= i0 {
                    b1 = b2;
                    if *evecp.offset(i as isize) > *qvecp.offset(i as isize) {
                        return 0 as i32;
                    }
                    b2 *= *evecp.offset(i as isize) / *qvecp.offset(i as isize);
                    a2 += b2;
                    if 0.563f64 < a2 || (if b1 > b2 { b1 } else { b2 }) < a2 * 0.01f64 {
                        break;
                    }
                    i -= 1;
                }
                a2 *= 1.05f64;
                if a2 < 0.563f64 {
                    s = *dn.offset(0 as i32 as isize) * (1.0f64 - a2.sqrt())
                        / (a2 + 1.0f64);
                } else {
                    s = *dmin.offset(0 as i32 as isize) * 0.25f64;
                }
            }
        } else if *dmin.offset(0 as i32 as isize)
            == *dn.offset(1 as i32 as isize)
        {
            if *evec1p.offset((n0 - 2 as i32) as isize)
                > *qvec1p.offset((n0 - 1 as i32) as isize)
                || *evecp.offset((n0 - 3 as i32) as isize)
                    > *qvecp.offset((n0 - 3 as i32) as isize)
            {
                return 0 as i32;
            }
            a2 = *evec1p.offset((n0 - 2 as i32) as isize)
                / *qvec1p.offset((n0 - 1 as i32) as isize);
            b2 = *evecp.offset((n0 - 3 as i32) as isize)
                / *qvecp.offset((n0 - 3 as i32) as isize);
            a2 += b2;
            i = n0 - 4 as i32;
            while i >= i0 {
                if b2 == 0.0f64 {
                    break;
                }
                b1 = b2;
                if *evecp.offset(i as isize) > *qvecp.offset(i as isize) {
                    return 0 as i32;
                }
                b2 *= *evecp.offset(i as isize) / *qvecp.offset(i as isize);
                a2 += b2;
                if (if b2 > b1 { b2 } else { b1 }) * 100.0f64 < a2 || 0.563f64 < a2 {
                    break;
                }
                i -= 1;
            }
            a2 *= 1.05f64;
            if a2 < 0.563f64 {
                s = *dn.offset(1 as i32 as isize) * (1.0f64 - a2.sqrt())
                    / (a2 + 1.0f64);
            } else {
                s = *dmin.offset(0 as i32 as isize) * 0.25f64;
            }
        } else if *dmin.offset(0 as i32 as isize)
            == *dn.offset(2 as i32 as isize)
        {
            if *evec1p.offset((n0 - 3 as i32) as isize)
                > *qvec1p.offset((n0 - 2 as i32) as isize)
                || *evec1p.offset((n0 - 2 as i32) as isize)
                    > *qvec1p.offset((n0 - 1 as i32) as isize)
            {
                return 0 as i32;
            }
            a2 = *evec1p.offset((n0 - 3 as i32) as isize)
                / *qvec1p.offset((n0 - 2 as i32) as isize)
                * (*evec1p.offset((n0 - 2 as i32) as isize)
                    / *qvec1p.offset((n0 - 1 as i32) as isize) + 1.0f64);
            if n0 - i0 > 3 as i32 {
                b2 = *evecp.offset((n0 - 4 as i32) as isize)
                    / *qvecp.offset((n0 - 4 as i32) as isize);
                a2 += b2;
                i = n0 - 5 as i32;
                while i >= i0 {
                    b1 = b2;
                    if *evecp.offset(i as isize) > *qvecp.offset(i as isize) {
                        return 0 as i32;
                    }
                    b2 *= *evecp.offset(i as isize) / *qvecp.offset(i as isize);
                    a2 += b2;
                    if 0.563f64 < a2 || (if b2 > b1 { b2 } else { b1 }) < a2 * 0.01f64 {
                        break;
                    }
                    i -= 1;
                }
                a2 *= 1.05f64;
            }
            if a2 < 0.563f64 {
                s = *dn.offset(2 as i32 as isize) * (1.0f64 - a2.sqrt())
                    / (a2 + 1.0f64);
            } else {
                s = *dmin.offset(0 as i32 as isize) * 0.25f64;
            }
        } else {
            s = *dmin.offset(0 as i32 as isize) * 0.25f64;
        }
    } else if n0init == n0 + 1 as i32 {
        if *dmin.offset(1 as i32 as isize)
            == *dn.offset(1 as i32 as isize)
            && *dmin.offset(2 as i32 as isize)
                == *dn.offset(2 as i32 as isize)
        {
            if *evecp.offset((n0 - 2 as i32) as isize)
                > *qvecp.offset((n0 - 2 as i32) as isize)
            {
                return 0 as i32;
            }
            s = *dmin.offset(1 as i32 as isize) * 0.333f64;
            b1 = *evecp.offset((n0 - 2 as i32) as isize)
                / *qvecp.offset((n0 - 2 as i32) as isize);
            b2 = b1;
            if b2 != 0.0f64 {
                i = n0 - 3 as i32;
                while i >= i0 {
                    a2 = b1;
                    if *evecp.offset(i as isize) > *qvecp.offset(i as isize) {
                        return 0 as i32;
                    }
                    b1 *= *evecp.offset(i as isize) / *qvecp.offset(i as isize);
                    b2 += b1;
                    if (if b1 > a2 { b1 } else { a2 }) * 100.0f64 < b2 {
                        break;
                    }
                    i -= 1;
                }
            }
            a2 = *dmin.offset(1 as i32 as isize) / (b2 * 1.05f64 + 1.0f64);
            b2 = (b2 * 1.05f64).sqrt();
            gap2 = *dmin.offset(2 as i32 as isize) * 0.5f64 - a2;
            if gap2 > 0.0f64 && gap2 > b2 * a2 {
                s = if s > a2 * (1.0f64 - a2 * 1.01f64 * (b2 / gap2) * b2) {
                    s
                } else {
                    a2 * (1.0f64 - a2 * 1.01f64 * (b2 / gap2) * b2)
                };
            } else {
                s = if s > a2 * (1.0f64 - b2 * 1.01f64) {
                    s
                } else {
                    a2 * (1.0f64 - b2 * 1.01f64)
                };
            }
        } else if *dmin.offset(1 as i32 as isize)
            == *dn.offset(1 as i32 as isize)
        {
            s = *dmin.offset(1 as i32 as isize) * 0.5f64;
        } else {
            s = *dmin.offset(1 as i32 as isize) * 0.25f64;
        }
    } else if n0init == n0 + 2 as i32 {
        if *dmin.offset(2 as i32 as isize)
            == *dn.offset(2 as i32 as isize)
            && *evecp.offset((n0 - 2 as i32) as isize) * 2.0f64
                < *qvecp.offset((n0 - 2 as i32) as isize)
        {
            if *evecp.offset((n0 - 2 as i32) as isize)
                > *qvecp.offset((n0 - 2 as i32) as isize)
            {
                return 0 as i32;
            }
            b1 = *evecp.offset((n0 - 2 as i32) as isize)
                / *qvecp.offset((n0 - 2 as i32) as isize);
            b2 = b1;
            if b2 != 0.0f64 {
                i = n0 - 2 as i32;
                while i > i0 {
                    if *evecp.offset((i - 1 as i32) as isize)
                        > *qvecp.offset((i - 1 as i32) as isize)
                    {
                        return 0 as i32;
                    }
                    b1
                        *= *evecp.offset((i - 1 as i32) as isize)
                            / *qvecp.offset((i - 1 as i32) as isize);
                    b2 += b1;
                    if b1 * 100.0f64 < b2 {
                        break;
                    }
                    i -= 1;
                }
            }
            s = *dmin.offset(2 as i32 as isize) * 0.333f64;
            a2 = *dmin.offset(2 as i32 as isize) / (b2 * 1.05f64 + 1.0f64);
            b2 = (b2 * 1.05f64).sqrt();
            gap2 = *qvecp.offset((n0 - 2 as i32) as isize)
                + *evecp.offset((n0 - 3 as i32) as isize)
                - (*qvecp.offset((n0 - 3 as i32) as isize)
                        * *evecp.offset((n0 - 3 as i32) as isize)
                ).sqrt() - a2;
            if gap2 > 0.0f64 && gap2 > b2 * a2 {
                s = if s > a2 * (1.0f64 - a2 * 1.01f64 * (b2 / gap2) * b2) {
                    s
                } else {
                    a2 * (1.0f64 - a2 * 1.01f64 * (b2 / gap2) * b2)
                };
            } else {
                s = if s > a2 * (1.0f64 - b2 * 1.01f64) {
                    s
                } else {
                    a2 * (1.0f64 - b2 * 1.01f64)
                };
            }
        } else {
            s = *dmin.offset(2 as i32 as isize) * 0.25f64;
        }
    } else if n0init > n0 + 2 as i32 {
        s = 0.0f64;
    }
    *tau = s;
    return 0 as i32;
}
unsafe extern "C" fn _dlasq5(
    mut i0: i32,
    mut n0: i32,
    mut qvecp: *mut f64,
    mut qvec1p: *mut f64,
    mut evecp: *mut f64,
    mut evec1p: *mut f64,
    mut tau: f64,
    mut tol: f64,
    mut dmin: *mut f64,
    mut dn: *mut f64,
) {
    let mut diag: f64 = *qvecp.offset(i0 as isize) - tau;
    let mut diag_min: f64 = diag;
    let mut temp: f64 = 0.;
    let mut j: i32 = 0;
    j = i0;
    while j < n0 - 3 as i32 {
        *qvec1p.offset(j as isize) = diag + *evecp.offset(j as isize);
        temp = *qvecp.offset((j + 1 as i32) as isize)
            / *qvec1p.offset(j as isize);
        diag = diag * temp - tau;
        if diag < tol {
            diag = 0.0f64;
        }
        diag_min = if diag_min < diag { diag_min } else { diag };
        *evec1p.offset(j as isize) = *evecp.offset(j as isize) * temp;
        j += 1;
    }
    *dn.offset(2 as i32 as isize) = diag;
    j = n0 - 3 as i32;
    *qvec1p.offset(j as isize) = diag + *evecp.offset(j as isize);
    temp = *qvecp.offset((j + 1 as i32) as isize) / *qvec1p.offset(j as isize);
    *evec1p.offset(j as isize) = *evecp.offset(j as isize) * temp;
    diag = diag * temp - tau;
    *dn.offset(1 as i32 as isize) = diag;
    j = n0 - 2 as i32;
    *qvec1p.offset(j as isize) = diag + *evecp.offset(j as isize);
    temp = *qvecp.offset((j + 1 as i32) as isize) / *qvec1p.offset(j as isize);
    *evec1p.offset(j as isize) = *evecp.offset(j as isize) * temp;
    diag = diag * temp - tau;
    *dn.offset(0 as i32 as isize) = diag;
    *qvec1p.offset((n0 - 1 as i32) as isize) = diag;
    *dmin.offset(2 as i32 as isize) = diag_min;
    *dmin
        .offset(
            1 as i32 as isize,
        ) = if *dmin.offset(2 as i32 as isize)
        < *dn.offset(1 as i32 as isize)
    {
        *dmin.offset(2 as i32 as isize)
    } else {
        *dn.offset(1 as i32 as isize)
    };
    *dmin
        .offset(
            0 as i32 as isize,
        ) = if *dmin.offset(1 as i32 as isize)
        < *dn.offset(0 as i32 as isize)
    {
        *dmin.offset(1 as i32 as isize)
    } else {
        *dn.offset(0 as i32 as isize)
    };
}
unsafe extern "C" fn _dlasq2(
    mut n: i32,
    mut work: *mut f64,
    mut diag: *mut f64,
    mut diag_off: *mut f64,
) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut itry: i32 = 0;
    let mut iwhilb: i32 = 0;
    let mut iter: i32 = 0;
    let mut i0: i32 = 0;
    let mut n0: i32 = 0;
    let mut n1: i32 = 0;
    let mut n2: i32 = 0;
    let mut n0init: i32 = 0;
    let mut nbig: i32 = 0;
    let mut emax: f64 = 0.;
    let mut qmin: f64 = 0.;
    let mut temp: f64 = 0.;
    let mut diag_sum: f64 = 0.;
    let mut tol: f64 = 0.;
    let mut tol2: f64 = 0.;
    let mut s: f64 = 0.;
    let mut t: f64 = 0.;
    let mut dmin: [f64; 3] = [0.0f64, 0.0f64, 0.0f64];
    let mut dn: [f64; 3] = [0.0f64, 0.0f64, 0.0f64];
    let mut sigma: f64 = 0.;
    let mut tau: f64 = 0.;
    let mut qvec: *mut f64 = 0 as *mut f64;
    let mut qvec1: *mut f64 = 0 as *mut f64;
    let mut evec: *mut f64 = 0 as *mut f64;
    let mut evec1: *mut f64 = 0 as *mut f64;
    let mut qvecp: *mut f64 = 0 as *mut f64;
    let mut qvec1p: *mut f64 = 0 as *mut f64;
    let mut evecp: *mut f64 = 0 as *mut f64;
    let mut evec1p: *mut f64 = 0 as *mut f64;
    let mut swap: *mut f64 = 0 as *mut f64;
    qvec = work;
    qvec1 = work.offset((n * 1 as i32) as isize);
    evec = work.offset((n * 2 as i32) as isize);
    evec1 = work.offset((n * 3 as i32) as isize);
    i = 0 as i32;
    j = n - 1 as i32;
    while i < n - 1 as i32 {
        temp = (*diag.offset(i as isize)).abs();
        *qvec1.offset(i as isize) = 0.0f64;
        *evec1.offset(i as isize) = 0.0f64;
        *evec
            .offset(
                (j - 1 as i32) as isize,
            ) = *diag_off.offset(i as isize) * *diag_off.offset(i as isize) * temp;
        *qvec.offset(j as isize) = temp;
        i += 1;
        j -= 1;
    }
    *qvec
        .offset(
            0 as i32 as isize,
        ) = (*diag.offset((n - 1 as i32) as isize)).abs();
    *qvec1.offset((n - 1 as i32) as isize) = 0.0f64;
    *evec.offset((n - 1 as i32) as isize) = 0.0f64;
    *evec1.offset((n - 1 as i32) as isize) = 0.0f64;
    if *qvec.offset(0 as i32 as isize)
        < *qvec.offset((n - 1 as i32) as isize) * 1.5f64
    {
        i = 0 as i32;
        j = n - 1 as i32;
        while i < n / 2 as i32 {
            temp = *qvec.offset(i as isize);
            *qvec.offset(i as isize) = *qvec.offset(j as isize);
            *qvec.offset(j as isize) = temp;
            temp = *evec.offset(i as isize);
            *evec.offset(i as isize) = *evec.offset((j - 1 as i32) as isize);
            *evec.offset((j - 1 as i32) as isize) = temp;
            i += 1;
            j -= 1;
        }
    }
    diag_sum = *qvec.offset(0 as i32 as isize);
    i = 0 as i32;
    while i < n - 1 as i32 {
        temp = diag_sum + *evec.offset(i as isize);
        *qvec1.offset(i as isize) = temp;
        *evec1
            .offset(
                i as isize,
            ) = *qvec.offset((i + 1 as i32) as isize)
            * (*evec.offset(i as isize) / temp);
        diag_sum = *qvec.offset((i + 1 as i32) as isize) * (diag_sum / temp);
        i += 1;
    }
    *qvec1.offset((n - 1 as i32) as isize) = diag_sum;
    diag_sum = *qvec1.offset(0 as i32 as isize);
    i = 0 as i32;
    while i < n - 1 as i32 {
        temp = diag_sum + *evec1.offset(i as isize);
        *qvec.offset(i as isize) = temp;
        *evec
            .offset(
                i as isize,
            ) = *qvec1.offset((i + 1 as i32) as isize)
            * (*evec1.offset(i as isize) / temp);
        diag_sum = *qvec1.offset((i + 1 as i32) as isize) * (diag_sum / temp);
        i += 1;
    }
    *qvec.offset((n - 1 as i32) as isize) = diag_sum;
    n0 = n;
    tau = 0.0f64;
    itry = 0 as i32;
    while n0 > 0 as i32 {
        if itry >= n {
            return 3 as i32;
        }
        sigma = -*evec.offset((n0 - 1 as i32) as isize);
        if sigma < 0.0f64 {
            return 1 as i32;
        }
        emax = 0.0f64;
        qmin = *qvec.offset((n0 - 1 as i32) as isize);
        i = n0 - 1 as i32;
        while i > 0 as i32 {
            if *evec.offset((i - 1 as i32) as isize) <= 0.0f64 {
                break;
            }
            if qmin >= emax * 4.0f64 {
                qmin = if qmin < *qvec.offset(i as isize) {
                    qmin
                } else {
                    *qvec.offset(i as isize)
                };
                emax = if emax > *evec.offset((i - 1 as i32) as isize) {
                    emax
                } else {
                    *evec.offset((i - 1 as i32) as isize)
                };
            }
            i -= 1;
        }
        i0 = i;
        qvecp = qvec;
        qvec1p = qvec1;
        evecp = evec;
        evec1p = evec1;
        if qmin < 0 as i32 as f64
            || emax < 0 as i32 as f64
        {
            eprintln!("dlasq2: qmin < 0 or emax < 0");
            // fprintf(
            //     stderr,
            //     b"dlasq2: qmin < 0 or emax < 0\0" as *const u8 as *const libc::c_char,
            // );
            return 1 as i32;
        }
        dmin[0 as i32
            as usize] = -if 0.0f64 > qmin - 2.0f64 * (qmin * emax).sqrt() {
            0.0f64
        } else {
            qmin - 2.0f64 * (qmin * emax).sqrt()
        };
        nbig = (n0 - i0) * 10 as i32;
        iwhilb = 0 as i32;
        while iwhilb < nbig {
            n0init = n0;
            tol = 2.2204460492503131e-16f64 * 100.0f64;
            tol2 = tol * tol;
            while n0 > i0 {
                n1 = n0 - 1 as i32;
                n2 = n0 - 2 as i32;
                if n1 == i0
                    || *evecp.offset(n2 as isize)
                        < tol2 * (sigma + *qvecp.offset(n1 as isize))
                    || *evec1p.offset(n2 as isize) < tol2 * *qvecp.offset(n2 as isize)
                {
                    *qvec.offset(n1 as isize) = *qvecp.offset(n1 as isize) + sigma;
                    n0 -= 1;
                } else {
                    if !(n2 == i0
                        || *evecp.offset((n1 - 2 as i32) as isize) < tol2 * sigma
                        || *evec1p.offset((n1 - 2 as i32) as isize)
                            < tol2 * *qvecp.offset((n1 - 2 as i32) as isize))
                    {
                        break;
                    }
                    if *qvecp.offset(n1 as isize) > *qvecp.offset(n2 as isize) {
                        s = *qvecp.offset(n1 as isize);
                        *qvecp.offset(n1 as isize) = *qvecp.offset(n2 as isize);
                        *qvecp.offset(n2 as isize) = s;
                    }
                    t = (*qvecp.offset(n2 as isize) - *qvecp.offset(n1 as isize)
                        + *evecp.offset(n2 as isize)) * 0.5f64;
                    if *evecp.offset(n2 as isize) > *qvecp.offset(n1 as isize) * tol2
                        && t != 0.0f64
                    {
                        s = *qvecp.offset(n1 as isize)
                            * (*evecp.offset(n2 as isize) / t);
                        s = *qvecp.offset(n1 as isize)
                            * (*evecp.offset(n2 as isize) / (t + (t * (t + s)).sqrt()));
                        t = *qvecp.offset(n2 as isize)
                            + (s + *evecp.offset(n2 as isize));
                        *qvecp.offset(n1 as isize) *= *qvecp.offset(n2 as isize) / t;
                        *qvecp.offset(n2 as isize) = t;
                    }
                    *qvec.offset(n2 as isize) = *qvecp.offset(n2 as isize) + sigma;
                    *qvec.offset(n1 as isize) = *qvecp.offset(n1 as isize) + sigma;
                    n0 += -(2 as i32);
                }
            }
            if n0 <= i0 {
                break;
            }
            if dmin[0 as i32 as usize] <= 0.0f64 {
                tau = -dmin[0 as i32 as usize];
            } else {
                _dlasq4(
                    i0,
                    n0,
                    n0init,
                    qvecp,
                    qvec1p,
                    evecp,
                    evec1p,
                    dmin.as_mut_ptr(),
                    dn.as_mut_ptr(),
                    &mut tau,
                );
            }
            tol = 2.2204460492503131e-16f64 * sigma;
            iter = 0 as i32;
            while iter < 3 as i32 {
                _dlasq5(
                    i0,
                    n0,
                    qvecp,
                    qvec1p,
                    evecp,
                    evec1p,
                    tau,
                    tol,
                    dmin.as_mut_ptr(),
                    dn.as_mut_ptr(),
                );
                if dmin[0 as i32 as usize] >= 0.0f64 {
                    break;
                }
                if dmin[1 as i32 as usize] > 0.0f64 {
                    tau += dmin[0 as i32 as usize];
                } else {
                    tau *= 0.25f64;
                }
                iter += 1;
            }
            if dmin[0 as i32 as usize] < 0 as i32 as f64 {
                tau = 0.0f64;
                _dlasq5(
                    i0,
                    n0,
                    qvecp,
                    qvec1p,
                    evecp,
                    evec1p,
                    tau,
                    tol,
                    dmin.as_mut_ptr(),
                    dn.as_mut_ptr(),
                );
            }
            sigma += tau;
            swap = qvecp;
            qvecp = qvec1p;
            qvec1p = swap;
            swap = evecp;
            evecp = evec1p;
            evec1p = swap;
            iwhilb += 1;
        }
        if iwhilb == nbig {
            eprintln!("dlasq2: Maximum number of iterations exceeded");
            // fprintf(
            //     stderr,
            //     b"dlasq2: Maximum number of iterations exceeded\0" as *const u8
            //         as *const libc::c_char,
            // );
            return 2 as i32;
        }
        itry += 1;
    }
    return 0 as i32;
}
unsafe extern "C" fn _compute_eigenvalues(
    mut n: i32,
    mut diag: *mut f64,
    mut diag_off1: *mut f64,
    mut w: *mut f64,
    mut werr: *mut f64,
    mut wgap: *mut f64,
    mut work: *mut f64,
) -> i32 {
    let mut gl: f64 = 0.;
    let mut gu: f64 = 0.;
    let mut eabs: f64 = 0.;
    let mut eold: f64 = 0.;
    let mut tmp: f64 = 0.;
    let mut tmp1: f64 = 0.;
    let mut dmax: f64 = 0.;
    let mut eps: f64 = 0.;
    let mut rtol: f64 = 0.;
    let mut rtl: f64 = 0.;
    let mut sigma: f64 = 0.;
    let mut tau: f64 = 0.;
    let mut sgndef: f64 = 0.;
    let mut spectral_diameter: f64 = 0.;
    let mut isleft: f64 = 0.;
    let mut isrght: f64 = 0.;
    let mut dpivot: f64 = 0.;
    let mut idum: i32 = 0;
    let mut ip: i32 = 0;
    let mut i: i32 = 0;
    let mut lcnt: i32 = 0;
    let mut rcnt: i32 = 0;
    let mut norep: i32 = 0;
    let mut iinfo: i32 = 0;
    if n <= 0 as i32 {
        return 0 as i32;
    }
    if n == 1 as i32 {
        *w.offset(0 as i32 as isize) = *diag.offset(0 as i32 as isize);
        *werr.offset(0 as i32 as isize) = 0.0f64;
        *wgap.offset(0 as i32 as isize) = 0.0f64;
        *diag_off1.offset(0 as i32 as isize) = 0.0f64;
        return 0 as i32;
    }
    eps = 2 as i32 as f64 * 2.2204460492503131e-16f64;
    rtl = 2.1e-8f64;
    rtol = 16.0f64 * 2.2204460492503131e-16f64;
    gl = *diag.offset(0 as i32 as isize);
    gu = *diag.offset(0 as i32 as isize);
    eold = 0.0f64;
    *diag_off1.offset((n - 1 as i32) as isize) = 0.0f64;
    i = 0 as i32;
    while i < n {
        *werr.offset(i as isize) = 0.0f64;
        *wgap.offset(i as isize) = 0.0f64;
        eabs = (*diag_off1.offset(i as isize)).abs();
        tmp1 = eabs + eold;
        gl = if gl < *diag.offset(i as isize) - tmp1 {
            gl
        } else {
            *diag.offset(i as isize) - tmp1
        };
        gu = if gu > *diag.offset(i as isize) + tmp1 {
            gu
        } else {
            *diag.offset(i as isize) + tmp1
        };
        eold = eabs;
        i += 1;
    }
    spectral_diameter = gu - gl;
    *diag_off1.offset((n - 1 as i32) as isize) = 0.0f64;
    if n == 1 as i32 {
        *w.offset(0 as i32 as isize) = *diag.offset(0 as i32 as isize);
        *werr.offset(0 as i32 as isize) = 0.0f64;
        *wgap.offset(0 as i32 as isize) = 0.0f64;
        return 0 as i32;
    }
    let mut e2: *mut f64 = work;
    work = work.offset(n as isize);
    i = 0 as i32;
    while i < n - 1 as i32 {
        *e2
            .offset(
                i as isize,
            ) = *diag_off1.offset(i as isize) * *diag_off1.offset(i as isize);
        i += 1;
    }
    iinfo = _dlarrk(n, 1 as i32, gl, gu, diag, e2, rtl, &mut tmp, &mut tmp1);
    if iinfo != 0 as i32 {
        return -(1 as i32);
    }
    isleft = if gl > tmp - tmp1 - eps * 100.0f64 * (tmp - tmp1).abs() {
        gl
    } else {
        tmp - tmp1 - eps * 100.0f64 * (tmp - tmp1).abs()
    };
    iinfo = _dlarrk(n, n, gl, gu, diag, e2, rtl, &mut tmp, &mut tmp1);
    if iinfo != 0 as i32 {
        return -(1 as i32);
    }
    isrght = if gu < tmp + tmp1 + eps * 100.0f64 * (tmp + tmp1).abs() {
        gu
    } else {
        tmp + tmp1 + eps * 100.0f64 * (tmp + tmp1).abs()
    };
    spectral_diameter = isrght - isleft;
    if n > 1 as i32 {
        _dlarrc(
            n,
            isleft + spectral_diameter * 0.25f64,
            isrght - spectral_diameter * 0.25f64,
            diag,
            e2,
            &mut lcnt,
            &mut rcnt,
        );
    }
    if n == 1 as i32 {
        sigma = gl;
        sgndef = 1.0f64;
    } else if lcnt - 1 as i32 >= n - rcnt {
        sigma = if isleft > gl { isleft } else { gl };
        sgndef = 1.0f64;
    } else {
        sigma = if isrght < gu { isrght } else { gu };
        sgndef = -1.0f64;
    }
    tau = (if spectral_diameter * n as f64 > 2.0f64 * sigma.abs() {
        spectral_diameter * n as f64
    } else {
        2.0f64 * sigma.abs()
    }) * eps;
    idum = 0 as i32;
    while idum < 6 as i32 {
        dpivot = *diag.offset(0 as i32 as isize) - sigma;
        *work.offset(0 as i32 as isize) = dpivot;
        dmax = (*work.offset(0 as i32 as isize)).abs();
        i = 0 as i32;
        while i < n - 1 as i32 {
            *work
                .offset(
                    (n * 2 as i32 + i) as isize,
                ) = 1.0f64 / *work.offset(i as isize);
            tmp = *diag_off1.offset(i as isize)
                * *work.offset((n * 2 as i32 + i) as isize);
            *work.offset((n + i) as isize) = tmp;
            dpivot = *diag.offset((i + 1 as i32) as isize) - sigma
                - tmp * *diag_off1.offset(i as isize);
            *work.offset((i + 1 as i32) as isize) = dpivot;
            dmax = if dmax > (dpivot).abs() { dmax } else { (dpivot).abs() };
            i += 1;
        }
        norep = (dmax > spectral_diameter * 64.0f64) as i32;
        if norep == 0 {
            i = 0 as i32;
            while i < n {
                tmp = sgndef * *work.offset(i as isize);
                if tmp < 0.0f64 {
                    norep = 1 as i32;
                    break;
                } else {
                    i += 1;
                }
            }
        }
        if !(norep != 0) {
            break;
        }
        if idum == 6 as i32 - 1 as i32 {
            if sgndef == 1.0f64 {
                sigma = gl - spectral_diameter * 2.0f64 * eps * n as f64;
            } else {
                sigma = gu + spectral_diameter * 2.0f64 * eps * n as f64;
            }
        } else if idum == 6 as i32 {
            return -(2 as i32)
        } else {
            sigma -= sgndef * tau;
            tau *= 2.0f64;
        }
        idum += 1;
    }
    *diag_off1.offset((n - 1 as i32) as isize) = sigma;
    ip = 0 as i32;
    while ip < n {
        *diag.offset(ip as isize) = *work.offset(ip as isize);
        ip += 1;
    }
    ip = 0 as i32;
    while ip < n - 1 as i32 {
        *diag_off1.offset(ip as isize) = *work.offset((n + ip) as isize);
        ip += 1;
    }
    iinfo = _dlasq2(n, work, diag, diag_off1);
    if iinfo != 0 as i32 {
        return -(5 as i32)
    } else {
        i = 0 as i32;
        while i < n {
            if *work.offset(i as isize) < 0.0f64 {
                eprintln!("dlarre: negative eigenvalues");
                // fprintf(
                //     stderr,
                //     b"dlarre: negative eigenvalues\n\0" as *const u8
                //         as *const libc::c_char,
                // );
                return -(6 as i32);
            }
            i += 1;
        }
    }
    if sgndef > 0.0f64 {
        i = 0 as i32;
        while i < n {
            *w.offset(i as isize) = *work.offset((n - 1 as i32 - i) as isize);
            i += 1;
        }
    } else {
        i = 0 as i32;
        while i < n {
            *w.offset(i as isize) = -*work.offset(i as isize);
            i += 1;
        }
    }
    i = 0 as i32;
    while i < n {
        *werr.offset(i as isize) = rtol * (*w.offset(i as isize)).abs();
        i += 1;
    }
    i = 0 as i32;
    while i < n - 1 as i32 {
        *wgap
            .offset(
                i as isize,
            ) = if 0.0f64
            > *w.offset((i + 1 as i32) as isize)
                - *werr.offset((i + 1 as i32) as isize)
                - (*w.offset(i as isize) + *werr.offset(i as isize))
        {
            0.0f64
        } else {
            *w.offset((i + 1 as i32) as isize)
                - *werr.offset((i + 1 as i32) as isize)
                - (*w.offset(i as isize) + *werr.offset(i as isize))
        };
        i += 1;
    }
    *wgap
        .offset(
            -(1 as i32) as isize,
        ) = if 0.0f64
        > *w.offset(0 as i32 as isize) - *werr.offset(0 as i32 as isize)
            - gl
    {
        0.0f64
    } else {
        *w.offset(0 as i32 as isize) - *werr.offset(0 as i32 as isize)
            - gl
    };
    *wgap
        .offset(
            (n - 1 as i32) as isize,
        ) = if 0.0f64
        > gu - sigma
            - (*w.offset((n - 1 as i32) as isize)
                + *werr.offset((n - 1 as i32) as isize))
    {
        0.0f64
    } else {
        gu - sigma
            - (*w.offset((n - 1 as i32) as isize)
                + *werr.offset((n - 1 as i32) as isize))
    };
    return 0 as i32;
}
unsafe extern "C" fn _dlarrf(
    mut n: i32,
    mut diag: *mut f64,
    mut diag_off1: *mut f64,
    mut ld: *mut f64,
    mut clstrt: i32,
    mut w: *mut f64,
    mut wgap: *mut f64,
    mut werr: *mut f64,
    mut clgapl: f64,
    mut sigma: *mut f64,
    mut dplus: *mut f64,
    mut lplus: *mut f64,
) -> i32 {
    let mut i: i32 = 0;
    let mut ktry: i32 = 0;
    let mut s: f64 = 0.;
    let mut tmp: f64 = 0.;
    let mut max1: f64 = 0.;
    let mut growthbound: f64 = 0.;
    let mut lsigma: f64 = 0.;
    lsigma = *w.offset(clstrt as isize) - *werr.offset(clstrt as isize);
    lsigma -= lsigma.abs() * 4.0f64 * 2.2204460492503131e-16f64;
    growthbound = *diag.offset(0 as i32 as isize) * 8.0f64;
    ktry = 0 as i32;
    while ktry < 2 as i32 {
        s = -lsigma;
        *dplus
            .offset(
                0 as i32 as isize,
            ) = *diag.offset(0 as i32 as isize) + s;
        max1 = (*dplus.offset(0 as i32 as isize)).abs();
        i = 0 as i32;
        while i < n - 1 as i32 {
            tmp = *ld.offset(i as isize) / *dplus.offset(i as isize);
            *lplus.offset(i as isize) = tmp;
            s = s * tmp * *diag_off1.offset(i as isize) - lsigma;
            *dplus
                .offset(
                    (i + 1 as i32) as isize,
                ) = *diag.offset((i + 1 as i32) as isize) + s;
            max1 = if max1 > (*dplus.offset((i + 1 as i32) as isize)).abs() {
                max1
            } else {
                (*dplus.offset((i + 1 as i32) as isize)).abs()
            };
            i += 1;
        }
        *sigma = lsigma;
        if max1 <= growthbound {
            return 0 as i32;
        }
        lsigma = lsigma
            - (if clgapl * 0.25f64 < *wgap.offset(clstrt as isize) {
                clgapl * 0.25f64
            } else {
                *wgap.offset(clstrt as isize)
            });
        ktry += 1;
    }
    if max1 > 1e16f64 {
        eprintln!("dlarrf max1 = {}", max1);
        // fprintf(stderr, b"dlarrf max1 = %g\0" as *const u8 as *const libc::c_char, max1);
        return 1 as i32;
    }
    return 0 as i32;
}
unsafe extern "C" fn _dlaneg(
    mut n: i32,
    mut diag: *mut f64,
    mut lld: *mut f64,
    mut sigma: f64,
    mut twist_index: i32,
) -> i32 {
    let mut j: i32 = 0;
    let mut negcnt: i32 = 0;
    let mut p: f64 = 0.;
    let mut t: f64 = 0.;
    let mut dplus: f64 = 0.;
    let mut dminus: f64 = 0.;
    negcnt = 0 as i32;
    t = -sigma;
    j = 0 as i32;
    while j < twist_index - 1 as i32 {
        dplus = *diag.offset(j as isize) + t;
        if dplus < 0.0f64 {
            negcnt += 1;
        }
        t = t / dplus * *lld.offset(j as isize) - sigma;
        j += 1;
    }
    p = *diag.offset((n - 1 as i32) as isize) - sigma;
    j = n - 2 as i32;
    while j >= twist_index - 1 as i32 {
        dminus = *lld.offset(j as isize) + p;
        if dminus < 0.0f64 {
            negcnt += 1;
        }
        p = p / dminus * *diag.offset(j as isize) - sigma;
        j -= 1;
    }
    if t + sigma + p < 0.0f64 {
        negcnt += 1;
    }
    return negcnt;
}
unsafe extern "C" fn _dlarrb(
    mut n: i32,
    mut diag: *mut f64,
    mut lld: *mut f64,
    mut ifirst: i32,
    mut ilast: i32,
    mut rtol1: f64,
    mut rtol2: f64,
    mut w: *mut f64,
    mut wgap: *mut f64,
    mut werr: *mut f64,
    mut twist_index: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut iter: i32 = 0;
    let mut negcnt: i32 = 0;
    let mut mid: f64 = 0.;
    let mut back: f64 = 0.;
    let mut left: f64 = 0.;
    let mut right: f64 = 0.;
    let mut width: f64 = 0.;
    let mut cvrgd: f64 = if rtol1 * *wgap.offset(ifirst as isize)
        > rtol2
            * (if (*w.offset(ifirst as isize)).abs()
                > (*w.offset((ilast - 1 as i32) as isize)).abs()
            {
                (*w.offset(ifirst as isize)).abs()
            } else {
                (*w.offset((ilast - 1 as i32) as isize)).abs()
            })
    {
        rtol1 * *wgap.offset(ifirst as isize)
    } else {
        rtol2
            * (if (*w.offset(ifirst as isize)).abs()
                > (*w.offset((ilast - 1 as i32) as isize)).abs()
            {
                (*w.offset(ifirst as isize)).abs()
            } else {
                (*w.offset((ilast - 1 as i32) as isize)).abs()
            })
    };
    i = ifirst;
    while i < ilast {
        if !(*werr.offset(i as isize) < cvrgd) {
            left = *w.offset(i as isize);
            back = *werr.offset(i as isize);
            negcnt = ilast;
            while negcnt > i {
                left -= back;
                back *= 2.0f64;
                negcnt = _dlaneg(n, diag, lld, left, twist_index);
            }
            right = *w.offset(i as isize);
            back = *werr.offset(i as isize);
            negcnt = ifirst;
            while negcnt <= i {
                right += back;
                back *= 2.0f64;
                negcnt = _dlaneg(n, diag, lld, right, twist_index);
            }
            iter = 0 as i32;
            while iter < 1000 as i32 {
                mid = (left + right) * 0.5f64;
                width = right - mid;
                if width < cvrgd {
                    break;
                }
                negcnt = _dlaneg(n, diag, lld, mid, twist_index);
                if negcnt <= i {
                    left = mid;
                } else {
                    right = mid;
                }
                iter += 1;
            }
            *w.offset(i as isize) = mid;
            *werr.offset(i as isize) = width;
        }
        i += 1;
    }
    i = ifirst;
    while i < ilast - 1 as i32 {
        *wgap
            .offset(
                i as isize,
            ) = if 0.0f64
            > *w.offset((i + 1 as i32) as isize)
                - *werr.offset((i + 1 as i32) as isize) - *w.offset(i as isize)
                - *werr.offset(i as isize)
        {
            0.0f64
        } else {
            *w.offset((i + 1 as i32) as isize)
                - *werr.offset((i + 1 as i32) as isize) - *w.offset(i as isize)
                - *werr.offset(i as isize)
        };
        i += 1;
    }
    return 0 as i32;
}
unsafe extern "C" fn _dlar1v(
    mut n: i32,
    mut lambda: f64,
    mut diag: *mut f64,
    mut diag_off1: *mut f64,
    mut ld: *mut f64,
    mut lld: *mut f64,
    mut gaptol: f64,
    mut vec: *mut f64,
    mut negcnt: *mut i32,
    mut twist_index: *mut i32,
    mut resid: *mut f64,
    mut rqcorr: *mut f64,
    mut work: *mut f64,
) {
    let mut i: i32 = 0;
    let mut r1: i32 = 0;
    let mut r2: i32 = 0;
    let mut neg1: i32 = 0;
    let mut neg2: i32 = 0;
    let mut s: f64 = 0.;
    let mut tmp: f64 = 0.;
    let mut nrminv: f64 = 0.;
    let mut mingma: f64 = 0.;
    let mut dplus: f64 = 0.;
    let mut dminus: f64 = 0.;
    let mut lplus: *mut f64 = work;
    let mut uminus: *mut f64 = work.offset(n as isize);
    let mut work_p: *mut f64 = work.offset((n * 2 as i32) as isize);
    if *twist_index == -(1 as i32) {
        r1 = 0 as i32;
        r2 = n;
        *twist_index = 0 as i32;
    } else {
        r1 = *twist_index;
        r2 = *twist_index + 1 as i32;
    }
    neg2 = 0 as i32;
    s = *diag.offset((n - 1 as i32) as isize) - lambda;
    *work_p.offset((n - 1 as i32) as isize) = s;
    i = n - 2 as i32;
    while i >= r1 {
        dminus = *lld.offset(i as isize) + s;
        if dminus < 0.0f64 {
            neg2 += 1;
        }
        tmp = *diag.offset(i as isize) / dminus;
        *uminus.offset(i as isize) = *diag_off1.offset(i as isize) * tmp;
        s = s * tmp - lambda;
        *work_p.offset(i as isize) = s;
        i -= 1;
    }
    neg1 = 0 as i32;
    s = -lambda;
    i = 0 as i32;
    while i < r1 {
        dplus = *diag.offset(i as isize) + s;
        if dplus < 0.0f64 {
            neg1 += 1;
        }
        *lplus.offset(i as isize) = *ld.offset(i as isize) / dplus;
        s = s * *lplus.offset(i as isize) * *diag_off1.offset(i as isize) - lambda;
        i += 1;
    }
    mingma = s + lambda + *work_p.offset(r1 as isize);
    if mingma < 0.0f64 {
        neg1 += 1;
    }
    *negcnt = neg1 + neg2;
    i = r1;
    while i < r2 - 1 as i32 {
        dplus = *diag.offset(i as isize) + s;
        *lplus.offset(i as isize) = *ld.offset(i as isize) / dplus;
        tmp = s * *lplus.offset(i as isize) * *diag_off1.offset(i as isize);
        s = tmp - lambda;
        tmp = tmp + *work_p.offset((i + 1 as i32) as isize);
        if tmp.abs() <= mingma.abs() {
            mingma = tmp;
            *twist_index = i + 1 as i32;
        }
        i += 1;
    }
    *vec.offset(*twist_index as isize) = 1.0f64;
    let mut ztz: f64 = 1.0f64;
    i = *twist_index - 1 as i32;
    while i >= 0 as i32 {
        tmp = -(*lplus.offset(i as isize)
            * *vec.offset((i + 1 as i32) as isize));
        ztz += tmp * tmp;
        *vec.offset(i as isize) = tmp;
        i -= 1;
    }
    i = *twist_index;
    while i < n - 1 as i32 {
        tmp = -(*uminus.offset(i as isize) * *vec.offset(i as isize));
        ztz += tmp * tmp;
        *vec.offset((i + 1 as i32) as isize) = tmp;
        i += 1;
    }
    tmp = 1.0f64 / ztz;
    nrminv = (tmp).sqrt();
    i = 0 as i32;
    while i < n {
        *vec.offset(i as isize) *= nrminv;
        i += 1;
    }
    *resid = mingma.abs() * nrminv;
    *rqcorr = mingma * tmp;
}
unsafe extern "C" fn _compute_eigenvectors(
    mut n: i32,
    mut diag: *mut f64,
    mut diag_off1: *mut f64,
    mut w: *mut f64,
    mut werr: *mut f64,
    mut wgap: *mut f64,
    mut vec: *mut f64,
    mut work: *mut f64,
    mut iwork: *mut i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut icls: i32 = 0;
    let mut iter: i32 = 0;
    let mut idone: i32 = 0;
    let mut ndepth: i32 = 0;
    let mut ncluster: i32 = 0;
    let mut ncluster1: i32 = 0;
    let mut negcnt: i32 = 0;
    let mut oldfst: i32 = 0;
    let mut oldlst: i32 = 0;
    let mut newfst: i32 = 0;
    let mut newlst: i32 = 0;
    let mut needbs: i32 = 0;
    let mut iinfo: i32 = 0;
    let mut fudge: f64 = 0.;
    let mut eps: f64 = 0.;
    let mut rqtol: f64 = 0.;
    let mut tol: f64 = 0.;
    let mut tmp: f64 = 0.;
    let mut left: f64 = 0.;
    let mut right: f64 = 0.;
    let mut gap: f64 = 0.;
    let mut bstw: f64 = 0.;
    let mut savgap: f64 = 0.;
    let mut gaptol: f64 = 0.;
    let mut sigma: f64 = 0.;
    let mut tau: f64 = 0.;
    let mut resid: f64 = 0.;
    let mut lambda: f64 = 0.;
    let mut bstres: f64 = 0.;
    let mut rqcorr: f64 = 0.;
    let mut resid_tol: f64 = 0.;
    let mut rqcorr_tol: f64 = 0.;
    let mut buf_w: *mut f64 = 0 as *mut f64;
    let mut buf_ld: *mut f64 = 0 as *mut f64;
    let mut buf_lld: *mut f64 = 0 as *mut f64;
    let mut buf_wrk: *mut f64 = 0 as *mut f64;
    let mut twist_indices: *mut i32 = 0 as *mut i32;
    let mut swap: *mut i32 = 0 as *mut i32;
    let mut old_cluster_range: *mut i32 = 0 as *mut i32;
    let mut new_cluster_range: *mut i32 = 0 as *mut i32;
    if n <= 0 as i32 {
        return 0 as i32;
    }
    buf_w = work;
    buf_ld = work.offset(n as isize);
    buf_lld = work.offset((n * 2 as i32) as isize);
    buf_wrk = work.offset((n * 3 as i32) as isize);
    i = 0 as i32;
    while i < n * 6 as i32 {
        *work.offset(i as isize) = 0.0f64;
        i += 1;
    }
    twist_indices = iwork;
    i = 0 as i32;
    while i < n {
        *twist_indices.offset(i as isize) = 0 as i32;
        i += 1;
    }
    old_cluster_range = iwork.offset(n as isize);
    new_cluster_range = iwork.offset((n * 3 as i32) as isize);
    eps = 2.2204460492503131e-16f64;
    rqtol = 2.2204460492503131e-16f64 * 2.0f64;
    tol = 2.2204460492503131e-16f64 * 8 as i32 as f64;
    sigma = *diag_off1.offset((n - 1 as i32) as isize);
    if 1 as i32 == n {
        *vec.offset(0 as i32 as isize) = 1.0f64;
        *w.offset(0 as i32 as isize) += sigma;
        return 0 as i32;
    }
    i = 0 as i32;
    while i < n {
        *buf_w.offset(i as isize) = *w.offset(i as isize);
        *w.offset(i as isize) += sigma;
        i += 1;
    }
    ncluster = 1 as i32;
    *old_cluster_range.offset(0 as i32 as isize) = 0 as i32;
    *old_cluster_range.offset(1 as i32 as isize) = n;
    idone = 0 as i32;
    ndepth = 0 as i32;
    while ndepth < n {
        if idone == n {
            break;
        }
        ncluster1 = ncluster;
        ncluster = 0 as i32;
        icls = 0 as i32;
        while icls < ncluster1 {
            oldfst = *old_cluster_range.offset((icls * 2 as i32) as isize);
            oldlst = *old_cluster_range
                .offset((icls * 2 as i32 + 1 as i32) as isize);
            if ndepth > 0 as i32 {
                i = 0 as i32;
                while i < n {
                    *diag.offset(i as isize) = *vec.offset((oldfst * n + i) as isize);
                    i += 1;
                }
                i = 0 as i32;
                while i < n - 1 as i32 {
                    *diag_off1
                        .offset(
                            i as isize,
                        ) = *vec.offset(((oldfst + 1 as i32) * n + i) as isize);
                    i += 1;
                }
                sigma = *vec
                    .offset(
                        ((oldfst + 2 as i32) * n - 1 as i32) as isize,
                    );
            }
            j = 0 as i32;
            while j < n - 1 as i32 {
                tmp = *diag.offset(j as isize) * *diag_off1.offset(j as isize);
                *buf_ld.offset(j as isize) = tmp;
                *buf_lld.offset(j as isize) = tmp * *diag_off1.offset(j as isize);
                j += 1;
            }
            newfst = oldfst;
            newlst = oldfst + 1 as i32;
            while newlst <= oldlst {
                if !(newlst < oldlst
                    && *wgap.offset((newlst - 1 as i32) as isize)
                        < 0.001f64
                            * (*buf_w.offset((newlst - 1 as i32) as isize)).abs())
                {
                    if newlst - newfst > 1 as i32 {
                        _dlarrb(
                            n,
                            diag,
                            buf_lld,
                            newfst,
                            newfst + 1 as i32,
                            rqtol,
                            rqtol,
                            buf_w,
                            wgap,
                            werr,
                            n,
                        );
                        _dlarrb(
                            n,
                            diag,
                            buf_lld,
                            newlst - 1 as i32,
                            newlst,
                            rqtol,
                            rqtol,
                            buf_w,
                            wgap,
                            werr,
                            n,
                        );
                        iinfo = _dlarrf(
                            n,
                            diag,
                            diag_off1,
                            buf_ld,
                            newfst,
                            buf_w,
                            wgap,
                            werr,
                            *wgap.offset((newfst - 1 as i32) as isize),
                            &mut tau,
                            vec.offset((newfst * n) as isize),
                            vec.offset(((newfst + 1 as i32) * n) as isize),
                        );
                        if iinfo != 0 as i32 {
                            return -(2 as i32);
                        }
                        *vec
                            .offset(
                                ((newfst + 2 as i32) * n - 1 as i32)
                                    as isize,
                            ) = sigma + tau;
                        k = newfst;
                        while k < newlst {
                            fudge = eps * 3.0f64 * (*buf_w.offset(k as isize)).abs();
                            *buf_w.offset(k as isize) -= tau;
                            fudge += eps * 4.0f64 * (*buf_w.offset(k as isize)).abs();
                            *werr.offset(k as isize) += fudge;
                            k += 1;
                        }
                        *new_cluster_range
                            .offset((ncluster * 2 as i32) as isize) = newfst;
                        *new_cluster_range
                            .offset(
                                (ncluster * 2 as i32 + 1 as i32) as isize,
                            ) = newlst;
                        ncluster += 1;
                    } else {
                        lambda = *buf_w.offset(newfst as isize);
                        left = lambda - *werr.offset(newfst as isize);
                        right = lambda + *werr.offset(newfst as isize);
                        gap = *wgap.offset(newfst as isize);
                        if newfst == 0 as i32 || newfst + 1 as i32 == n {
                            gaptol = 0 as i32 as f64;
                        } else {
                            gaptol = gap * eps;
                        }
                        savgap = gap;
                        resid_tol = tol * gap;
                        rqcorr_tol = rqtol * lambda.abs();
                        needbs = 0 as i32;
                        bstres = 1e307f64;
                        bstw = 0 as i32 as f64;
                        *twist_indices.offset(newfst as isize) = -(1 as i32);
                        iter = 0 as i32;
                        while iter < 6 as i32 {
                            _dlar1v(
                                n,
                                lambda,
                                diag,
                                diag_off1,
                                buf_ld,
                                buf_lld,
                                gaptol,
                                vec.offset((newfst * n) as isize),
                                &mut negcnt,
                                twist_indices.offset(newfst as isize),
                                &mut resid,
                                &mut rqcorr,
                                buf_wrk,
                            );
                            if resid < bstres {
                                bstres = resid;
                                bstw = lambda;
                            }
                            if resid < resid_tol || rqcorr.abs() < rqcorr_tol {
                                break;
                            }
                            if lambda + rqcorr > right || lambda + rqcorr < left {
                                needbs = 1 as i32;
                                break;
                            } else {
                                if newfst < negcnt {
                                    if rqcorr > 0 as i32 as f64 {
                                        needbs = 1 as i32;
                                        break;
                                    } else {
                                        right = lambda;
                                    }
                                } else if rqcorr < 0 as i32 as f64 {
                                    needbs = 1 as i32;
                                    break;
                                } else {
                                    left = lambda;
                                }
                                *buf_w.offset(newfst as isize) = (right + left) * 0.5f64;
                                lambda += rqcorr;
                                *werr.offset(newfst as isize) = (right - left) * 0.5f64;
                                if right - left < rqcorr_tol {
                                    if bstres < resid {
                                        lambda = bstw;
                                        _dlar1v(
                                            n,
                                            lambda,
                                            diag,
                                            diag_off1,
                                            buf_ld,
                                            buf_lld,
                                            gaptol,
                                            vec.offset((newfst * n) as isize),
                                            &mut negcnt,
                                            twist_indices.offset(newfst as isize),
                                            &mut resid,
                                            &mut rqcorr,
                                            buf_wrk,
                                        );
                                    }
                                    break;
                                } else {
                                    iter += 1;
                                }
                            }
                        }
                        if needbs != 0 {
                            _dlarrb(
                                n,
                                diag,
                                buf_lld,
                                newfst,
                                newfst + 1 as i32,
                                0.0f64,
                                eps * 2.0f64,
                                buf_w,
                                wgap,
                                werr,
                                *twist_indices.offset(newfst as isize) + 1 as i32,
                            );
                            lambda = *buf_w.offset(newfst as isize);
                            *twist_indices.offset(newfst as isize) = -(1 as i32);
                            _dlar1v(
                                n,
                                lambda,
                                diag,
                                diag_off1,
                                buf_ld,
                                buf_lld,
                                gaptol,
                                vec.offset((newfst * n) as isize),
                                &mut negcnt,
                                twist_indices.offset(newfst as isize),
                                &mut resid,
                                &mut rqcorr,
                                buf_wrk,
                            );
                        }
                        *w.offset(newfst as isize) = lambda + sigma;
                        if newfst > 0 as i32 {
                            *wgap
                                .offset(
                                    (newfst - 1 as i32) as isize,
                                ) = if *wgap.offset((newfst - 1 as i32) as isize)
                                > *w.offset(newfst as isize) - *werr.offset(newfst as isize)
                                    - *w.offset((newfst - 1 as i32) as isize)
                                    - *werr.offset((newfst - 1 as i32) as isize)
                            {
                                *wgap.offset((newfst - 1 as i32) as isize)
                            } else {
                                *w.offset(newfst as isize) - *werr.offset(newfst as isize)
                                    - *w.offset((newfst - 1 as i32) as isize)
                                    - *werr.offset((newfst - 1 as i32) as isize)
                            };
                        }
                        if newfst < n - 1 as i32 {
                            *wgap
                                .offset(
                                    newfst as isize,
                                ) = if savgap
                                > *w.offset((newfst + 1 as i32) as isize)
                                    - *werr.offset((newfst + 1 as i32) as isize)
                                    - *w.offset(newfst as isize) - *werr.offset(newfst as isize)
                            {
                                savgap
                            } else {
                                *w.offset((newfst + 1 as i32) as isize)
                                    - *werr.offset((newfst + 1 as i32) as isize)
                                    - *w.offset(newfst as isize) - *werr.offset(newfst as isize)
                            };
                        }
                        idone += 1;
                    }
                    newfst = newlst;
                }
                newlst += 1;
            }
            icls += 1;
        }
        swap = old_cluster_range;
        old_cluster_range = new_cluster_range;
        new_cluster_range = swap;
        ndepth += 1;
    }
    if idone < n {
        return -(2 as i32);
    }
    return 0 as i32;
}
unsafe extern "C" fn _dlaev2(
    mut eig: *mut f64,
    mut vec: *mut f64,
    mut diag: *mut f64,
    mut diag_off1: *mut f64,
) -> i32 {
    let mut a: f64 = *diag.offset(0 as i32 as isize);
    let mut b: f64 = *diag_off1.offset(0 as i32 as isize);
    let mut c: f64 = *diag.offset(1 as i32 as isize);
    let mut df: f64 = 0.;
    let mut cs: f64 = 0.;
    let mut ct: f64 = 0.;
    let mut tb: f64 = 0.;
    let mut sm: f64 = 0.;
    let mut tn: f64 = 0.;
    let mut rt: f64 = 0.;
    let mut tmp: f64 = 0.;
    let mut rt1: f64 = 0.;
    let mut rt2: f64 = 0.;
    let mut cs1: f64 = 0.;
    let mut sn1: f64 = 0.;
    let mut sgn1: i32 = 0;
    let mut sgn2: i32 = 0;
    sm = a + c;
    df = a - c;
    tb = b + b;
    rt = (tb * tb + df * df).sqrt();
    if sm > 0.0f64 {
        rt1 = (sm + rt) * 0.5f64;
        sgn1 = 1 as i32;
        rt2 = (a * c - b * b) / rt1;
    } else if sm < 0.0f64 {
        rt1 = (sm - rt) * 0.5f64;
        sgn1 = -(1 as i32);
        rt2 = (a * c - b * b) / rt1;
    } else {
        rt1 = rt * 0.5f64;
        rt2 = rt * -0.5f64;
        sgn1 = 1 as i32;
    }
    if df >= 0.0f64 {
        cs = df + rt;
        sgn2 = 1 as i32;
    } else {
        cs = df - rt;
        sgn2 = -(1 as i32);
    }
    if cs.abs() > tb.abs() {
        ct = -tb / cs;
        sn1 = 1.0f64 / (ct * ct + 1.0f64).sqrt();
        cs1 = ct * sn1;
    } else if b == 0.0f64 {
        cs1 = 1.0f64;
        sn1 = 0.0f64;
    } else {
        tn = -cs / tb;
        cs1 = 1.0f64 / (tn * tn + 1.0f64).sqrt();
        sn1 = tn * cs1;
    }
    if sgn1 == sgn2 {
        tmp = cs1;
        cs1 = -sn1;
        sn1 = tmp;
    }
    *eig.offset(0 as i32 as isize) = rt2;
    *eig.offset(1 as i32 as isize) = rt1;
    *vec.offset(0 as i32 as isize) = -sn1;
    *vec.offset(1 as i32 as isize) = cs1;
    *vec.offset(2 as i32 as isize) = cs1;
    *vec.offset(3 as i32 as isize) = sn1;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _CINTdiagonalize(
    mut n: i32,
    mut diag: *mut f64,
    mut diag_off1: *mut f64,
    mut eig: *mut f64,
    mut vec: *mut f64,
) -> i32 {
    if n == 0 as i32 {
        return 0 as i32
    } else if n == 1 as i32 {
        *eig.offset(0 as i32 as isize) = *diag.offset(0 as i32 as isize);
        *vec.offset(0 as i32 as isize) = 1.0f64;
        return 0 as i32;
    } else if n == 2 as i32 {
        return _dlaev2(eig, vec, diag, diag_off1)
    }
    let mut iwork: [i32; 160] = [0; 160];
    let mut work: [f64; 289] = [0.; 289];
    let mut buf_err: *mut f64 = work.as_mut_ptr().offset(n as isize);
    let mut buf_gp: *mut f64 = work
        .as_mut_ptr()
        .offset((n * 2 as i32) as isize)
        .offset(1 as i32 as isize);
    let mut buf_wrk: *mut f64 = work
        .as_mut_ptr()
        .offset((n * 3 as i32) as isize)
        .offset(1 as i32 as isize);
    let mut info: i32 = 0;
    info = _compute_eigenvalues(n, diag, diag_off1, eig, buf_err, buf_gp, buf_wrk);
    if info == 0 as i32 {
        info = _compute_eigenvectors(
            n,
            diag,
            diag_off1,
            eig,
            buf_err,
            buf_gp,
            vec,
            buf_wrk,
            iwork.as_mut_ptr(),
        );
    }
    return info;
}
