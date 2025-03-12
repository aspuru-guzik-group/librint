import numpy as np
import pyscf

COMP = 'R'

import utils

if COMP == 'R':
    import librscf as libscf
elif COMP == 'C':
    import libcscf as libscf

mol = pyscf.gto.M(atom='''
                    H 0 0 -0.8
                    H 0 0 0.8''',
                    basis='sto-3g')

atm = np.asarray(mol._atm, dtype=np.int32, order='C')
bas = np.asarray(mol._bas, dtype=np.int32, order='C')
env = np.asarray(mol._env, dtype=np.double, order='C')

nelec = 2

print("## H2 ##\n")

P = libscf.RHF(atm, bas, env, nelec)
print("P:")
utils.pmat(P)

E = libscf.energy(atm, bas, env, P)
print("E:\n", E)

S = libscf.int1e(atm, bas, env, 'ovlp')
print("S:")
utils.pmat(S)

T = libscf.int1e(atm, bas, env, 'kin')
print("T:")
utils.pmat(T)

denv = libscf.grad(atm, bas, env, P)
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

# P = libscf.RHF(atm, bas, env, nelec)
# print("P:")
# utils.pmat(P)

# E = libscf.energy(atm, bas, env, P)
# print("E:\n", E)

# S = libscf.int1e(atm, bas, env, 'ovlp')
# print("S:")
# utils.pmat(S)

# T = libscf.int1e(atm, bas, env, 'kin')
# print("T:")
# utils.pmat(T)

# denv = libscf.grad(atm, bas, env, P)
# print("d:\n", denv)
