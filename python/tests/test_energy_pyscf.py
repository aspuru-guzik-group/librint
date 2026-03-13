import pytest
import numpy as np
import pyscf

import librint
from pyscf_comp.geometries import geometries

MAX_ITER = 4000

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
def test_energy_consistency(basis, geo):
    molecule = geometries[geo]

    # Parse and generate atom format strings
    atom = '\n'.join([f"{a[0]} {0.529*a[2][0]} {0.529*a[2][1]} {0.529*a[2][2]}" for a in molecule])

    mol_pyscf = pyscf.gto.M(atom=atom, basis=basis, verbose=0)
    # make it cartesian
    mol_pyscf.cart = True
    

    mf = pyscf.scf.RHF(mol_pyscf)
    mf.max_cycle = MAX_ITER
    mf.verbose = 0
    mf.conv_tol = 1e-6
    E_pyscf = mf.kernel()
    P_pyscf = mf.make_rdm1()


    P_librint = librint.scf.density(mol_pyscf, imax=MAX_ITER)
    E_librint = librint.scf.energy(mol_pyscf, P_pyscf)
    # E_librint = librint.scf.scf(mol_pyscf, imax=MAX_ITER, conv=1e-6)

    # Validate against PySCF energy
    np.testing.assert_allclose(E_librint, E_pyscf, atol=1e-6, rtol=1e-6)

    print(E_librint, E_pyscf)
