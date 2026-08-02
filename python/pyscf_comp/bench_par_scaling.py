"""How well does the whole gradient thread, end to end?

bench_par_loops.py measures the two pair loops in isolation, which flatters
them: it skips getF and the 1e terms, so it answers "how well does dR scale"
rather than "how much faster is a gradient". This measures danalytical_par,
the assembled thing a caller actually invokes, and reports both baselines:

  vs danalyticalf   what a user gets by switching to the threaded entry point
  vs danalytical_par@1  how good the parallelization itself is

Reporting only the first would credit threading for any serial-path overhead
it happens to avoid; reporting only the second would hide any overhead the
threaded path adds. The gap between them is itself the interesting number.

Writes bench_scaling_results.json so the figure is drawn from measurements
rather than from a log scrape, with a _meta block recording the machine.

Every system needs a quiet exclusive node -- a co-tenant reads as poor thread
scaling -- so --only/--out exist to put one system on one node and merge the
per-system JSONs afterwards. Nodes must then be the same CPU model, since the
figure plots systems against each other.

Usage: LIBRINT_SO=/path/to/librint.so python bench_par_scaling.py
       ... python bench_par_scaling.py --only C2H6/def2-tzvp --out shard.json
"""
import argparse
import json
import os
import platform
import time

import numpy as np
import pyscf

import librint
import librint.dscf
import librint.utils

from bench_fair import _cpu_model, _mem_limit_kb, _nphys
from geometries import geometries

SYSTEMS = [
    ("CH4", "def2-svp"),
    ("C3H8", "def2-svp"),
    ("C2H6", "def2-tzvp"),
    ("H2O", "def2-qzvp"),
]
THREADS = [1, 2, 4, 8, 16, 32, 64]
REPEATS = 3
OUT_JSON = "bench_scaling_results.json"


def build(geo, basis):
    atom = "\n".join(
        f"{a[0]} {0.529 * a[2][0]} {0.529 * a[2][1]} {0.529 * a[2][2]}"
        for a in geometries[geo]
    )
    mol = pyscf.gto.M(atom=atom, basis=basis, verbose=0)
    mol.cart = True
    return mol


def median_time(fn, n):
    ts = []
    for _ in range(n):
        t0 = time.perf_counter()
        out = fn()
        ts.append(time.perf_counter() - t0)
    return float(np.median(ts)), out


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
                         "separated); one shard per exclusive node")
    ap.add_argument("--out", default=OUT_JSON, help="results file")
    args = ap.parse_args()
    systems = select(args.only)

    # These nodes are 48 physical cores with 2-way SMT, so sched_getaffinity
    # returns 96 and calling that "cores" overstates the machine by 2x -- the
    # same mislabel _nphys already fixes in bench_fair. The sweep still runs
    # past the core count (T=64 is the fastest point on some systems), it is
    # just reported as threads, which is what it is.
    mask = os.sched_getaffinity(0)
    ncpus = len(mask)
    ncores = _nphys(mask)
    threads = [t for t in THREADS if t <= ncpus]
    print(f"physical cores: {ncores}   hw threads: {ncpus}   "
          f"thread counts: {threads}   median of {REPEATS}", flush=True)

    results = {"_meta": {"node": platform.node(), "ncores": ncores,
                         "ncpus": ncpus, "mem_limit_kb": _mem_limit_kb(),
                         "cpu_model": _cpu_model(), "repeats": REPEATS}}
    failures = 0
    for geo, basis in systems:
        mol = build(geo, basis)
        mf = pyscf.scf.RHF(mol)
        mf.verbose = 0
        mf.conv_tol = 1e-10
        mf.max_cycle = 200
        mf.max_memory = 200
        mf.kernel()
        P = mf.make_rdm1()

        tag = f"{geo}/{basis}"
        print(f"\n{tag}  nao={mol.nao}  nbas={mol.nbas}", flush=True)
        t_ser, g_ser = median_time(lambda: librint.dscf.danalyticalf(mol, P),
                                   REPEATS)
        print(f"  danalyticalf (serial reference)  {t_ser:9.3f}s", flush=True)
        scale = max(float(np.abs(g_ser).max()), 1e-30)

        row = {"nao": int(mol.nao), "nbas": int(mol.nbas),
               "serial": t_ser, "threads": {}}
        t_one = None
        for T in threads:
            t, g = median_time(
                lambda: librint.dscf.danalytical_par(mol, P, T), REPEATS)
            if t_one is None:
                t_one = t
            err = float(np.abs(np.asarray(g) - g_ser).max()) / scale
            if not (err < 1e-9):
                failures += 1
            row["threads"][str(T)] = {"median": t, "rel_err": err}
            print(f"     T={T:3d}  {t:9.3f}s   vs serial {t_ser / t:6.2f}x   "
                  f"vs par@1 {t_one / t:6.2f}x  eff={t_one / t / T:5.2f}   "
                  f"rel|par-ser|={err:.2e}"
                  f"{'' if err < 1e-9 else '   <-- MISMATCH'}", flush=True)
        results[tag] = row
        with open(args.out, "w") as f:   # write-as-you-go, survive a timeout
            json.dump(results, f, indent=1)

    print(f"\nresults -> {args.out}")
    if failures:
        raise SystemExit(f"{failures} timed result(s) disagreed with serial")


if __name__ == "__main__":
    main()
