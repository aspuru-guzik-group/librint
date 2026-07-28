"""Arbitrate librint gradients against the TRUE dE/dp: central finite
differences with a fully reconverged SCF at every perturbed geometry.
This is convention-free ground truth (no frozen-P/Q assumptions).

Usage: python validate_grad_fd.py            # standard system list
"""
import numpy as np
import pyscf
import librint
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

        g_den = librint.dscf.denergyf(mol, P)
        g_ana = librint.dscf.danalyticalf(mol, P)
        e_den = np.abs(g_true - g_den).max()
        e_ana = np.abs(g_true - g_ana).max()
        e_dd = np.abs(g_den - g_ana).max()
        ok = e_den < 1e-5 and e_ana < 1e-5
        failures += 0 if ok else 1
        print(
            f"{geo}/{basis:9s} params={s2-s1:3d} |grad|={np.linalg.norm(g_true):9.4f} "
            f"|true-denergyf|={e_den:.2e} |true-danalyticalf|={e_ana:.2e} "
            f"|den-ana|={e_dd:.2e}  {'OK' if ok else 'FAIL'}",
            flush=True,
        )
    if failures:
        raise SystemExit(f"{failures} system(s) FAILED FD validation")
    print("all gradients FD-validated")


if __name__ == "__main__":
    main()
