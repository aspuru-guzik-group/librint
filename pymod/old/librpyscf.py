import numpy as np
import pyscf

import utils

import libcscf

def prep(mol):
    atm = np.asarray(mol._atm, dtype=np.int32, order='C')
    bas = np.asarray(mol._bas, dtype=np.int32, order='C')
    env = np.asarray(mol._env, dtype=np.double, order='C')

    nelec = mol.nelec[0] + mol.nelec[1]

    return atm, bas, env, nelec

def density(mol, imax=100):
    atm, bas, env, nelec = prep(mol)

    return libcscf.density(atm, bas, env, nelec, imax=imax)

def danalyticalf(mol, P):
    atm, bas, env, nelec = prep(mol)

    return libcscf.danalyticalf(atm, bas, env, P)

def denergyf(mol, P):
    atm, bas, env, nelec = prep(mol)
    
    return libcscf.denergyf(atm, bas, env, P)

def grad(mol, P):
    atm, bas, env, nelec = prep(mol)

    grad_value = libcscf.grad(atm, bas, env, P)

    return grad_value