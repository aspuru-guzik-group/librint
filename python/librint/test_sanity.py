"""
Self-contained sanity check for librint.
Run with:  python -m pytest librint/test_sanity.py -v

Verifies HF energy and analytical gradient for H2O / 6-31G (cartesian)
against hardcoded reference values obtained from PySCF and finite-difference.
"""
import numpy as np
import librint


# ── H2O / 6-31G reference data ──────────────────────────────────────────────
H2O_ATOM = "O 0 0 0; H 0 0.757 0.587; H 0 -0.757 0.587"
H2O_BASIS = "6-31g"

# HF energy (PySCF, conv_tol=1e-10)
H2O_REFERENCE_ENERGY = -75.983948498105605

# Analytical gradient (cross-validated against finite-difference, h=1e-6)
H2O_REFERENCE_GRADIENT = np.array([
    -8.35452220e-09, -2.30069013e-07, -8.92484904e-07,  5.69517836e-05,
    -2.08768641e-04,  9.08256402e-05,  1.80645035e-05,  2.32936065e-05,
     8.93680639e-05, -5.86133054e-05, -8.63122774e-05,  1.67403702e-04,
     6.27265146e-04, -1.51412500e-03,  9.48692036e-03,  1.04680930e-04,
    -8.42395819e-04, -2.05877895e-04, -3.16288072e-02,  1.51420205e-07,
    -4.75735435e-04, -8.16365830e-03,  2.73007682e-02, -1.64810408e-03,
     2.31871106e-03, -4.09376959e-04,  9.51543875e-02,  1.01033115e-06,
    -4.34904111e-05,  2.42327454e-03, -2.41982355e-02,  1.65519430e-03,
     3.89384188e-04, -1.19897610e-03, -5.29307501e-04,  1.11341012e-07,
])


def _make_mol(atom, basis):
    """Build a PySCF Mole object (pyscf is a runtime dependency of librint)."""
    import pyscf
    mol = pyscf.gto.M(atom=atom, basis=basis, verbose=0)
    mol.cart = True
    return mol


def test_sanity_hf_energy():
    """Librint HF energy for H2O/6-31G must match the known reference value."""
    mol = _make_mol(H2O_ATOM, H2O_BASIS)

    P = librint.scf.density(mol, imax=4000)
    E = librint.scf.energy(mol, P)

    np.testing.assert_allclose(E, H2O_REFERENCE_ENERGY, atol=1e-6, rtol=1e-6)


def test_sanity_hf_gradient():
    """Librint analytical gradient for H2O/6-31G must match the known reference."""
    mol = _make_mol(H2O_ATOM, H2O_BASIS)

    P = librint.scf.density(mol, imax=4000)
    grad = librint.dscf.danalyticalf(mol, P)

    np.testing.assert_allclose(
        np.sort(grad), np.sort(H2O_REFERENCE_GRADIENT), atol=1e-5, rtol=1e-4
    )
