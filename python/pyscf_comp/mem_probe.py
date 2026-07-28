"""Per-stage peak-RSS attribution for the librint T1 gradient worker.

Each stage runs in its OWN subprocess so peak RSS (VmHWM) is clean:

  scf   : build mol + _converged_P (bench's direct-SCF P), save P.  <- SCF cost
  grad  : import librint, load that P, time danalyticalf ONLY.      <- librint
  repro : bench's exact worker path (mol + _converged_P + grad).    <- combined
  intor : mol.intor('int2e') (full uncompressed nao^4).             <- nao^4 ref

grad << scf ~ repro ~ intor  =>  the nao^4 lives in pyscf's incore ERI, not the
librint gradient (whose memory is `grad`).  grad ~ nao^4  =>  librint is nao^4.

Usage: python mem_probe.py [--fix]              # driver (--fix: post-getF-fix
       python mem_probe.py STAGE GEO BASIS [PF] #  rerun -> its own output file)
"""
import json
import os
import subprocess
import sys
import threading
import time

# --fix: rerun against the post-getF-fix .so (1 timing rep, separate output).
FIX = "--fix" in sys.argv
OUT = "mem_probe_fix_results.json" if FIX else "mem_probe_results.json"

# (basis, geo, nao_cart) ladder; nao for the nao^4 reference / guards
CASES = [
    ("def2-svp", "CH4", 35),
    ("def2-svp", "C2H6", 60),
    ("def2-svp", "C3H8", 85),
    ("def2-svp", "C4H10", 110),
    ("def2-tzvp", "CH4", 60),
    ("def2-tzvp", "C2H6", 108),
    ("def2-tzvp", "C3H8", 156),
    ("def2-tzvp", "C4H10", 204),
    ("def2-tzvp", "C6H6", 252),
]
# diagnostic stages only on these (intor/repro build nao^4 -> keep nao modest)
DIAG = {("def2-tzvp", "C2H6"), ("def2-tzvp", "C3H8")}


def _vmhwm_kb():
    for line in open("/proc/self/status"):
        if line.startswith("VmHWM"):
            return int(line.split()[1])
    return -1


def _start_sampler(state):
    def loop():
        while True:
            state["peak_kb"] = max(state["peak_kb"], _vmhwm_kb())
            time.sleep(0.05)
    threading.Thread(target=loop, daemon=True).start()


def _median(xs):
    xs = sorted(xs)
    n = len(xs)
    return xs[n // 2] if n % 2 else 0.5 * (xs[n // 2 - 1] + xs[n // 2])


def _mol(geo, basis):
    import pyscf
    from geometries import geometries
    molecule = geometries[geo]
    atom = "\n".join(
        f"{a[0]} {0.529 * a[2][0]} {0.529 * a[2][1]} {0.529 * a[2][2]}"
        for a in molecule
    )
    mol = pyscf.gto.M(atom=atom, basis=basis, verbose=0)
    mol.cart = True
    return mol


def _converged_P(mol):
    """Byte-for-byte the bench's _converged_P: direct SCF, tiny max_memory so
    (per pyscf _is_mem_enough) no incore _eri SHOULD be built."""
    import pyscf
    mf = pyscf.scf.RHF(mol)
    mf.verbose = 0
    mf.conv_tol = 1e-10
    mf.max_cycle = 200
    mf.max_memory = 200
    mf.direct_scf = True
    mf.kernel()
    return mf.make_rdm1()


def worker(stage, geo, basis, pf):
    import numpy as np
    state = {"peak_kb": _vmhwm_kb()}
    _start_sampler(state)
    out = {"stage": stage}

    if stage == "scf":
        mol = _mol(geo, basis)
        P = _converged_P(mol)
        np.save(pf, P)
        out["nao"] = int(P.shape[0])

    elif stage == "grad":
        import librint
        mol = _mol(geo, basis)
        P = np.load(pf)
        g = librint.dscf.danalyticalf(mol, P)
        ts = []
        for _ in range(int(os.environ.get("MEM_PROBE_REPS", "3"))):
            t0 = time.perf_counter()
            librint.dscf.danalyticalf(mol, P)
            ts.append(time.perf_counter() - t0)
        out["median"] = _median(ts)
        out["gnorm"] = float(np.linalg.norm(g))
        out["nparams"] = int(g.shape[0])

    elif stage == "repro":  # bench's exact worker path, no P-file isolation
        import librint
        mol = _mol(geo, basis)
        P = _converged_P(mol)
        librint.dscf.danalyticalf(mol, P)

    elif stage == "intor":
        mol = _mol(geo, basis)
        eri = mol.intor("int2e")
        out["nao"] = int(round(eri.shape[0]))

    time.sleep(0.1)
    out["peak_kb"] = max(state["peak_kb"], _vmhwm_kb())
    out["peak_g"] = out["peak_kb"] / 1048576.0
    print("PROBE_JSON " + json.dumps(out), flush=True)


def spawn(stage, geo, basis, pf=""):
    env = dict(os.environ)
    env["PYTHONHASHSEED"] = "0"
    for v in ("OMP_NUM_THREADS", "OPENBLAS_NUM_THREADS", "MKL_NUM_THREADS",
              "VECLIB_MAXIMUM_THREADS", "NUMEXPR_NUM_THREADS"):
        env[v] = "1"
    env["MEM_PROBE_REPS"] = "1" if FIX else "3"
    t0 = time.perf_counter()
    p = subprocess.run(
        [sys.executable, os.path.abspath(__file__), stage, geo, basis, pf],
        env=env, cwd=os.path.dirname(os.path.abspath(__file__)),
        capture_output=True, text=True,
    )
    wall = time.perf_counter() - t0
    res = None
    for line in p.stdout.splitlines():
        if line.startswith("PROBE_JSON "):
            res = json.loads(line[len("PROBE_JSON "):])
    if res is None:
        tail = "\n".join((p.stderr or p.stdout or "").splitlines()[-4:])
        return {"stage": stage, "status": f"FAIL rc={p.returncode}", "tail": tail,
                "wall": wall}
    res.update(status="ok", wall=wall)
    return res


def main():
    import tempfile
    results = {}
    print(f"{'case':18s} {'nao':>4s} {'stage':>7s} {'peak(G)':>9s} "
          f"{'nao^4*8G':>9s} {'grad(s)':>9s}  note", flush=True)
    for basis, geo, nao in CASES:
        tag = f"{geo}/{basis}"
        n4 = nao ** 4 * 8 / 1073741824.0
        fd, pf = tempfile.mkstemp(prefix=f"{'Pf' if FIX else 'P'}_{geo}_{basis}_", suffix=".npy")
        os.close(fd)

        stages = ["scf", "grad"]
        if (basis, geo) in DIAG:
            stages += ["intor", "repro"]
        for stage in stages:
            r = spawn(stage, geo, basis, pf)
            results[f"{tag}|{stage}"] = r
            pk = f"{r.get('peak_g', -1):.2f}" if r.get("status") == "ok" else r.get("status", "?")
            gs = f"{r['median']:.3f}" if r.get("median") else ""
            note = ""
            if stage == "grad" and r.get("status") == "ok":
                note = f"{'FIXED' if FIX else 'baseline'} librint (gnorm={r.get('gnorm', 0):.3f}, np={r.get('nparams')})"
            if r.get("status") != "ok":
                note = r.get("tail", "").replace("\n", " | ")[:80]
            print(f"{tag:18s} {nao:>4d} {stage:>7s} {pk:>9s} {n4:>9.2f} {gs:>9s}  {note}",
                  flush=True)
        if os.path.exists(pf):
            os.remove(pf)
        with open(OUT, "w") as f:
            json.dump(results, f, indent=1)

    print("\n=== VERDICT ===", flush=True)
    for basis, geo, nao in CASES:
        tag = f"{geo}/{basis}"
        g = results.get(f"{tag}|grad", {})
        if g.get("status") != "ok":
            continue
        if FIX:  # librint gradient peak vs the nao^4 ERI tensor it avoids
            n4 = nao ** 4 * 8 / 1073741824.0
            print(f"{tag:18s} librint_grad={g['peak_g']:.2f}G  "
                  f"nao^4={n4:.2f}G  drop={n4/max(g['peak_g'],1e-9):.0f}x", flush=True)
        else:    # librint gradient peak vs the pyscf direct-SCF peak
            s = results.get(f"{tag}|scf", {})
            if s.get("status") == "ok":
                print(f"{tag:18s} librint_grad={g['peak_g']:.2f}G  "
                      f"pyscf_scf={s['peak_g']:.2f}G  "
                      f"ratio={s['peak_g']/max(g['peak_g'],1e-9):.1f}x", flush=True)
    print(f"results -> {OUT}", flush=True)


if __name__ == "__main__":
    if len(sys.argv) >= 4:
        worker(sys.argv[1], sys.argv[2], sys.argv[3],
               sys.argv[4] if len(sys.argv) > 4 else "")
    else:
        main()
