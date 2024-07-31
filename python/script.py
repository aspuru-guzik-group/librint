import numpy as np
import pyscf

COMP = 'C'

if COMP == 'R':
    import librscf as libscf
    from librscf import pmat
elif COMP == 'C':
    import libcscf as libscf
    from libcscf import pmat

mol = pyscf.gto.M(atom='''
                    H 0 0 -0.8
                    H 0 0 0.8''',
                    basis='sto-3g')

atm = np.asarray(mol._atm, dtype=np.int32, order='C')
bas = np.asarray(mol._bas, dtype=np.int32, order='C')
env = np.asarray(mol._env, dtype=np.double, order='C')

natm, nbas, nelec, nshells = 2, 2, 2, 2

print("## H2 ##\n")

P = libscf.RHF(natm, nbas, nelec, nshells, atm, bas, env)
print("P:")
pmat(P)

E = libscf.energy(natm, nbas, nshells, atm, bas, env, P)
print("E:\n", E)

# S = libscf.int1e(natm, nbas, nshells, atm, bas, env, 'ovlp')
# print("S:")
# pmat(S)

# T = libscf.int1e(natm, nbas, nshells, atm, bas, env, 'kin')
# print("T:")
# pmat(T)

denv = libscf.grad(natm, nbas, nshells, atm, bas, env, P)
print("d:\n", denv[28:34])
print()

print("## H2O ##\n")

mol = pyscf.gto.M(atom='''
                    O   -0.0000000   -0.1113512    0.0000000
                    H    0.0000000    0.4454047   -0.7830363
                    H   -0.0000000    0.4454047    0.7830363''',
                    basis='sto-3g')

atm = np.asarray(mol._atm, dtype=np.int32, order='C')
bas = np.asarray(mol._bas, dtype=np.int32, order='C')
env = np.asarray(mol._env, dtype=np.double, order='C')

natm, nbas, nelec, nshells = 3, 5, 10, 7

P = libscf.RHF(natm, nbas, nelec, nshells, atm, bas, env)
print("P:")
pmat(P)

E = libscf.energy(natm, nbas, nshells, atm, bas, env, P)
print("E:\n", E)

# S = libscf.int1e(natm, nbas, nshells, atm, bas, env, 'ovlp')
# print("S:")
# pmat(S)

# T = libscf.int1e(natm, nbas, nshells, atm, bas, env, 'kin')
# print("T:")
# pmat(T)

denv = libscf.grad(natm, nbas, nshells, atm, bas, env, P)
print("d:\n", denv[32:56])