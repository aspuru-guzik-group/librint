from scipy.optimize import minimize, Bounds
import numpy as np
import pyscf
import utils
import librpyscf

from basis_set_exchange import get_basis

sto_2g = get_basis('sto-2g', fmt='nwchem')
sto_3g = 'sto-3g'

MAX_ITER=4000


import jax
from pyscfad import gto, scf

jax.config.update("jax_platform_name", "cpu")


basis = sto_2g

def build_mol(atom, charge):
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

molecule = geometries['NH3']


charge = 0


atom = '\n'.join([f"{atom[0]} {0.529*atom[2][0]} {0.529*atom[2][1]} {0.529*atom[2][2]}" for atom in molecule])
nelec = sum(atom[1] for atom in molecule)

mol = build_mol(atom, charge)
E, grad = jax.value_and_grad(hf_energy)(mol)

full_grad = np.concatenate((grad.ctr_coeff, grad.exp))

print("pyscfad:", np.sort(full_grad))

mol = pyscf.gto.M(atom=atom, 
                  basis=basis)

P = librpyscf.density(mol, nelec, imax=MAX_ITER)
gradient = librpyscf.denergyf(mol, P)

print("librscf:", np.sort(gradient))

print("diff:", np.abs(np.sort(full_grad) - np.sort(gradient)))