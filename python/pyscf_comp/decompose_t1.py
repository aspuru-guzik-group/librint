"""Decompose the librint T1 gradient cost into its parts (dHcore / dR / dS)
and check + time the primal int2e path (CINTOpt) against pyscf's intor.

Usage: python decompose_t1.py
"""
import os
import time

os.environ.setdefault("OMP_NUM_THREADS", "1")
os.environ.setdefault("OPENBLAS_NUM_THREADS", "1")
os.environ.setdefault("MKL_NUM_THREADS", "1")
try:
    os.sched_setaffinity(0, {sorted(os.sched_getaffinity(0))[0]})
except OSError:
    pass

import numpy as np
import pyscf
import librint
import librint.scf
import librint.dscf
import librint.utils

from geometries import geometries

SYSTEMS = [
    ("H2", "sto-3g"),
    ("H2O", "sto-3g"),
    ("NH3", "sto-3g"),
    ("CH4", "sto-3g"),
    ("H2O", "def2-svp"),
    ("NH3", "def2-svp"),
    ("CH4", "def2-svp"),
]


def build(geo, basis):
    molecule = geometries[geo]
    atom = "\n".join(
        f"{a[0]} {0.529 * a[2][0]} {0.529 * a[2][1]} {0.529 * a[2][2]}"
        for a in molecule
    )
    mol = pyscf.gto.M(atom=atom, basis=basis, verbose=0)
    mol.cart = True
    return mol


def med(f, n=3):
    ts = []
    for _ in range(n):
        t0 = time.perf_counter()
        f()
        ts.append(time.perf_counter() - t0)
    return sorted(ts)[n // 2]


def main():
    print(f"{'system':16s} {'int2e':>7s} {'pyscf':>7s} {'ierr':>8s} "
          f"{'dHcore':>7s} {'dR':>7s} {'dS':>7s} {'sum':>7s} {'danaf':>7s}")
    for geo, basis in SYSTEMS:
        mol = build(geo, basis)
        mf = pyscf.scf.RHF(mol)
        mf.verbose = 0
        mf.conv_tol = 1e-12
        mf.kernel()
        P = mf.make_rdm1()

        R = librint.scf.int2e(mol)
        Rr = mol.intor("int2e")
        ierr = float(np.abs(np.asarray(R).reshape(Rr.shape) - Rr).max())

        t_int = med(lambda: librint.scf.int2e(mol))
        t_ref = med(lambda: mol.intor("int2e"))
        t_dh = med(lambda: librint.dscf.dHcoref(mol, P))
        t_dr = med(lambda: librint.dscf.dRf(mol, P))
        t_ds = med(lambda: librint.dscf.dSf(mol, P))
        t_all = med(lambda: librint.dscf.danalyticalf(mol, P))

        print(f"{geo + '/' + basis:16s} {t_int:7.3f} {t_ref:7.3f} {ierr:8.1e} "
              f"{t_dh:7.3f} {t_dr:7.3f} {t_ds:7.3f} "
              f"{t_dh + t_dr + t_ds:7.3f} {t_all:7.3f}", flush=True)


if __name__ == "__main__":
    main()
