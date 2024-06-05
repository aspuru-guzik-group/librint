#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
}

pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
unsafe extern "C" fn _dlarrk(
    mut n: libc::c_int,
    mut iw: libc::c_int,
    mut gl: libc::c_double,
    mut gu: libc::c_double,
    mut diag: *mut libc::c_double,
    mut e2: *mut libc::c_double,
    mut reltol: libc::c_double,
    mut w: *mut libc::c_double,
    mut werr: *mut libc::c_double,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut it: libc::c_int = 0;
    let mut mid: libc::c_double = 0.;
    let mut tmp1: libc::c_double = 0.;
    let mut left: libc::c_double = 0.;
    let mut right: libc::c_double = 0.;
    let mut tnorm: libc::c_double = 0.;
    let mut negcnt: libc::c_int = 0;
    let mut info: libc::c_int = 0;
    if n <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    tnorm = if fabs(gl) > fabs(gu) { fabs(gl) } else { fabs(gu) };
    info = -(1 as libc::c_int);
    left = gl - tnorm * 2.0f64 * 2.2204460492503131e-16f64 * n as libc::c_double;
    right = gu + tnorm * 2.0f64 * 2.2204460492503131e-16f64 * n as libc::c_double;
    it = 0 as libc::c_int;
    while it < 1000 as libc::c_int {
        tmp1 = fabs(right - left);
        if tmp1 <= 0 as libc::c_int as libc::c_double
            || tmp1
                < reltol
                    * (if fabs(right) > fabs(left) { fabs(right) } else { fabs(left) })
        {
            info = 0 as libc::c_int;
            break;
        } else {
            mid = (left + right) * 0.5f64;
            negcnt = 0 as libc::c_int;
            tmp1 = *diag.offset(0 as libc::c_int as isize) - mid;
            if tmp1 <= 0.0f64 {
                negcnt += 1;
                negcnt;
            }
            i = 1 as libc::c_int;
            while i < n {
                tmp1 = *diag.offset(i as isize)
                    - *e2.offset((i - 1 as libc::c_int) as isize) / tmp1 - mid;
                if tmp1 <= 0.0f64 {
                    negcnt += 1;
                    negcnt;
                }
                i += 1;
                i;
            }
            if negcnt >= iw {
                right = mid;
            } else {
                left = mid;
            }
            it += 1;
            it;
        }
    }
    *w = (left + right) * 0.5f64;
    *werr = fabs(right - left) * 0.5f64;
    return info;
}
unsafe extern "C" fn _dlarrc(
    mut n: libc::c_int,
    mut vl: libc::c_double,
    mut vu: libc::c_double,
    mut diag: *mut libc::c_double,
    mut e2: *mut libc::c_double,
    mut lcnt: *mut libc::c_int,
    mut rcnt: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut left_count: libc::c_int = 0 as libc::c_int;
    let mut right_count: libc::c_int = 0 as libc::c_int;
    let mut tmp: libc::c_double = 0.;
    let mut lpivot: libc::c_double = 0.;
    let mut rpivot: libc::c_double = 0.;
    lpivot = *diag.offset(0 as libc::c_int as isize) - vl;
    rpivot = *diag.offset(0 as libc::c_int as isize) - vu;
    if lpivot <= 0.0f64 {
        left_count += 1;
        left_count;
    }
    if rpivot <= 0.0f64 {
        right_count += 1;
        right_count;
    }
    i = 0 as libc::c_int;
    while i < n - 1 as libc::c_int {
        tmp = *e2.offset(i as isize);
        lpivot = *diag.offset((i + 1 as libc::c_int) as isize) - vl - tmp / lpivot;
        rpivot = *diag.offset((i + 1 as libc::c_int) as isize) - vu - tmp / rpivot;
        if lpivot <= 0.0f64 {
            left_count += 1;
            left_count;
        }
        if rpivot <= 0.0f64 {
            right_count += 1;
            right_count;
        }
        i += 1;
        i;
    }
    *lcnt = left_count;
    *rcnt = right_count;
}
unsafe extern "C" fn _dlasq4(
    mut i0: libc::c_int,
    mut n0: libc::c_int,
    mut n0init: libc::c_int,
    mut qvecp: *mut libc::c_double,
    mut qvec1p: *mut libc::c_double,
    mut evecp: *mut libc::c_double,
    mut evec1p: *mut libc::c_double,
    mut dmin: *mut libc::c_double,
    mut dn: *mut libc::c_double,
    mut tau: *mut libc::c_double,
) -> libc::c_int {
    let mut a2: libc::c_double = 0.;
    let mut b1: libc::c_double = 0.;
    let mut b2: libc::c_double = 0.;
    let mut gap1: libc::c_double = 0.;
    let mut gap2: libc::c_double = 0.;
    let mut s: libc::c_double = 0.0f64;
    let mut i: libc::c_int = 0;
    if n0init == n0 {
        if *dmin.offset(0 as libc::c_int as isize)
            == *dn.offset(0 as libc::c_int as isize)
        {
            if *dmin.offset(1 as libc::c_int as isize)
                == *dn.offset(1 as libc::c_int as isize)
            {
                b1 = sqrt(
                    *qvecp.offset((n0 - 1 as libc::c_int) as isize)
                        * *evecp.offset((n0 - 2 as libc::c_int) as isize),
                );
                b2 = sqrt(
                    *qvecp.offset((n0 - 2 as libc::c_int) as isize)
                        * *evecp.offset((n0 - 3 as libc::c_int) as isize),
                );
                a2 = *qvecp.offset((n0 - 2 as libc::c_int) as isize)
                    + *evecp.offset((n0 - 2 as libc::c_int) as isize);
                gap2 = *dmin.offset(2 as libc::c_int as isize) - a2
                    - *dmin.offset(2 as libc::c_int as isize) * 0.25f64;
                if gap2 > b2 {
                    gap1 = a2 - *dn.offset(0 as libc::c_int as isize) - b2 / gap2 * b2;
                } else {
                    gap1 = a2 - *dn.offset(0 as libc::c_int as isize) - (b1 + b2);
                }
                if gap1 > b1 {
                    s = if *dn.offset(0 as libc::c_int as isize) - b1 / gap1 * b1
                        > *dmin.offset(0 as libc::c_int as isize) * 0.5f64
                    {
                        *dn.offset(0 as libc::c_int as isize) - b1 / gap1 * b1
                    } else {
                        *dmin.offset(0 as libc::c_int as isize) * 0.5f64
                    };
                } else {
                    s = 0.0f64;
                    if *dn.offset(0 as libc::c_int as isize) > b1 {
                        s = *dn.offset(0 as libc::c_int as isize) - b1;
                    }
                    if a2 > b1 + b2 {
                        s = if s < a2 - (b1 + b2) { s } else { a2 - (b1 + b2) };
                    }
                    s = if s > *dmin.offset(0 as libc::c_int as isize) * 0.333f64 {
                        s
                    } else {
                        *dmin.offset(0 as libc::c_int as isize) * 0.333f64
                    };
                }
            } else {
                if *evecp.offset((n0 - 2 as libc::c_int) as isize)
                    > *qvecp.offset((n0 - 2 as libc::c_int) as isize)
                {
                    return 0 as libc::c_int;
                }
                b2 = *evecp.offset((n0 - 2 as libc::c_int) as isize)
                    / *qvecp.offset((n0 - 2 as libc::c_int) as isize);
                a2 = b2;
                i = n0 - 3 as libc::c_int;
                while i >= i0 {
                    b1 = b2;
                    if *evecp.offset(i as isize) > *qvecp.offset(i as isize) {
                        return 0 as libc::c_int;
                    }
                    b2 *= *evecp.offset(i as isize) / *qvecp.offset(i as isize);
                    a2 += b2;
                    if 0.563f64 < a2 || (if b1 > b2 { b1 } else { b2 }) < a2 * 0.01f64 {
                        break;
                    }
                    i -= 1;
                    i;
                }
                a2 *= 1.05f64;
                if a2 < 0.563f64 {
                    s = *dn.offset(0 as libc::c_int as isize) * (1.0f64 - sqrt(a2))
                        / (a2 + 1.0f64);
                } else {
                    s = *dmin.offset(0 as libc::c_int as isize) * 0.25f64;
                }
            }
        } else if *dmin.offset(0 as libc::c_int as isize)
            == *dn.offset(1 as libc::c_int as isize)
        {
            if *evec1p.offset((n0 - 2 as libc::c_int) as isize)
                > *qvec1p.offset((n0 - 1 as libc::c_int) as isize)
                || *evecp.offset((n0 - 3 as libc::c_int) as isize)
                    > *qvecp.offset((n0 - 3 as libc::c_int) as isize)
            {
                return 0 as libc::c_int;
            }
            a2 = *evec1p.offset((n0 - 2 as libc::c_int) as isize)
                / *qvec1p.offset((n0 - 1 as libc::c_int) as isize);
            b2 = *evecp.offset((n0 - 3 as libc::c_int) as isize)
                / *qvecp.offset((n0 - 3 as libc::c_int) as isize);
            a2 += b2;
            i = n0 - 4 as libc::c_int;
            while i >= i0 {
                if b2 == 0.0f64 {
                    break;
                }
                b1 = b2;
                if *evecp.offset(i as isize) > *qvecp.offset(i as isize) {
                    return 0 as libc::c_int;
                }
                b2 *= *evecp.offset(i as isize) / *qvecp.offset(i as isize);
                a2 += b2;
                if (if b2 > b1 { b2 } else { b1 }) * 100.0f64 < a2 || 0.563f64 < a2 {
                    break;
                }
                i -= 1;
                i;
            }
            a2 *= 1.05f64;
            if a2 < 0.563f64 {
                s = *dn.offset(1 as libc::c_int as isize) * (1.0f64 - sqrt(a2))
                    / (a2 + 1.0f64);
            } else {
                s = *dmin.offset(0 as libc::c_int as isize) * 0.25f64;
            }
        } else if *dmin.offset(0 as libc::c_int as isize)
            == *dn.offset(2 as libc::c_int as isize)
        {
            if *evec1p.offset((n0 - 3 as libc::c_int) as isize)
                > *qvec1p.offset((n0 - 2 as libc::c_int) as isize)
                || *evec1p.offset((n0 - 2 as libc::c_int) as isize)
                    > *qvec1p.offset((n0 - 1 as libc::c_int) as isize)
            {
                return 0 as libc::c_int;
            }
            a2 = *evec1p.offset((n0 - 3 as libc::c_int) as isize)
                / *qvec1p.offset((n0 - 2 as libc::c_int) as isize)
                * (*evec1p.offset((n0 - 2 as libc::c_int) as isize)
                    / *qvec1p.offset((n0 - 1 as libc::c_int) as isize) + 1.0f64);
            if n0 - i0 > 3 as libc::c_int {
                b2 = *evecp.offset((n0 - 4 as libc::c_int) as isize)
                    / *qvecp.offset((n0 - 4 as libc::c_int) as isize);
                a2 += b2;
                i = n0 - 5 as libc::c_int;
                while i >= i0 {
                    b1 = b2;
                    if *evecp.offset(i as isize) > *qvecp.offset(i as isize) {
                        return 0 as libc::c_int;
                    }
                    b2 *= *evecp.offset(i as isize) / *qvecp.offset(i as isize);
                    a2 += b2;
                    if 0.563f64 < a2 || (if b2 > b1 { b2 } else { b1 }) < a2 * 0.01f64 {
                        break;
                    }
                    i -= 1;
                    i;
                }
                a2 *= 1.05f64;
            }
            if a2 < 0.563f64 {
                s = *dn.offset(2 as libc::c_int as isize) * (1.0f64 - sqrt(a2))
                    / (a2 + 1.0f64);
            } else {
                s = *dmin.offset(0 as libc::c_int as isize) * 0.25f64;
            }
        } else {
            s = *dmin.offset(0 as libc::c_int as isize) * 0.25f64;
        }
    } else if n0init == n0 + 1 as libc::c_int {
        if *dmin.offset(1 as libc::c_int as isize)
            == *dn.offset(1 as libc::c_int as isize)
            && *dmin.offset(2 as libc::c_int as isize)
                == *dn.offset(2 as libc::c_int as isize)
        {
            if *evecp.offset((n0 - 2 as libc::c_int) as isize)
                > *qvecp.offset((n0 - 2 as libc::c_int) as isize)
            {
                return 0 as libc::c_int;
            }
            s = *dmin.offset(1 as libc::c_int as isize) * 0.333f64;
            b1 = *evecp.offset((n0 - 2 as libc::c_int) as isize)
                / *qvecp.offset((n0 - 2 as libc::c_int) as isize);
            b2 = b1;
            if b2 != 0.0f64 {
                i = n0 - 3 as libc::c_int;
                while i >= i0 {
                    a2 = b1;
                    if *evecp.offset(i as isize) > *qvecp.offset(i as isize) {
                        return 0 as libc::c_int;
                    }
                    b1 *= *evecp.offset(i as isize) / *qvecp.offset(i as isize);
                    b2 += b1;
                    if (if b1 > a2 { b1 } else { a2 }) * 100.0f64 < b2 {
                        break;
                    }
                    i -= 1;
                    i;
                }
            }
            a2 = *dmin.offset(1 as libc::c_int as isize) / (b2 * 1.05f64 + 1.0f64);
            b2 = sqrt(b2 * 1.05f64);
            gap2 = *dmin.offset(2 as libc::c_int as isize) * 0.5f64 - a2;
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
        } else if *dmin.offset(1 as libc::c_int as isize)
            == *dn.offset(1 as libc::c_int as isize)
        {
            s = *dmin.offset(1 as libc::c_int as isize) * 0.5f64;
        } else {
            s = *dmin.offset(1 as libc::c_int as isize) * 0.25f64;
        }
    } else if n0init == n0 + 2 as libc::c_int {
        if *dmin.offset(2 as libc::c_int as isize)
            == *dn.offset(2 as libc::c_int as isize)
            && *evecp.offset((n0 - 2 as libc::c_int) as isize) * 2.0f64
                < *qvecp.offset((n0 - 2 as libc::c_int) as isize)
        {
            if *evecp.offset((n0 - 2 as libc::c_int) as isize)
                > *qvecp.offset((n0 - 2 as libc::c_int) as isize)
            {
                return 0 as libc::c_int;
            }
            b1 = *evecp.offset((n0 - 2 as libc::c_int) as isize)
                / *qvecp.offset((n0 - 2 as libc::c_int) as isize);
            b2 = b1;
            if b2 != 0.0f64 {
                i = n0 - 2 as libc::c_int;
                while i > i0 {
                    if *evecp.offset((i - 1 as libc::c_int) as isize)
                        > *qvecp.offset((i - 1 as libc::c_int) as isize)
                    {
                        return 0 as libc::c_int;
                    }
                    b1
                        *= *evecp.offset((i - 1 as libc::c_int) as isize)
                            / *qvecp.offset((i - 1 as libc::c_int) as isize);
                    b2 += b1;
                    if b1 * 100.0f64 < b2 {
                        break;
                    }
                    i -= 1;
                    i;
                }
            }
            s = *dmin.offset(2 as libc::c_int as isize) * 0.333f64;
            a2 = *dmin.offset(2 as libc::c_int as isize) / (b2 * 1.05f64 + 1.0f64);
            b2 = sqrt(b2 * 1.05f64);
            gap2 = *qvecp.offset((n0 - 2 as libc::c_int) as isize)
                + *evecp.offset((n0 - 3 as libc::c_int) as isize)
                - sqrt(
                    *qvecp.offset((n0 - 3 as libc::c_int) as isize)
                        * *evecp.offset((n0 - 3 as libc::c_int) as isize),
                ) - a2;
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
            s = *dmin.offset(2 as libc::c_int as isize) * 0.25f64;
        }
    } else if n0init > n0 + 2 as libc::c_int {
        s = 0.0f64;
    }
    *tau = s;
    return 0 as libc::c_int;
}
unsafe extern "C" fn _dlasq5(
    mut i0: libc::c_int,
    mut n0: libc::c_int,
    mut qvecp: *mut libc::c_double,
    mut qvec1p: *mut libc::c_double,
    mut evecp: *mut libc::c_double,
    mut evec1p: *mut libc::c_double,
    mut tau: libc::c_double,
    mut tol: libc::c_double,
    mut dmin: *mut libc::c_double,
    mut dn: *mut libc::c_double,
) {
    let mut diag: libc::c_double = *qvecp.offset(i0 as isize) - tau;
    let mut diag_min: libc::c_double = diag;
    let mut temp: libc::c_double = 0.;
    let mut j: libc::c_int = 0;
    j = i0;
    while j < n0 - 3 as libc::c_int {
        *qvec1p.offset(j as isize) = diag + *evecp.offset(j as isize);
        temp = *qvecp.offset((j + 1 as libc::c_int) as isize)
            / *qvec1p.offset(j as isize);
        diag = diag * temp - tau;
        if diag < tol {
            diag = 0.0f64;
        }
        diag_min = if diag_min < diag { diag_min } else { diag };
        *evec1p.offset(j as isize) = *evecp.offset(j as isize) * temp;
        j += 1;
        j;
    }
    *dn.offset(2 as libc::c_int as isize) = diag;
    j = n0 - 3 as libc::c_int;
    *qvec1p.offset(j as isize) = diag + *evecp.offset(j as isize);
    temp = *qvecp.offset((j + 1 as libc::c_int) as isize) / *qvec1p.offset(j as isize);
    *evec1p.offset(j as isize) = *evecp.offset(j as isize) * temp;
    diag = diag * temp - tau;
    *dn.offset(1 as libc::c_int as isize) = diag;
    j = n0 - 2 as libc::c_int;
    *qvec1p.offset(j as isize) = diag + *evecp.offset(j as isize);
    temp = *qvecp.offset((j + 1 as libc::c_int) as isize) / *qvec1p.offset(j as isize);
    *evec1p.offset(j as isize) = *evecp.offset(j as isize) * temp;
    diag = diag * temp - tau;
    *dn.offset(0 as libc::c_int as isize) = diag;
    *qvec1p.offset((n0 - 1 as libc::c_int) as isize) = diag;
    *dmin.offset(2 as libc::c_int as isize) = diag_min;
    *dmin
        .offset(
            1 as libc::c_int as isize,
        ) = if *dmin.offset(2 as libc::c_int as isize)
        < *dn.offset(1 as libc::c_int as isize)
    {
        *dmin.offset(2 as libc::c_int as isize)
    } else {
        *dn.offset(1 as libc::c_int as isize)
    };
    *dmin
        .offset(
            0 as libc::c_int as isize,
        ) = if *dmin.offset(1 as libc::c_int as isize)
        < *dn.offset(0 as libc::c_int as isize)
    {
        *dmin.offset(1 as libc::c_int as isize)
    } else {
        *dn.offset(0 as libc::c_int as isize)
    };
}
unsafe extern "C" fn _dlasq2(
    mut n: libc::c_int,
    mut work: *mut libc::c_double,
    mut diag: *mut libc::c_double,
    mut diag_off: *mut libc::c_double,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut itry: libc::c_int = 0;
    let mut iwhilb: libc::c_int = 0;
    let mut iter: libc::c_int = 0;
    let mut i0: libc::c_int = 0;
    let mut n0: libc::c_int = 0;
    let mut n1: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let mut n0init: libc::c_int = 0;
    let mut nbig: libc::c_int = 0;
    let mut emax: libc::c_double = 0.;
    let mut qmin: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    let mut diag_sum: libc::c_double = 0.;
    let mut tol: libc::c_double = 0.;
    let mut tol2: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut dmin: [libc::c_double; 3] = [0.0f64, 0.0f64, 0.0f64];
    let mut dn: [libc::c_double; 3] = [0.0f64, 0.0f64, 0.0f64];
    let mut sigma: libc::c_double = 0.;
    let mut tau: libc::c_double = 0.;
    let mut qvec: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut qvec1: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut evec: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut evec1: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut qvecp: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut qvec1p: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut evecp: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut evec1p: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut swap: *mut libc::c_double = 0 as *mut libc::c_double;
    qvec = work;
    qvec1 = work.offset((n * 1 as libc::c_int) as isize);
    evec = work.offset((n * 2 as libc::c_int) as isize);
    evec1 = work.offset((n * 3 as libc::c_int) as isize);
    i = 0 as libc::c_int;
    j = n - 1 as libc::c_int;
    while i < n - 1 as libc::c_int {
        temp = fabs(*diag.offset(i as isize));
        *qvec1.offset(i as isize) = 0.0f64;
        *evec1.offset(i as isize) = 0.0f64;
        *evec
            .offset(
                (j - 1 as libc::c_int) as isize,
            ) = *diag_off.offset(i as isize) * *diag_off.offset(i as isize) * temp;
        *qvec.offset(j as isize) = temp;
        i += 1;
        i;
        j -= 1;
        j;
    }
    *qvec
        .offset(
            0 as libc::c_int as isize,
        ) = fabs(*diag.offset((n - 1 as libc::c_int) as isize));
    *qvec1.offset((n - 1 as libc::c_int) as isize) = 0.0f64;
    *evec.offset((n - 1 as libc::c_int) as isize) = 0.0f64;
    *evec1.offset((n - 1 as libc::c_int) as isize) = 0.0f64;
    if *qvec.offset(0 as libc::c_int as isize)
        < *qvec.offset((n - 1 as libc::c_int) as isize) * 1.5f64
    {
        i = 0 as libc::c_int;
        j = n - 1 as libc::c_int;
        while i < n / 2 as libc::c_int {
            temp = *qvec.offset(i as isize);
            *qvec.offset(i as isize) = *qvec.offset(j as isize);
            *qvec.offset(j as isize) = temp;
            temp = *evec.offset(i as isize);
            *evec.offset(i as isize) = *evec.offset((j - 1 as libc::c_int) as isize);
            *evec.offset((j - 1 as libc::c_int) as isize) = temp;
            i += 1;
            i;
            j -= 1;
            j;
        }
    }
    diag_sum = *qvec.offset(0 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < n - 1 as libc::c_int {
        temp = diag_sum + *evec.offset(i as isize);
        *qvec1.offset(i as isize) = temp;
        *evec1
            .offset(
                i as isize,
            ) = *qvec.offset((i + 1 as libc::c_int) as isize)
            * (*evec.offset(i as isize) / temp);
        diag_sum = *qvec.offset((i + 1 as libc::c_int) as isize) * (diag_sum / temp);
        i += 1;
        i;
    }
    *qvec1.offset((n - 1 as libc::c_int) as isize) = diag_sum;
    diag_sum = *qvec1.offset(0 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < n - 1 as libc::c_int {
        temp = diag_sum + *evec1.offset(i as isize);
        *qvec.offset(i as isize) = temp;
        *evec
            .offset(
                i as isize,
            ) = *qvec1.offset((i + 1 as libc::c_int) as isize)
            * (*evec1.offset(i as isize) / temp);
        diag_sum = *qvec1.offset((i + 1 as libc::c_int) as isize) * (diag_sum / temp);
        i += 1;
        i;
    }
    *qvec.offset((n - 1 as libc::c_int) as isize) = diag_sum;
    n0 = n;
    tau = 0.0f64;
    itry = 0 as libc::c_int;
    while n0 > 0 as libc::c_int {
        if itry >= n {
            return 3 as libc::c_int;
        }
        sigma = -*evec.offset((n0 - 1 as libc::c_int) as isize);
        if sigma < 0.0f64 {
            return 1 as libc::c_int;
        }
        emax = 0.0f64;
        qmin = *qvec.offset((n0 - 1 as libc::c_int) as isize);
        i = n0 - 1 as libc::c_int;
        while i > 0 as libc::c_int {
            if *evec.offset((i - 1 as libc::c_int) as isize) <= 0.0f64 {
                break;
            }
            if qmin >= emax * 4.0f64 {
                qmin = if qmin < *qvec.offset(i as isize) {
                    qmin
                } else {
                    *qvec.offset(i as isize)
                };
                emax = if emax > *evec.offset((i - 1 as libc::c_int) as isize) {
                    emax
                } else {
                    *evec.offset((i - 1 as libc::c_int) as isize)
                };
            }
            i -= 1;
            i;
        }
        i0 = i;
        qvecp = qvec;
        qvec1p = qvec1;
        evecp = evec;
        evec1p = evec1;
        if qmin < 0 as libc::c_int as libc::c_double
            || emax < 0 as libc::c_int as libc::c_double
        {
            fprintf(
                stderr,
                b"dlasq2: qmin < 0 or emax < 0\0" as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
        dmin[0 as libc::c_int
            as usize] = -if 0.0f64 > qmin - 2.0f64 * sqrt(qmin * emax) {
            0.0f64
        } else {
            qmin - 2.0f64 * sqrt(qmin * emax)
        };
        nbig = (n0 - i0) * 10 as libc::c_int;
        iwhilb = 0 as libc::c_int;
        while iwhilb < nbig {
            n0init = n0;
            tol = 2.2204460492503131e-16f64 * 100.0f64;
            tol2 = tol * tol;
            while n0 > i0 {
                n1 = n0 - 1 as libc::c_int;
                n2 = n0 - 2 as libc::c_int;
                if n1 == i0
                    || *evecp.offset(n2 as isize)
                        < tol2 * (sigma + *qvecp.offset(n1 as isize))
                    || *evec1p.offset(n2 as isize) < tol2 * *qvecp.offset(n2 as isize)
                {
                    *qvec.offset(n1 as isize) = *qvecp.offset(n1 as isize) + sigma;
                    n0 -= 1;
                    n0;
                } else {
                    if !(n2 == i0
                        || *evecp.offset((n1 - 2 as libc::c_int) as isize) < tol2 * sigma
                        || *evec1p.offset((n1 - 2 as libc::c_int) as isize)
                            < tol2 * *qvecp.offset((n1 - 2 as libc::c_int) as isize))
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
                            * (*evecp.offset(n2 as isize) / (t + sqrt(t * (t + s))));
                        t = *qvecp.offset(n2 as isize)
                            + (s + *evecp.offset(n2 as isize));
                        *qvecp.offset(n1 as isize) *= *qvecp.offset(n2 as isize) / t;
                        *qvecp.offset(n2 as isize) = t;
                    }
                    *qvec.offset(n2 as isize) = *qvecp.offset(n2 as isize) + sigma;
                    *qvec.offset(n1 as isize) = *qvecp.offset(n1 as isize) + sigma;
                    n0 += -(2 as libc::c_int);
                }
            }
            if n0 <= i0 {
                break;
            }
            if dmin[0 as libc::c_int as usize] <= 0.0f64 {
                tau = -dmin[0 as libc::c_int as usize];
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
            iter = 0 as libc::c_int;
            while iter < 3 as libc::c_int {
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
                if dmin[0 as libc::c_int as usize] >= 0.0f64 {
                    break;
                }
                if dmin[1 as libc::c_int as usize] > 0.0f64 {
                    tau += dmin[0 as libc::c_int as usize];
                } else {
                    tau *= 0.25f64;
                }
                iter += 1;
                iter;
            }
            if dmin[0 as libc::c_int as usize] < 0 as libc::c_int as libc::c_double {
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
            iwhilb;
        }
        if iwhilb == nbig {
            fprintf(
                stderr,
                b"dlasq2: Maximum number of iterations exceeded\0" as *const u8
                    as *const libc::c_char,
            );
            return 2 as libc::c_int;
        }
        itry += 1;
        itry;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _compute_eigenvalues(
    mut n: libc::c_int,
    mut diag: *mut libc::c_double,
    mut diag_off1: *mut libc::c_double,
    mut w: *mut libc::c_double,
    mut werr: *mut libc::c_double,
    mut wgap: *mut libc::c_double,
    mut work: *mut libc::c_double,
) -> libc::c_int {
    let mut gl: libc::c_double = 0.;
    let mut gu: libc::c_double = 0.;
    let mut eabs: libc::c_double = 0.;
    let mut eold: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut tmp1: libc::c_double = 0.;
    let mut dmax: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    let mut rtol: libc::c_double = 0.;
    let mut rtl: libc::c_double = 0.;
    let mut sigma: libc::c_double = 0.;
    let mut tau: libc::c_double = 0.;
    let mut sgndef: libc::c_double = 0.;
    let mut spectral_diameter: libc::c_double = 0.;
    let mut isleft: libc::c_double = 0.;
    let mut isrght: libc::c_double = 0.;
    let mut dpivot: libc::c_double = 0.;
    let mut idum: libc::c_int = 0;
    let mut ip: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut lcnt: libc::c_int = 0;
    let mut rcnt: libc::c_int = 0;
    let mut norep: libc::c_int = 0;
    let mut iinfo: libc::c_int = 0;
    if n <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if n == 1 as libc::c_int {
        *w.offset(0 as libc::c_int as isize) = *diag.offset(0 as libc::c_int as isize);
        *werr.offset(0 as libc::c_int as isize) = 0.0f64;
        *wgap.offset(0 as libc::c_int as isize) = 0.0f64;
        *diag_off1.offset(0 as libc::c_int as isize) = 0.0f64;
        return 0 as libc::c_int;
    }
    eps = 2 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64;
    rtl = 2.1e-8f64;
    rtol = 16.0f64 * 2.2204460492503131e-16f64;
    gl = *diag.offset(0 as libc::c_int as isize);
    gu = *diag.offset(0 as libc::c_int as isize);
    eold = 0.0f64;
    *diag_off1.offset((n - 1 as libc::c_int) as isize) = 0.0f64;
    i = 0 as libc::c_int;
    while i < n {
        *werr.offset(i as isize) = 0.0f64;
        *wgap.offset(i as isize) = 0.0f64;
        eabs = fabs(*diag_off1.offset(i as isize));
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
        i;
    }
    spectral_diameter = gu - gl;
    *diag_off1.offset((n - 1 as libc::c_int) as isize) = 0.0f64;
    if n == 1 as libc::c_int {
        *w.offset(0 as libc::c_int as isize) = *diag.offset(0 as libc::c_int as isize);
        *werr.offset(0 as libc::c_int as isize) = 0.0f64;
        *wgap.offset(0 as libc::c_int as isize) = 0.0f64;
        return 0 as libc::c_int;
    }
    let mut e2: *mut libc::c_double = work;
    work = work.offset(n as isize);
    i = 0 as libc::c_int;
    while i < n - 1 as libc::c_int {
        *e2
            .offset(
                i as isize,
            ) = *diag_off1.offset(i as isize) * *diag_off1.offset(i as isize);
        i += 1;
        i;
    }
    iinfo = _dlarrk(n, 1 as libc::c_int, gl, gu, diag, e2, rtl, &mut tmp, &mut tmp1);
    if iinfo != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    isleft = if gl > tmp - tmp1 - eps * 100.0f64 * fabs(tmp - tmp1) {
        gl
    } else {
        tmp - tmp1 - eps * 100.0f64 * fabs(tmp - tmp1)
    };
    iinfo = _dlarrk(n, n, gl, gu, diag, e2, rtl, &mut tmp, &mut tmp1);
    if iinfo != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    isrght = if gu < tmp + tmp1 + eps * 100.0f64 * fabs(tmp + tmp1) {
        gu
    } else {
        tmp + tmp1 + eps * 100.0f64 * fabs(tmp + tmp1)
    };
    spectral_diameter = isrght - isleft;
    if n > 1 as libc::c_int {
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
    if n == 1 as libc::c_int {
        sigma = gl;
        sgndef = 1.0f64;
    } else if lcnt - 1 as libc::c_int >= n - rcnt {
        sigma = if isleft > gl { isleft } else { gl };
        sgndef = 1.0f64;
    } else {
        sigma = if isrght < gu { isrght } else { gu };
        sgndef = -1.0f64;
    }
    tau = (if spectral_diameter * n as libc::c_double > 2.0f64 * fabs(sigma) {
        spectral_diameter * n as libc::c_double
    } else {
        2.0f64 * fabs(sigma)
    }) * eps;
    idum = 0 as libc::c_int;
    while idum < 6 as libc::c_int {
        dpivot = *diag.offset(0 as libc::c_int as isize) - sigma;
        *work.offset(0 as libc::c_int as isize) = dpivot;
        dmax = fabs(*work.offset(0 as libc::c_int as isize));
        i = 0 as libc::c_int;
        while i < n - 1 as libc::c_int {
            *work
                .offset(
                    (n * 2 as libc::c_int + i) as isize,
                ) = 1.0f64 / *work.offset(i as isize);
            tmp = *diag_off1.offset(i as isize)
                * *work.offset((n * 2 as libc::c_int + i) as isize);
            *work.offset((n + i) as isize) = tmp;
            dpivot = *diag.offset((i + 1 as libc::c_int) as isize) - sigma
                - tmp * *diag_off1.offset(i as isize);
            *work.offset((i + 1 as libc::c_int) as isize) = dpivot;
            dmax = if dmax > fabs(dpivot) { dmax } else { fabs(dpivot) };
            i += 1;
            i;
        }
        norep = (dmax > spectral_diameter * 64.0f64) as libc::c_int;
        if norep == 0 {
            i = 0 as libc::c_int;
            while i < n {
                tmp = sgndef * *work.offset(i as isize);
                if tmp < 0.0f64 {
                    norep = 1 as libc::c_int;
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
        }
        if !(norep != 0) {
            break;
        }
        if idum == 6 as libc::c_int - 1 as libc::c_int {
            if sgndef == 1.0f64 {
                sigma = gl - spectral_diameter * 2.0f64 * eps * n as libc::c_double;
            } else {
                sigma = gu + spectral_diameter * 2.0f64 * eps * n as libc::c_double;
            }
        } else if idum == 6 as libc::c_int {
            return -(2 as libc::c_int)
        } else {
            sigma -= sgndef * tau;
            tau *= 2.0f64;
        }
        idum += 1;
        idum;
    }
    *diag_off1.offset((n - 1 as libc::c_int) as isize) = sigma;
    ip = 0 as libc::c_int;
    while ip < n {
        *diag.offset(ip as isize) = *work.offset(ip as isize);
        ip += 1;
        ip;
    }
    ip = 0 as libc::c_int;
    while ip < n - 1 as libc::c_int {
        *diag_off1.offset(ip as isize) = *work.offset((n + ip) as isize);
        ip += 1;
        ip;
    }
    iinfo = _dlasq2(n, work, diag, diag_off1);
    if iinfo != 0 as libc::c_int {
        return -(5 as libc::c_int)
    } else {
        i = 0 as libc::c_int;
        while i < n {
            if *work.offset(i as isize) < 0.0f64 {
                fprintf(
                    stderr,
                    b"dlarre: negative eigenvalues\n\0" as *const u8
                        as *const libc::c_char,
                );
                return -(6 as libc::c_int);
            }
            i += 1;
            i;
        }
    }
    if sgndef > 0.0f64 {
        i = 0 as libc::c_int;
        while i < n {
            *w.offset(i as isize) = *work.offset((n - 1 as libc::c_int - i) as isize);
            i += 1;
            i;
        }
    } else {
        i = 0 as libc::c_int;
        while i < n {
            *w.offset(i as isize) = -*work.offset(i as isize);
            i += 1;
            i;
        }
    }
    i = 0 as libc::c_int;
    while i < n {
        *werr.offset(i as isize) = rtol * fabs(*w.offset(i as isize));
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < n - 1 as libc::c_int {
        *wgap
            .offset(
                i as isize,
            ) = if 0.0f64
            > *w.offset((i + 1 as libc::c_int) as isize)
                - *werr.offset((i + 1 as libc::c_int) as isize)
                - (*w.offset(i as isize) + *werr.offset(i as isize))
        {
            0.0f64
        } else {
            *w.offset((i + 1 as libc::c_int) as isize)
                - *werr.offset((i + 1 as libc::c_int) as isize)
                - (*w.offset(i as isize) + *werr.offset(i as isize))
        };
        i += 1;
        i;
    }
    *wgap
        .offset(
            -(1 as libc::c_int) as isize,
        ) = if 0.0f64
        > *w.offset(0 as libc::c_int as isize) - *werr.offset(0 as libc::c_int as isize)
            - gl
    {
        0.0f64
    } else {
        *w.offset(0 as libc::c_int as isize) - *werr.offset(0 as libc::c_int as isize)
            - gl
    };
    *wgap
        .offset(
            (n - 1 as libc::c_int) as isize,
        ) = if 0.0f64
        > gu - sigma
            - (*w.offset((n - 1 as libc::c_int) as isize)
                + *werr.offset((n - 1 as libc::c_int) as isize))
    {
        0.0f64
    } else {
        gu - sigma
            - (*w.offset((n - 1 as libc::c_int) as isize)
                + *werr.offset((n - 1 as libc::c_int) as isize))
    };
    return 0 as libc::c_int;
}
unsafe extern "C" fn _dlarrf(
    mut n: libc::c_int,
    mut diag: *mut libc::c_double,
    mut diag_off1: *mut libc::c_double,
    mut ld: *mut libc::c_double,
    mut clstrt: libc::c_int,
    mut w: *mut libc::c_double,
    mut wgap: *mut libc::c_double,
    mut werr: *mut libc::c_double,
    mut clgapl: libc::c_double,
    mut sigma: *mut libc::c_double,
    mut dplus: *mut libc::c_double,
    mut lplus: *mut libc::c_double,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ktry: libc::c_int = 0;
    let mut s: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut max1: libc::c_double = 0.;
    let mut growthbound: libc::c_double = 0.;
    let mut lsigma: libc::c_double = 0.;
    lsigma = *w.offset(clstrt as isize) - *werr.offset(clstrt as isize);
    lsigma -= fabs(lsigma) * 4.0f64 * 2.2204460492503131e-16f64;
    growthbound = *diag.offset(0 as libc::c_int as isize) * 8.0f64;
    ktry = 0 as libc::c_int;
    while ktry < 2 as libc::c_int {
        s = -lsigma;
        *dplus
            .offset(
                0 as libc::c_int as isize,
            ) = *diag.offset(0 as libc::c_int as isize) + s;
        max1 = fabs(*dplus.offset(0 as libc::c_int as isize));
        i = 0 as libc::c_int;
        while i < n - 1 as libc::c_int {
            tmp = *ld.offset(i as isize) / *dplus.offset(i as isize);
            *lplus.offset(i as isize) = tmp;
            s = s * tmp * *diag_off1.offset(i as isize) - lsigma;
            *dplus
                .offset(
                    (i + 1 as libc::c_int) as isize,
                ) = *diag.offset((i + 1 as libc::c_int) as isize) + s;
            max1 = if max1 > fabs(*dplus.offset((i + 1 as libc::c_int) as isize)) {
                max1
            } else {
                fabs(*dplus.offset((i + 1 as libc::c_int) as isize))
            };
            i += 1;
            i;
        }
        *sigma = lsigma;
        if max1 <= growthbound {
            return 0 as libc::c_int;
        }
        lsigma = lsigma
            - (if clgapl * 0.25f64 < *wgap.offset(clstrt as isize) {
                clgapl * 0.25f64
            } else {
                *wgap.offset(clstrt as isize)
            });
        ktry += 1;
        ktry;
    }
    if max1 > 1e16f64 {
        fprintf(stderr, b"dlarrf max1 = %g\0" as *const u8 as *const libc::c_char, max1);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _dlaneg(
    mut n: libc::c_int,
    mut diag: *mut libc::c_double,
    mut lld: *mut libc::c_double,
    mut sigma: libc::c_double,
    mut twist_index: libc::c_int,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut negcnt: libc::c_int = 0;
    let mut p: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut dplus: libc::c_double = 0.;
    let mut dminus: libc::c_double = 0.;
    negcnt = 0 as libc::c_int;
    t = -sigma;
    j = 0 as libc::c_int;
    while j < twist_index - 1 as libc::c_int {
        dplus = *diag.offset(j as isize) + t;
        if dplus < 0.0f64 {
            negcnt += 1;
            negcnt;
        }
        t = t / dplus * *lld.offset(j as isize) - sigma;
        j += 1;
        j;
    }
    p = *diag.offset((n - 1 as libc::c_int) as isize) - sigma;
    j = n - 2 as libc::c_int;
    while j >= twist_index - 1 as libc::c_int {
        dminus = *lld.offset(j as isize) + p;
        if dminus < 0.0f64 {
            negcnt += 1;
            negcnt;
        }
        p = p / dminus * *diag.offset(j as isize) - sigma;
        j -= 1;
        j;
    }
    if t + sigma + p < 0.0f64 {
        negcnt += 1;
        negcnt;
    }
    return negcnt;
}
unsafe extern "C" fn _dlarrb(
    mut n: libc::c_int,
    mut diag: *mut libc::c_double,
    mut lld: *mut libc::c_double,
    mut ifirst: libc::c_int,
    mut ilast: libc::c_int,
    mut rtol1: libc::c_double,
    mut rtol2: libc::c_double,
    mut w: *mut libc::c_double,
    mut wgap: *mut libc::c_double,
    mut werr: *mut libc::c_double,
    mut twist_index: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut iter: libc::c_int = 0;
    let mut negcnt: libc::c_int = 0;
    let mut mid: libc::c_double = 0.;
    let mut back: libc::c_double = 0.;
    let mut left: libc::c_double = 0.;
    let mut right: libc::c_double = 0.;
    let mut width: libc::c_double = 0.;
    let mut cvrgd: libc::c_double = if rtol1 * *wgap.offset(ifirst as isize)
        > rtol2
            * (if fabs(*w.offset(ifirst as isize))
                > fabs(*w.offset((ilast - 1 as libc::c_int) as isize))
            {
                fabs(*w.offset(ifirst as isize))
            } else {
                fabs(*w.offset((ilast - 1 as libc::c_int) as isize))
            })
    {
        rtol1 * *wgap.offset(ifirst as isize)
    } else {
        rtol2
            * (if fabs(*w.offset(ifirst as isize))
                > fabs(*w.offset((ilast - 1 as libc::c_int) as isize))
            {
                fabs(*w.offset(ifirst as isize))
            } else {
                fabs(*w.offset((ilast - 1 as libc::c_int) as isize))
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
            iter = 0 as libc::c_int;
            while iter < 1000 as libc::c_int {
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
                iter;
            }
            *w.offset(i as isize) = mid;
            *werr.offset(i as isize) = width;
        }
        i += 1;
        i;
    }
    i = ifirst;
    while i < ilast - 1 as libc::c_int {
        *wgap
            .offset(
                i as isize,
            ) = if 0.0f64
            > *w.offset((i + 1 as libc::c_int) as isize)
                - *werr.offset((i + 1 as libc::c_int) as isize) - *w.offset(i as isize)
                - *werr.offset(i as isize)
        {
            0.0f64
        } else {
            *w.offset((i + 1 as libc::c_int) as isize)
                - *werr.offset((i + 1 as libc::c_int) as isize) - *w.offset(i as isize)
                - *werr.offset(i as isize)
        };
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _dlar1v(
    mut n: libc::c_int,
    mut lambda: libc::c_double,
    mut diag: *mut libc::c_double,
    mut diag_off1: *mut libc::c_double,
    mut ld: *mut libc::c_double,
    mut lld: *mut libc::c_double,
    mut gaptol: libc::c_double,
    mut vec: *mut libc::c_double,
    mut negcnt: *mut libc::c_int,
    mut twist_index: *mut libc::c_int,
    mut resid: *mut libc::c_double,
    mut rqcorr: *mut libc::c_double,
    mut work: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut r1: libc::c_int = 0;
    let mut r2: libc::c_int = 0;
    let mut neg1: libc::c_int = 0;
    let mut neg2: libc::c_int = 0;
    let mut s: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut nrminv: libc::c_double = 0.;
    let mut mingma: libc::c_double = 0.;
    let mut dplus: libc::c_double = 0.;
    let mut dminus: libc::c_double = 0.;
    let mut lplus: *mut libc::c_double = work;
    let mut uminus: *mut libc::c_double = work.offset(n as isize);
    let mut work_p: *mut libc::c_double = work.offset((n * 2 as libc::c_int) as isize);
    if *twist_index == -(1 as libc::c_int) {
        r1 = 0 as libc::c_int;
        r2 = n;
        *twist_index = 0 as libc::c_int;
    } else {
        r1 = *twist_index;
        r2 = *twist_index + 1 as libc::c_int;
    }
    neg2 = 0 as libc::c_int;
    s = *diag.offset((n - 1 as libc::c_int) as isize) - lambda;
    *work_p.offset((n - 1 as libc::c_int) as isize) = s;
    i = n - 2 as libc::c_int;
    while i >= r1 {
        dminus = *lld.offset(i as isize) + s;
        if dminus < 0.0f64 {
            neg2 += 1;
            neg2;
        }
        tmp = *diag.offset(i as isize) / dminus;
        *uminus.offset(i as isize) = *diag_off1.offset(i as isize) * tmp;
        s = s * tmp - lambda;
        *work_p.offset(i as isize) = s;
        i -= 1;
        i;
    }
    neg1 = 0 as libc::c_int;
    s = -lambda;
    i = 0 as libc::c_int;
    while i < r1 {
        dplus = *diag.offset(i as isize) + s;
        if dplus < 0.0f64 {
            neg1 += 1;
            neg1;
        }
        *lplus.offset(i as isize) = *ld.offset(i as isize) / dplus;
        s = s * *lplus.offset(i as isize) * *diag_off1.offset(i as isize) - lambda;
        i += 1;
        i;
    }
    mingma = s + lambda + *work_p.offset(r1 as isize);
    if mingma < 0.0f64 {
        neg1 += 1;
        neg1;
    }
    *negcnt = neg1 + neg2;
    i = r1;
    while i < r2 - 1 as libc::c_int {
        dplus = *diag.offset(i as isize) + s;
        *lplus.offset(i as isize) = *ld.offset(i as isize) / dplus;
        tmp = s * *lplus.offset(i as isize) * *diag_off1.offset(i as isize);
        s = tmp - lambda;
        tmp = tmp + *work_p.offset((i + 1 as libc::c_int) as isize);
        if fabs(tmp) <= fabs(mingma) {
            mingma = tmp;
            *twist_index = i + 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    *vec.offset(*twist_index as isize) = 1.0f64;
    let mut ztz: libc::c_double = 1.0f64;
    i = *twist_index - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        tmp = -(*lplus.offset(i as isize)
            * *vec.offset((i + 1 as libc::c_int) as isize));
        ztz += tmp * tmp;
        *vec.offset(i as isize) = tmp;
        i -= 1;
        i;
    }
    i = *twist_index;
    while i < n - 1 as libc::c_int {
        tmp = -(*uminus.offset(i as isize) * *vec.offset(i as isize));
        ztz += tmp * tmp;
        *vec.offset((i + 1 as libc::c_int) as isize) = tmp;
        i += 1;
        i;
    }
    tmp = 1.0f64 / ztz;
    nrminv = sqrt(tmp);
    i = 0 as libc::c_int;
    while i < n {
        *vec.offset(i as isize) *= nrminv;
        i += 1;
        i;
    }
    *resid = fabs(mingma) * nrminv;
    *rqcorr = mingma * tmp;
}
unsafe extern "C" fn _compute_eigenvectors(
    mut n: libc::c_int,
    mut diag: *mut libc::c_double,
    mut diag_off1: *mut libc::c_double,
    mut w: *mut libc::c_double,
    mut werr: *mut libc::c_double,
    mut wgap: *mut libc::c_double,
    mut vec: *mut libc::c_double,
    mut work: *mut libc::c_double,
    mut iwork: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut icls: libc::c_int = 0;
    let mut iter: libc::c_int = 0;
    let mut idone: libc::c_int = 0;
    let mut ndepth: libc::c_int = 0;
    let mut ncluster: libc::c_int = 0;
    let mut ncluster1: libc::c_int = 0;
    let mut negcnt: libc::c_int = 0;
    let mut oldfst: libc::c_int = 0;
    let mut oldlst: libc::c_int = 0;
    let mut newfst: libc::c_int = 0;
    let mut newlst: libc::c_int = 0;
    let mut needbs: libc::c_int = 0;
    let mut iinfo: libc::c_int = 0;
    let mut fudge: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    let mut rqtol: libc::c_double = 0.;
    let mut tol: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut left: libc::c_double = 0.;
    let mut right: libc::c_double = 0.;
    let mut gap: libc::c_double = 0.;
    let mut bstw: libc::c_double = 0.;
    let mut savgap: libc::c_double = 0.;
    let mut gaptol: libc::c_double = 0.;
    let mut sigma: libc::c_double = 0.;
    let mut tau: libc::c_double = 0.;
    let mut resid: libc::c_double = 0.;
    let mut lambda: libc::c_double = 0.;
    let mut bstres: libc::c_double = 0.;
    let mut rqcorr: libc::c_double = 0.;
    let mut resid_tol: libc::c_double = 0.;
    let mut rqcorr_tol: libc::c_double = 0.;
    let mut buf_w: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut buf_ld: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut buf_lld: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut buf_wrk: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut twist_indices: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut swap: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut old_cluster_range: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut new_cluster_range: *mut libc::c_int = 0 as *mut libc::c_int;
    if n <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    buf_w = work;
    buf_ld = work.offset(n as isize);
    buf_lld = work.offset((n * 2 as libc::c_int) as isize);
    buf_wrk = work.offset((n * 3 as libc::c_int) as isize);
    i = 0 as libc::c_int;
    while i < n * 6 as libc::c_int {
        *work.offset(i as isize) = 0.0f64;
        i += 1;
        i;
    }
    twist_indices = iwork;
    i = 0 as libc::c_int;
    while i < n {
        *twist_indices.offset(i as isize) = 0 as libc::c_int;
        i += 1;
        i;
    }
    old_cluster_range = iwork.offset(n as isize);
    new_cluster_range = iwork.offset((n * 3 as libc::c_int) as isize);
    eps = 2.2204460492503131e-16f64;
    rqtol = 2.2204460492503131e-16f64 * 2.0f64;
    tol = 2.2204460492503131e-16f64 * 8 as libc::c_int as libc::c_double;
    sigma = *diag_off1.offset((n - 1 as libc::c_int) as isize);
    if 1 as libc::c_int == n {
        *vec.offset(0 as libc::c_int as isize) = 1.0f64;
        *w.offset(0 as libc::c_int as isize) += sigma;
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < n {
        *buf_w.offset(i as isize) = *w.offset(i as isize);
        *w.offset(i as isize) += sigma;
        i += 1;
        i;
    }
    ncluster = 1 as libc::c_int;
    *old_cluster_range.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    *old_cluster_range.offset(1 as libc::c_int as isize) = n;
    idone = 0 as libc::c_int;
    ndepth = 0 as libc::c_int;
    while ndepth < n {
        if idone == n {
            break;
        }
        ncluster1 = ncluster;
        ncluster = 0 as libc::c_int;
        icls = 0 as libc::c_int;
        while icls < ncluster1 {
            oldfst = *old_cluster_range.offset((icls * 2 as libc::c_int) as isize);
            oldlst = *old_cluster_range
                .offset((icls * 2 as libc::c_int + 1 as libc::c_int) as isize);
            if ndepth > 0 as libc::c_int {
                i = 0 as libc::c_int;
                while i < n {
                    *diag.offset(i as isize) = *vec.offset((oldfst * n + i) as isize);
                    i += 1;
                    i;
                }
                i = 0 as libc::c_int;
                while i < n - 1 as libc::c_int {
                    *diag_off1
                        .offset(
                            i as isize,
                        ) = *vec.offset(((oldfst + 1 as libc::c_int) * n + i) as isize);
                    i += 1;
                    i;
                }
                sigma = *vec
                    .offset(
                        ((oldfst + 2 as libc::c_int) * n - 1 as libc::c_int) as isize,
                    );
            }
            j = 0 as libc::c_int;
            while j < n - 1 as libc::c_int {
                tmp = *diag.offset(j as isize) * *diag_off1.offset(j as isize);
                *buf_ld.offset(j as isize) = tmp;
                *buf_lld.offset(j as isize) = tmp * *diag_off1.offset(j as isize);
                j += 1;
                j;
            }
            newfst = oldfst;
            newlst = oldfst + 1 as libc::c_int;
            while newlst <= oldlst {
                if !(newlst < oldlst
                    && *wgap.offset((newlst - 1 as libc::c_int) as isize)
                        < 0.001f64
                            * fabs(*buf_w.offset((newlst - 1 as libc::c_int) as isize)))
                {
                    if newlst - newfst > 1 as libc::c_int {
                        _dlarrb(
                            n,
                            diag,
                            buf_lld,
                            newfst,
                            newfst + 1 as libc::c_int,
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
                            newlst - 1 as libc::c_int,
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
                            *wgap.offset((newfst - 1 as libc::c_int) as isize),
                            &mut tau,
                            vec.offset((newfst * n) as isize),
                            vec.offset(((newfst + 1 as libc::c_int) * n) as isize),
                        );
                        if iinfo != 0 as libc::c_int {
                            return -(2 as libc::c_int);
                        }
                        *vec
                            .offset(
                                ((newfst + 2 as libc::c_int) * n - 1 as libc::c_int)
                                    as isize,
                            ) = sigma + tau;
                        k = newfst;
                        while k < newlst {
                            fudge = eps * 3.0f64 * fabs(*buf_w.offset(k as isize));
                            *buf_w.offset(k as isize) -= tau;
                            fudge += eps * 4.0f64 * fabs(*buf_w.offset(k as isize));
                            *werr.offset(k as isize) += fudge;
                            k += 1;
                            k;
                        }
                        *new_cluster_range
                            .offset((ncluster * 2 as libc::c_int) as isize) = newfst;
                        *new_cluster_range
                            .offset(
                                (ncluster * 2 as libc::c_int + 1 as libc::c_int) as isize,
                            ) = newlst;
                        ncluster += 1;
                        ncluster;
                    } else {
                        lambda = *buf_w.offset(newfst as isize);
                        left = lambda - *werr.offset(newfst as isize);
                        right = lambda + *werr.offset(newfst as isize);
                        gap = *wgap.offset(newfst as isize);
                        if newfst == 0 as libc::c_int || newfst + 1 as libc::c_int == n {
                            gaptol = 0 as libc::c_int as libc::c_double;
                        } else {
                            gaptol = gap * eps;
                        }
                        savgap = gap;
                        resid_tol = tol * gap;
                        rqcorr_tol = rqtol * fabs(lambda);
                        needbs = 0 as libc::c_int;
                        bstres = 1e307f64;
                        bstw = 0 as libc::c_int as libc::c_double;
                        *twist_indices.offset(newfst as isize) = -(1 as libc::c_int);
                        iter = 0 as libc::c_int;
                        while iter < 6 as libc::c_int {
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
                            if resid < resid_tol || fabs(rqcorr) < rqcorr_tol {
                                break;
                            }
                            if lambda + rqcorr > right || lambda + rqcorr < left {
                                needbs = 1 as libc::c_int;
                                break;
                            } else {
                                if newfst < negcnt {
                                    if rqcorr > 0 as libc::c_int as libc::c_double {
                                        needbs = 1 as libc::c_int;
                                        break;
                                    } else {
                                        right = lambda;
                                    }
                                } else if rqcorr < 0 as libc::c_int as libc::c_double {
                                    needbs = 1 as libc::c_int;
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
                                    iter;
                                }
                            }
                        }
                        if needbs != 0 {
                            _dlarrb(
                                n,
                                diag,
                                buf_lld,
                                newfst,
                                newfst + 1 as libc::c_int,
                                0.0f64,
                                eps * 2.0f64,
                                buf_w,
                                wgap,
                                werr,
                                *twist_indices.offset(newfst as isize) + 1 as libc::c_int,
                            );
                            lambda = *buf_w.offset(newfst as isize);
                            *twist_indices.offset(newfst as isize) = -(1 as libc::c_int);
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
                        if newfst > 0 as libc::c_int {
                            *wgap
                                .offset(
                                    (newfst - 1 as libc::c_int) as isize,
                                ) = if *wgap.offset((newfst - 1 as libc::c_int) as isize)
                                > *w.offset(newfst as isize) - *werr.offset(newfst as isize)
                                    - *w.offset((newfst - 1 as libc::c_int) as isize)
                                    - *werr.offset((newfst - 1 as libc::c_int) as isize)
                            {
                                *wgap.offset((newfst - 1 as libc::c_int) as isize)
                            } else {
                                *w.offset(newfst as isize) - *werr.offset(newfst as isize)
                                    - *w.offset((newfst - 1 as libc::c_int) as isize)
                                    - *werr.offset((newfst - 1 as libc::c_int) as isize)
                            };
                        }
                        if newfst < n - 1 as libc::c_int {
                            *wgap
                                .offset(
                                    newfst as isize,
                                ) = if savgap
                                > *w.offset((newfst + 1 as libc::c_int) as isize)
                                    - *werr.offset((newfst + 1 as libc::c_int) as isize)
                                    - *w.offset(newfst as isize) - *werr.offset(newfst as isize)
                            {
                                savgap
                            } else {
                                *w.offset((newfst + 1 as libc::c_int) as isize)
                                    - *werr.offset((newfst + 1 as libc::c_int) as isize)
                                    - *w.offset(newfst as isize) - *werr.offset(newfst as isize)
                            };
                        }
                        idone += 1;
                        idone;
                    }
                    newfst = newlst;
                }
                newlst += 1;
                newlst;
            }
            icls += 1;
            icls;
        }
        swap = old_cluster_range;
        old_cluster_range = new_cluster_range;
        new_cluster_range = swap;
        ndepth += 1;
        ndepth;
    }
    if idone < n {
        return -(2 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _dlaev2(
    mut eig: *mut libc::c_double,
    mut vec: *mut libc::c_double,
    mut diag: *mut libc::c_double,
    mut diag_off1: *mut libc::c_double,
) -> libc::c_int {
    let mut a: libc::c_double = *diag.offset(0 as libc::c_int as isize);
    let mut b: libc::c_double = *diag_off1.offset(0 as libc::c_int as isize);
    let mut c: libc::c_double = *diag.offset(1 as libc::c_int as isize);
    let mut df: libc::c_double = 0.;
    let mut cs: libc::c_double = 0.;
    let mut ct: libc::c_double = 0.;
    let mut tb: libc::c_double = 0.;
    let mut sm: libc::c_double = 0.;
    let mut tn: libc::c_double = 0.;
    let mut rt: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut rt1: libc::c_double = 0.;
    let mut rt2: libc::c_double = 0.;
    let mut cs1: libc::c_double = 0.;
    let mut sn1: libc::c_double = 0.;
    let mut sgn1: libc::c_int = 0;
    let mut sgn2: libc::c_int = 0;
    sm = a + c;
    df = a - c;
    tb = b + b;
    rt = sqrt(tb * tb + df * df);
    if sm > 0.0f64 {
        rt1 = (sm + rt) * 0.5f64;
        sgn1 = 1 as libc::c_int;
        rt2 = (a * c - b * b) / rt1;
    } else if sm < 0.0f64 {
        rt1 = (sm - rt) * 0.5f64;
        sgn1 = -(1 as libc::c_int);
        rt2 = (a * c - b * b) / rt1;
    } else {
        rt1 = rt * 0.5f64;
        rt2 = rt * -0.5f64;
        sgn1 = 1 as libc::c_int;
    }
    if df >= 0.0f64 {
        cs = df + rt;
        sgn2 = 1 as libc::c_int;
    } else {
        cs = df - rt;
        sgn2 = -(1 as libc::c_int);
    }
    if fabs(cs) > fabs(tb) {
        ct = -tb / cs;
        sn1 = 1.0f64 / sqrt(ct * ct + 1.0f64);
        cs1 = ct * sn1;
    } else if b == 0.0f64 {
        cs1 = 1.0f64;
        sn1 = 0.0f64;
    } else {
        tn = -cs / tb;
        cs1 = 1.0f64 / sqrt(tn * tn + 1.0f64);
        sn1 = tn * cs1;
    }
    if sgn1 == sgn2 {
        tmp = cs1;
        cs1 = -sn1;
        sn1 = tmp;
    }
    *eig.offset(0 as libc::c_int as isize) = rt2;
    *eig.offset(1 as libc::c_int as isize) = rt1;
    *vec.offset(0 as libc::c_int as isize) = -sn1;
    *vec.offset(1 as libc::c_int as isize) = cs1;
    *vec.offset(2 as libc::c_int as isize) = cs1;
    *vec.offset(3 as libc::c_int as isize) = sn1;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _CINTdiagonalize(
    mut n: libc::c_int,
    mut diag: *mut libc::c_double,
    mut diag_off1: *mut libc::c_double,
    mut eig: *mut libc::c_double,
    mut vec: *mut libc::c_double,
) -> libc::c_int {
    if n == 0 as libc::c_int {
        return 0 as libc::c_int
    } else if n == 1 as libc::c_int {
        *eig.offset(0 as libc::c_int as isize) = *diag.offset(0 as libc::c_int as isize);
        *vec.offset(0 as libc::c_int as isize) = 1.0f64;
        return 0 as libc::c_int;
    } else if n == 2 as libc::c_int {
        return _dlaev2(eig, vec, diag, diag_off1)
    }
    let mut iwork: [libc::c_int; 160] = [0; 160];
    let mut work: [libc::c_double; 289] = [0.; 289];
    let mut buf_err: *mut libc::c_double = work.as_mut_ptr().offset(n as isize);
    let mut buf_gp: *mut libc::c_double = work
        .as_mut_ptr()
        .offset((n * 2 as libc::c_int) as isize)
        .offset(1 as libc::c_int as isize);
    let mut buf_wrk: *mut libc::c_double = work
        .as_mut_ptr()
        .offset((n * 3 as libc::c_int) as isize)
        .offset(1 as libc::c_int as isize);
    let mut info: libc::c_int = 0;
    info = _compute_eigenvalues(n, diag, diag_off1, eig, buf_err, buf_gp, buf_wrk);
    if info == 0 as libc::c_int {
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
