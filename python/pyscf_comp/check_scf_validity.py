"""End-to-end SCF regression check: is librint's own density usable?

Covers the two failures that the benchmark's T2 tier recorded as `status: ok`:

  * CH4/sto-3g converged to a density that was not a density -- tr(PS) = 10.69
    for a 10-electron molecule, max|PSP - 2P| = 1.95, E = -40.305 below the
    variational RHF minimum of -39.727.
  * CH4/def2-svp never converged; density() zeroed P, printed a line to stdout,
    and returned it as if it had succeeded.

Both came from faer's general eigensolver being used on the symmetric S and F',
which returns a non-orthonormal basis across a degenerate eigenvalue (CH4 is
Td). Molecules without degeneracies (H2O, NH3) were unaffected, which is why
this went unnoticed.

Also checks that the ctypes return buffers are freed (int2e_c hands back
nao**4 doubles per call).

Usage: .venv/bin/python check_scf_validity.py
"""
import os

os.environ.setdefault("OMP_NUM_THREADS", "1")
os.environ.setdefault("OPENBLAS_NUM_THREADS", "1")
os.environ.setdefault("MKL_NUM_THREADS", "1")

import numpy as np
import pyscf

import librint
import librint.scf
import librint.dscf

from geometries import geometries

SYSTEMS = [
    ("H2", "sto-3g"),
    ("H2O", "sto-3g"),
    ("NH3", "sto-3g"),
    ("CH4", "sto-3g"),     # Td: 4 degenerate overlap eigenvalue pairs
    ("CH4", "def2-svp"),   # Td: 18
    ("H2O", "def2-svp"),
]
TOL_P = 1e-6
TOL_E = 1e-8


def build(geo, basis):
    atom = "\n".join(
        f"{a[0]} {0.529 * a[2][0]} {0.529 * a[2][1]} {0.529 * a[2][2]}"
        for a in geometries[geo]
    )
    mol = pyscf.gto.M(atom=atom, basis=basis, verbose=0)
    mol.cart = True
    return mol


def rss_kb():
    for line in open("/proc/self/status"):
        if line.startswith("VmRSS"):
            return int(line.split()[1])
    return -1


def check_scf():
    print(f"{'system':16s} {'tr(PS)':>12s} {'|PSP-2P|':>10s} {'max|dP|':>10s} "
          f"{'E_librint':>16s} {'E_pyscf':>16s} {'dE':>10s}")
    failures = 0
    for geo, basis in SYSTEMS:
        mol = build(geo, basis)
        S = mol.intor("int1e_ovlp")
        nelec = sum(mol.nelec)

        mf = pyscf.scf.RHF(mol)
        mf.verbose = 0
        mf.conv_tol = 1e-12
        mf.max_cycle = 500
        mf.kernel()
        P_ref = mf.make_rdm1()

        try:
            P = np.asarray(librint.scf.density(mol, imax=4000, conv=1e-8))
        except RuntimeError as e:
            print(f"{geo + '/' + basis:16s} RAISED: {e}")
            failures += 1
            continue

        trace = float(np.trace(P @ S))
        idem = float(np.abs(P @ S @ P - 2 * P).max())
        dP = float(np.abs(P - P_ref).max())
        E = librint.scf.energy(mol, P)
        dE = E - mf.e_tot
        ok = (abs(trace - nelec) < TOL_P and idem < TOL_P and abs(dE) < TOL_E)
        failures += 0 if ok else 1
        print(f"{geo + '/' + basis:16s} {trace:12.6f} {idem:10.2e} {dP:10.2e} "
              f"{E:16.9f} {mf.e_tot:16.9f} {dE:10.2e}{'' if ok else '  <-- FAIL'}")
    return failures


def check_free():
    """Each int2e call used to leak a whole nao**4 buffer."""
    mol = build("H2O", "def2-svp")
    nao = mol.nao_nr()
    per_call_mb = nao ** 4 * 8 / 1e6
    librint.scf.int2e(mol)
    before = rss_kb()
    for _ in range(10):
        librint.scf.int2e(mol)
    growth_mb = (rss_kb() - before) / 1024.0
    leaked = growth_mb > 0.5 * per_call_mb
    print(f"\nint2e x10 (nao={nao}, {per_call_mb:.2f} MB/call): "
          f"RSS growth {growth_mb:+.1f} MB{'  <-- LEAKING' if leaked else ''}")
    return 1 if leaked else 0


def main():
    failures = check_scf() + check_free()
    if failures:
        raise SystemExit(f"{failures} check(s) FAILED")
    print("\nall SCF validity checks passed")


if __name__ == "__main__":
    main()
