#![allow(dead_code, unused, non_snake_case, non_upper_case_globals,unused_variables,improper_ctypes_definitions,static_mut_refs)]
#![feature(autodiff)]

extern "C" {
    pub fn sqrt(_: f64) -> f64;
}

use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;

use std::env;
use std::time::Instant;

use std::autodiff::*; //::autodiff;


enum Token {
    Int(i32),
    Float(f64),
    Delimiter,
    Invalid,
}

pub fn read_basis(
    path: &str, // std::path::PathBuf,
    atm: &mut Vec<i32>, 
    bas: &mut Vec<i32>, 
    env: &mut Vec<f64>
) -> io::Result<()> {
    // assert!(path.exists());
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let mut tokens = contents.split_whitespace().map(|s| {
        if s == "|" {
            Token::Delimiter
        } else if let Ok(int_val) = s.parse::<i32>() {
            Token::Int(int_val)
        } else if let Ok(float_val) = s.parse::<f64>() {
            Token::Float(float_val)
        } else {
            Token::Invalid
        }
    });

    while let Some(token) = tokens.next() {
        match token {
            Token::Int(value) => {
                atm.push(value);
            }
            Token::Delimiter => {
                break;
            }
            Token::Float(_) | Token::Invalid => {
                println!("Error: Expected int in file.");
            }
        }
    }

    while let Some(token) = tokens.next() {
        match token {
            Token::Int(value) => {
                bas.push(value);
            }
            Token::Delimiter => {
                break;
            }
            Token::Float(_) | Token::Invalid => {
                println!("Error: Expected int in file.");
            }
        }
    }

    while let Some(token) = tokens.next() {
        match token {
            Token::Float(value) => {
                env.push(value);
            }
            Token::Delimiter => (),
            Token::Int(_) | Token::Invalid => {
                println!("Error: Expected float in file.");
            }
        }
    }
    Ok(())
}


#[no_mangle]
pub fn CINTcgto_cart(
    bas_id: usize,
    bas: &[i32],
) -> i32 {
    let mut l: i32 = bas[8 * bas_id + 1];
    return (l + 1) * (l + 2) / 2 * bas[8 * bas_id + 3];
}

impl CINTEnvVars {
    pub fn new() -> Self {
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: 0 as *mut i32,
        bas: 0 as *mut i32,
        env: 0 as *mut f64,
        shls: 0 as *mut i32,
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: 0 as *mut f64,
        rx_in_rklrx: 0 as *mut f64,
        ri: 0 as *mut f64,
        rj: 0 as *mut f64,
        rk: 0 as *mut f64,
        c2rust_unnamed_1: C2RustUnnamed {
            rl: 0 as *mut f64,
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: 0 as *mut CINTOpt,
        idx: 0 as *mut i32,
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    envs
    }
}

#[no_mangle]
pub unsafe extern "C" fn c2s_cart_1e(
    mut opij: *mut f64,
    mut gctr: *mut f64,
    mut dims: *mut i32,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut f64,
) {
    let mut i_ctr: i32 = (*envs).x_ctr[0 as usize];
    let mut j_ctr: i32 = (*envs).x_ctr[1 as usize];
    let mut nfi: i32 = (*envs).nfi;
    let mut nfj: i32 = (*envs).nfj;
    let mut nf: i32 = (*envs).nf;
    let mut ni: i32 = *dims.offset(0 as isize);
    let mut nj: i32 = *dims.offset(1 as isize);
    let mut ofj: i32 = ni * nfj;
    let mut ic: i32 = 0;
    let mut jc: i32 = 0;
    let mut popij: *mut f64 = 0 as *mut f64;
    jc = 0 as i32;
    while jc < j_ctr {
        ic = 0 as i32;
        while ic < i_ctr {
            popij = opij.offset((ofj * jc) as isize).offset((nfi * ic) as isize);
            dcopy_ij(popij, gctr, ni, nj, nfi, nfj);
            gctr = gctr.offset(nf as isize);
            ic += 1;
            ic;
        }
        jc += 1;
        jc;
    }
}
unsafe extern "C" fn dcopy_ij(
    mut out: *mut f64,
    mut gctr: *mut f64,
    ni: i32,
    nj: i32,
    mi: i32,
    mj: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    j = 0 as i32;
    while j < mj {
        i = 0 as i32;
        while i < mi {
            *out.offset((j * ni + i) as isize) = *gctr.offset((j * mi + i) as isize);
            i += 1;
            i;
        }
        j += 1;
        j;
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PairData {
    pub rij: [f64; 3],
    pub eij: f64,
    pub cceij: f64,
}

fn SQUARE(r: *mut f64) -> f64 {
    unsafe {
        (*r.add(0) * *r.add(0)) + (*r.add(1) * *r.add(1)) + (*r.add(2) * *r.add(2))
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTrys_roots(
    mut nroots: i32,
    mut x: f64,
    mut u: *mut f64,
    mut w: *mut f64,
) {
    if x <= 3e-7f64 {
        let mut off: i32 = nroots * (nroots - 1 as i32)
            / 2 as i32;
        let mut i: i32 = 0;
        i = 0 as i32;
        return;
    } else if x >= (35 as i32 + nroots * 5 as i32) as f64 {
        let mut off_0: i32 = nroots * (nroots - 1 as i32)
            / 2 as i32;
        let mut i_0: i32 = 0;
        let mut rt: f64 = 0.;
        let mut t: f64 = sqrt(0.78539816339744827900f64 / x);
        i_0 = 0 as i32;
        while i_0 < nroots {
            rt = 3.3;
            *u.offset(i_0 as isize) = rt / (x - rt);
            *w.offset(i_0 as isize) = 3.14;
            i_0 += 1;
            i_0;
        }
        return;
    }
    let mut err: i32 = 0;
    match nroots {
        1 => {
            err = 42; //rys_root1(x, u, w);
        }
        2 => {
            err = 42; //rys_root2(x, u, w);
        }
        3 => {
            err = 42; //rys_root3(x, u, w);
        }
        4 => {
            err = 42; //rys_root4(x, u, w);
        }
        5 => {
            err = 42; //rys_root5(x, u, w);
        }
        6 | 7 => {
            err = 42;
        }
        8 => {
            err = 42;
        }
        9 => {
            err = 42;
        }
        10 | 11 => {
            err = 42;
        }
        12 => {
            err = 42;
        }
        _ => {
            err = 42;
        }
    }
    if err != 0 {
        eprintln!("rys_roots fails: nroots={} x={}", nroots, x);
        std::process::exit(err);
    }
}

#[no_mangle]
pub unsafe extern "C" fn CINTnuc_mod(
    mut aij: f64,
    mut nuc_id: i32,
    mut atm: *mut i32,
    mut env: *mut f64,
) -> f64 {
    let mut zeta: f64 = 0.;
    if nuc_id < 0 as i32 {
        zeta = *env.offset(7 as isize);
    } else if *atm.offset((6 as i32 * nuc_id + 2 as i32) as isize)
        == 2 as i32
    {
        zeta = *env
            .offset(
                *atm.offset((6 as i32 * nuc_id + 3 as i32) as isize)
                    as isize,
            );
    } else {
        zeta = 0 as f64;
    }
    if zeta > 0 as f64 {
        return (zeta / (aij + zeta)).sqrt()
    } else {
        return 1 as f64
    };
}

#[no_mangle]
pub fn nmol(
    atm: &Vec<i32>,
    bas: &Vec<i32>,
) 
-> (usize, usize) {
    let natm: usize = atm.len() / ATM_SLOTS;
    let nbas: usize = bas.len() / BAS_SLOTS;
    return (natm, nbas);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub rl: *mut f64,
    pub grids: *mut f64,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub nfl: i32,
    pub ngrids: i32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub nfk: i32,
    pub grids_offset: i32,
}

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

#[no_mangle]
pub fn cint1e_ovlp_cart(
    out: &mut [f64],
    shls: &mut [i32],
    atm: &mut [i32],
    natm: i32,
    bas: &mut [i32],
    nbas: i32,
    env: &mut [f64],
    opt: *mut CINTOpt,
) -> i32 {
    unsafe {
        return int1e_ovlp_cart(
            out.as_mut_ptr(),
            0 as *mut i32,
            shls.as_mut_ptr(),
            atm.as_mut_ptr(),
            natm,
            bas.as_mut_ptr(),
            nbas,
            env.as_mut_ptr(),
            opt,
            0 as *mut f64,
        );
    }
}
extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type uintptr_t = u64;
#[no_mangle]
pub unsafe extern "C" fn int1e_cache_size(mut envs: *mut CINTEnvVars) -> i32 {
    let mut shls: *mut i32 = (*envs).shls;
    let mut bas: *mut i32 = (*envs).bas;
    let mut i_prim: i32 = *bas
        .offset(
            (8 as i32 * *shls.offset(0 as isize)
                + 2 as i32) as isize,
        );
    let mut j_prim: i32 = *bas
        .offset(
            (8 as i32 * *shls.offset(1 as isize)
                + 2 as i32) as isize,
        );
    let mut x_ctr: *mut i32 = ((*envs).x_ctr).as_mut_ptr();
    let mut nc: i32 = (*envs).nf * *x_ctr.offset(0 as isize)
        * *x_ctr.offset(1 as isize);
    let mut n_comp: i32 = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut leng: i32 = (*envs).g_size * 3 as i32
        * (((1 as i32) << (*envs).gbits) + 1 as i32);
    let mut lenj: i32 = (*envs).nf * nc * n_comp;
    let mut leni: i32 = (*envs).nf * *x_ctr.offset(0 as isize)
        * n_comp;
    let mut len0: i32 = (*envs).nf * n_comp;
    let mut pdata_size: i32 = i_prim * j_prim * 5 as i32
        + i_prim * *x_ctr.offset(0 as isize)
        + j_prim * *x_ctr.offset(1 as isize)
        + (i_prim + j_prim) * 2 as i32 + (*envs).nf * 3 as i32;
    let mut cache_size: i32 = if nc * n_comp + leng + lenj + leni + len0
        + pdata_size > nc * n_comp + (*envs).nf * 8 as i32 * 2 as i32
    {
        nc * n_comp + leng + lenj + leni + len0 + pdata_size
    } else {
        nc * n_comp + (*envs).nf * 8 as i32 * 2 as i32
    };
    return cache_size;
}

#[no_mangle]
pub unsafe extern "C" fn CINT1e_drv(
    mut out: *mut f64,
    mut dims: *mut i32,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut f64,
    mut f_c2s: Option::<unsafe extern "C" fn() -> ()>,
    mut int1e_type: i32,
) -> i32 {
    let mut x_ctr: *mut i32 = ((*envs).x_ctr).as_mut_ptr();
    let mut nc: i32 = (*envs).nf * *x_ctr.offset(0 as isize)
        * *x_ctr.offset(1 as isize);
    let mut n_comp: i32 = 42;
    let mut stack: *mut f64 = 0 as *mut f64;
    if cache.is_null() {
        let mut cache_size: u64 = int1e_cache_size(envs) as u64;
        stack = malloc(
            (::core::mem::size_of::<f64>() as u64)
                .wrapping_mul(cache_size),
        ) as *mut f64;
        cache = stack;
    }
    let mut gctr: *mut f64 = 0 as *mut f64;
    gctr = ((cache as uintptr_t).wrapping_add(7 as u64)
        & (8 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut f64;
    cache = gctr.offset((nc * n_comp) as isize);
    let mut empty: [i32; 4] = [
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ];
    let mut jempty: *mut i32 = empty
        .as_mut_ptr()
        .offset(2 as isize);
    let mut idx: *mut i32 = 0 as *mut i32;
    let mut g: *mut f64 = 0 as *mut f64;
    let mut gout: *mut f64 = 0 as *mut f64;
    g = ((cache as uintptr_t).wrapping_add(7 as u64)
        & (8 as uintptr_t).wrapping_neg()) as *mut libc::c_void
        as *mut f64;
    CINTg1e_nuc(g, envs, -(1 as i32));
    ::core::mem::transmute::<
        _,
        fn(_, _, _, _, _),
    >(
        (Some(((*envs).f_gout).expect("non-null function pointer")))
            .expect("non-null function pointer"),
    )(gout, g, idx, envs, empty);
    return (*jempty == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn CINTg1e_nuc(
    mut g: *mut f64,
    mut envs: *mut CINTEnvVars,
    mut nuc_id: i32,
) -> i32 {
    let mut nrys_roots: i32 = (*envs).nrys_roots;
    let mut atm: *mut i32 = (*envs).atm;
    let mut env: *mut f64 = (*envs).env;
    let mut rij: *mut f64 = ((*envs).rij).as_mut_ptr();
    let mut gx: *mut f64 = g;
    let mut gy: *mut f64 = g.offset((*envs).g_size as isize);
    let mut gz: *mut f64 = g
        .offset(((*envs).g_size * 2 as i32) as isize);
    let mut u: [f64; 32] = [0.; 32];
    let mut w: *mut f64 = gz;
    let mut cr: *mut f64 = 0 as *mut f64;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    let mut crij: [f64; 3] = [0.; 3];
    let mut x: f64 = 0.;
    let mut fac1: f64 = 0.;
    let mut aij: f64 = (*envs).ai[0 as usize]
        + (*envs).aj[0 as usize];
    let mut tau: f64 = CINTnuc_mod(aij, nuc_id, atm, env);
    if nuc_id < 0 as i32 {
        fac1 = 2 as f64 * 3.14159265358979323846f64
            * (*envs).fac[0 as usize] * tau / aij;
        cr = env.offset(4 as isize);
    } else if *atm.offset((6 as i32 * nuc_id + 2 as i32) as isize)
        == 3 as i32
    {
        fac1 = 2 as f64 * 3.14159265358979323846f64
            * -*env
                .offset(
                    *atm.offset((4 as i32 + nuc_id * 6 as i32) as isize)
                        as isize,
                ) * (*envs).fac[0 as usize] * tau / aij;
        cr = env
            .offset(
                *atm.offset((6 as i32 * nuc_id + 1 as i32) as isize)
                    as isize,
            );
    } else {
        fac1 = 2 as f64 * 3.14159265358979323846f64
            * -(*atm.offset((0 as i32 + nuc_id * 6 as i32) as isize)).abs()
                as f64 * (*envs).fac[0 as usize] * tau / aij;
        cr = env
            .offset(
                *atm.offset((6 as i32 * nuc_id + 1 as i32) as isize)
                    as isize,
            );
    }
    crij[0 as i32
        as usize] = *cr.offset(0 as isize)
        - *rij.offset(0 as isize);
    crij[1 as i32
        as usize] = *cr.offset(1 as isize)
        - *rij.offset(1 as isize);
    crij[2 as i32
        as usize] = *cr.offset(2 as isize)
        - *rij.offset(2 as isize);
    x = aij * tau * tau * SQUARE(crij.as_mut_ptr()) as f64;
    CINTrys_roots(nrys_roots, x, u.as_mut_ptr(), w);
    i = 0 as i32;
    while i < nrys_roots {
        *gx.offset(i as isize) = 1 as f64;
        *gy.offset(i as isize) = 1 as f64;
        *gz.offset(i as isize) *= fac1;
        i += 1;
        i;
    }
    let mut nmax: i32 = (*envs).li_ceil + (*envs).lj_ceil;
    if nmax == 0 as i32 {
        return 1 as i32;
    }
    let mut p0x: *mut f64 = 0 as *mut f64;
    let mut p0y: *mut f64 = 0 as *mut f64;
    let mut p0z: *mut f64 = 0 as *mut f64;
    let mut p1x: *mut f64 = 0 as *mut f64;
    let mut p1y: *mut f64 = 0 as *mut f64;
    let mut p1z: *mut f64 = 0 as *mut f64;
    let mut p2x: *mut f64 = 0 as *mut f64;
    let mut p2y: *mut f64 = 0 as *mut f64;
    let mut p2z: *mut f64 = 0 as *mut f64;
    let mut lj: i32 = 0;
    let mut di: i32 = 0;
    let mut dj: i32 = 0;
    let mut rx: *mut f64 = 0 as *mut f64;
    if (*envs).li_ceil > (*envs).lj_ceil {
        lj = (*envs).lj_ceil;
        di = (*envs).g_stride_i;
        dj = (*envs).g_stride_j;
        rx = (*envs).ri;
    } else {
        lj = (*envs).li_ceil;
        di = (*envs).g_stride_j;
        dj = (*envs).g_stride_i;
        rx = (*envs).rj;
    }
    let mut rijrx: f64 = *rij.offset(0 as isize)
        - *rx.offset(0 as isize);
    let mut rijry: f64 = *rij.offset(1 as isize)
        - *rx.offset(1 as isize);
    let mut rijrz: f64 = *rij.offset(2 as isize)
        - *rx.offset(2 as isize);
    let mut aij2: f64 = 0.5f64 / aij;
    let mut ru: f64 = 0.;
    let mut rt: f64 = 0.;
    let mut r0: f64 = 0.;
    let mut r1: f64 = 0.;
    let mut r2: f64 = 0.;
    p0x = gx.offset(di as isize);
    p0y = gy.offset(di as isize);
    p0z = gz.offset(di as isize);
    p1x = gx.offset(-(di as isize));
    p1y = gy.offset(-(di as isize));
    p1z = gz.offset(-(di as isize));
    n = 0 as i32;
    while n < nrys_roots {
        ru = tau * tau * u[n as usize]
            / (1 as f64 + u[n as usize]);
        rt = aij2 - aij2 * ru;
        r0 = rijrx + ru * crij[0 as usize];
        r1 = rijry + ru * crij[1 as usize];
        r2 = rijrz + ru * crij[2 as usize];
        *p0x.offset(n as isize) = r0 * *gx.offset(n as isize);
        *p0y.offset(n as isize) = r1 * *gy.offset(n as isize);
        *p0z.offset(n as isize) = r2 * *gz.offset(n as isize);
        i = 1 as i32;
        while i < nmax {
            *p0x
                .offset(
                    (n + i * di) as isize,
                ) = i as f64 * rt * *p1x.offset((n + i * di) as isize)
                + r0 * *gx.offset((n + i * di) as isize);
            *p0y
                .offset(
                    (n + i * di) as isize,
                ) = i as f64 * rt * *p1y.offset((n + i * di) as isize)
                + r1 * *gy.offset((n + i * di) as isize);
            *p0z
                .offset(
                    (n + i * di) as isize,
                ) = i as f64 * rt * *p1z.offset((n + i * di) as isize)
                + r2 * *gz.offset((n + i * di) as isize);
            i += 1;
            i;
        }
        n += 1;
        n;
    }
    let mut rirjx: f64 = (*envs).rirj[0 as usize];
    let mut rirjy: f64 = (*envs).rirj[1 as usize];
    let mut rirjz: f64 = (*envs).rirj[2 as usize];
    j = 1 as i32;
    while j <= lj {
        p0x = gx.offset((j * dj) as isize);
        p0y = gy.offset((j * dj) as isize);
        p0z = gz.offset((j * dj) as isize);
        p1x = p0x.offset(-(dj as isize));
        p1y = p0y.offset(-(dj as isize));
        p1z = p0z.offset(-(dj as isize));
        p2x = p1x.offset(di as isize);
        p2y = p1y.offset(di as isize);
        p2z = p1z.offset(di as isize);
        i = 0 as i32;
        while i <= nmax - j {
            n = 0 as i32;
            while n < nrys_roots {
                *p0x
                    .offset(
                        (n + i * di) as isize,
                    ) = *p2x.offset((n + i * di) as isize)
                    + rirjx * *p1x.offset((n + i * di) as isize);
                *p0y
                    .offset(
                        (n + i * di) as isize,
                    ) = *p2y.offset((n + i * di) as isize)
                    + rirjy * *p1y.offset((n + i * di) as isize);
                *p0z
                    .offset(
                        (n + i * di) as isize,
                    ) = *p2z.offset((n + i * di) as isize)
                    + rirjz * *p1z.offset((n + i * di) as isize);
                n += 1;
                n;
            }
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    return 1 as i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CINTOpt {
    pub index_xyz_array: *mut *mut i32,
    pub non0ctr: *mut *mut i32,
    pub sortedidx: *mut *mut i32,
    pub nbas: i32,
    pub log_max_coeff: *mut *mut f64,
    pub pairdata: *mut *mut PairData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CINTEnvVars {
    pub atm: *mut i32,
    pub bas: *mut i32,
    pub env: *mut f64,
    pub shls: *mut i32,
    pub natm: i32,
    pub nbas: i32,
    pub i_l: i32,
    pub j_l: i32,
    pub k_l: i32,
    pub l_l: i32,
    pub nfi: i32,
    pub nfj: i32,
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed_0,
    pub nf: i32,
    pub rys_order: i32,
    pub x_ctr: [i32; 4],
    pub gbits: i32,
    pub ncomp_e1: i32,
    pub ncomp_e2: i32,
    pub ncomp_tensor: i32,
    pub li_ceil: i32,
    pub lj_ceil: i32,
    pub lk_ceil: i32,
    pub ll_ceil: i32,
    pub g_stride_i: i32,
    pub g_stride_k: i32,
    pub g_stride_l: i32,
    pub g_stride_j: i32,
    pub nrys_roots: i32,
    pub g_size: i32,
    pub g2d_ijmax: i32,
    pub g2d_klmax: i32,
    pub common_factor: f64,
    pub expcutoff: f64,
    pub rirj: [f64; 3],
    pub rkrl: [f64; 3],
    pub rx_in_rijrx: *mut f64,
    pub rx_in_rklrx: *mut f64,
    pub ri: *mut f64,
    pub rj: *mut f64,
    pub rk: *mut f64,
    pub c2rust_unnamed_1: C2RustUnnamed,
    pub f_g0_2e: Option::<unsafe extern "C" fn() -> i32>,
    pub f_g0_2d4d: Option::<unsafe extern "C" fn() -> ()>,
    pub f_gout: Option::<unsafe extern "C" fn() -> ()>,
    pub opt: *mut CINTOpt,
    pub idx: *mut i32,
    pub ai: [f64; 1],
    pub aj: [f64; 1],
    pub ak: [f64; 1],
    pub al: [f64; 1],
    pub fac: [f64; 1],
    pub rij: [f64; 3],
    pub rkl: [f64; 3],
}

#[no_mangle]
pub unsafe extern "C" fn int1e_ovlp_cart(
    mut out: *mut f64,
    mut dims: *mut i32,
    mut shls: *mut i32,
    mut atm: *mut i32,
    mut natm: i32,
    mut bas: *mut i32,
    mut nbas: i32,
    mut env: *mut f64,
    mut opt: *mut CINTOpt,
    mut cache: *mut f64,
) -> i32 {
    let mut envs: CINTEnvVars = CINTEnvVars::new();
    return CINT1e_drv(
        out,
        dims,
        &mut envs,
        cache,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut f64,
                    *mut f64,
                    *mut i32,
                    *mut CINTEnvVars,
                    *mut f64,
                ) -> (),
            >,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut f64,
                        *mut f64,
                        *mut i32,
                        *mut CINTEnvVars,
                        *mut f64,
                    ) -> (),
            ),
        ),
        0 as i32,
    );
}

#[no_mangle]
#[autodiff_forward(dovlppfor, Dual, Const, Const, Const, Const, Const, Dual)]
fn ovlpp(
    out: &mut [f64], 
    shls: &mut [i32], 
    atm: &mut [i32],
    natm: usize, 
    bas: &mut [i32], 
    nbas: usize, 
    env: &mut [f64]
) {
    cint1e_ovlp_cart(
        out, 
        shls, 
        atm, 
        natm as i32, 
        bas, 
        nbas as i32, 
        env,
        std::ptr::null_mut(),
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} file", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];

    let mut atm: Vec<i32> = Vec::new();
    let mut bas: Vec<i32> = Vec::new();
    let mut env: Vec<f64> = Vec::new();

    _ = read_basis(&path, &mut atm, &mut bas, &mut env);

    let (natm, nbas) = nmol(&mut atm, &mut bas);

    let mut shls: [i32; 4] = [0, 0, 0, 0];

    let mut total_ovlp_time = 0.0;
    let mut total_dovlp_time = 0.0;
    let mut count = 0;

    println!("{} {}", natm, nbas);

    for i in 0..nbas {
        for j in 0..nbas {
            shls[0] = i as i32;
            shls[1] = j as i32;

            let di = CINTcgto_cart(i, &bas);
            let dj = CINTcgto_cart(j, &bas);

            let size = (di * dj) as usize;
            let mut buf = vec![0.0; size];
            let mut dbuf = vec![0.0; size];
            
            dbuf[0] = 1.0;

            // fix
            // loop through di * dj

            // Time primal function
            let start_ovlp = Instant::now();
            ovlpp(
                &mut buf,
                &mut shls,
                &mut atm,
                natm,
                &mut bas,
                nbas,
                &mut env,
            );
            let duration_ovlp = start_ovlp.elapsed().as_secs_f64();
            total_ovlp_time += duration_ovlp;

            // Time autodiff function
            let mut denv = vec![0.0f64; env.len()];
            let start_dovlp = Instant::now();
            // dovlpp(
            //     &mut buf,
            //     &mut dbuf,
            //     &mut shls,
            //     &mut atm,
            //     natm,
            //     &mut bas,
            //     nbas,
            //     &mut env,
            //     &mut denv,
            // );
            let duration_dovlp = start_dovlp.elapsed().as_secs_f64();
            total_dovlp_time += duration_dovlp;

            count += 1;
        }
    }    

    println!("count {}", count);
    println!("total ovlp time:    {:.6} sec", total_ovlp_time);
    println!("total dovlp time:   {:.6} sec", total_dovlp_time);
    println!("average ovlp time:  {:.6} sec", total_ovlp_time / count as f64);
    println!("average dovlp time: {:.6} sec", total_dovlp_time / count as f64);
    println!("avg overhead:       {:.6}", total_dovlp_time / total_ovlp_time);


    // FORWARD MODE

    let mut total_ovlp_time_for = 0.0;
    let mut total_dovlp_time_for = 0.0;
    count = 0;

    for i in 0..nbas {
        for j in 0..nbas {
            shls[0] = i as i32;
            shls[1] = j as i32;

            let di = CINTcgto_cart(i, &bas);
            let dj = CINTcgto_cart(j, &bas);

            let size = (di * dj) as usize;
            let mut buf = vec![0.0; size];
            let mut dbuf = vec![0.0; size];

            // Time primal function
            let start_ovlp = Instant::now();
            ovlpp(
                &mut buf,
                &mut shls,
                &mut atm,
                natm,
                &mut bas,
                nbas,
                &mut env,
            );
            let duration_ovlp = start_ovlp.elapsed().as_secs_f64();
            total_ovlp_time_for += duration_ovlp;

            // Time autodiff function
            let mut denv = vec![0.0f64; env.len()];

            denv[0] = 1.0;

            // loop through 0..denv loop through ROI in denv only
            // denv[k] = 1.0;

            let start_dovlp = Instant::now();
            dovlppfor(
                &mut buf,
                &mut dbuf,
                &mut shls,
                &mut atm,
                natm,
                &mut bas,
                nbas,
                &mut env,
                &mut denv,
            );
            let duration_dovlp = start_dovlp.elapsed().as_secs_f64();
            total_dovlp_time_for += duration_dovlp;

            count += 1;

            // buf 1x1
            // env [x, y, z]
            // denv [dx, dy, dz]

            // buf w
            // dbuf [dw] denv[x] = 1.0
            // dbuf [dw] denv[y] = 1.0
            // dbuf [dw] denv[z] = 1.0
        }
    }    

    println!("count {}", count);
    println!("total ovlp time:    {:.6} sec", total_ovlp_time_for);
    println!("total dovlp time:   {:.6} sec", total_dovlp_time_for);
    println!("average ovlp time:  {:.6} sec", total_ovlp_time_for / count as f64);
    println!("average dovlp time: {:.6} sec", total_dovlp_time_for / count as f64);
    println!("avg overhead:       {:.6}", total_dovlp_time_for / total_ovlp_time_for);

    // let mut total_repp_time = 0.0;
    // let mut total_drepp_time = 0.0;
    // count = 0;

    // for i in 0..nbas {
    //     for j in 0..nbas {
    //         for k in 0..nbas {
    //             for l in 0..nbas {
    //                 // Set shell quartet indices
    //                 let mut shls: [i32; 4] = [0; 4];
    //                 shls[0] = i as i32;
    //                 shls[1] = j as i32;
    //                 shls[2] = k as i32;
    //                 shls[3] = l as i32;

    //                 // Compute basis function counts for each shell
    //                 let di = CINTcgto_cart(i, &bas);
    //                 let dj = CINTcgto_cart(j, &bas);
    //                 let dk = CINTcgto_cart(k, &bas);
    //                 let dl = CINTcgto_cart(l, &bas);

    //                 // Compute size of output array: product of all basis function counts
    //                 let size = (di * dj * dk * dl) as usize;

    //                 let mut buf = vec![0.0f64; size];
    //                 let mut dbuf = vec![0.0f64; size];
    //                 dbuf[0] = 1.0;

    //                 // Time primal function
    //                 let start_repp = Instant::now();
    //                 repp(
    //                     &mut buf,
    //                     &mut shls,
    //                     &mut atm,
    //                     natm,
    //                     &mut bas,
    //                     nbas,
    //                     &mut env,
    //                 );
    //                 let duration_repp = start_repp.elapsed().as_secs_f64();
    //                 total_repp_time += duration_repp;

    //                 // Time autodiff function
    //                 let mut denv = vec![0.0f64; env.len()];
    //                 let start_drepp = Instant::now();
    //                 drepp(
    //                     &mut buf,
    //                     &mut dbuf,
    //                     &mut shls,
    //                     &mut atm,
    //                     natm,
    //                     &mut bas,
    //                     nbas,
    //                     &mut env,
    //                     &mut denv,
    //                 );
    //                 let duration_drepp = start_drepp.elapsed().as_secs_f64();
    //                 total_drepp_time += duration_drepp;

    //                 count += 1;
    //             }
    //         }
    //     }
    // }

    // println!("count {}", count);
    // println!("total rep time:     {:.6} sec", total_repp_time);
    // println!("total drep time:    {:.6} sec", total_drepp_time);
    // println!("average rep time:   {:.6} sec", total_repp_time / count as f64);
    // println!("average drep time:  {:.6} sec", total_drepp_time / count as f64);
    // println!("avg overhead:       {:.6}", total_drepp_time / total_repp_time);
}
