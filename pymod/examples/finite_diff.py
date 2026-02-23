import numpy as np
import pyscf

from librint import scf, dscf, utils

import time

# SAVE = True

h = 1e-6

H2 = 0

if H2:
    mol = pyscf.gto.M(atom='''
                        H 0 0 -0.4
                        H 0 0 0.4''',
                        basis='sto-3g')
    nelec = 2
else:
    mol = pyscf.gto.M(atom='''
                        O   -0.0000000   -0.1113512    0.0000000
                        H    0.0000000    0.4454047   -0.7830363
                        H   -0.0000000    0.4454047    0.7830363''',
                        basis='sto-3g')
    nelec = 10

atm = np.asarray(mol._atm, dtype=np.int32, order='C')
bas = np.asarray(mol._bas, dtype=np.int32, order='C')
env = np.asarray(mol._env, dtype=np.double, order='C')

a, b = utils.split(bas)

def derivatives(mol):
    dS = []

    for i in range(a, b):
        env[i] += h
        S1 = scf.int1e(mol, 'ovlp')

        env[i] -= 2.0*h
        S2 = scf.int1e(mol, 'ovlp')

        dS.append(-(S2 - S1)/(2.0*h))

        env[i] += h

    dS = np.array(dS)

    dH = []

    for i in range(a, b):
        env[i] += h
        T1 = scf.int1e(mol, 'kin')
        V1 = scf.int1e(mol, 'nuc')
        H1 = T1 + V1

        env[i] -= 2.0*h
        T2 = scf.int1e(mol, 'kin')
        V2 = scf.int1e(mol, 'nuc')
        H2 = T2 + V2

        dH.append(-(H2 - H1)/(2.0*h))

        env[i] += h

    dH = np.array(dH)

    dR = []

    for i in range(a, b):
        env[i] += h
        R1 = scf.int2e(mol)

        env[i] -= 2.0*h
        R2 = scf.int2e(mol)

        dR.append(-(R2 - R1)/(2.0*h))

        env[i] += h

    dR = np.array(dR)

    return dH, dR, dS


np.set_printoptions(precision=5)

P = scf.density(mol)
E = scf.energy(mol, P)

def calcF(mol, P):
    H = scf.int1e(mol, 'kin') + scf.int1e(mol, 'nuc')
    R = scf.int2e(mol)
    # ls * mnsl - ls * mlsn
    J = np.einsum('ijkl,lk->ij', R, P)
    K = np.einsum('ilkj,lk->ij', R, P)

    F = H + (J - 0.5*K)
    return F

F = calcF(mol, P)

dH, dR, dS = derivatives(mol)

grad_hcore = 0.5 * np.tensordot(dH, P)
grad_two = 0.5 * 0.25 * np.tensordot(np.tensordot(dR, P), P)
grad_ovlp = - 0.25 * np.tensordot(dS, P @ F @ P)

dH0 = dscf.dHcoref(mol, P)
dR0 = dscf.dRf(mol, P)
dS0 = dscf.dSf(mol, P)

print()
print("derivatives: dH, dR, dS")
print("dH fd:    ", np.tensordot(dH, P))
print("dH rust:  ", dH0)

print("dR fd:    ", 0.25 * np.tensordot(np.tensordot(dR, P), P))
print("dR rust:  ", dR0)

print("dS fd:    ", np.tensordot(dS, P @ F @ P))
print("dS rust:  ", dS0)
print()

denv = dscf.grad(mol, P)

print()
print("ad energy vs hcore + two")
print("ad energy:      ", denv)
print("dH + 0.25 * dR: ", dH0 + dR0)
print()

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


print()
print("gradients")
print("finite diff:           ", fd)
print("ad e - 0.5 dS:         ", denv - 0.5 * dS0)
print("dH + dR - 0.5 dS:      ", dH0 + dR0 - 0.5 * dS0)
print()

de = dscf.denergyf(mol, P)
# import librpyscf
# de = librpyscf.denergyf(mol, P)

print()
print("de                     ", de)