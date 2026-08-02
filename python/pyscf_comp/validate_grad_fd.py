"""Arbitrate librint gradients against the TRUE dE/dp: central finite
differences with a fully reconverged SCF at every perturbed geometry.
This is convention-free ground truth (no frozen-P/Q assumptions).

Usage: python validate_grad_fd.py            # standard system list, serial path
       python validate_grad_fd.py --par 64   # same, through the threaded path
"""
import argparse

import numpy as np
import pyscf
import librint
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
    # f shells (l=3): exercises the rys_tab.rs nroots 6-7 Chebyshev path
    ("H2O", "def2-tzvp"),
    ("NH3", "def2-tzvp"),
    # general contraction (nctr>1): exercises the eri_cart_gc path
    ("CH4", "cc-pvdz"),
    ("H2O", "cc-pvtz"),
    # g shells (l=4): exercises the rys_tab.rs nroots 8-9 Chebyshev path
    ("H2O", "def2-qzvp"),
]
H = 1e-6


def build(geo, basis):
    molecule = geometries[geo]
    atom = "\n".join(
        f"{a[0]} {0.529 * a[2][0]} {0.529 * a[2][1]} {0.529 * a[2][2]}"
        for a in molecule
    )
    mol = pyscf.gto.M(atom=atom, basis=basis, verbose=0)
    mol.cart = True
    return mol


def scf_e(mol):
    mf = pyscf.scf.RHF(mol.copy())
    mf.verbose = 0
    mf.conv_tol = 1e-12
    mf.max_cycle = 500
    return mf.kernel()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument(
        "--par", type=int, default=None, metavar="N",
        help="validate the threaded path (src/par.rs) with N rayon threads; "
             "0 uses rayon's global pool. Default is the serial path.",
    )
    args = ap.parse_args()

    label = ("danalyticalf" if args.par is None
             else f"danalyticalf and danalytical_par(T={args.par})")
    print(f"validating {label} against central finite differences (h={H})",
          flush=True)

    failures = 0
    for geo, basis in SYSTEMS:
        mol = build(geo, basis)
        mf = pyscf.scf.RHF(mol)
        mf.verbose = 0
        mf.conv_tol = 1e-12
        mf.max_cycle = 500
        mf.kernel()
        P = mf.make_rdm1()

        s1, s2 = librint.utils.split(mol._bas)
        g_true = np.zeros(s2 - s1)
        for j in range(s1, s2):
            mol._env[j] += H
            ep = scf_e(mol)
            mol._env[j] -= 2 * H
            em = scf_e(mol)
            mol._env[j] += H
            g_true[j - s1] = (ep - em) / (2 * H)

        # The serial gradient is always measured against FD, so one --par run
        # validates both paths: the threaded one directly, the serial one
        # alongside it.
        g_ser = librint.dscf.danalyticalf(mol, P)
        e_ser = np.abs(g_true - g_ser).max()
        # denergyf is the same assembled path (denergy_c -> denergyfast), so
        # this is a wiring guard that the two entry points stay in sync, not an
        # independent gradient check.
        wired = np.array_equal(librint.dscf.denergyf(mol, P), g_ser)
        extra = f"denergyf_same_path={wired}"
        ok = e_ser < 1e-5 and wired

        if args.par is not None:
            g_par = librint.dscf.danalytical_par(mol, P, args.par)
            e_par = np.abs(g_true - g_par).max()
            # Work stealing reassociates the sum, so the threaded result agrees
            # with the serial one to round-off rather than bitwise.
            scale = max(float(np.abs(g_ser).max()), 1e-30)
            d_par = float(np.abs(g_par - g_ser).max()) / scale
            extra += f" |true-par|={e_par:.2e} par_vs_serial={d_par:.2e}"
            ok = ok and e_par < 1e-5 and d_par < 1e-9

        failures += 0 if ok else 1
        print(
            f"{geo}/{basis:9s} params={s2-s1:3d} |grad|={np.linalg.norm(g_true):9.4f} "
            f"|true-serial|={e_ser:.2e} {extra}"
            f"  {'OK' if ok else 'FAIL'}",
            flush=True,
        )
    if failures:
        raise SystemExit(f"{failures} system(s) FAILED FD validation")
    print("all gradients FD-validated")


if __name__ == "__main__":
    main()
