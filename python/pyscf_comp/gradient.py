"""Per-molecule gradient/timing dumps under grad/ and timing/, plotted by
plot_gradient.py and plot_timing.py.

The committed dumps were deleted: they dated from 2026-03, months before
src/eri.rs and the batched dRg landed, and recorded a "denergy" column that no
longer exists as a separate method (see src/dscf.rs). Re-run this script to
regenerate them against the current build rather than reading the old numbers.
"""
import os
import timeit

import numpy as np

from jax import value_and_grad
import jax

import pyscf
from pyscfad import gto, scf

import librint
from geometries import geometries

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

def test_fd(mol):
    a, b = librint.utils.split(mol._bas)
    h = 1e-12

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

def gradient_test(mol_jax, mol_rpyscf, P):
    n_runs = 4

    time_fd = timeit.timeit(lambda: test_fd(mol_rpyscf), number=n_runs)
    time_jax = timeit.timeit(lambda: test_jax(mol_jax), number=n_runs)
    time_analytical = timeit.timeit(lambda: test_analytical(mol_rpyscf, P), number=n_runs)
    time_librpyscf = timeit.timeit(lambda: test_librpyscf(mol_rpyscf, P), number=n_runs)
    time_grad = timeit.timeit(lambda: test_grad(mol_rpyscf, P), number=n_runs)

    return (time_fd / n_runs, time_jax / n_runs, time_analytical / n_runs, time_librpyscf / n_runs, time_grad / n_runs)

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
        # ("6-31g*", "C6H6")
    ]

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

        g0 = test_fd(mol_rpyscf)
        g1 = np.concatenate([c, b])
        g2 = librint.dscf.danalyticalf(mol_rpyscf, P)
        g3 = librint.dscf.denergyf(mol_rpyscf, P)
        g4 = librint.dscf.grad(mol_rpyscf, P)

        (t0, t1, t2, t3, t4) = gradient_test(mol_jax, mol_rpyscf, P)

        print(f"{geo} {basis}")
        print(f"Avg Times")
        print(f"finite difference : {t0:.6f} s/run")
        print(f"jax.value_and_grad: {t1:.6f} s/run")
        print(f"librint.analytical: {t2:.6f} s/run")
        print(f"librint.denergy   : {t3:.6f} s/run")
        print(f"librint.grad      : {t4:.6f} s/run")

        print("\nGradients")
        print("{}".format(np.sort(g0)))
        print("{}".format(np.sort(g1)))
        print("{}".format(np.sort(g2)))
        print("{}".format(np.sort(g3)))
        print("{}".format(np.sort(g4)))

        file = "_run1"

        file_time = "./time" + file + "/" + geo + "_" + basis

        os.makedirs(os.path.dirname(file_time), exist_ok=True)

        with open(file_time, 'a') as f:
            # f.write(f"{geo} {basis}\n")
            f.write(f"fd:         {t0}\n")
            f.write(f"jax:        {t1}\n")
            f.write(f"analytical: {t2}\n")
            f.write(f"denergy:    {t3}\n")
            f.write(f"grad:       {t4}\n")
            f.write(f"librint / jax: {(t3 / t1):.6f}\n\n")
        
        file_grad = "./grad" + file + "/" + geo + "_" + basis

        os.makedirs(os.path.dirname(file_grad), exist_ok=True)

        with open(file_grad, 'a') as f:
            # f.write(f"{geo} {basis}\n")
            f.write(f"fd:         {np.sort(g0)}\n")
            f.write(f"jax:        {np.sort(g1)}\n")
            f.write(f"analytical: {np.sort(g2)}\n")
            f.write(f"denergy:    {np.sort(g3)}\n")
            f.write(f"grad:       {np.sort(g4)}\n")

