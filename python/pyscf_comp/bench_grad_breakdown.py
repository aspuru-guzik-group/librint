"""Where does danalyticalf's wall time actually go?

danalyticalg = dHcoreg + dRg - 0.5 dSg, and dSg internally builds the Fock
matrix (getF -> integral2e_fock, an O(nbas^4) primal loop) to form the
energy-weighted density Q = P F P. Amdahl's law is decided by the terms that
stay serial, not by the ones that speed up, so this measures every term
separately.

That is not a rhetorical point. When dS and dR were the only threaded terms,
this script measured the remaining serial fraction at 0.153 on CH4/def2-svp --
a hard ceiling of 6.5x on the whole gradient no matter how many cores it was
given, with the 2e reverse itself scaling ~40x. getF was 14% of that and had
no parallel version at all, because it is a primal build rather than an Enzyme
reverse and so never came up while the autodiff loops were the subject.

getF is not exported on its own, so it is inferred:

    t(getF) ~= t(dscf.dSf) - t(dS_par @ 1 thread)

both of which run the same dS pair loop; only dSf additionally builds F and
forms P F P.

Usage: LIBRINT_SO=/path/to/librint.so python bench_grad_breakdown.py
       ... python bench_grad_breakdown.py --only C2H6/def2-tzvp --out shard.json

--only/--out put one system on one node so the four can be measured at once;
the per-system JSONs merge cleanly because each is keyed by "GEO/BASIS".
"""
import argparse
import ctypes
import json
import os
import platform
import time

import numpy as np
import pyscf

import librint
import librint.dscf
import librint.utils
from librint import library

from bench_fair import _cpu_model, _mem_limit_kb
from geometries import geometries

# same list, same order as bench_par_scaling.py, so the two JSONs can be shown
# side by side on one figure without a per-system caveat
SYSTEMS = [
    ("CH4", "def2-svp"),
    ("C3H8", "def2-svp"),
    ("C2H6", "def2-tzvp"),
    ("H2O", "def2-qzvp"),
]
OUT_JSON = "bench_breakdown_results.json"

_SIG = (
    ctypes.POINTER(ctypes.c_int), ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_int), ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double), ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double), ctypes.c_size_t,
    ctypes.c_size_t,
)
for _fn in (library.dS_par_c, library.dR_par_c):
    _fn.argtypes = _SIG
    _fn.restype = ctypes.POINTER(ctypes.c_double)


def build(geo, basis):
    atom = "\n".join(
        f"{a[0]} {0.529 * a[2][0]} {0.529 * a[2][1]} {0.529 * a[2][2]}"
        for a in geometries[geo]
    )
    mol = pyscf.gto.M(atom=atom, basis=basis, verbose=0)
    mol.cart = True
    return mol


def call_par(fn, mol, W, nthreads):
    atm, bas, env, _ = librint.utils.prep(mol)
    W = np.ascontiguousarray(W)
    s1, s2 = librint.utils.split(bas)
    ptr = fn(
        atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int)), atm.size,
        bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int)), bas.size,
        env.ctypes.data_as(ctypes.POINTER(ctypes.c_double)), env.size,
        W.ctypes.data_as(ctypes.POINTER(ctypes.c_double)), W.size,
        nthreads,
    )
    return librint.utils.take(ptr, (s2 - s1,))


def timed(label, fn):
    t0 = time.perf_counter()
    out = fn()
    dt = time.perf_counter() - t0
    print(f"    {label:26s} {dt:9.3f}s", flush=True)
    return dt, out


def select(only):
    """SYSTEMS, or the subset named as GEO/BASIS on the command line."""
    if not only:
        return SYSTEMS
    wanted = [s.strip() for arg in only for s in arg.split(",") if s.strip()]
    chosen = [(g, b) for g, b in SYSTEMS if f"{g}/{b}" in wanted]
    missing = set(wanted) - {f"{g}/{b}" for g, b in chosen}
    if missing:
        raise SystemExit(f"unknown system(s): {', '.join(sorted(missing))}")
    return chosen


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--only", action="append", metavar="GEO/BASIS",
                    help="measure only these systems (repeatable, or comma-"
                         "separated)")
    ap.add_argument("--out", default=OUT_JSON, help="results file")
    args = ap.parse_args()

    results = {"_meta": {"node": platform.node(),
                         "ncores": len(os.sched_getaffinity(0)),
                         "mem_limit_kb": _mem_limit_kb(),
                         "cpu_model": _cpu_model()}}
    for geo, basis in select(args.only):
        mol = build(geo, basis)
        mf = pyscf.scf.RHF(mol)
        mf.verbose = 0
        mf.conv_tol = 1e-10
        mf.max_cycle = 200
        mf.kernel()
        P = mf.make_rdm1()

        h = mol.intor("int1e_kin") + mol.intor("int1e_nuc")
        eri = mol.intor("int2e")
        F = (h + np.einsum("kl,ijkl->ij", P, eri)
             - 0.5 * np.einsum("kl,ikjl->ij", P, eri))
        Q = P @ F @ P

        print(f"\n{geo}/{basis}  nao={mol.nao}  nbas={mol.nbas}", flush=True)
        t_h, _ = timed("dHcoref (dT+dV, serial)", lambda: librint.dscf.dHcoref(mol, P))
        t_r, _ = timed("dRf     (2e, serial)", lambda: librint.dscf.dRf(mol, P))
        t_s, _ = timed("dSf     (getF+dS, serial)", lambda: librint.dscf.dSf(mol, P))
        t_s1, _ = timed("dS_par  (dS only, T=1)", lambda: call_par(library.dS_par_c, mol, Q, 1))
        t_r1, _ = timed("dR_par  (2e only,  T=1)", lambda: call_par(library.dR_par_c, mol, P, 1))
        t_tot, _ = timed("danalyticalf (total)", lambda: librint.dscf.danalyticalf(mol, P))

        t_getf = t_s - t_s1
        parts = [
            ("dHcore (dT+dV)", t_h, "dHcore_par"),
            ("dR 2e pair loop", t_r1, "dR_par"),
            ("getF = int2e_fock + PFP", t_getf, "fock2e_par (PFP serial)"),
            ("dS pair loop", t_s1, "dS_par"),
        ]
        acct = sum(p[1] for p in parts)
        print(f"    {'-' * 68}", flush=True)
        for name, t, status in parts:
            print(f"    {name:26s} {t:9.3f}s  {100 * t / acct:5.1f}%  {status}",
                  flush=True)
        print(f"    {'accounted':26s} {acct:9.3f}s  (danalyticalf {t_tot:.3f}s)",
              flush=True)

        # Why every nbas^4 term had to be threaded, not just the 2e reverse:
        # this is the ceiling that applied when dS and dR were the only ones
        # with a parallel version, no matter how many cores were thrown at it.
        was_serial = t_h + t_getf
        print(f"    when only dS+dR were threaded: serial fraction "
              f"{was_serial / acct:.3f} -> capped at {acct / was_serial:5.1f}x "
              f"at infinite threads", flush=True)
        print(f"    now threaded: all four. What is left serial is the O(nao^3) "
              f"P F P matmults and the nbas^2 1e primals inside getF.",
              flush=True)

        results[f"{geo}/{basis}"] = {
            "nao": int(mol.nao), "nbas": int(mol.nbas),
            "dHcore": t_h, "dR": t_r1, "getF": t_getf, "dS": t_s1,
            "accounted": acct, "danalyticalf": t_tot,
            # the ceiling that applied before fock2e_par/dHcore_par existed
            "old_serial_frac": was_serial / acct,
            "old_ceiling": acct / was_serial,
        }
        with open(args.out, "w") as f:
            json.dump(results, f, indent=1)

    print(f"\nresults -> {args.out}")


if __name__ == "__main__":
    main()
