import numpy as np
import pyscf

import utils
import libcscf

SAVE = True

mol = pyscf.gto.M(atom='''
                    O   -0.0000000   -0.1113512    0.0000000
                    H    0.0000000    0.4454047   -0.7830363
                    H   -0.0000000    0.4454047    0.7830363''',
                    basis='sto-3g')

atm = np.asarray(mol._atm, dtype=np.int32, order='C')
bas = np.asarray(mol._bas, dtype=np.int32, order='C')
env = np.asarray(mol._env, dtype=np.double, order='C')

nelec = 10

# mol = pyscf.gto.M(atom='''
#                     H 0 0 -0.4
#                     H 0 0 0.4''',
#                     basis='sto-3g')

# atm = np.asarray(mol._atm, dtype=np.int32, order='C')
# bas = np.asarray(mol._bas, dtype=np.int32, order='C')
# env = np.asarray(mol._env, dtype=np.double, order='C')

# nelec = 2

a, b = utils.split(bas)
# print(a, b)

S = libcscf.int1e(atm, bas, env, 'ovlp')
# print("S: ")
# utils.pmat(S)

alpha = 1e-6
max_i = 50000
epsilon = 1e-5
delta = 1
i = 0

norm_arr = []
E_arr = []

f1 = open("files/h2oE.txt", "w", buffering=1)
f2 = open("files/h2onorm.txt", "w", buffering=1)

while not (delta < epsilon or i == max_i):
    utils.basis_atm(bas, env)
    
    P = libcscf.RHF(atm, bas, env, nelec)
    E = libcscf.energy(atm, bas, env, P)
    denv = libcscf.grad(atm, bas, env, P)
    
    env[a:b] = env[a:b] - alpha * denv #/np.linalg.norm(denv[a:b]))
    delta = np.linalg.norm(denv)

    norm_arr.append(delta)
    E_arr.append(E)

    f1.write(str(E) + "\n")
    f2.write(str(delta) + "\n")

    i += 1

f1.close()
f2.close()

# if i == max_i:
#     print("did not converge")
# else:
#     print("converged")

# print("env:   ", env[a:b])
# # print("P:   ", P)
# print("E:   ", E)
# print("grad:", denv)

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