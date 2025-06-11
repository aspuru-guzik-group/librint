import numpy as np
import pyscf

from librint.scf import density, energy, int1e
from librint.dscf import grad
from librint import utils

mol = pyscf.gto.M(atom='''
                    H 0 0 -0.8
                    H 0 0 0.8''',
                    basis='sto-3g')

atm = np.asarray(mol._atm, dtype=np.int32, order='C')
bas = np.asarray(mol._bas, dtype=np.int32, order='C')
env = np.asarray(mol._env, dtype=np.double, order='C')

nelec = 2

print("## H2 ##\n")

P = density(mol, nelec)
print("P:")
utils.pmat(P)

E = energy(mol, P)
print("E:\n", E)

S = int1e(mol, 'ovlp')
print("S:")
utils.pmat(S)

T = int1e(mol, 'kin')
print("T:")
utils.pmat(T)

denv = grad(mol, P)
print("d:\n", denv)
print()

# print("## H2O ##\n")

# mol = pyscf.gto.M(atom='''
#                     O   -0.0000000   -0.1113512    0.0000000
#                     H    0.0000000    0.4454047   -0.7830363
#                     H   -0.0000000    0.4454047    0.7830363''',
#                     basis='sto-3g')

# atm = np.asarray(mol._atm, dtype=np.int32, order='C')
# bas = np.asarray(mol._bas, dtype=np.int32, order='C')
# env = np.asarray(mol._env, dtype=np.double, order='C')

# nelec = 10

# P = libscf.RHF(mol, nelec)
# print("P:")
# utils.pmat(P)

# E = libscf.energy(mol, P)
# print("E:\n", E)

# S = libscf.int1e(mol, 'ovlp')
# print("S:")
# utils.pmat(S)

# T = libscf.int1e(mol, 'kin')
# print("T:")
# utils.pmat(T)

# denv = libscf.grad(mol, P)
# print("d:\n", denv)
