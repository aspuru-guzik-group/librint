import numpy as np
import pyscf

import utils
import libcscf

import time

SAVE = True

# mol = pyscf.gto.M(atom='''
#                     O   -0.0000000   -0.1113512    0.0000000
#                     H    0.0000000    0.4454047   -0.7830363
#                     H   -0.0000000    0.4454047    0.7830363''',
#                     basis='sto-3g')

# atm = np.asarray(mol._atm, dtype=np.int32, order='C')
# bas = np.asarray(mol._bas, dtype=np.int32, order='C')
# env = np.asarray(mol._env, dtype=np.double, order='C')

# nelec = 10

mol = pyscf.gto.M(atom='''
                    H 0 0 -0.4
                    H 0 0 0.4''',
                    basis='sto-3g')

atm = np.asarray(mol._atm, dtype=np.int32, order='C')
bas = np.asarray(mol._bas, dtype=np.int32, order='C')
env = np.asarray(mol._env, dtype=np.double, order='C')

nelec = 2

a, b = utils.split(bas)
print(a, b)

# S = libcscf.int1e(atm, bas, env, 'ovlp')
# print("S: ")
# utils.pmat(S)

alpha = 1e-4
max_i = 10000
epsilon = 1e-4
delta = 1
i = 0

h = 1e-6

norm_arr = []
E_arr = []

while not (delta < epsilon or i == max_i):
    # NORMALIZE ENV
    utils.basis_atm(bas, env)
    
    P = libcscf.RHF(atm, bas, env, nelec)
    E = libcscf.energy(atm, bas, env, P)
    denv = libcscf.grad(atm, bas, env, P)

    dH = libcscf.dHcoref(atm, bas, env, P)
    dR = 0.25 * libcscf.dRf(atm, bas, env, P)
    dS = - 0.5 * libcscf.dSf(atm, bas, env, P)

    grad = dH + dR + dS

    full = denv + dS

    fd = np.zeros(b-a)
    for j in range(a, b):
        env[j] -= h
        P1 = libcscf.RHF(atm, bas, env, nelec)
        E1 = libcscf.energy(atm, bas, env, P1)
        env[j] += 2.0*h
        P2 = libcscf.RHF(atm, bas, env, nelec)
        E2 = libcscf.energy(atm, bas, env, P2)

        fd[j-a] = (E2 - E1)/(2.0*h)
        env[j] -= h
    
    
    env[a:b] = env[a:b] - alpha * fd
    delta = np.linalg.norm(fd)

    norm_arr.append(delta)
    E_arr.append(E)
    
    print(i)
    print("E: ", E)
    print("delta: ", delta)
    print("fd:           ", fd)
    print("ad E + dovlp: ", full)
    print("analytical:   ", grad)
    # if (np.linalg.norm(grad - fd) > 1e-4):
    #     print("grad and fd diff i: {0}".format(i))
    #     print("{0}".format(grad)) 
    #     print("{0}".format(fd))
    
    exit(1)

    i += 1

if i == max_i:
    print("did not converge")
else:
    print("converged")

print("env:   ", env[a:b])
# print("P:   ", P)
print("E:   ", E)
# print("grad:", grad)

# S = libcscf.int1e(atm, bas, env, 'ovlp')
# print("S:")
# utils.pmat(S)

if SAVE == True:
    from datetime import datetime

    current_time = datetime.now()
    formatted_time = current_time.strftime("%H%M-%d%m%y")

    f1 = open("files/" + formatted_time + "_norm.txt", "w")
    for norm in norm_arr:
        f1.write(str(norm) + "\n")
    f1.close()

    f2 = open("files/" + formatted_time + "_E.txt", "w")
    for EN in E_arr:
        f2.write(str(EN) + "\n")
    f2.close()