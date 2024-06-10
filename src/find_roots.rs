#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

// extern "C" {
    // pub type _IO_wide_data;
    // pub type _IO_codecvt;
    // pub type _IO_marker;
    // static mut stderr: *mut FILE;
    // fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> i32;
    // fn sqrt(_: f64) -> f64;
    // fn fabs(_: f64) -> f64;
// }
// pub type size_t = libc::c_ulong;
// pub type __off_t = libc::c_long;
// pub type __off64_t = libc::c_long;

// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct _IO_FILE {
//     pub _flags: i32,
//     pub _IO_read_ptr: *mut libc::c_char,
//     pub _IO_read_end: *mut libc::c_char,
//     pub _IO_read_base: *mut libc::c_char,
//     pub _IO_write_base: *mut libc::c_char,
//     pub _IO_write_ptr: *mut libc::c_char,
//     pub _IO_write_end: *mut libc::c_char,
//     pub _IO_buf_base: *mut libc::c_char,
//     pub _IO_buf_end: *mut libc::c_char,
//     pub _IO_save_base: *mut libc::c_char,
//     pub _IO_backup_base: *mut libc::c_char,
//     pub _IO_save_end: *mut libc::c_char,
//     pub _markers: *mut _IO_marker,
//     pub _chain: *mut _IO_FILE,
//     pub _fileno: i32,
//     pub _flags2: i32,
//     pub _old_offset: __off_t,
//     pub _cur_column: libc::c_ushort,
//     pub _vtable_offset: libc::c_schar,
//     pub _shortbuf: [libc::c_char; 1],
//     pub _lock: *mut libc::c_void,
//     pub _offset: __off64_t,
//     pub _codecvt: *mut _IO_codecvt,
//     pub _wide_data: *mut _IO_wide_data,
//     pub _freeres_list: *mut _IO_FILE,
//     pub _freeres_buf: *mut libc::c_void,
//     pub __pad5: size_t,
//     pub _mode: i32,
//     pub _unused2: [libc::c_char; 20],
// }
// pub type _IO_lock_t = ();
// pub type FILE = _IO_FILE;
unsafe extern "C" fn R_dnode(
    mut a: *mut f64,
    mut roots: *mut f64,
    mut order: i32,
) -> i32 {
    let accrt: f64 = 1e-15f64;
    let mut x0: f64 = 0.;
    let mut x1: f64 = 0.;
    let mut xi: f64 = 0.;
    let mut x1init: f64 = 0.;
    let mut p0: f64 = 0.;
    let mut p1: f64 = 0.;
    let mut pi: f64 = 0.;
    let mut p1init: f64 = 0.;
    let mut i: i32 = 0;
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    x1init = 0 as i32 as f64;
    p1init = *a.offset(0 as i32 as isize);
    m = 0 as i32;
    while m < order {
        x0 = x1init;
        p0 = p1init;
        x1init = *roots.offset(m as isize);
        p1init = *a.offset(order as isize);
        i = 1 as i32;
        while i <= order {
            p1init = p1init * x1init + *a.offset((order - i) as isize);
            i += 1;
        }
        if !(p1init == 0 as i32 as f64) {
            if p0 * p1init > 0 as i32 as f64 {
                eprintln!("ROOT NUMBER {} WAS NOT FOUND FOR POLYNOMIAL OF ORDER {}", m, order);
                // fprintf(
                //     stderr,
                //     b"ROOT NUMBER %d WAS NOT FOUND FOR POLYNOMIAL OF ORDER %d\n\0"
                //         as *const u8 as *const libc::c_char,
                //     m,
                //     order,
                // );
                return 1 as i32;
            }
            if x0 <= x1init {
                x1 = x1init;
                p1 = p1init;
            } else {
                x1 = x0;
                p1 = p0;
                x0 = x1init;
                p0 = p1init;
            }
            if p1 == 0 as i32 as f64 {
                *roots.offset(m as isize) = x1;
            } else if p0 == 0 as i32 as f64 {
                *roots.offset(m as isize) = x0;
            } else {
                xi = x0 + (x0 - x1) / (p1 - p0) * p0;
                n = 0 as i32;
                while (x1 - x0).abs() > x1 * accrt {
                    n += 1;
                    if n > 200 as i32 {
                        eprintln!("libcint::rys_roots NO CONV. IN R_dnode");
                        // fprintf(
                        //     stderr,
                        //     b"libcint::rys_roots NO CONV. IN R_dnode\n\0" as *const u8
                        //         as *const libc::c_char,
                        // );
                        return 1 as i32;
                    }
                    pi = *a.offset(order as isize);
                    i = 1 as i32;
                    while i <= order {
                        pi = pi * xi + *a.offset((order - i) as isize);
                        i += 1;
                    }
                    if pi == 0 as i32 as f64 {
                        break;
                    }
                    if p0 * pi <= 0 as i32 as f64 {
                        x1 = xi;
                        p1 = pi;
                        xi = x0 * 0.25f64 + xi * 0.75f64;
                    } else {
                        x0 = xi;
                        p0 = pi;
                        xi = xi * 0.75f64 + x1 * 0.25f64;
                    }
                    pi = *a.offset(order as isize);
                    i = 1 as i32;
                    while i <= order {
                        pi = pi * xi + *a.offset((order - i) as isize);
                        i += 1;
                    }
                    if pi == 0 as i32 as f64 {
                        break;
                    }
                    if p0 * pi <= 0 as i32 as f64 {
                        x1 = xi;
                        p1 = pi;
                    } else {
                        x0 = xi;
                        p0 = pi;
                    }
                    xi = x0 + (x0 - x1) / (p1 - p0) * p0;
                }
                *roots.offset(m as isize) = xi;
            }
        }
        m += 1;
    }
    return 0 as i32;
}
unsafe extern "C" fn _qr_step(
    mut A: *mut f64,
    mut nroots: i32,
    mut n0: i32,
    mut n1: i32,
    mut shift: f64,
) {
    let mut m1: i32 = n0 + 1 as i32;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut m3: i32 = 0;
    let mut j1: i32 = 0;
    let mut j2: i32 = 0;
    let mut c: f64 = *A.offset((n0 * nroots + n0) as isize) - shift;
    let mut s: f64 = *A.offset((m1 * nroots + n0) as isize);
    let mut v: f64 = (c * c + s * s).sqrt();
    let mut x: f64 = 0.;
    let mut y: f64 = 0.;
    if v == 0 as i32 as f64 {
        v = 1 as i32 as f64;
        c = 1 as i32 as f64;
        s = 0 as i32 as f64;
    }
    v = 1.0f64 / v;
    c *= v;
    s *= v;
    k = n0;
    while k < nroots {
        x = *A.offset((n0 * nroots + k) as isize);
        y = *A.offset((m1 * nroots + k) as isize);
        *A.offset((n0 * nroots + k) as isize) = c * x + s * y;
        *A.offset((m1 * nroots + k) as isize) = c * y - s * x;
        k += 1;
    }
    m3 = if n1 < n0 + 3 as i32 { n1 } else { n0 + 3 as i32 };
    k = 0 as i32;
    while k < m3 {
        x = *A.offset((k * nroots + n0) as isize);
        y = *A.offset((k * nroots + m1) as isize);
        *A.offset((k * nroots + n0) as isize) = c * x + s * y;
        *A.offset((k * nroots + m1) as isize) = c * y - s * x;
        k += 1;
    }
    j = n0;
    while j < n1 - 2 as i32 {
        j1 = j + 1 as i32;
        j2 = j + 2 as i32;
        c = *A.offset((j1 * nroots + j) as isize);
        s = *A.offset((j2 * nroots + j) as isize);
        v = (c * c + s * s).sqrt();
        *A.offset((j1 * nroots + j) as isize) = v;
        *A.offset((j2 * nroots + j) as isize) = 0 as i32 as f64;
        if v == 0 as i32 as f64 {
            v = 1 as i32 as f64;
            c = 1 as i32 as f64;
            s = 0 as i32 as f64;
        }
        v = 1.0f64 / v;
        c *= v;
        s *= v;
        k = j1;
        while k < nroots {
            x = *A.offset((j1 * nroots + k) as isize);
            y = *A.offset((j2 * nroots + k) as isize);
            *A.offset((j1 * nroots + k) as isize) = c * x + s * y;
            *A.offset((j2 * nroots + k) as isize) = c * y - s * x;
            k += 1;
        }
        m3 = if n1 < j + 4 as i32 { n1 } else { j + 4 as i32 };
        k = 0 as i32;
        while k < m3 {
            x = *A.offset((k * nroots + j1) as isize);
            y = *A.offset((k * nroots + j2) as isize);
            *A.offset((k * nroots + j1) as isize) = c * x + s * y;
            *A.offset((k * nroots + j2) as isize) = c * y - s * x;
            k += 1;
        }
        j += 1;
    }
}
unsafe extern "C" fn _hessenberg_qr(
    mut A: *mut f64,
    mut nroots: i32,
) -> i32 {
    let mut eps: f64 = 1e-15f64;
    let mut maxits: i32 = 30 as i32;
    let mut n0: i32 = 0 as i32;
    let mut n1: i32 = nroots;
    let mut its: i32 = 0 as i32;
    let mut k: i32 = 0;
    let mut ic: i32 = 0;
    let mut k1: i32 = 0;
    ic = 0 as i32;
    while ic < nroots * maxits {
        k = n0;
        while (k + 1 as i32) < n1 {
            let mut s: f64 = (*A.offset((k * nroots + k) as isize)).abs()
                + (
                    *A
                        .offset(
                            ((k + 1 as i32) * nroots + k + 1 as i32)
                                as isize,
                        )
                ).abs();
            if (*A.offset(((k + 1 as i32) * nroots + k) as isize)).abs() < eps * s
            {
                break;
            }
            k += 1 as i32;
        }
        k1 = k + 1 as i32;
        if k1 < n1 {
            *A.offset((k1 * nroots + k) as isize) = 0 as i32 as f64;
            n0 = k1;
            its = 0 as i32;
            if n0 + 1 as i32 >= n1 {
                n0 = 0 as i32;
                n1 = k1;
                if n1 < 2 as i32 {
                    return 0 as i32;
                }
            }
        } else {
            let mut m1: i32 = n1 - 1 as i32;
            let mut m2: i32 = n1 - 2 as i32;
            let mut a11: f64 = *A.offset((m1 * nroots + m1) as isize);
            let mut a22: f64 = *A.offset((m2 * nroots + m2) as isize);
            let mut shift: f64 = 0.;
            let mut t: f64 = a11 + a22;
            let mut s_0: f64 = (a11 - a22) * (a11 - a22);
            s_0
                += 4 as i32 as f64
                    * *A.offset((m1 * nroots + m2) as isize)
                    * *A.offset((m2 * nroots + m1) as isize);
            if s_0 > 0 as i32 as f64 {
                s_0 = s_0.sqrt();
                let mut a: f64 = (t + s_0) * 0.5f64;
                let mut b: f64 = (t - s_0) * 0.5f64;
                if (a11 - a).abs() > (a11 - b).abs() {
                    shift = b;
                } else {
                    shift = a;
                }
            } else {
                if n1 == 2 as i32 {
                    eprintln!("hessenberg_qr: failed to find real roots");
                    // fprintf(
                    //     stderr,
                    //     b"hessenberg_qr: failed to find real roots\n\0" as *const u8
                    //         as *const libc::c_char,
                    // );
                    return 1 as i32;
                }
                shift = t * 0.5f64;
            }
            its += 1 as i32;
            _qr_step(A, nroots, n0, n1, shift);
            if its > maxits {
                eprintln!("hessenberg_qr: failed to converge after {} steps", its);
                // fprintf(
                //     stderr,
                //     b"hessenberg_qr: failed to converge after %d steps\n\0" as *const u8
                //         as *const libc::c_char,
                //     its,
                // );
                return 1 as i32;
            }
        }
        ic += 1;
    }
    eprintln!("hessenberg_qr failed");
    // fprintf(stderr, b"hessenberg_qr failed\n\0" as *const u8 as *const libc::c_char);
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _CINT_polynomial_roots(
    mut roots: *mut f64,
    mut cs: *mut f64,
    mut nroots: i32,
) -> i32 {
    if nroots == 1 as i32 {
        *roots
            .offset(
                0 as i32 as isize,
            ) = -*cs.offset(2 as i32 as isize)
            / *cs.offset(3 as i32 as isize);
        return 0 as i32;
    } else if nroots == 2 as i32 {
        let mut dum: f64 = (
            *cs.offset((2 as i32 * 3 as i32 + 1 as i32) as isize)
                * *cs
                    .offset(
                        (2 as i32 * 3 as i32 + 1 as i32) as isize,
                    )
                - 4 as i32 as f64
                    * *cs
                        .offset(
                            (2 as i32 * 3 as i32 + 0 as i32)
                                as isize,
                        )
                    * *cs
                        .offset(
                            (2 as i32 * 3 as i32 + 2 as i32)
                                as isize,
                        )
        ).sqrt();
        *roots
            .offset(
                0 as i32 as isize,
            ) = (-*cs
            .offset((2 as i32 * 3 as i32 + 1 as i32) as isize)
            - dum)
            / *cs
                .offset(
                    (2 as i32 * 3 as i32 + 2 as i32) as isize,
                ) / 2 as i32 as f64;
        *roots
            .offset(
                1 as i32 as isize,
            ) = (-*cs
            .offset((2 as i32 * 3 as i32 + 1 as i32) as isize)
            + dum)
            / *cs
                .offset(
                    (2 as i32 * 3 as i32 + 2 as i32) as isize,
                ) / 2 as i32 as f64;
        return 0 as i32;
    }
    let mut A: [f64; 1024] = [0.; 1024];
    let mut nroots1: i32 = nroots + 1 as i32;
    let mut i: i32 = 0;
    let mut fac: f64 = -1.0f64
        / *cs.offset((nroots * nroots1 + nroots) as isize);
    i = 0 as i32;
    while i < nroots {
        A[(nroots - 1 as i32 - i)
            as usize] = *cs.offset((nroots * nroots1 + i) as isize) * fac;
        i += 1;
    }
    i = nroots;
    while i < nroots * nroots {
        A[i as usize] = 0 as i32 as f64;
        i += 1;
    }
    i = 0 as i32;
    while i < nroots - 1 as i32 {
        A[((i + 1 as i32) * nroots + i) as usize] = 1.0f64;
        i += 1;
    }
    let mut err: i32 = _hessenberg_qr(A.as_mut_ptr(), nroots);
    if err == 0 as i32 {
        i = 0 as i32;
        while i < nroots {
            *roots
                .offset(
                    (nroots - 1 as i32 - i) as isize,
                ) = A[(i * nroots + i) as usize];
            i += 1;
        }
    } else {
        let mut k: i32 = 0;
        let mut order: i32 = 0;
        let mut a: *mut f64 = 0 as *mut f64;
        let mut dum_0: f64 = (
            *cs.offset((2 as i32 * nroots1 + 1 as i32) as isize)
                * *cs.offset((2 as i32 * nroots1 + 1 as i32) as isize)
                - 4 as i32 as f64
                    * *cs
                        .offset((2 as i32 * nroots1 + 0 as i32) as isize)
                    * *cs
                        .offset((2 as i32 * nroots1 + 2 as i32) as isize)
        ).sqrt();
        *roots
            .offset(
                0 as i32 as isize,
            ) = 0.5f64
            * (-*cs.offset((2 as i32 * nroots1 + 1 as i32) as isize)
                - dum_0)
            / *cs.offset((2 as i32 * nroots1 + 2 as i32) as isize);
        *roots
            .offset(
                1 as i32 as isize,
            ) = 0.5f64
            * (-*cs.offset((2 as i32 * nroots1 + 1 as i32) as isize)
                + dum_0)
            / *cs.offset((2 as i32 * nroots1 + 2 as i32) as isize);
        i = 2 as i32;
        while i < nroots {
            *roots.offset(i as isize) = 1 as i32 as f64;
            i += 1;
        }
        k = 2 as i32;
        while k < nroots {
            order = k + 1 as i32;
            a = cs.offset((order * nroots1) as isize);
            err = R_dnode(a, roots, order);
            if err != 0 {
                break;
            }
            k += 1;
        }
    }
    return err;
}
