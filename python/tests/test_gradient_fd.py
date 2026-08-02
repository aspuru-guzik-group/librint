import pytest
import numpy as np
import jax

import pyscf
from pyscf import scf

import librint
from pyscf_comp.geometries import geometries

jax.config.update("jax_enable_x64", True)

MAX_ITER = 4000

def hf_energy(mol):
    mf = scf.RHF(mol)
    mf.verbose = 0
    ehf = mf.kernel()
    return ehf

def calc_fd(mol):
    a, b = librint.utils.split(mol._bas)
    h = 1e-6

    fd = np.zeros(b-a)
    for j in range(a, b):
        mol._env[j] -= h
        P1 = librint.scf.density(mol, imax=MAX_ITER)
        E1 = librint.scf.energy(mol, P1)
        
        mol._env[j] += 2.0*h
        P2 = librint.scf.density(mol, imax=MAX_ITER)
        E2 = librint.scf.energy(mol, P2)

        fd[j-a] = (E2 - E1)/(2.0*h)
        mol._env[j] -= h

    return fd

MOLECULES = [
    ("sto-3g", "H2"),
    # ("sto-3g", "HF"),
    ("sto-3g", "LIH"),
    ("sto-3g", "H2O"),
    ("sto-3g", "NH3"),
    # ("sto-3g", "CH4"),
]

@pytest.mark.parametrize("basis, geo", MOLECULES)
def test_gradient_consistency(basis, geo):
    molecule = geometries[geo]

    # Parse and generate atom format strings
    atom = '\n'.join([f"{a[0]} {0.529*a[2][0]} {0.529*a[2][1]} {0.529*a[2][2]}" for a in molecule])
    charge = 0

    mol_rpyscf = pyscf.gto.M(atom=atom, basis=basis)
    P = librint.scf.density(mol_rpyscf, imax=MAX_ITER)

    grad_fd = calc_fd(mol_rpyscf)
    grad_analytical = librint.dscf.danalyticalf(mol_rpyscf, P)
    grad_denergy = librint.dscf.denergyf(mol_rpyscf, P)


    # Sort the gradients to match the original gradient.py representation
    grad_fd_sorted = np.sort(grad_fd)
    grad_analytical_sorted = np.sort(grad_analytical)

    # Validate against jax benchmark
    np.testing.assert_allclose(grad_analytical_sorted, grad_fd_sorted, atol=1e-5, rtol=1e-4)


@pytest.mark.skipif(
    not librint._bindings.HAS_PAR,
    reason="librint.so has no threaded entry points; rebuild and set LIBRINT_SO",
)
@pytest.mark.parametrize("basis, geo", MOLECULES)
def test_gradient_consistency_threaded(basis, geo):
    """The same finite-difference check, through the threaded path.

    test_par_equiv.py already ties danalytical_par to danalyticalf, and the
    test above ties danalyticalf to finite differences, so this is transitively
    covered. It is here anyway because the transitive argument breaks silently
    if either link is ever weakened, and this one is direct.
    """
    molecule = geometries[geo]
    atom = '\n'.join([f"{a[0]} {0.529*a[2][0]} {0.529*a[2][1]} {0.529*a[2][2]}" for a in molecule])

    mol_rpyscf = pyscf.gto.M(atom=atom, basis=basis)
    P = librint.scf.density(mol_rpyscf, imax=MAX_ITER)

    grad_fd = calc_fd(mol_rpyscf)
    # 0 = rayon's global pool, sized by RAYON_NUM_THREADS; whatever the machine
    # running the suite happens to have is a fine width for a correctness check
    grad_par = librint.dscf.danalytical_par(mol_rpyscf, P, 0)

    np.testing.assert_allclose(np.sort(grad_par), np.sort(grad_fd),
                               atol=1e-5, rtol=1e-4)
