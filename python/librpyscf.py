import numpy as np
import pyscf

COMP = 'C'

import utils

if COMP == 'R':
    import librscf as libscf
elif COMP == 'C':
    import libcscf as libscf

def prep(mol):
    atm = np.asarray(mol._atm, dtype=np.int32, order='C')
    bas = np.asarray(mol._bas, dtype=np.int32, order='C')
    env = np.asarray(mol._env, dtype=np.double, order='C')

    nelec = mol.nelec[0]

    return atm, bas, env, nelec

def density(mol, nelec, imax=100):
    atm, bas, env, nelec = prep(mol)

    return libscf.density(atm, bas, env, nelec, imax=imax)

def danalyticalf(mol, P):
    atm, bas, env, nelec = prep(mol)

    return libscf.danalyticalf(atm, bas, env, P)

def denergyf(mol, P):
    atm, bas, env, nelec = prep(mol)
    
    return libscf.denergyf(atm, bas, env, P)

def grad(mol, P):
    atm, bas, env, nelec = prep(mol)

    grad_value = libscf.grad(atm, bas, env, P)

    return grad_value