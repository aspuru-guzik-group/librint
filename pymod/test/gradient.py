import numpy as np
import pyscf

from librint import scf, dscf, utils

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

# print(atm)
# print(bas)
# print(env)
# print(nelec)

# print(bas)


S = scf.int1e(mol, 'ovlp')
T = scf.int1e(mol, 'kin')
V = scf.int1e(mol, 'nuc')

# print(S)
# print(T)
# print(V)

P = scf.density(mol)
E = scf.energy(mol, P)

print(P)
print(E)

# start = time.time()
denv = dscf.grad(mol, P)
# end = time.time()
# print("denv time:         ", (end - start) * 1000000)

# start = time.time()
dS = dscf.dSf(mol, P)
dH = dscf.dHcoref(mol, P)
dR = dscf.dRf(mol, P)
# end = time.time()
# print("dHdRdS time:       ", (end - start) * 1000000)

start = time.time()
denergy = dscf.denergyf(mol, P)
end = time.time()
print("denergy time:       ", (end - start) * 1000000)

# start = time.time()
danalytical = dscf.danalyticalf(mol, P)
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
    P1 = scf.density(mol)
    E1 = scf.energy(mol, P1)
    env[j] += 2.0*h
    P2 = scf.density(mol)
    E2 = scf.energy(mol, P2)

    fd[j-a] = (E2 - E1)/(2.0*h)
    env[j] -= h

print(fd)

full = dH + dR - 0.5 * dS

print(full)

print(danalytical)
print(denergy)

