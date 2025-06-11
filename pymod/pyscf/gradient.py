import os
import timeit

import numpy as np

from jax import value_and_grad
import jax

import pyscf
from pyscfad import gto, scf

import librint


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

# Define functions to time
def test_jax(mol_jax):
    # mol_jax = build_mol(atom, charge, basis)
    E, grad = value_and_grad(hf_energy)(mol_jax)
    return grad.coords, grad.ctr_coeff, grad.exp

def test_analytical(mol_rpyscf, P):
    gradient = librint.dscf.danalyticalf(mol_rpyscf, P)
    return gradient

def test_librpyscf(mol_rpyscf, P):
    gradient = librint.dscf.denergyf(mol_rpyscf, P)
    return gradient

def test_grad(mol_rpyscf, P):
    gradient = librint.dscf.grad(mol_rpyscf, P)
    return gradient

def gradient_test(mol_jax, mol_rpyscf, P):
    n_runs = 4

    time_jax = timeit.timeit(lambda: test_jax(mol_jax), number=n_runs)
    time_analytical = timeit.timeit(lambda: test_analytical(mol_rpyscf, P), number=n_runs)
    time_librpyscf = timeit.timeit(lambda: test_librpyscf(mol_rpyscf, P), number=n_runs)
    time_grad = timeit.timeit(lambda: test_grad(mol_rpyscf, P), number=n_runs)

    # Print results
    # print(f"{geo} {basis}")
    # print(f"Average time for jax.value_and_grad: {time_jax / n_runs:.6f} s/run")
    # print(f"Average time for librint.analytical: {time_analytical / n_runs:.6f} s/run")
    # print(f"Average time for librint.denergy   : {time_librpyscf / n_runs:.6f} s/run")
    # print(f"Average time for librint.grad      : {time_grad / n_runs:.6f} s/run")

    return (time_jax / n_runs, time_analytical / n_runs, time_librpyscf / n_runs, time_grad / n_runs)

if __name__ == '__main__':
    molecules = [
        ("sto-3g", "H2"),
        ("sto-3g", "HF"),
        ("sto-3g", "LIH"),
        ("sto-3g", "H2O"),
        ("def2-svp", "H2O"),
        ("sto-3g", "NH3"),
        ("sto-3g", "CH4"),
        # ("sto-3g", "C6H6"),
        # ("6-31g*", "CH4")
    ]
    
    # basis = "sto-3g"
    # basis = "6-31g*"
    # geo = "H2"
    # geo = "C6H6"

    for (basis, geo) in molecules:
        molecule = geometries[geo]

        atom = '\n'.join([f"{atom[0]} {0.529*atom[2][0]} {0.529*atom[2][1]} {0.529*atom[2][2]}" for atom in molecule])
        nelec = sum(atom[1] for atom in molecule)

        mol = pyscf.gto.M(atom=atom, basis=basis)

        charge = 0

        mol_rpyscf = pyscf.gto.M(atom=atom, 
                                basis=basis)

        P = librint.scf.density(mol_rpyscf, imax=MAX_ITER)

        mol_jax = build_mol(atom, charge, basis)  # Precomputed mol for JAX

        a, b, c = test_jax(mol_jax)
        g1 = np.concatenate([c, b])
        g2 = librint.dscf.danalyticalf(mol_rpyscf, P)
        g3 = librint.dscf.denergyf(mol_rpyscf, P)
        g4 = librint.dscf.grad(mol_rpyscf, P)

        (t1, t2, t3, t4) = gradient_test(mol_jax, mol_rpyscf, P)

        print(f"{geo} {basis}")
        print(f"Avg Times")
        print(f"jax.value_and_grad: {t1:.6f} s/run")
        print(f"librint.analytical: {t2:.6f} s/run")
        print(f"librint.denergy   : {t3:.6f} s/run")
        print(f"librint.grad      : {t4:.6f} s/run")

        print("\nGradients")
        print("{}".format(np.sort(g1)))
        print("{}".format(np.sort(g2)))
        print("{}".format(np.sort(g3)))
        print("{}".format(np.sort(g4)))

        # file_time = "./timing/" + geo + "_" + basis

        # os.makedirs(os.path.dirname(file_time), exist_ok=True)

        # with open(file_time, 'a') as f:
        #     # f.write(f"{geo} {basis}\n")
        #     f.write(f"jax: {t1}\n")
        #     f.write(f"analytical: {t2}\n")
        #     f.write(f"denergy: {t3}\n")
        #     f.write(f"grad: {t4}\n")
        #     f.write(f"librint / jax: {(t3 / t1):.6f}\n\n")
        
        file_grad = "./grad/" + geo + "_" + basis

        os.makedirs(os.path.dirname(file_grad), exist_ok=True)

        with open(file_grad, 'a') as f:
            # f.write(f"{geo} {basis}\n")
            f.write(f"jax: {np.sort(g1)}\n")
            f.write(f"analytical: {np.sort(g2)}\n")
            f.write(f"denergy: {np.sort(g3)}\n")
            f.write(f"grad: {np.sort(g4)}\n")

