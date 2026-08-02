// Idiomatic AD-friendly contracted cartesian 2e ERI kernel (rys quadrature),
// restructured from libcint's CINT2e chain for Enzyme reverse mode: no function
// pointers or PairData table, only static-bound local arrays and slice
// indexing. The g-buffer recursion uses the lj/l-on-ket convention (VRR
// accumulates angular momentum on j and l, HRR then shifts it onto i and k).
//
// Restrictions (asserted): cartesian, l <= LMAX per shell, no range separation
// (env[8]==0). General contraction (nctr > 1) IS supported.

pub const LMAX: usize = 4;
const NFMAX: usize = (LMAX + 1) * (LMAX + 2) / 2; // 15 (g)
const NRMAX: usize = 2 * LMAX + 1; // 9 roots for an all-g quartet
                                   // Per-ROOT g-buffer: the VRR/HRR are run one rys root at a time (the root loop
                                   // wraps prim_g_root + the gout accumulation), so the buffer no longer carries
                                   // the nroots factor. That both shrinks it (l=3: 784 vs 5488) and keeps l=4
                                   // (25*81 = 2025) under the ~5k-double reverse-region alloca ceiling that makes
                                   // Enzyme's pass SIGSEGV -- the all-roots l=4 buffer would have been 18225.
pub const GSIZE_MAX: usize = (LMAX + 1) * (LMAX + 1) * (2 * LMAX + 1) * (2 * LMAX + 1);

fn common_fac_sp(l: i32) -> f64 {
    match l {
        0 => 0.282_094_791_773_878_14,
        1 => 0.488_602_511_902_919_9,
        _ => 1.0,
    }
}

// (nx, ny, nz) exponents of the cartesian components of shell l, in
// libcint/pyscf order (x-major descending).
pub fn cart_comp(l: usize) -> ([usize; NFMAX], [usize; NFMAX], [usize; NFMAX]) {
    let mut nx = [0usize; NFMAX];
    let mut ny = [0usize; NFMAX];
    let mut nz = [0usize; NFMAX];
    let mut n = 0;
    let mut lx = l as isize;
    while lx >= 0 {
        let mut ly = l as isize - lx;
        while ly >= 0 {
            nx[n] = lx as usize;
            ny[n] = ly as usize;
            nz[n] = (l as isize - lx - ly) as usize;
            n += 1;
            ly -= 1;
        }
        lx -= 1;
    }
    (nx, ny, nz)
}

// Shell-quartet parameters read from shls/atm/bas/env, shared by both
// contraction paths.
struct QuartetCtx {
    li: usize,
    lj: usize,
    lk: usize,
    ll: usize,
    i_prim: usize,
    j_prim: usize,
    k_prim: usize,
    l_prim: usize,
    nctri: usize,
    nctrj: usize,
    nctrk: usize,
    nctrl: usize,
    pai: usize,
    paj: usize,
    pak: usize,
    pal: usize,
    pci: usize,
    pcj: usize,
    pck: usize,
    pcl: usize,
    ri: [f64; 3],
    rj: [f64; 3],
    rk: [f64; 3],
    rl: [f64; 3],
    nfi: usize,
    nfj: usize,
    nfk: usize,
    nfl: usize,
    nmax: usize,
    mmax: usize,
    nroots: usize,
    di: usize,
    dk: usize,
    dl: usize,
    dj: usize,
    common_factor: f64,
    expcutoff: f64,
    rirj: [f64; 3],
    rkrl: [f64; 3],
    rr_ij: f64,
    rr_kl: f64,
    log_rr_ij: f64,
    log_rr_kl: f64,
}

fn quartet_ctx(shls: &[i32], atm: &[i32], bas: &[i32], env: &[f64]) -> QuartetCtx {
    let i_sh = shls[0] as usize;
    let j_sh = shls[1] as usize;
    let k_sh = shls[2] as usize;
    let l_sh = shls[3] as usize;

    let li = bas[8 * i_sh + 1] as usize;
    let lj = bas[8 * j_sh + 1] as usize;
    let lk = bas[8 * k_sh + 1] as usize;
    let ll = bas[8 * l_sh + 1] as usize;
    assert!(li <= LMAX && lj <= LMAX && lk <= LMAX && ll <= LMAX);
    assert!(env[8] == 0.0);

    let i_prim = bas[8 * i_sh + 2] as usize;
    let j_prim = bas[8 * j_sh + 2] as usize;
    let k_prim = bas[8 * k_sh + 2] as usize;
    let l_prim = bas[8 * l_sh + 2] as usize;

    let pai = bas[8 * i_sh + 5] as usize;
    let paj = bas[8 * j_sh + 5] as usize;
    let pak = bas[8 * k_sh + 5] as usize;
    let pal = bas[8 * l_sh + 5] as usize;
    let pci = bas[8 * i_sh + 6] as usize;
    let pcj = bas[8 * j_sh + 6] as usize;
    let pck = bas[8 * k_sh + 6] as usize;
    let pcl = bas[8 * l_sh + 6] as usize;

    let pri = atm[6 * bas[8 * i_sh] as usize + 1] as usize;
    let prj = atm[6 * bas[8 * j_sh] as usize + 1] as usize;
    let prk = atm[6 * bas[8 * k_sh] as usize + 1] as usize;
    let prl = atm[6 * bas[8 * l_sh] as usize + 1] as usize;
    let ri = [env[pri], env[pri + 1], env[pri + 2]];
    let rj = [env[prj], env[prj + 1], env[prj + 2]];
    let rk = [env[prk], env[prk + 1], env[prk + 2]];
    let rl = [env[prl], env[prl + 1], env[prl + 2]];

    let nfi = (li + 1) * (li + 2) / 2;
    let nfj = (lj + 1) * (lj + 2) / 2;
    let nfk = (lk + 1) * (lk + 2) / 2;
    let nfl = (ll + 1) * (ll + 2) / 2;

    let nmax = li + lj;
    let mmax = lk + ll;
    let nroots = (nmax + mmax) / 2 + 1;

    // Per-root g-buffer strides, lj2d4d convention: di=1 (single root; the
    // root loop is in the caller), dk=li+1, dl=dk*(lk+1), dj=dl*(mmax+1).
    // VRR fills the j and l axes to full combined momentum.
    let di = 1;
    let dk = di * (li + 1);
    let dl = dk * (lk + 1);
    let dj = dl * (mmax + 1);
    assert!(dj * (nmax + 1) <= GSIZE_MAX);

    let common_factor = std::f64::consts::PI * std::f64::consts::PI * std::f64::consts::PI * 2.0
        / 1.772_453_850_905_516
        * common_fac_sp(li as i32)
        * common_fac_sp(lj as i32)
        * common_fac_sp(lk as i32)
        * common_fac_sp(ll as i32);
    let expcutoff = if env[0] == 0.0 {
        60.0
    } else {
        (if 40.0 > env[0] { 40.0 } else { env[0] }) + 1.0
    };

    let rirj = [ri[0] - rj[0], ri[1] - rj[1], ri[2] - rj[2]];
    let rkrl = [rk[0] - rl[0], rk[1] - rl[1], rk[2] - rl[2]];
    let rr_ij = rirj[0] * rirj[0] + rirj[1] * rirj[1] + rirj[2] * rirj[2];
    let rr_kl = rkrl[0] * rkrl[0] + rkrl[1] * rkrl[1] + rkrl[2] * rkrl[2];

    // Screening thresholds, replicated from CINTset_pairdata /
    // CINT2e_loop_nopt (omega == 0 branches).
    let aij_last = env[pai + i_prim - 1] + env[paj + j_prim - 1];
    let mut log_rr_ij = 1.7 - 1.5 * aij_last.ln();
    if nmax > 0 {
        log_rr_ij += (nmax as f64) * (rr_ij.sqrt() + 1.0).ln();
    }
    let akl_last = env[pak + k_prim - 1] + env[pal + l_prim - 1];
    let mut log_rr_kl = 1.7 - 1.5 * akl_last.ln();
    if mmax > 0 {
        log_rr_kl += (mmax as f64) * (rr_kl.sqrt() + 1.0).ln();
    }

    QuartetCtx {
        li,
        lj,
        lk,
        ll,
        i_prim,
        j_prim,
        k_prim,
        l_prim,
        nctri: bas[8 * i_sh + 3] as usize,
        nctrj: bas[8 * j_sh + 3] as usize,
        nctrk: bas[8 * k_sh + 3] as usize,
        nctrl: bas[8 * l_sh + 3] as usize,
        pai,
        paj,
        pak,
        pal,
        pci,
        pcj,
        pck,
        pcl,
        ri,
        rj,
        rk,
        rl,
        nfi,
        nfj,
        nfk,
        nfl,
        nmax,
        mmax,
        nroots,
        di,
        dk,
        dl,
        dj,
        common_factor,
        expcutoff,
        rirj,
        rkrl,
        rr_ij,
        rr_kl,
        log_rr_ij,
        log_rr_kl,
    }
}

// Root-independent setup for a primitive quartet: a0/fac1 prefactors, the P-Q
// separation, and the rys roots/weights (into u/w). Split from prim_g_root so
// the Enzyme-differentiated rys_roots_ad runs once per primitive, not per root.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn prim_prep(
    nroots: usize,
    aij: f64,
    akl: f64,
    fac: f64,
    rij: [f64; 3],
    rkl: [f64; 3],
    u: &mut [f64; NRMAX],
    w: &mut [f64; NRMAX],
) -> (f64, f64, [f64; 3]) {
    let a1 = aij * akl;
    let a0 = a1 / (aij + akl);
    let fac1 = (a0 / (a1 * a1 * a1)).sqrt() * fac;
    let rijrkl = [rij[0] - rkl[0], rij[1] - rkl[1], rij[2] - rkl[2]];
    let rr = rijrkl[0] * rijrkl[0] + rijrkl[1] * rijrkl[1] + rijrkl[2] * rijrkl[2];
    let x = a0 * rr;
    // nroots <= 5 and extreme x: c2rust closed-form branches; nroots 6-9
    // mid-range: Chebyshev tables (the c2rust eigensolve there is not Enzyme-
    // safe, and for 8-9 fails outright even in forward mode).
    crate::rys_tab::rys_roots_ad(nroots, x, u, w);
    (a0, fac1, rijrkl)
}

// 2D VRR for ONE cartesian axis (CINTg0_2e_2d with dn=dj, dm=dl), single root
// at buffer base 0. Split out of prim_g_root so the three axes are three
// straight-line inlined bodies instead of one loop over a phi of the three
// buffer pointers.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn vrr_axis(
    gax: &mut [f64],
    cn: f64,
    cm: f64,
    b00: f64,
    b10: f64,
    b01: f64,
    nmax: usize,
    mmax: usize,
    dl: usize,
    dj: usize,
) {
    if nmax > 0 {
        let mut s0 = gax[0];
        let mut s1 = cn * s0;
        gax[dj] = s1;
        for n in 1..nmax {
            let s2 = cn * s1 + (n as f64) * b10 * s0;
            gax[(n + 1) * dj] = s2;
            s0 = s1;
            s1 = s2;
        }
    }
    if mmax > 0 {
        let mut s0 = gax[0];
        let mut s1 = cm * s0;
        gax[dl] = s1;
        for m in 1..mmax {
            let s2 = cm * s1 + (m as f64) * b01 * s0;
            gax[(m + 1) * dl] = s2;
            s0 = s1;
            s1 = s2;
        }
        if nmax > 0 {
            let mut s0 = gax[dj];
            let mut s1 = cm * s0 + b00 * gax[0];
            gax[dj + dl] = s1;
            for m in 1..mmax {
                let s2 = cm * s1 + (m as f64) * b01 * s0 + b00 * gax[m * dl];
                gax[dj + (m + 1) * dl] = s2;
                s0 = s1;
                s1 = s2;
            }
        }
    }
    if nmax > 1 {
        for m in 1..mmax + 1 {
            let base = m * dl;
            let mut s0 = gax[base];
            let mut s1 = gax[base + dj];
            for n in 1..nmax {
                let s2 =
                    cn * s1 + (n as f64) * b10 * s0 + (m as f64) * b00 * gax[base + n * dj - dl];
                gax[base + (n + 1) * dj] = s2;
                s0 = s1;
                s1 = s2;
            }
        }
    }
}

// HRR (CINTg0_lj2d_4d) for ONE cartesian axis: j -> i, then l -> k.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn hrr_axis(
    gax: &mut [f64],
    rij_x: f64,
    rkl_x: f64,
    li: usize,
    lj: usize,
    lk: usize,
    nmax: usize,
    mmax: usize,
    di: usize,
    dk: usize,
    dl: usize,
    dj: usize,
) {
    // HRR shift coefficients are B-A and D-C (momentum moves from j to i,
    // from l to k).
    let rx = -rij_x;
    for i in 1..li + 1 {
        for j in 0..nmax - i + 1 {
            for l in 0..mmax + 1 {
                let p = j * dj + l * dl + i * di;
                gax[p] = rx * gax[p - di] + gax[p - di + dj];
            }
        }
    }
    let rx = -rkl_x;
    for j in 0..lj + 1 {
        for k in 1..lk + 1 {
            for l in 0..mmax - k + 1 {
                let p = j * dj + l * dl + k * dk;
                for n in p..p + dk {
                    gax[n] = rx * gax[n - dk] + gax[n - dk + dl];
                }
            }
        }
    }
}

// The loop-invariant *shape* of a quartet: the angular momenta and the g-buffer
// strides derived from them. Split out of QuartetCtx (which also carries the
// active f64 geometry) so it can be built from compile-time-constant l values
// in the specialized dispatch below -- every loop bound, every branch and every
// `j*dj + l*dl` index in the VRR/HRR then folds at compile time.
#[derive(Clone, Copy)]
struct Shape {
    li: usize,
    lj: usize,
    lk: usize,
    nfi: usize,
    nfj: usize,
    nfk: usize,
    nfl: usize,
    nmax: usize,
    mmax: usize,
    nroots: usize,
    di: usize,
    dk: usize,
    dl: usize,
    dj: usize,
}

impl Shape {
    #[inline(always)]
    fn new(li: usize, lj: usize, lk: usize, ll: usize) -> Shape {
        let nmax = li + lj;
        let mmax = lk + ll;
        // lj2d4d strides: di=1 (single root; the root loop is in the caller),
        // dk=li+1, dl=dk*(lk+1), dj=dl*(mmax+1).
        let di = 1;
        let dk = di * (li + 1);
        let dl = dk * (lk + 1);
        let dj = dl * (mmax + 1);
        Shape {
            li,
            lj,
            lk,
            nfi: (li + 1) * (li + 2) / 2,
            nfj: (lj + 1) * (lj + 2) / 2,
            nfk: (lk + 1) * (lk + 2) / 2,
            nfl: (ll + 1) * (ll + 2) / 2,
            nmax,
            mmax,
            nroots: (nmax + mmax) / 2 + 1,
            di,
            dk,
            dl,
            dj,
        }
    }
}

// One rys root of one primitive quartet: 2D VRR + HRR into the per-axis,
// per-root g-buffers (di == 1: a single root, so buffer size no longer scales
// with nroots). gz carries the w*fac1 weight. The caller loops roots and
// accumulates each root's gout product.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn prim_g_root(
    gx: &mut [f64; GSIZE_MAX],
    gy: &mut [f64; GSIZE_MAX],
    gz: &mut [f64; GSIZE_MAX],
    q: &QuartetCtx,
    s: Shape,
    aij: f64,
    akl: f64,
    a0: f64,
    fac1: f64,
    u_r: f64,
    w_r: f64,
    rij: [f64; 3],
    rkl: [f64; 3],
    rijrkl: [f64; 3],
) {
    let (nmax, mmax) = (s.nmax, s.mmax);
    let (di, dk, dl, dj) = (s.di, s.dk, s.dl, s.dj);
    let a1 = aij * akl;

    let u2 = a0 * u_r;
    let tmp4 = 0.5 / (u2 * (aij + akl) + a1);
    let tmp5 = u2 * tmp4;
    let tmp1 = 2.0 * tmp5;
    let tmp2 = tmp1 * akl;
    let tmp3 = tmp1 * aij;
    let b00 = tmp5;
    let b10 = tmp5 + tmp4 * akl;
    let b01 = tmp5 + tmp4 * aij;
    // VRR builds momentum on centers j (bra) and l (ket):
    // c00 = (P - Rj) - tmp2*(P - Q), c0p = (Q - Rl) + tmp3*(P - Q)
    let c00 = [
        rij[0] - q.rj[0] - tmp2 * rijrkl[0],
        rij[1] - q.rj[1] - tmp2 * rijrkl[1],
        rij[2] - q.rj[2] - tmp2 * rijrkl[2],
    ];
    let c0p = [
        rkl[0] - q.rl[0] + tmp3 * rijrkl[0],
        rkl[1] - q.rl[1] + tmp3 * rijrkl[1],
        rkl[2] - q.rl[2] + tmp3 * rijrkl[2],
    ];

    gx[0] = 1.0;
    gy[0] = 1.0;
    gz[0] = w_r * fac1;

    // 2D VRR (CINTg0_2e_2d with dn=dj, dm=dl), single root at buffer base 0
    vrr_axis(gx, c00[0], c0p[0], b00, b10, b01, nmax, mmax, dl, dj);
    vrr_axis(gy, c00[1], c0p[1], b00, b10, b01, nmax, mmax, dl, dj);
    vrr_axis(gz, c00[2], c0p[2], b00, b10, b01, nmax, mmax, dl, dj);

    // --- HRR (CINTg0_lj2d_4d): j -> i, then l -> k --- (di == 1, one root)
    let (li, lj, lk) = (s.li, s.lj, s.lk);
    if li > 0 || lk > 0 {
        let (rirj, rkrl) = (q.rirj, q.rkrl);
        hrr_axis(gx, rirj[0], rkrl[0], li, lj, lk, nmax, mmax, di, dk, dl, dj);
        hrr_axis(gy, rirj[1], rkrl[1], li, lj, lk, nmax, mmax, di, dk, dl, dj);
        hrr_axis(gz, rirj[2], rkrl[2], li, lj, lk, nmax, mmax, di, dk, dl, dj);
    }
}

// One contracted cartesian ERI shell quartet (ij|kl), out in cint2e_cart's
// layout out[I + ni*(J + nj*(K + nk*L))], I = fi + nfi*ci, ni = nfi*nctr_i.
//
// CONTRACT: `out` must arrive zero-initialized and is ACCUMULATED into. Do NOT
// zero it here: Enzyme turns a zeroing loop over the Duplicated output into a
// memset that also clears the shadow, zeroing every gradient. A local
// accumulator is no escape either -- the Enzyme pass SIGSEGVs on ~10k-double
// stack allocas (the LMAX=3 gout).
pub fn eri_cart(out: &mut [f64], shls: &[i32], atm: &[i32], bas: &[i32], env: &[f64]) {
    let q = quartet_ctx(shls, atm, bas, env);
    if q.nctri == 1 && q.nctrj == 1 && q.nctrk == 1 && q.nctrl == 1 {
        eri_cart_seg(out, &q, env);
    } else {
        eri_cart_gc(out, &q, env);
    }
}

// Segmented path (all nctr == 1). Dispatch on the four angular momenta so the
// body is instantiated with *constant* l values for the s/p quartets that make
// up essentially all of a minimal/split-valence basis. Everything the hot loops
// branch on or index with -- nmax, mmax, di/dk/dl/dj, nfi..nfl, nroots -- is
// then a compile-time constant in those instances, so the VRR/HRR loop bounds,
// the `if nmax > 0` style guards and the `j*dj + l*dl` address arithmetic all
// fold away instead of being recomputed (and, in the Enzyme reverse, taped).
// The `_` arm keeps the fully dynamic body for d/f/g shells.
//
// HARD LIMIT ON THE ARM COUNT: at **8** specialized arms the Enzyme reverse
// SIGSEGVs at runtime (garbage `gx` base pointer in the primal called from
// `dtwo_ad`; the primal binary alone is fine, so it is Enzyme, not this code).
// 7 arms still work, so 6 is deliberately two below the cliff. Measured on
// c6h6/STO-3G: 0 arms 66.5e9 insns, 1 arm 64.3e9, 2 arms 63.9e9, 4 arms 60.8e9,
// 6 arms 58.7e9, 7 arms 58.1e9, 8 arms CRASH. Do not add arms without
// re-running `gradtime fd`.
fn eri_cart_seg(out: &mut [f64], q: &QuartetCtx, env: &[f64]) {
    // ONE set of g-buffers for every instantiation below. They must be
    // allocated here, not in `seg_body`: `seg_body` is inlined once per
    // specialization, and a per-copy `[f64; GSIZE_MAX]` (plus its Enzyme
    // shadow) multiplies the stack frame by the number of arms -- which the
    // frame's page-probe loop then walks on *every* call. Measured: 12.7x
    // slower with per-copy buffers.
    let mut gx = [0.0f64; GSIZE_MAX];
    let mut gy = [0.0f64; GSIZE_MAX];
    let mut gz = [0.0f64; GSIZE_MAX];

    macro_rules! spec {
        ($(($li:literal, $lj:literal, $lk:literal, $ll:literal)),* $(,)?) => {
            match (q.li, q.lj, q.lk, q.ll) {
                $(($li, $lj, $lk, $ll) =>
                    seg_body(out, q, env, &mut gx, &mut gy, &mut gz, $li, $lj, $lk, $ll),)*
                _ => seg_body(out, q, env, &mut gx, &mut gy, &mut gz, q.li, q.lj, q.lk, q.ll),
            }
        };
    }
    spec![
        (0, 0, 0, 0),
        (1, 1, 1, 1),
        (1, 1, 0, 0),
        (0, 0, 1, 1),
        (1, 0, 1, 0),
        (0, 1, 0, 1),
    ]
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn seg_body(
    out: &mut [f64],
    q: &QuartetCtx,
    env: &[f64],
    gx: &mut [f64; GSIZE_MAX],
    gy: &mut [f64; GSIZE_MAX],
    gz: &mut [f64; GSIZE_MAX],
    li: usize,
    lj: usize,
    lk: usize,
    ll: usize,
) {
    let s = Shape::new(li, lj, lk, ll);
    let (nfi, nfj, nfk, nfl) = (s.nfi, s.nfj, s.nfk, s.nfl);
    let (di, dk, dl, dj) = (s.di, s.dk, s.dl, s.dj);
    let nroots = s.nroots;

    let (i_nx, i_ny, i_nz) = cart_comp(li);
    let (j_nx, j_ny, j_nz) = cart_comp(lj);
    let (k_nx, k_ny, k_nz) = cart_comp(lk);
    let (l_nx, l_ny, l_nz) = cart_comp(ll);

    for lp in 0..q.l_prim {
        let al = env[q.pal + lp];
        let fac1l = q.common_factor * env[q.pcl + lp];
        let log_maxcl = env[q.pcl + lp].abs().ln();
        for kp in 0..q.k_prim {
            let ak = env[q.pak + kp];
            let akl = ak + al;
            let ekl_val = q.rr_kl * ak * al / akl;
            let ccekl = ekl_val - q.log_rr_kl - env[q.pck + kp].abs().ln() - log_maxcl;
            if ccekl > q.expcutoff {
                continue;
            }
            let rkl = [
                (ak * q.rk[0] + al * q.rl[0]) / akl,
                (ak * q.rk[1] + al * q.rl[1]) / akl,
                (ak * q.rk[2] + al * q.rl[2]) / akl,
            ];
            let eijcutoff = q.expcutoff - ccekl;
            let ekl = (-ekl_val).exp();
            let fac1k = fac1l * env[q.pck + kp];
            for jp in 0..q.j_prim {
                let aj = env[q.paj + jp];
                let fac1j = fac1k * env[q.pcj + jp];
                let log_maxcj = env[q.pcj + jp].abs().ln();
                for ip in 0..q.i_prim {
                    let ai = env[q.pai + ip];
                    let aij = ai + aj;
                    let eij_val = q.rr_ij * ai * aj / aij;
                    let cceij = eij_val - q.log_rr_ij - env[q.pci + ip].abs().ln() - log_maxcj;
                    if cceij > eijcutoff {
                        continue;
                    }
                    let wj = aj / aij;
                    let rij = [
                        q.ri[0] + wj * (q.rj[0] - q.ri[0]),
                        q.ri[1] + wj * (q.rj[1] - q.ri[1]),
                        q.ri[2] + wj * (q.rj[2] - q.ri[2]),
                    ];
                    let fac = fac1j * env[q.pci + ip] * (-eij_val).exp() * ekl;

                    let mut u = [0.0f64; NRMAX];
                    let mut w = [0.0f64; NRMAX];
                    let (a0, fac1, rijrkl) =
                        prim_prep(nroots, aij, akl, fac, rij, rkl, &mut u, &mut w);

                    // one rys root at a time: fill the per-root g-buffers, then
                    // accumulate this root's gout product straight into out
                    // (summing over roots via the += is the CINTgout2e root sum)
                    for r in 0..nroots {
                        prim_g_root(
                            gx, gy, gz, q, s, aij, akl, a0, fac1, u[r], w[r], rij, rkl, rijrkl,
                        );
                        for jf in 0..nfj {
                            let oj = [j_nx[jf] * dj, j_ny[jf] * dj, j_nz[jf] * dj];
                            for lf in 0..nfl {
                                let ol = [
                                    oj[0] + l_nx[lf] * dl,
                                    oj[1] + l_ny[lf] * dl,
                                    oj[2] + l_nz[lf] * dl,
                                ];
                                for kf in 0..nfk {
                                    let ok = [
                                        ol[0] + k_nx[kf] * dk,
                                        ol[1] + k_ny[kf] * dk,
                                        ol[2] + k_nz[kf] * dk,
                                    ];
                                    for if_ in 0..nfi {
                                        let ix = ok[0] + i_nx[if_] * di;
                                        let iy = ok[1] + i_ny[if_] * di;
                                        let iz = ok[2] + i_nz[if_] * di;
                                        out[if_ + nfi * (jf + nfj * (kf + nfk * lf))] +=
                                            gx[ix] * gy[iy] * gz[iz];
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// General-contraction path (any nctr > 1): coefficients are NOT folded into
// the prefactor; per-contraction partials accumulate hierarchically after
// each primitive loop level (CINTprim_to_ctr analog). All accumulators are
// fresh heap Vecs with linear indexed `+=` only -- the access pattern Enzyme
// handles (the strided-VRR-into-malloc pattern trips its TypeAnalysis, and
// re-zeroing a live accumulator risks the shadow-memset bug, so buffers are
// re-created per loop iteration instead).
fn eri_cart_gc(out: &mut [f64], q: &QuartetCtx, env: &[f64]) {
    let s = Shape::new(q.li, q.lj, q.lk, q.ll);
    let (nfi, nfj, nfk, nfl) = (q.nfi, q.nfj, q.nfk, q.nfl);
    let (nctri, nctrj, nctrk, nctrl) = (q.nctri, q.nctrj, q.nctrk, q.nctrl);
    let (di, dk, dl, dj) = (q.di, q.dk, q.dl, q.dj);
    let nroots = q.nroots;
    let nf = nfi * nfj * nfk * nfl;

    let ni = nfi * nctri;
    let nj = nfj * nctrj;
    let nk = nfk * nctrk;
    let nl = nfl * nctrl;
    assert!(out.len() >= ni * nj * nk * nl);

    let mut gx = [0.0f64; GSIZE_MAX];
    let mut gy = [0.0f64; GSIZE_MAX];
    let mut gz = [0.0f64; GSIZE_MAX];

    let (i_nx, i_ny, i_nz) = cart_comp(q.li);
    let (j_nx, j_ny, j_nz) = cart_comp(q.lj);
    let (k_nx, k_ny, k_nz) = cart_comp(q.lk);
    let (l_nx, l_ny, l_nz) = cart_comp(q.ll);

    // screening uses the largest |coefficient| over the contractions of a
    // primitive (CINTset_pairdata's log_maxc semantics)
    let log_maxc = |pc: usize, nprim: usize, nctr: usize, p: usize| -> f64 {
        let mut m = 0.0f64;
        for c in 0..nctr {
            let a = env[pc + c * nprim + p].abs();
            if a > m {
                m = a;
            }
        }
        m.ln()
    };

    let mut gc4 = vec![0.0f64; nf * nctri * nctrj * nctrk * nctrl];
    for lp in 0..q.l_prim {
        let al = env[q.pal + lp];
        let log_maxcl = log_maxc(q.pcl, q.l_prim, nctrl, lp);
        let mut gc3 = vec![0.0f64; nf * nctri * nctrj * nctrk];
        for kp in 0..q.k_prim {
            let ak = env[q.pak + kp];
            let akl = ak + al;
            let ekl_val = q.rr_kl * ak * al / akl;
            let ccekl = ekl_val - q.log_rr_kl - log_maxc(q.pck, q.k_prim, nctrk, kp) - log_maxcl;
            if ccekl > q.expcutoff {
                continue;
            }
            let rkl = [
                (ak * q.rk[0] + al * q.rl[0]) / akl,
                (ak * q.rk[1] + al * q.rl[1]) / akl,
                (ak * q.rk[2] + al * q.rl[2]) / akl,
            ];
            let eijcutoff = q.expcutoff - ccekl;
            let ekl = (-ekl_val).exp();
            let mut gc2 = vec![0.0f64; nf * nctri * nctrj];
            for jp in 0..q.j_prim {
                let aj = env[q.paj + jp];
                let log_maxcj = log_maxc(q.pcj, q.j_prim, nctrj, jp);
                let mut gc1 = vec![0.0f64; nf * nctri];
                for ip in 0..q.i_prim {
                    let ai = env[q.pai + ip];
                    let aij = ai + aj;
                    let eij_val = q.rr_ij * ai * aj / aij;
                    let cceij =
                        eij_val - q.log_rr_ij - log_maxc(q.pci, q.i_prim, nctri, ip) - log_maxcj;
                    if cceij > eijcutoff {
                        continue;
                    }
                    let wj = aj / aij;
                    let rij = [
                        q.ri[0] + wj * (q.rj[0] - q.ri[0]),
                        q.ri[1] + wj * (q.rj[1] - q.ri[1]),
                        q.ri[2] + wj * (q.rj[2] - q.ri[2]),
                    ];
                    let fac = q.common_factor * (-eij_val).exp() * ekl;

                    let mut u = [0.0f64; NRMAX];
                    let mut w = [0.0f64; NRMAX];
                    let (a0, fac1, rijrkl) =
                        prim_prep(nroots, aij, akl, fac, rij, rkl, &mut u, &mut w);

                    // one rys root at a time; gc1 accumulates over roots too,
                    // weighted by the i-shell contraction coefficients
                    for r in 0..nroots {
                        prim_g_root(
                            &mut gx, &mut gy, &mut gz, q, s, aij, akl, a0, fac1, u[r], w[r], rij,
                            rkl, rijrkl,
                        );
                        for jf in 0..nfj {
                            let oj = [j_nx[jf] * dj, j_ny[jf] * dj, j_nz[jf] * dj];
                            for lf in 0..nfl {
                                let ol = [
                                    oj[0] + l_nx[lf] * dl,
                                    oj[1] + l_ny[lf] * dl,
                                    oj[2] + l_nz[lf] * dl,
                                ];
                                for kf in 0..nfk {
                                    let ok = [
                                        ol[0] + k_nx[kf] * dk,
                                        ol[1] + k_ny[kf] * dk,
                                        ol[2] + k_nz[kf] * dk,
                                    ];
                                    for if_ in 0..nfi {
                                        let ix = ok[0] + i_nx[if_] * di;
                                        let iy = ok[1] + i_ny[if_] * di;
                                        let iz = ok[2] + i_nz[if_] * di;
                                        let s = gx[ix] * gy[iy] * gz[iz];
                                        let idx = if_ + nfi * (jf + nfj * (kf + nfk * lf));
                                        for ci in 0..nctri {
                                            gc1[ci * nf + idx] +=
                                                env[q.pci + ci * q.i_prim + ip] * s;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                for cj in 0..nctrj {
                    let cc = env[q.pcj + cj * q.j_prim + jp];
                    let o = cj * nf * nctri;
                    for m in 0..nf * nctri {
                        gc2[o + m] += cc * gc1[m];
                    }
                }
            }
            for ck in 0..nctrk {
                let cc = env[q.pck + ck * q.k_prim + kp];
                let o = ck * nf * nctri * nctrj;
                for m in 0..nf * nctri * nctrj {
                    gc3[o + m] += cc * gc2[m];
                }
            }
        }
        for cl in 0..nctrl {
            let cc = env[q.pcl + cl * q.l_prim + lp];
            let o = cl * nf * nctri * nctrj * nctrk;
            for m in 0..nf * nctri * nctrj * nctrk {
                gc4[o + m] += cc * gc3[m];
            }
        }
    }

    // gc4 blocks [cl][ck][cj][ci][fi-fastest nf] -> c2s_cart_2e1 out layout
    for cl in 0..nctrl {
        for ck in 0..nctrk {
            for cj in 0..nctrj {
                for ci in 0..nctri {
                    let base = (((cl * nctrk + ck) * nctrj + cj) * nctri + ci) * nf;
                    let mut n = 0;
                    for lf in 0..nfl {
                        let ol = ni * nj * nk * (lf + nfl * cl);
                        for kf in 0..nfk {
                            let ok = ol + ni * nj * (kf + nfk * ck);
                            for jf in 0..nfj {
                                let oj = ok + ni * (jf + nfj * cj);
                                for fi in 0..nfi {
                                    out[oj + fi + nfi * ci] += gc4[base + n];
                                    n += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// NOTE: correctness is validated harness-free -- cargo test can't link the
// cdylib-only lib and the libtest harness miscompiles under -Zautodiff. The
// full FD gradient check (f shells + general contraction) lives in the
// self-contained probe-eri/ crate.
