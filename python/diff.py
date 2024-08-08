import numpy as np
import pyscf

import libcscf2
import libcscf
import utils

import time

SAVE = True

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

def derivatives(atm, bas, env):
    dS = []

    for i in range(a, b):
        env[i] += h
        S1 = libcscf2.int1e(atm, bas, env, 'ovlp')

        env[i] -= 2.0*h
        S2 = libcscf2.int1e(atm, bas, env, 'ovlp')

        dS.append(-(S2 - S1)/(2.0*h))

        env[i] += h

    dS = np.array(dS)

    dH = []

    for i in range(a, b):
        env[i] += h
        T1 = libcscf2.int1e(atm, bas, env, 'kin')
        V1 = libcscf2.int1e(atm, bas, env, 'nuc')
        H1 = T1 + V1

        env[i] -= 2.0*h
        T2 = libcscf2.int1e(atm, bas, env, 'kin')
        V2 = libcscf2.int1e(atm, bas, env, 'nuc')
        H2 = T2 + V2

        dH.append(-(H2 - H1)/(2.0*h))

        env[i] += h

    dH = np.array(dH)

    dR = []

    for i in range(a, b):
        env[i] += h
        R1 = libcscf2.int2e(atm, bas, env)

        env[i] -= 2.0*h
        R2 = libcscf2.int2e(atm, bas, env)

        dR.append(-(R2 - R1)/(2.0*h))

        env[i] += h

    dR = np.array(dR)

    return dH, dR, dS


np.set_printoptions(precision=5)

P = libcscf2.RHF(atm, bas, env, nelec)
E = libcscf2.energy(atm, bas, env, P)

# P0 = libcscf.RHF(atm, bas, env, nelec)
# E0 = libcscf.energy(atm, bas, env, P)

# print()
# print("P & E")
# print("norm(P-P): ", np.linalg.norm(P - P0))
# print("E-E:       ", E - E0)
# print()

F = libcscf2.calcF(atm, bas, env, P)

dH, dR, dS = derivatives(atm, bas, env)

grad_hcore = 0.5 * np.tensordot(dH, P)
grad_two = 0.5 * 0.25 * np.tensordot(np.tensordot(dR, P), P)
grad_ovlp = - 0.25 * np.tensordot(dS, P @ F @ P)

dH0 = libcscf.dHcoref(atm, bas, env, P)
dR0 = libcscf.dRf(atm, bas, env, P)
dS0 = libcscf.dSf(atm, bas, env, P)

print()
print("derivatives: dH, dR, dS")
print("dH fd:    ", np.tensordot(dH, P))
print("dH rust:  ", dH0)

print("dR fd:    ", np.tensordot(np.tensordot(dR, P), P))
print("dR rust:  ", dR0)

print("dS fd:    ", np.tensordot(dS, P @ F @ P))
print("dS rust:  ", dS0)
print()

denv = libcscf.grad(atm, bas, env, P)

print()
print("ad energy vs hcore + two")
print("ad energy:      ", denv)
print("dH + 0.25 * dR: ", dH0 + 0.25 * dR0)
print()

h = 1e-6

fd = np.zeros(b-a)
for j in range(a, b):
    env[j] -= h
    P1 = libcscf2.RHF(atm, bas, env, nelec)
    E1 = libcscf2.energy(atm, bas, env, P1)
    env[j] += 2.0*h
    P2 = libcscf2.RHF(atm, bas, env, nelec)
    E2 = libcscf2.energy(atm, bas, env, P2)

    fd[j-a] = (E2 - E1)/(2.0*h)
    env[j] -= h


print()
print("gradients")
print("finite diff:           ", fd)
print("ad e - 0.5 dS:         ", denv - 0.5 * dS0)
print("dH + 0.25 dR - 0.5 dS: ", dH0 + 0.25*dR0 - 0.5 * dS0)
print()
