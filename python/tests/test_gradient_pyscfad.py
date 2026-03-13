import pytest
import numpy as np
import jax

import pyscf

from pyscfad import gto, scf

import librint
from pyscf_comp.geometries import geometries

jax.config.update("jax_enable_x64", True)

MAX_ITER = 4000

def build_mol(atom, charge, basis):
    mol = gto.Mole(
        atom=atom,
        unit='Angstrom',
        basis=basis,
        charge=charge,
        verbose=0
    )
    mol.build(trace_coords=True, trace_exp=True, trace_ctr_coeff=True)
    return mol

def hf_energy(mol):
    mf = scf.RHF(mol)
    mf.verbose = 0
    ehf = mf.kernel()
    return ehf

def calc_jax(mol_jax):
    E, grad = jax.value_and_grad(hf_energy)(mol_jax)
    return grad.coords, grad.ctr_coeff, grad.exp

MOLECULES = [
    ("sto-3g", "H2"),
    ("sto-3g", "HF"),
    ("sto-3g", "LIH"),
    ("sto-3g", "H2O"),
    ("def2-svp", "H2O"),
    ("sto-3g", "NH3"),
    ("sto-3g", "CH4"),
]

@pytest.mark.parametrize("basis, geo", MOLECULES)
def test_gradient_consistency(basis, geo):
    molecule = geometries[geo]

    # Parse and generate atom format strings
    atom = '\n'.join([f"{a[0]} {0.529*a[2][0]} {0.529*a[2][1]} {0.529*a[2][2]}" for a in molecule])
    charge = 0

    mol_rpyscf = pyscf.gto.M(atom=atom, basis=basis)
    mol_rpyscf.cart = True
    P_librint = librint.scf.density(mol_rpyscf, imax=MAX_ITER)

    mol_jax = build_mol(atom, charge, basis)
    mol_jax.cart = True

    a, b, c = calc_jax(mol_jax)
    mf = pyscf.scf.RHF(mol_jax)
    mf.kernel()
    P_jax = mf.make_rdm1()

    # grad_fd = calc_fd(mol_rpyscf)
    grad_jax = np.concatenate([c, b])
    grad_analytical = librint.dscf.danalyticalf(mol_rpyscf, P_jax)
    # grad_denergy = librint.dscf.denergyf(mol_rpyscf, P)

    # Sort the gradients to match the original gradient.py representation
    # grad_fd_sorted = np.sort(grad_fd)
    grad_jax_sorted = np.sort(grad_jax)
    grad_analytical_sorted = np.sort(grad_analytical)
    # grad_denergy_sorted = np.sort(grad_denergy)

    # Validate against jax benchmark
    # np.testing.assert_allclose(grad_analytical, grad_jax, atol=1e-5, rtol=1e-4)
    np.testing.assert_allclose(grad_analytical_sorted, grad_jax_sorted, atol=1e-5, rtol=1e-4)
    # np.testing.assert_allclose(grad_denergy_sorted, grad_jax_sorted, atol=1e-5, rtol=1e-4)
    # np.testing.assert_allclose(grad_fd_sorted, grad_jax_sorted, atol=1e-5, rtol=1e-4)
    # np.testing.assert_allclose(grad_fd_sorted, grad_denergy_sorted, atol=1e-5, rtol=1e-4)
