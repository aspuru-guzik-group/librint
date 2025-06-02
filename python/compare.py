import timeit

# Import the necessary libraries
import librpyscf
from jax import value_and_grad

import os

import jax

import pyscf
from pyscfad import gto, scf


import libcscf

do_test_primal = True

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
    'C6H6': [
    ('C', 6,      ( -1.8771137684,     -1.8237635912,      2.2841118024)),
    ('C', 6,      ( -4.5038328925,     -1.7156712646,      2.2625689261)),
    ('C', 6,      ( -5.7931929345,     -1.5410605833,     -0.0215428763)),
    ('C', 6,      ( -4.4564007702,     -1.4743532559,     -2.2843007750)),
    ('C', 6,      ( -1.8298706186,     -1.5826345550,     -2.2625689261)),
    ('C', 6,      ( -0.5403216040,     -1.7572452364,      0.0215428763)),
    ('H', 1,      ( -0.8625198851,     -1.9613356431,      4.0818081353)),
    ('H', 1,      ( -5.5562212955,     -1.7682056471,      4.0426908074)),
    ('H', 1,      ( -7.8601752208,     -1.4562118864,     -0.0385504102)),
    ('H', 1,      ( -5.4711836260,     -1.3369701765,     -4.0816191627)),
    ('H', 1,      ( -0.7780491334,     -1.5304781177,     -4.0430687526)),
    ('H', 1,      (  1.5264717097,     -1.8422829059,      0.0387393828))
    ],
}

# basis = "sto-3g"
# # basis = "6-31g*"
# geo = "CH4"
# # geo = "C6H6"

bases = ["sto-3g", "6-31g*"]

geos = ["H2", "LIH", "HF", "CH4", "H2O", "NH3", "C6H6"]

for basis in bases:
    print(basis)
    for geo in geos:
        print(geo)
        molecule = geometries[geo]

        atom = '\n'.join([f"{atom[0]} {0.529*atom[2][0]} {0.529*atom[2][1]} {0.529*atom[2][2]}" for atom in molecule])
        nelec = sum(atom[1] for atom in molecule)

        mol = pyscf.gto.M(atom=atom, 
                        basis=basis)

        charge = 0

        mol_rpyscf = pyscf.gto.M(atom=atom, 
                                basis=basis)

        mol_jax = build_mol(atom, charge, basis)  # Precomputed mol for JAX


        # AA: stuff below is mislabeled as JAX, but actually calling PySCF
        # reason was to test the int1e_ovlp JAX derivative, which turned out to be not easy

        def jax_int1e_ovlp(mol_jax):
            return mol_jax.intor('int1e_ovlp')

        def test_jax():
            return jax_int1e_ovlp(mol_jax)

        def test_mol_int1e_ovlp():
            return mol.intor('int1e_ovlp')

        def test_librpyscf():
            atm, bas, env, nelec = librpyscf.prep(mol_rpyscf)
            return libcscf.int1e(atm, bas, env, 'ovlp')

        n_runs = 5
        time_jax = timeit.timeit(test_jax, number=n_runs)
        time_ = timeit.timeit(test_mol_int1e_ovlp, number=n_runs)
        time_librpyscf = timeit.timeit(test_librpyscf, number=n_runs)

        print(f"Average time for jax.int1e_ovlp : {time_jax / n_runs:.6f} seconds per run")
        print(f"Average time for mol.int1e_ovlp : {time_ / n_runs:.6f} seconds per run")
        print(f"Average time for librpyscf.int1e: {time_librpyscf / n_runs:.6f} seconds per run")


# basis = "sto-3g"
basis = "6-31g*"
# geo = "CH4"
geo = "C6H6"

molecule = geometries[geo]

atom = '\n'.join([f"{atom[0]} {0.529*atom[2][0]} {0.529*atom[2][1]} {0.529*atom[2][2]}" for atom in molecule])
nelec = sum(atom[1] for atom in molecule)

charge = 0

mol_jax = build_mol(atom, charge, basis)

def fs():
    return mol_jax.intor('int1e_ovlp')

n_runs = 5
asd = timeit.timeit(fs, number=n_runs)

print(f"Average time for jax.int1e_ovlp : {asd / n_runs:.6f} seconds per run")

# for _ in range(3):
#     print(f"{timeit.timeit(fs, number=n_runs) / n_runs:.6f}")