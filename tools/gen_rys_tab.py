#!/usr/bin/env python3
"""Generate src/rys_tab.rs: piecewise-Chebyshev fits of the Rys quadrature
roots/weights for nroots 6 and 7 over the mid-range x where CINTrys_roots
falls back to the iterative Schmidt/Jacobi eigensolvers (which Enzyme cannot
safely reverse-differentiate; the small-x and large-x branches are closed
form and stay as-is).

Math. The n-point Rys quadrature satisfies, for polynomials f of degree
<= 2n-1,
    int_0^1 f(t^2) exp(-x t^2) dt = sum_r w_r f(t_r^2).
Substituting v = t^2 it is Gauss quadrature for the measure
dmu(v) = (1/2) v^(-1/2) exp(-x v) dv on [0,1], with moments
    mu_k(x) = int_0^1 t^(2k) exp(-x t^2) dt
            = gammainc_lower(k+1/2, x) / (2 x^(k+1/2)).
Nodes v_r(x) and weights w_r(x) are smooth in the single scalar x, so we
fit them here once in 40-digit arithmetic (Hankel Cholesky -> orthonormal
polynomials -> polyroots -> Christoffel weights) and emit Rust const
tables. CINTrys_roots returns u = t^2/(1-t^2) (proof: rys_root1 returns
F1/(WW1-F1) = mu1/(mu0-mu1) = v/(1-v); the large-x branch returns
rt/(x-rt) with v ~ rt/x). The tables store v; Rust applies u = v/(1-v).

Usage:
    python gen_rys_tab.py --so target/release/librint-node.so \
        --out src/rys_tab.rs --out2 ../probe-eri/src/rys_tab.rs
"""

import argparse
import ctypes
import math
import multiprocessing as mp_pool
import os
import sys

from mpmath import mp, mpf, matrix, cholesky, polyroots, gammainc

DPS = 40
# nroots needing tables: c2rust CINTrys_roots dispatches these to the
# Enzyme-unsafe Schmidt/Jacobi eigensolve in the mid-range. 6-7 cover l<=3
# quartets; 8-9 add l=4 (an all-g quartet needs nroots=(8+8)/2+1=9).
NROOTS_TAB = (6, 7, 8, 9)
SMALLX = 3e-7
# Fit acceptance: absolute error of the double-precision Clenshaw evaluation
# against the 40-digit reference. Roots and weights are both O(1) or smaller
# on the fitted range, so a flat absolute gate is meaningful.
TOL = 2e-14
# (segment length, chebyshev degree) ladder, tried in order until TOL holds.
CONFIGS = [(2.5, 14), (2.5, 16), (1.25, 14), (1.25, 16), (0.625, 16),
           (0.3125, 16), (0.3125, 18)]  # finer fallbacks for nroots 8-9
VAL_PER_SEG = 24  # dense validation points per segment


def cutoff(n):
    return float(35 + 5 * n)


def moments(n2, x):
    """mu_k(x) for k = 0..n2 at mp precision."""
    xm = mpf(x)
    if xm == 0:
        return [mpf(1) / (2 * k + 1) for k in range(n2 + 1)]
    mus = []
    for k in range(n2 + 1):
        a = mpf(k) + mpf(1) / 2
        mus.append(gammainc(a, 0, xm) / (2 * xm ** a))
    return mus


def rys_vw(n, x):
    """Nodes v_r = t_r^2 (ascending) and weights w_r of the n-point Rys
    quadrature, at mp precision."""
    mus = moments(2 * n, x)
    m = n + 1
    G = matrix(m, m)
    for i in range(m):
        for j in range(m):
            G[i, j] = mus[i + j]
    L = cholesky(G)
    # C = L^-1 (lower): orthonormal polys p_k(v) = sum_i C[k][i] v^i.
    C = [[mpf(0)] * m for _ in range(m)]
    for j in range(m):
        C[j][j] = 1 / L[j, j]
        for i in range(j + 1, m):
            s = mpf(0)
            for k in range(j, i):
                s += L[i, k] * C[k][j]
            C[i][j] = -s / L[i, i]
    pn = C[n]  # coefficients of p_n, ascending powers

    roots = polyroots(list(reversed(pn)), maxsteps=200, extraprec=80)
    vs = []
    for r in roots:
        assert abs(r.imag) < mpf(10) ** (-25), (n, x, r)
        v = r.real
        assert 0 < v < 1, (n, x, v)
        vs.append(v)
    vs.sort()

    ws = []
    for v in vs:
        s = mpf(0)
        for k in range(m):  # include p_n: ~0 at its own root, harmless
            pk = mpf(0)
            for i in range(k, -1, -1):
                pk = pk * v + C[k][i]
            s += pk * pk
        ws.append(1 / s)

    # self-check: quadrature reproduces the moments it was built from
    for k in range(2 * n):
        q = sum(w * v ** k for v, w in zip(vs, ws))
        assert abs(q - mus[k]) < mpf(10) ** (-28), (n, x, k, q - mus[k])
    return vs, ws


def _pool_init():
    mp.dps = DPS


def _pool_eval(job):
    n, x = job
    vs, ws = rys_vw(n, x)
    return (n, x, [mp.nstr(v, 30) for v in vs], [mp.nstr(w, 30) for w in ws])


def eval_many(jobs, procs):
    """{(n, x): (vs, ws)} with mpf values reconstructed from 30-digit strings
    (fit target is 2e-14 absolute; 30 digits is far beyond that)."""
    out = {}
    with mp_pool.Pool(procs, initializer=_pool_init) as pool:
        for n, x, vs, ws in pool.imap_unordered(_pool_eval, jobs, chunksize=4):
            out[(n, x)] = ([mpf(s) for s in vs], [mpf(s) for s in ws])
    return out


def cheb_nodes(deg):
    npts = deg + 1
    return [mp.cos(mp.pi * (mpf(2 * j + 1) / (2 * npts))) for j in range(npts)]


def cheb_coeffs(fvals, deg):
    """Chebyshev interpolation coefficients from values at the deg+1
    first-kind nodes (same ordering as cheb_nodes)."""
    npts = deg + 1
    cs = []
    for i in range(npts):
        s = mpf(0)
        for j in range(npts):
            s += fvals[j] * mp.cos(mp.pi * i * mpf(2 * j + 1) / (2 * npts))
        c = 2 * s / npts
        if i == 0:
            c /= 2
        cs.append(float(c))
    return cs


def clenshaw(cs, t):
    """Double-precision Clenshaw, mirroring the emitted Rust exactly."""
    b1 = 0.0
    b2 = 0.0
    t2 = 2.0 * t
    for i in range(len(cs) - 1, 0, -1):
        b0 = cs[i] + t2 * b1 - b2
        b2 = b1
        b1 = b0
    return cs[0] + t * b1 - b2


def fit_n(n, procs):
    """Fit nroots=n. Returns (nseg, deg, rt_coeffs, ww_coeffs, report) where
    the coeff arrays are flat [seg][root][coef] lists of Python floats."""
    cut = cutoff(n)
    for seglen0, deg in CONFIGS:
        nseg = int(math.ceil(cut / seglen0))
        seglen = cut / nseg
        nodes = cheb_nodes(deg)

        jobs = []
        for s in range(nseg):
            mid = (s + mpf(1) / 2) * mpf(seglen)
            half = mpf(seglen) / 2
            for u in nodes:
                jobs.append((n, float(mid + half * u)))
        for s in range(nseg):
            for j in range(VAL_PER_SEG):
                xv = (s + (j + 0.5) / VAL_PER_SEG) * seglen
                if xv > SMALLX:
                    jobs.append((n, xv))
        ref = eval_many(sorted(set(jobs)), procs)

        rt = [0.0] * (nseg * n * (deg + 1))
        ww = [0.0] * (nseg * n * (deg + 1))
        for s in range(nseg):
            mid = (s + mpf(1) / 2) * mpf(seglen)
            half = mpf(seglen) / 2
            samples = [ref[(n, float(mid + half * u))] for u in nodes]
            for r in range(n):
                base = (s * n + r) * (deg + 1)
                rt[base:base + deg + 1] = cheb_coeffs(
                    [vs[r] for vs, _ in samples], deg)
                ww[base:base + deg + 1] = cheb_coeffs(
                    [ws[r] for _, ws in samples], deg)

        err_rt = 0.0
        err_ww = 0.0
        for s in range(nseg):
            for j in range(VAL_PER_SEG):
                xv = (s + (j + 0.5) / VAL_PER_SEG) * seglen
                if xv <= SMALLX:
                    continue
                vs, ws = ref[(n, xv)]
                t = (xv - (s + 0.5) * seglen) * (2.0 / seglen)
                for r in range(n):
                    base = (s * n + r) * (deg + 1)
                    e1 = abs(clenshaw(rt[base:base + deg + 1], t) - float(vs[r]))
                    e2 = abs(clenshaw(ww[base:base + deg + 1], t) - float(ws[r]))
                    err_rt = max(err_rt, e1)
                    err_ww = max(err_ww, e2)
        report = (f"nroots={n}: nseg={nseg} deg={deg} seglen={seglen:.4f} "
                  f"max|drt|={err_rt:.3e} max|dww|={err_ww:.3e}")
        print(report, flush=True)
        if err_rt <= TOL and err_ww <= TOL:
            return nseg, deg, rt, ww, report
    raise RuntimeError(f"no config met TOL={TOL} for nroots={n}")


def crosscheck_so(so_path):
    """Convention/sanity check against the c2rust CINTrys_roots (double
    precision, so agreement ~1e-12 is expected; gate at 1e-9)."""
    lib = ctypes.CDLL(so_path)
    fn = lib.CINTrys_roots
    fn.restype = None
    fn.argtypes = [ctypes.c_int, ctypes.c_double,
                   ctypes.POINTER(ctypes.c_double),
                   ctypes.POINTER(ctypes.c_double)]
    worst = 0.0
    nmax = max(NROOTS_TAB)
    for n in range(1, nmax + 1):
        cut = cutoff(n)
        for x in (1e-7, 1e-6, 0.05, 0.9, 4.0, 11.5, 19.0, 33.0,
                  cut - 1.0, cut + 15.0):
            # c2rust's mid-range eigensolve is broken for n>=8 (calls
            # process::exit); rys_roots_ad never forwards those x there (tables
            # cover them), so only crosscheck the extreme-x branches for n>=8.
            if n >= 8 and SMALLX < x < cut:
                continue
            u = (ctypes.c_double * n)()
            w = (ctypes.c_double * n)()
            fn(n, x, u, w)
            vs, ws = rys_vw(n, x)
            for r in range(n):
                u_ref = float(vs[r] / (1 - vs[r]))
                du = abs(u[r] - u_ref) / max(abs(u_ref), 1e-300)
                dw = abs(w[r] - float(ws[r])) / max(abs(float(ws[r])), 1e-300)
                worst = max(worst, du, dw)
                if du > 1e-9 or dw > 1e-9:
                    print(f"MISMATCH n={n} x={x} r={r}: "
                          f"u {u[r]!r} vs {u_ref!r}, w {w[r]!r} vs {float(ws[r])!r}")
                    raise SystemExit(1)
    print(f"crosscheck vs {os.path.basename(so_path)}: n=1..{nmax} OK "
          f"(worst rel dev {worst:.3e}; c2rust is double precision)", flush=True)


def emit_array(f, name, vals):
    f.write(f"static {name}: [f64; {len(vals)}] = [\n")
    for i in range(0, len(vals), 4):
        f.write("    " + ", ".join(repr(v) for v in vals[i:i + 4]) + ",\n")
    f.write("];\n")


def emit(path, fits, reports):
    with open(path, "w") as f:
        f.write("""\
// AUTOGENERATED by tools/gen_rys_tab.py -- DO NOT EDIT.
//
// Piecewise-Chebyshev fits of the Rys quadrature roots/weights for nroots 6
// and 7 on the mid-range x where the c2rust CINTrys_roots dispatches to the
// iterative Schmidt/Jacobi eigensolvers. Enzyme's reverse of those solvers
// corrupts memory (jobs 30043103/30043677), so the AD path evaluates these
// tables instead: fixed-degree polynomials, branch-free per segment, exactly
// differentiable. The small-x (<= 3e-7) and large-x (>= 35+5n) branches of
// CINTrys_roots are closed-form and generic over nroots; they stay in use.
//
// Convention matches CINTrys_roots: u = v/(1-v) with v = t^2 the Gauss node
// for the measure (1/2) v^(-1/2) exp(-x v) dv on [0,1]; w are the plain
// quadrature weights (sum_r w_r = F0-like zeroth moment). Tables store v.
//
// Fit quality (absolute error of double Clenshaw vs 40-digit reference):
""")
        for rep in reports:
            f.write(f"//   {rep}\n")
        f.write("""
use crate::rys_roots::CINTrys_roots;

#[inline]
fn cheb(c: &[f64], t: f64) -> f64 {
    let mut b1 = 0.0f64;
    let mut b2 = 0.0f64;
    let t2 = 2.0 * t;
    let mut i = c.len() - 1;
    while i > 0 {
        let b0 = c[i] + t2 * b1 - b2;
        b2 = b1;
        b1 = b0;
        i -= 1;
    }
    c[0] + t * b1 - b2
}

""")
        # Single FLAT tables + integer offset arrays. A match over separate
        # per-nroots statics (RT6..RT9) feeding the differentiated cheb() gives
        # the slice 4-way pointer provenance and SIGSEGVs Enzyme's TypeAnalysis
        # (jobs 30614292/30614429: 6,7 build, 6-9 crash). One static = one
        # provenance; nroots selects an integer offset, not a pointer.
        maxn = max(fits)
        rt_all, ww_all, off = [], [], {}
        for n in sorted(fits):
            off[n] = len(rt_all)
            _, _, rt, ww = fits[n]
            rt_all.extend(rt)
            ww_all.extend(ww)
        emit_array(f, "RT_ALL", rt_all)
        emit_array(f, "WW_ALL", ww_all)

        def idx_arr(name, valfn):
            vals = [valfn(n) if n in fits else 0 for n in range(maxn + 1)]
            f.write(f"static {name}: [usize; {maxn + 1}] "
                    f"= [{', '.join(str(v) for v in vals)}];\n")

        idx_arr("RYS_OFF", lambda n: off[n])
        idx_arr("RYS_NSEG", lambda n: fits[n][0])
        idx_arr("RYS_DEG", lambda n: fits[n][1])
        f.write(f"""
// Drop-in replacement for CINTrys_roots on the AD path. nroots <= 5 and the
// closed-form extreme-x branches forward to the c2rust dispatch (safe under
// Enzyme). Mid-range 6..={maxn} index the single flat RT_ALL/WW_ALL tables via
// an integer offset -- a match over separate per-nroots statics feeding the
// differentiated cheb() crashes Enzyme's TypeAnalysis on the merged provenance.
pub fn rys_roots_ad(nroots: usize, x: f64, u: &mut [f64], w: &mut [f64]) {{
    let cut = 35.0 + 5.0 * nroots as f64;
    if nroots <= 5 || nroots > {maxn} || x <= 3e-7 || x >= cut {{
        unsafe {{ CINTrys_roots(nroots as i32, x, u.as_mut_ptr(), w.as_mut_ptr()) }};
        return;
    }}
    let nseg = RYS_NSEG[nroots];
    let deg = RYS_DEG[nroots];
    let stride = deg + 1;
    let seglen = cut / nseg as f64;
    let mut k = (x / seglen) as usize;
    if k >= nseg {{
        k = nseg - 1;
    }}
    let t = (x - (k as f64 + 0.5) * seglen) * (2.0 / seglen);
    let base = RYS_OFF[nroots] + k * nroots * stride;
    for r in 0..nroots {{
        let b = base + r * stride;
        let v = cheb(&RT_ALL[b..b + stride], t);
        u[r] = v / (1.0 - v);
        w[r] = cheb(&WW_ALL[b..b + stride], t);
    }}
}}
""")
    print(f"wrote {path}", flush=True)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--so", help="librint .so for the convention crosscheck")
    ap.add_argument("--out", required=True)
    ap.add_argument("--out2")
    ap.add_argument("--procs", type=int, default=max(1, (os.cpu_count() or 4) - 2))
    args = ap.parse_args()

    mp.dps = DPS
    if args.so:
        crosscheck_so(args.so)
    else:
        print("WARNING: --so not given, skipping convention crosscheck")

    fits = {}
    reports = []
    for n in NROOTS_TAB:
        nseg, deg, rt, ww, rep = fit_n(n, args.procs)
        fits[n] = (nseg, deg, rt, ww)
        reports.append(rep)

    emit(args.out, fits, reports)
    if args.out2:
        emit(args.out2, fits, reports)
    print("OK", flush=True)


if __name__ == "__main__":
    main()
