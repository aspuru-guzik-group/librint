import numpy as np
import pyscf

# import libcscf
# import utils

import libcscf
import utils

import time

mol = pyscf.gto.M(atom='''
                    H 0 0 -0.4
                    H 0 0 0.4''',
                    basis='sto-3g')

# mol = pyscf.gto.M(atom='''
#                         O   -0.0000000   -0.1113512    0.0000000
#                         H    0.0000000    0.4454047   -0.7830363
#                         H   -0.0000000    0.4454047    0.7830363''',
#                         basis='def2-svp')
nelec = 2

atm = np.asarray(mol._atm, dtype=np.int32, order='C')
bas = np.asarray(mol._bas, dtype=np.int32, order='C')
env = np.asarray(mol._env, dtype=np.double, order='C')

print(bas)

S = libcscf.int1e(atm, bas, env, 'ovlp')
T = libcscf.int1e(atm, bas, env, 'kin')
V = libcscf.int1e(atm, bas, env, 'nuc')

# print(S)
# print(T)
# print(V)

P = libcscf.density(atm, bas, env, nelec)
E = libcscf.energy(atm, bas, env, P)

# print(P)
# print(E)

# start = time.time()
denv = libcscf.grad(atm, bas, env, P)
# end = time.time()
# print("denv time:         ", (end - start) * 1000000)

# start = time.time()
dS = libcscf.dSf(atm, bas, env, P)
dH = libcscf.dHcoref(atm, bas, env, P)
dR = libcscf.dRf(atm, bas, env, P)
# end = time.time()
# print("dHdRdS time:       ", (end - start) * 1000000)

start = time.time()
denergy = libcscf.denergyf(atm, bas, env, P)
end = time.time()
print("denergy time:       ", (end - start) * 1000000)

# start = time.time()
danalytical = libcscf.danalyticalf(atm, bas, env, P)
# end = time.time()
# print("analytical time:   ", (end - start) * 1000000)

print(denergy)
print(danalytical)

np.set_printoptions(precision=5)

print(dH)
print(dR)
print(dS)

print(denv)
print(dH + dR)

a, b = utils.split(bas)
h = 1e-6


fd = np.zeros(b-a)
for j in range(a, b):
    env[j] -= h
    P1 = libcscf.density(atm, bas, env, nelec)
    E1 = libcscf.energy(atm, bas, env, P1)
    env[j] += 2.0*h
    P2 = libcscf.density(atm, bas, env, nelec)
    E2 = libcscf.energy(atm, bas, env, P2)

    fd[j-a] = (E2 - E1)/(2.0*h)
    env[j] -= h

print(fd)

full = dH + dR - 0.5 * dS

print(full)

print(danalytical)
print(denergy)

