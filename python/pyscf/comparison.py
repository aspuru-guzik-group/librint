import timeit

# Import the necessary libraries
from jax import value_and_grad

import os

import jax

import pyscf
from pyscfad import gto, scf

from basis_set_exchange import get_basis

import librint
from geometries import geometries

basis = "sto-3g"
# basis = "6-31g*"
geo = "CH4"
# geo = "C6H6"

MAX_ITER=4000

do_test_gradient = True
do_test_primal = False

jax.config.update("jax_platform_name", "cpu")

def build_mol(atom, charge, basis):
    mol = gto.Mole()
    mol.atom = atom
    mol.unit = 'Angstrom'
    mol.basis = basis
    mol.charge = charge
    mol.verbose = 0
    mass = mol.atom_mass_list(isotope_avg=True)
    mass *= 1822.88839 # amu to au
    mol.build(trace_coords=True, trace_exp=True, trace_ctr_coeff=True)
    return mol

def hf_energy(mol):
    mf = scf.RHF(mol)
    mf.verbose = 0
    mf.max_cycle = 100
    mf.conv_tol = 1e-8
    mf.conv_tol_grad = 1e-6
    mf.conv_tol_normt = 1e-6
    mf.conv_tol_energy = 1e-8
    mf.conv_tol_de = 1e-8
    ehf = mf.kernel()
    return ehf

jax.config.update("jax_enable_x64", True)


molecule = geometries[geo]

atom = '\n'.join([f"{atom[0]} {0.529*atom[2][0]} {0.529*atom[2][1]} {0.529*atom[2][2]}" for atom in molecule])
nelec = sum(atom[1] for atom in molecule)

mol = pyscf.gto.M(atom=atom, 
                  basis=basis)

charge = 0

mol_rpyscf = pyscf.gto.M(atom=atom, 
                          basis=basis)

P = librint.scf.density(mol_rpyscf, imax=MAX_ITER)

mol_jax = build_mol(atom, charge, basis)  # Precomputed mol for JAX

# Define functions to time
def test_librpyscf():
    gradient = librint.dscf.denergyf(mol_rpyscf, P)
    return gradient

def test_jax():
    mol_jax = build_mol(atom, charge, basis)
    E, grad = value_and_grad(hf_energy)(mol_jax)
    return grad.coords, grad.ctr_coeff, grad.exp

def test_analytical():
    gradient = librint.dscf.danalyticalf(mol_rpyscf, P)
    return gradient
    
def test_grad():
    gradient = librint.dscf.grad(mol_rpyscf, P)
    return gradient

if do_test_gradient:
    # Timing the functions
    n_runs = 4  # Number of runs for averaging

    time_librpyscf = timeit.timeit(test_librpyscf, number=n_runs)
    time_jax = timeit.timeit(test_jax, number=n_runs)
    time_analytical = timeit.timeit(test_analytical, number=n_runs)
    time_grad = timeit.timeit(test_grad, number=n_runs)

    # Print results
    print(f"{geo} {basis}")
    print(f"Average time for librpyscf.denergy: {time_librpyscf / n_runs:.6f} seconds per run")
    print(f"Average time for jax.value_and_grad: {time_jax / n_runs:.6f} seconds per run")
    print(f"Average time for librpyscf.analytical: {time_analytical / n_runs:.6f} seconds per run")
    print(f"Average time for librpyscf.grad: {time_grad / n_runs:.6f} seconds per run")


    # # print grads
    # grad_jax = test_jax()
    # grad_librpyscf = test_librpyscf()
    # print("Gradient from JAX:", grad_jax)
    # print("Gradient from librpyscf:", grad_librpyscf)



# hackie code for writing results to file

# sto_2g = get_basis('sto-2g', fmt='nwchem')
# sto_3g = 'sto-3g'

# if (basis == sto_2g):
#     bas = "sto-2g"
# else:
#     bas = "sto-3g"


# file_path = "timing/test"

# os.makedirs(os.path.dirname(file_path), exist_ok=True)


# with open(file_path, 'a') as f:
#     f.write(f"{geo} {bas}\n")
#     f.write(f"librpyscf.denergy: {time_librpyscf / n_runs:.6f} seconds per run\n")
#     f.write(f"jax.value_and_grad: {time_jax / n_runs:.6f} seconds per run\n")
#     f.write(f"jax / librpyscf: {(time_jax / n_runs)/(time_librpyscf / n_runs):.6f}\n\n")



# AA: stuff below is mislabeled as JAX, but actually calling PySCF
# reason was to test the int1e_ovlp JAX derivative, which turned out to be not easy
def jax_int1e_ovlp(mol_jax):
    return mol_jax.intor('int1e_ovlp')
    # return mol.intor('int2e')
    # return mol_jax.intor('int2e')


def test_jax():
    mol_jax = build_mol(atom, charge, basis)
    # return value_and_grad(jax_int1e_ovlp)(mol_jax)
    return jax_int1e_ovlp(mol_jax)

def test_librpyscf():
    # atm, bas, env, nelec = librint.utils.prep(mol_rpyscf)
    mol_rpyscf = pyscf.gto.M(atom=atom, 
                          basis=basis,
                          charge=charge)
    return librint.scf.int1e(mol_rpyscf, 'ovlp')

# # time
if do_test_primal:
    n_runs = 3
    
    time_librpyscf = timeit.timeit(test_librpyscf, number=n_runs)
    # time_mol = timeit.timeit(jax_int1e_ovlp, number=n_runs)
    time_jax = timeit.timeit(test_jax, number=n_runs)

    print(f"Average time for librpyscf.int1e: {time_librpyscf / n_runs:.6f} seconds per run")
    # print(f"Average time for mol.int1e_ovlp : {time_mol / n_runs:.6f} seconds per run")
    print(f"Average time for jax.int1e_ovlp : {time_jax / n_runs:.6f} seconds per run")
