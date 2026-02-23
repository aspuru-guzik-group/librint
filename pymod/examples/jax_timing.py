import timeit

import librint
from jax import value_and_grad

import os

import jax

import pyscf
from pyscfad import gto, scf

from basis_set_exchange import get_basis

basis = "sto-3g"
geo = "HF"

MAX_ITER=4000

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


geometries = {
    'HF': [
        ('H', 1, (0.0,0.0,-1.645509)),
        ('F', 9, (0.0,0.0,0.087291))
    ],
    'CH4': [
        ('C', 6, (0.00000000000000,0.000000000000000,0.000000000000000)),
        ('H', 1, (1.182181057825485, -1.182181057825485,1.182181057825485)),
        ('H', 1, (-1.182181057825485, 1.182181057825485,1.182181057825485)),
        ('H', 1, (1.182181057825485, 1.182181057825485,-1.182181057825485)),
        ('H', 1,(-1.182181057825485, -1.182181057825485,-1.182181057825485))
    ],
    'H2O': [
        ('O', 8, (0.0,0.0,0.091685801102911746)),
        ('H', 1, (1.4229678834888837,0.0,-0.98120954931681137)),
        ('H', 1, (-1.4229678834888837,0.0,-0.98120954931681137))
    ],
    'NH3': [
        ('N', 7, (0.0,0.0,0.127872)),
        ('H', 1, (1.77164,0.0,-0.592238)),
        ('H', 1, (0.88582,1.54288,-0.592238)),
        ('H', 1, (0.88582,-1.54288,-0.592238))
    ],
    'H2': [
        ('H', 1, (0.0,0.0,0.69217920969236535)),
        ('H', 1, (0.0,0.0,-0.69217920969236535))
    ],
    'LIH': [
        ('H', 1, (0.0,0.0,1.4624207)),
        ('Li', 3, (0.0,0.0,0.1))
    ],
}


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
def test_librint():
    gradient = librint.dscf.denergyf(mol_rpyscf, P)
    return gradient

def test_jax():
    # mol_jax = build_mol(atom, charge, basis)
    E, grad = value_and_grad(hf_energy)(mol_jax)
    return grad.coords, grad.ctr_coeff, grad.exp

def test_analytical():
    gradient = librint.dscf.danalyticalf(mol_rpyscf, P)
    return gradient
    
def test_grad():
    gradient = librint.dscf.grad(mol_rpyscf, P)
    return gradient


# Timing the functions
n_runs = 10  # Number of runs for averaging

time_librint = timeit.timeit(test_librint, number=n_runs)
time_jax = timeit.timeit(test_jax, number=n_runs)
time_analytical = timeit.timeit(test_analytical, number=n_runs)
time_grad = timeit.timeit(test_grad, number=n_runs)

# Print results
print(f"{geo} {basis}")
print(f"Average time for jax.value_and_grad: {time_jax / n_runs:.6f} seconds per run")
print(f"Average time for librint.denergy: {time_librint / n_runs:.6f} seconds per run")
print(f"Average time for librint.analytical: {time_analytical / n_runs:.6f} seconds per run")
print(f"Average time for librint.grad: {time_grad / n_runs:.6f} seconds per run")
