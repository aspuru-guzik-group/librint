import librscf
import numpy as np
import pyscf

mol = pyscf.gto.M(atom='''
                    H 0 0 -0.8
                    H 0 0 0.8''',
                    basis='sto-3g')

atm = np.asarray(mol._atm, dtype=np.int32, order='C')
bas = np.asarray(mol._bas, dtype=np.int32, order='C')
env = np.asarray(mol._env, dtype=np.double, order='C')

natm, nbas, nelec, nshells = 2, 2, 2, 2

P = librscf.RHF(natm, nbas, nelec, nshells, atm, bas, env)
print("P:\n", P)

E = librscf.energy(natm, nbas, nshells, atm, bas, env, P)
print("E:\n", E)

S = librscf.int1e(natm, nbas, nshells, atm, bas, env, 'ovlp')
print("S:\n", S)

T = librscf.int1e(natm, nbas, nshells, atm, bas, env, 'kin')
print("T:\n", T)