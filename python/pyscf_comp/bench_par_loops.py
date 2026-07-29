"""Correctness and scaling of the rayon-parallel gradient loops (src/par.rs).

Checks both halves of the design:

  dS_par  1e overlap term, pairs over the full nbas x nbas loop
  dR_par  2e term, pairs over canonical i >= j with the 8-fold reduction intact

For each, the parallel result must match the serial dscf entry point (to
round-off -- the per-task partial sums are summed in a different order), and the
speedup is measured against the same code at one thread, which is the honest
baseline: the serial dSf/dRf wall time also includes the getF Fock build that
the parallel loops do not perform.

Usage: LIBRINT_SO=/path/to/librint.so .venv/bin/python bench_par_loops.py
"""
import ctypes
import os
import time

import numpy as np
import pyscf

import librint
import librint.dscf
import librint.utils
from librint import library

from geometries import geometries

SYSTEMS = [
    ("H2O", "sto-3g"),
    ("CH4", "def2-svp"),
    ("H2O", "def2-qzvp"),
    ("C2H6", "def2-tzvp"),
]
THREADS = [1, 2, 4, 8, 16, 32, 64]

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


def sweep(label, fn, mol, W, ref, ncores):
    scale = max(np.abs(ref).max(), 1e-30)
    t0 = time.perf_counter()
    call_par(fn, mol, W, 1)
    t_one = time.perf_counter() - t0
    print(f"  {label}: 1-thread {t_one:.3f}s")
    failures = 0
    for n in THREADS:
        if n > ncores:
            continue
        t0 = time.perf_counter()
        got = call_par(fn, mol, W, n)
        dt = time.perf_counter() - t0
        err = float(np.abs(got - ref).max())
        ok = err < 1e-9 * scale
        failures += 0 if ok else 1
        print(f"     threads={n:3d}  {dt:8.4f}s  speedup={t_one / dt:6.2f}x  "
              f"eff={t_one / dt / n:5.2f}  max|par-serial|={err:.2e}"
              f"{'' if ok else '   <-- MISMATCH'}")
    return failures


def main():
    ncores = len(os.sched_getaffinity(0))
    print(f"usable cores: {ncores}")
    failures = 0
    for geo, basis in SYSTEMS:
        mol = build(geo, basis)
        mf = pyscf.scf.RHF(mol)
        mf.verbose = 0
        mf.conv_tol = 1e-10
        mf.max_cycle = 200
        mf.max_memory = 200
        mf.kernel()
        P = mf.make_rdm1()

        # dSg seeds with the energy-weighted density Q = P F P, which it builds
        # itself via getF; the parallel entry takes the seed directly.
        h = mol.intor("int1e_kin") + mol.intor("int1e_nuc")
        eri = mol.intor("int2e")
        F = (h + np.einsum("kl,ijkl->ij", P, eri)
             - 0.5 * np.einsum("kl,ikjl->ij", P, eri))
        Q = P @ F @ P

        print(f"\n{geo}/{basis}  nbas={mol.nbas}  nparam={np.diff(librint.utils.split(np.asarray(mol._bas)))[0]}")
        failures += sweep("dS 1e", library.dS_par_c, mol, Q,
                          np.asarray(librint.dscf.dSf(mol, P)), ncores)
        failures += sweep("dR 2e", library.dR_par_c, mol, P,
                          np.asarray(librint.dscf.dRf(mol, P)), ncores)
    if failures:
        raise SystemExit(f"{failures} parallel result(s) disagreed with serial")
    print("\nrayon-parallel loops agree with serial everywhere")


if __name__ == "__main__":
    main()
