"""Apples-to-apples librint vs pyscfad benchmark.

Fairness measures:
  * threads pinned (OMP/BLAS env + affinity) BEFORE imports, one worker
    subprocess per config; `threads=free` rows show unpinned multicore jax.
  * T1 compares the SAME function both sides: gradient of the frozen-density
    HF energy E[P] w.r.t. basis exponents+contractions (jax gets an explicit
    frozen-P energy, not a full SCF), both consuming the same converged P.
  * jax cold (trace+compile) reported separately from warm; peak RSS (VmHWM)
    per worker, OOM recorded not fatal.

Tiers:
  T0  primal integrals: librint int1e+int2e vs pyscf mol.intor
  T1  gradient of E[P] (frozen P): librint denergyf vs jax value_and_grad
  T2  end-to-end SCF+gradient: librint density()+denergyf vs pyscfad

Usage:  ../../.venv/bin/python bench_fair.py [--quick]   # --quick: sto-3g only
Results: printed table + bench_fair_results.json
"""
import argparse
import json
import os
import subprocess
import sys
import threading
import time

MOLECULES = [
    ("sto-3g", "H2"),
    ("sto-3g", "H2O"),
    ("sto-3g", "NH3"),
    ("sto-3g", "CH4"),
    ("def2-svp", "H2O"),
    ("def2-svp", "NH3"),
    ("def2-svp", "CH4"),
    # f shells (l=3): 2e gradient runs through the rys_tab.rs nroots 6-7 path
    ("def2-tzvp", "H2O"),
    ("def2-tzvp", "NH3"),
    ("def2-tzvp", "C2H6"),
    ("def2-tzvp", "C6H6"),
]
# librint's fixed-point SCF (no DIIS) makes T2 impractical for big systems:
# a benzene/def2-tzvp Fock build is ~seconds and max_cycle=4000 would run for
# hours. T1 (frozen-P, the fair gradient comparison) still runs.
T1_ONLY = {"C6H6/def2-tzvp"}
# --suite alkanes: CnH2n+2 ladder for the scaling plot (t0 + t1 only)
ALKANES = [
    ("def2-svp", "CH4"),
    ("def2-svp", "C2H6"),
    ("def2-svp", "C3H8"),
    ("def2-svp", "C4H10"),
    ("def2-tzvp", "CH4"),
    ("def2-tzvp", "C2H6"),
    ("def2-tzvp", "C3H8"),
    ("def2-tzvp", "C4H10"),
    ("def2-tzvp", "C6H6"),  # jax OOMs; librint gradient measured with P isolated
]
SCF_CONV = 1e-8
SCF_MAXITER = 4000


# ── worker-side helpers ──────────────────────────────────────────────────────

def _vmhwm_kb():
    for line in open("/proc/self/status"):
        if line.startswith("VmHWM"):
            return int(line.split()[1])
    return -1


def _start_sampler(state):
    def loop():
        while True:
            state["peak_kb"] = _vmhwm_kb()
            time.sleep(0.25)
    threading.Thread(target=loop, daemon=True).start()


def _median(xs):
    xs = sorted(xs)
    n = len(xs)
    return xs[n // 2] if n % 2 else 0.5 * (xs[n // 2 - 1] + xs[n // 2])


def _time_n(fn, n):
    ts = []
    for _ in range(n):
        t0 = time.perf_counter()
        fn()
        ts.append(time.perf_counter() - t0)
    return ts


def _build_pyscf_mol(geo, basis):
    import pyscf
    from geometries import geometries
    molecule = geometries[geo]
    atom = "\n".join(
        f"{a[0]} {0.529 * a[2][0]} {0.529 * a[2][1]} {0.529 * a[2][2]}"
        for a in molecule
    )
    mol = pyscf.gto.M(atom=atom, basis=basis, verbose=0)
    mol.cart = True
    return atom, mol


def _converged_P(mol):
    """Deterministic converged RHF density both engines share. Forced DIRECT
    (small max_memory) so no full N^4 ERI tensor is ever materialized in the
    process: that keeps the librint T1 worker's peak RSS the gradient's own
    footprint, not pyscf's incore setup. Slower setup, but setup isn't timed."""
    import pyscf
    mf = pyscf.scf.RHF(mol)
    mf.verbose = 0
    mf.conv_tol = 1e-10
    mf.max_cycle = 200
    mf.max_memory = 200      # << ERI tensor -> direct SCF, no incore _eri
    mf.direct_scf = True
    mf.kernel()
    return mf.make_rdm1()


def _load_or_make_P(geo, basis, mol):
    """T1 gradient input P. Prefer a precomputed npy (via env LIBRINT_P_FILE)
    so the measured worker's peak RSS reflects the gradient ALONE -- pyscf's
    incore-SCF tensor for P lives in a separate setup_P process, not here."""
    pf = os.environ.get("LIBRINT_P_FILE")
    if pf and os.path.exists(pf):
        import numpy as np
        return np.load(pf)
    return _converged_P(mol)


# ── workers (one engine + tier per process) ──────────────────────────────────

def worker_t0(geo, basis, out):
    import numpy as np
    import librint
    _, mol = _build_pyscf_mol(geo, basis)

    def lib_ints():
        S = librint.scf.int1e(mol, "ovlp")
        T = librint.scf.int1e(mol, "kin")
        V = librint.scf.int1e(mol, "nuc")
        R = librint.int2e(mol) if hasattr(librint, "int2e") else librint.scf.int2e(mol)
        return S, T, V, R

    def ref_ints():
        S = mol.intor("int1e_ovlp")
        T = mol.intor("int1e_kin")
        V = mol.intor("int1e_nuc")
        R = mol.intor("int2e")
        return S, T, V, R

    n = 5
    t_lib = _time_n(lambda: lib_ints(), n)
    t_ref = _time_n(lambda: ref_ints(), n)

    S, T, V, R = lib_ints()
    Sr, Tr, Vr, Rr = ref_ints()
    err = max(
        float(np.abs(S - Sr).max()),
        float(np.abs(T - Tr).max()),
        float(np.abs(V - Vr).max()),
        float(np.abs(R.reshape(Rr.shape) - Rr).max()),
    )
    out.update(
        librint_median=_median(t_lib),
        ref_median=_median(t_ref),
        max_abs_err=err,
    )


def worker_t1_librint(geo, basis, out):
    import numpy as np
    import librint
    _, mol = _build_pyscf_mol(geo, basis)
    P = _load_or_make_P(geo, basis, mol)

    # danalyticalf is the validated gradient (5e-8 vs reconverged-SCF FD);
    # denergyf (fused Enzyme pass) is faster but returns wrong values and
    # its huge tape OOMs first -- time it last, emit partial results before.
    g = librint.dscf.danalyticalf(mol, P)
    ts = _time_n(lambda: librint.dscf.danalyticalf(mol, P), 3)
    out.update(median=_median(ts), grad_sorted=np.sort(g).tolist())
    out["peak_kb"] = _vmhwm_kb()
    print("BENCH_JSON " + json.dumps(out), flush=True)  # partial, pre-denergyf
    ts_den = _time_n(lambda: librint.dscf.denergyf(mol, P), 3)
    out.update(median_denergy=_median(ts_den))


def worker_t1_jax(geo, basis, out):
    import numpy as np
    import jax
    import jax.numpy as jnp
    jax.config.update("jax_platform_name", "cpu")
    jax.config.update("jax_enable_x64", True)
    from pyscfad import gto

    import numpy as onp
    atom, mol_ref = _build_pyscf_mol(geo, basis)
    Pn = _load_or_make_P(geo, basis, mol_ref)
    # frozen energy-weighted density Q = P F P, exactly librint's dSg contraction
    hn = mol_ref.intor("int1e_kin") + mol_ref.intor("int1e_nuc")
    erin = mol_ref.intor("int2e")
    Fn = (hn + onp.einsum("kl,ijkl->ij", Pn, erin)
          - 0.5 * onp.einsum("kl,ikjl->ij", Pn, erin))
    Qn = Pn @ Fn @ Pn
    P, Q = jnp.asarray(Pn), jnp.asarray(Qn)

    mol = gto.Mole()
    mol.atom = atom
    mol.unit = "Angstrom"
    mol.basis = basis
    mol.charge = 0
    mol.verbose = 0
    mol.cart = True
    mol.build(trace_coords=False, trace_exp=True, trace_ctr_coeff=True)

    def e_frozen(m):
        # identical function to librint's denergyf target:
        # E[P] - 1/2 tr(Q S), with P and Q=PFP frozen
        h = m.intor("int1e_kin") + m.intor("int1e_nuc")
        eri = m.intor("int2e")
        S = m.intor("int1e_ovlp")
        e1 = jnp.einsum("ij,ji->", P, h)
        J = jnp.einsum("kl,ijkl->ij", P, eri)
        K = jnp.einsum("kl,ikjl->ij", P, eri)
        e2 = 0.5 * jnp.einsum("ij,ji->", P, J) - 0.25 * jnp.einsum("ij,ji->", P, K)
        pulay = -0.5 * jnp.einsum("ij,ji->", Q, S)
        return e1 + e2 + pulay  # Enuc independent of exp/ctr -> zero gradient

    vg = jax.value_and_grad(e_frozen)
    t0 = time.perf_counter()
    E, grad = vg(mol)
    jax.block_until_ready(E)
    cold = time.perf_counter() - t0

    def warm():
        e, g = vg(mol)
        jax.block_until_ready(e)
        return g

    ts = _time_n(warm, 3)
    g = np.concatenate([np.asarray(grad.exp).ravel(),
                        np.asarray(grad.ctr_coeff).ravel()])
    out.update(cold=cold, median=_median(ts), grad_sorted=np.sort(g).tolist())


def worker_t2_librint(geo, basis, out):
    import numpy as np
    import librint
    _, mol = _build_pyscf_mol(geo, basis)

    def run():
        P = librint.scf.density(mol, imax=SCF_MAXITER, conv=SCF_CONV)
        return librint.dscf.danalyticalf(mol, P)

    g = run()
    ts = _time_n(run, 3)
    out.update(median=_median(ts), grad_sorted=np.sort(g).tolist())


def worker_t2_jax(geo, basis, out):
    import numpy as np
    import jax
    jax.config.update("jax_platform_name", "cpu")
    jax.config.update("jax_enable_x64", True)
    from jax import value_and_grad
    from pyscfad import gto, scf

    atom, _ = _build_pyscf_mol(geo, basis)
    mol = gto.Mole()
    mol.atom = atom
    mol.unit = "Angstrom"
    mol.basis = basis
    mol.charge = 0
    mol.verbose = 0
    mol.cart = True
    mol.build(trace_coords=False, trace_exp=True, trace_ctr_coeff=True)

    def hf_energy(m):
        mf = scf.RHF(m)
        mf.verbose = 0
        mf.max_cycle = SCF_MAXITER
        mf.conv_tol = SCF_CONV
        return mf.kernel()

    def run():
        E, grad = value_and_grad(hf_energy)(mol)
        return grad

    t0 = time.perf_counter()
    grad = run()
    cold = time.perf_counter() - t0
    ts = _time_n(run, 3)
    g = np.concatenate([np.asarray(grad.exp).ravel(),
                        np.asarray(grad.ctr_coeff).ravel()])
    out.update(cold=cold, median=_median(ts), grad_sorted=np.sort(g).tolist())


WORKERS = {
    "t0": worker_t0,
    "t1_librint": worker_t1_librint,
    "t1_jax": worker_t1_jax,
    "t2_librint": worker_t2_librint,
    "t2_jax": worker_t2_jax,
}


def worker_setup_P(geo, basis, out_path):
    """Compute the shared converged P in its OWN process and save it, so the
    pyscf incore-SCF tensor never inflates a gradient worker's peak RSS."""
    import numpy as np
    _, mol = _build_pyscf_mol(geo, basis)
    np.save(out_path, _converged_P(mol))


def run_worker(argv):
    kind, geo, basis, arg4 = argv
    if kind == "setup_P":
        worker_setup_P(geo, basis, arg4)  # arg4 = output npy path
        return
    threads = arg4
    if threads != "free":
        try:
            # first core of OUR allocation (works inside SLURM cgroups too)
            core = sorted(os.sched_getaffinity(0))[0]
            os.sched_setaffinity(0, {core})
        except OSError:
            pass
    state = {"peak_kb": -1}
    _start_sampler(state)
    out = {}
    WORKERS[kind](geo, basis, out)
    out["peak_kb"] = max(state["peak_kb"], _vmhwm_kb())
    print("BENCH_JSON " + json.dumps(out), flush=True)


# ── parent orchestration ─────────────────────────────────────────────────────

def make_P_file(geo, basis, timeout):
    """Precompute the shared T1 density P in a throwaway multicore process and
    return the npy path (None on failure). Not RSS-measured -> keeps pyscf's
    incore-SCF tensor out of the gradient workers' peak."""
    import tempfile
    fd, path = tempfile.mkstemp(prefix=f"P_{geo}_{basis}_", suffix=".npy")
    os.close(fd)
    env = dict(os.environ)
    env["PYTHONHASHSEED"] = "0"  # match the gradient workers' _env ordering
    try:
        subprocess.run(
            [sys.executable, os.path.abspath(__file__),
             "--worker", "setup_P", geo, basis, path],
            env=env, cwd=os.path.dirname(os.path.abspath(__file__)),
            capture_output=True, text=True, timeout=timeout,
        )
    except subprocess.TimeoutExpired:
        return None
    return path if os.path.exists(path) and os.path.getsize(path) > 0 else None


def spawn(kind, geo, basis, threads, timeout=900, p_file=None):
    env = dict(os.environ)
    # pyscf packs element basis blocks into _env in hash order -> pin the seed
    # so gradient vectors are elementwise comparable across processes
    env["PYTHONHASHSEED"] = "0"
    if p_file:  # T1 workers load this P instead of running incore SCF
        env["LIBRINT_P_FILE"] = p_file
    nthr = "1" if threads != "free" else str(os.cpu_count())
    if threads != "free":
        for var in ("OMP_NUM_THREADS", "OPENBLAS_NUM_THREADS", "MKL_NUM_THREADS",
                    "VECLIB_MAXIMUM_THREADS", "NUMEXPR_NUM_THREADS"):
            env[var] = nthr
        env["XLA_FLAGS"] = ("--xla_cpu_multi_thread_eigen=false "
                            "intra_op_parallelism_threads=1")
    t0 = time.perf_counter()
    try:
        proc = subprocess.run(
            [sys.executable, os.path.abspath(__file__),
             "--worker", kind, geo, basis, threads],
            env=env, cwd=os.path.dirname(os.path.abspath(__file__)),
            capture_output=True, text=True, timeout=timeout,
        )
    except subprocess.TimeoutExpired:
        return {"status": "TIMEOUT", "wall": time.perf_counter() - t0}
    wall = time.perf_counter() - t0
    res = None
    for line in proc.stdout.splitlines():
        if line.startswith("BENCH_JSON "):
            res = json.loads(line[len("BENCH_JSON "):])  # keep last (most complete)
    if res is not None:
        res.update(status="ok", wall=wall)
        if proc.returncode in (137, -9):
            res["note"] = "denergyf OOM after partial results"
        return res
    status = "OOM" if proc.returncode in (137, -9) else f"FAIL rc={proc.returncode}"
    tail = "\n".join((proc.stderr or "").splitlines()[-3:])
    return {"status": status, "wall": wall, "stderr_tail": tail}


def fmt_t(res, key="median"):
    if res.get("status") != "ok":
        return res.get("status", "?")
    return f"{res[key]:.3f}"


def fmt_mem(res):
    if res.get("status") != "ok" or res.get("peak_kb", -1) < 0:
        return "-"
    return f"{res['peak_kb'] / 1048576:.1f}G"


def grad_err(a, b):
    import numpy as np
    if a.get("status") != "ok" or b.get("status") != "ok":
        return "-"
    ga, gb = np.array(a["grad_sorted"]), np.array(b["grad_sorted"])
    if ga.shape != gb.shape:
        return f"len {ga.size} vs {gb.size}"
    return f"{np.abs(ga - gb).max():.1e}"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--worker", nargs=4, metavar=("KIND", "GEO", "BASIS", "THREADS"))
    ap.add_argument("--quick", action="store_true", help="sto-3g only")
    ap.add_argument("--suite", choices=["default", "alkanes"], default="default")
    ap.add_argument("--timeout", type=int, default=900,
                    help="per-worker wall limit (s)")
    args = ap.parse_args()
    if args.worker:
        run_worker(args.worker)
        return

    if args.suite == "alkanes":
        mols = ALKANES
        tiers = ("t1",)
        out_json = "bench_alkanes_results.json"
    else:
        mols = [m for m in MOLECULES if not args.quick or m[0] == "sto-3g"]
        tiers = ("t1", "t2")
        out_json = "bench_fair_results.json"
    results = {}

    def key(*parts):
        return "|".join(parts)

    for basis, geo in mols:
        tag = f"{geo}/{basis}"
        print(f"── {tag}", flush=True)
        results[key("t0", tag)] = spawn("t0", geo, basis, "pin",
                                        timeout=args.timeout)
        # shared frozen-P density, precomputed out-of-process; T1 workers load
        # it so their peak RSS is the gradient's footprint, not pyscf's setup
        pf = make_P_file(geo, basis, args.timeout) if "t1" in tiers else None
        for tier in tiers:
            if tier == "t2" and tag in T1_ONLY:
                continue
            pf_t = pf if tier == "t1" else None
            for eng in ("librint", "jax"):
                r = spawn(f"{tier}_{eng}", geo, basis, "pin",
                          timeout=args.timeout, p_file=pf_t)
                results[key(tier, eng, tag, "pin")] = r
                print(f"   {tier} {eng:8s} pin : {fmt_t(r)}s  peak {fmt_mem(r)}",
                      flush=True)
            rf = spawn(f"{tier}_jax", geo, basis, "free", timeout=args.timeout,
                       p_file=pf_t)
            results[key(tier, "jax", tag, "free")] = rf
            print(f"   {tier} jax      free: {fmt_t(rf)}s  peak {fmt_mem(rf)}",
                  flush=True)
        if pf and os.path.exists(pf):
            os.remove(pf)
        with open(out_json, "w") as f:  # write-as-you-go: survive job timeouts
            json.dump(results, f, indent=1)

    # ── tables ──
    print("\n=== T0 primal integrals (S,T,V + full 2e; 1 core; median of 5) ===")
    print(f"{'system':16s} {'librint':>9s} {'libcint':>9s} {'ratio':>7s} {'max|Δ|':>9s}")
    for basis, geo in mols:
        tag = f"{geo}/{basis}"
        r = results[key("t0", tag)]
        if r.get("status") != "ok":
            print(f"{tag:16s} {r['status']}")
            continue
        ratio = r["librint_median"] / r["ref_median"]
        print(f"{tag:16s} {r['librint_median']:9.4f} {r['ref_median']:9.4f} "
              f"{ratio:7.1f} {r['max_abs_err']:9.1e}")

    for tier, desc in (("t1", "gradient of frozen-P E[P] (same function both sides)"),
                       ("t2", f"end-to-end SCF+gradient (conv_tol={SCF_CONV})")):
        if tier not in tiers:
            continue
        print(f"\n=== {tier.upper()} {desc}; median of 3; librint = danalyticalf ===")
        print(f"{'system':16s} {'librint@1c':>11s} {'lib-dene':>9s} {'jax@1c':>9s} "
              f"{'jax cold':>9s} {'jax@free':>9s} {'lib peak':>9s} {'jax peak':>9s} "
              f"{'max|Δg|':>9s}")
        for basis, geo in mols:
            tag = f"{geo}/{basis}"
            rl = results.get(key(tier, "librint", tag, "pin"))
            if rl is None:  # T1_ONLY system
                continue
            rj = results[key(tier, "jax", tag, "pin")]
            rf = results[key(tier, "jax", tag, "free")]
            cold = fmt_t(rj, "cold") if rj.get("status") == "ok" else "-"
            dene = (f"{rl['median_denergy']:.3f}" if rl.get("median_denergy")
                    else ("OOM" if rl.get("note") else "-"))
            print(f"{tag:16s} {fmt_t(rl):>11s} {dene:>9s} {fmt_t(rj):>9s} {cold:>9s} "
                  f"{fmt_t(rf):>9s} {fmt_mem(rl):>9s} {fmt_mem(rj):>9s} "
                  f"{grad_err(rl, rj):>9s}")

    print(f"\nresults -> {out_json}")


if __name__ == "__main__":
    main()
