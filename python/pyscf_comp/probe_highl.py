"""Confirm the 2e-gradient domain guard fails GRACEFULLY, not with a crash.

The AD path covers cartesian l<=4 (rys_tab.rs tables) and any contraction
(eri_cart_gc); l>=5 or range separation is out of domain, where librint.dscf
must raise a catchable ValueError before entering Rust (the reverse would
otherwise corrupt memory / abort). Each CASES entry runs in a fresh subprocess;
rc<0 (abort) means the guard failed to prevent the crash.

Usage: python probe_highl.py                 # driver
       python probe_highl.py GEO BASIS        # worker
"""
import sys
import subprocess
import numpy as np

CASES = [
    ("H2O", "def2-svp", "in-domain -> gradient OK"),
    ("H2O", "def2-tzvp", "f shells (l=3, rys_tab) -> gradient OK"),
    ("CH4", "cc-pvdz", "general contraction (nctr>1, gc path) -> gradient OK"),
    ("H2O", "def2-qzvp", "g shells (l=4, rys_tab nroots 8-9) -> gradient OK"),
    ("CH4", "cc-pv5z", "h shells (l=5) -> expect ValueError"),
]


def worker(geo, basis):
    import pyscf
    import librint
    from geometries import geometries

    molecule = geometries[geo]
    atom = "\n".join(
        f"{a[0]} {0.529 * a[2][0]} {0.529 * a[2][1]} {0.529 * a[2][2]}"
        for a in molecule
    )
    mol = pyscf.gto.M(atom=atom, basis=basis, verbose=0)
    mol.cart = True
    lmax = max(int(mol._bas[i, 1]) for i in range(mol.nbas))
    max_nctr = max(int(mol._bas[i, 3]) for i in range(mol.nbas))
    print(f"    nbas={mol.nbas} lmax={lmax} max_nctr={max_nctr}", flush=True)

    mf = pyscf.scf.RHF(mol)
    mf.verbose = 0
    mf.max_cycle = 500
    mf.kernel()
    P = mf.make_rdm1()

    try:
        g = librint.dscf.danalyticalf(mol, P)
        print(f"    GRADIENT OK  |g|={np.linalg.norm(g):.4f}", flush=True)
    except ValueError as e:
        print(f"    ValueError (graceful): {str(e)[:80]}...", flush=True)


def main():
    for geo, basis, note in CASES:
        print(f"== {geo}/{basis}  ({note})", flush=True)
        r = subprocess.run(
            [sys.executable, __file__, geo, basis],
            capture_output=True, text=True,
        )
        if r.returncode == 0:
            tag = "clean (rc=0)"
        elif r.returncode in (-6, 134, -11, 139):
            tag = f"ABORT rc={r.returncode}  <-- GUARD FAILED"
        else:
            tag = f"exit rc={r.returncode}"
        print(f"  -> {tag}", flush=True)
        for line in (r.stdout + r.stderr).strip().splitlines()[-3:]:
            print(f"      {line}", flush=True)


if __name__ == "__main__":
    if len(sys.argv) == 3:
        worker(sys.argv[1], sys.argv[2])
    else:
        main()
