import os
import ctypes
import numpy as np

import utils

path = '/u/jpmedina/libcint/librint/python/librint.so'

libc = ctypes.CDLL(path)

libc.int1e_c.argtypes = (
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
    ctypes.c_int,
    ctypes.c_int,
)
libc.int1e_c.restype = ctypes.POINTER(ctypes.c_double)

libc.int2e_c.argtypes = (
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
    ctypes.c_int,
)
libc.int2e_c.restype = ctypes.POINTER(ctypes.c_double)

libc.RHF_c.argtypes = (
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
    ctypes.c_size_t,
    ctypes.c_int,
    ctypes.c_double,
)
libc.RHF_c.restype = ctypes.POINTER(ctypes.c_double)

libc.energy_c.argtypes =(
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
)
libc.energy_c.restype = ctypes.c_double

libc.grad_c.argtypes =(
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
)
libc.grad_c.restype = ctypes.POINTER(ctypes.c_double)

libc.dS_c.argtypes =(
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
)
libc.dS_c.restype = ctypes.POINTER(ctypes.c_double)

libc.dHcore_c.argtypes =(
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
)
libc.dHcore_c.restype = ctypes.POINTER(ctypes.c_double)

libc.dR_c.argtypes =(
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
)
libc.dR_c.restype = ctypes.POINTER(ctypes.c_double)

libc.danalytical_c.argtypes =(
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
)
libc.danalytical_c.restype = ctypes.POINTER(ctypes.c_double)

libc.denergy_c.argtypes =(
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
)
libc.denergy_c.restype = ctypes.POINTER(ctypes.c_double)


def int1e(atm: np.ndarray, bas: np.ndarray, env: np.ndarray, typei: str = 'ovlp', coord: str = 'cart') -> np.ndarray:
    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))

    if (typei == 'ovlp'):
        flag = 0
    elif (typei == 'kin'):
        flag = 1
    elif (typei == 'nuc'):
        flag = 2
    else:
        print("integral type does not exist: ovlp, kin, nuc")
        return None
    
    if (coord == 'cart'):
        c = 0
    elif (coord == 'sph'):
        c = 1
    else:
        print("coordinate type does not exist: cart, sph")
        return None

    nshells = utils.angl(bas)

    R_c = libc.int1e_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), c, flag)
    R = np.ctypeslib.as_array(R_c, shape=(nshells, nshells))
    return R


def int2e(atm: np.ndarray, bas: np.ndarray, env: np.ndarray, coord: str = 'cart') -> np.ndarray:
    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))

    if (coord == 'cart'):
        c = 0
    elif (coord == 'sph'):
        c = 1
    else:
        print("coordinate type does not exist: cart, sph")
        return None

    nshells = utils.angl(bas)

    R_c = libc.int2e_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), c)
    R = np.ctypeslib.as_array(R_c, shape=(nshells, nshells, nshells, nshells))
    return R


def RHF(atm: np.ndarray, bas: np.ndarray, env: np.ndarray, nelec: int, imax: int = 200, conv: float = 1e-6) -> np.ndarray:    
    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))

    nshells = utils.angl(bas)

    P_c = libc.RHF_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), nelec, imax, conv)
    P = np.ctypeslib.as_array(P_c, shape=(nshells, nshells))
    return P


def energy(atm: np.ndarray, bas: np.ndarray, env: np.ndarray, P: np.ndarray) -> float:
    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    P_ctypes = P.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    return libc.energy_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), P_ctypes, len(P.flatten()))


def grad(atm: np.ndarray, bas: np.ndarray, env: np.ndarray, P: np.ndarray) -> np.ndarray:    
    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    P_ctypes = P.ctypes.data_as(ctypes.POINTER(ctypes.c_double))

    s1, s2 = utils.split(bas)
    denv_c = libc.grad_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), P_ctypes, len(P.flatten()))
    denv = np.ctypeslib.as_array(denv_c, shape=(1, s2-s1))
    return denv.flatten()


def dSf(atm: np.ndarray, bas: np.ndarray, env: np.ndarray, P: np.ndarray) -> np.ndarray:    
    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    P_ctypes = P.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    
    s1, s2 = utils.split(bas)

    dS_c = libc.dS_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), P_ctypes, len(P.flatten()))
    dS = np.ctypeslib.as_array(dS_c, shape=(s2-s1, ))
    return dS


def dHcoref(atm: np.ndarray, bas: np.ndarray, env: np.ndarray, P: np.ndarray) -> np.ndarray:    
    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    P_ctypes = P.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    
    s1, s2 = utils.split(bas)
    nshells = utils.angl(bas)

    dH_c = libc.dHcore_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), P_ctypes, len(P.flatten()))
    dH = np.ctypeslib.as_array(dH_c, shape=(s2-s1, ))
    return dH # return dH.reshape(2, 2, 6).transpose(2, 0, 1)

def dRf(atm: np.ndarray, bas: np.ndarray, env: np.ndarray, P: np.ndarray) -> np.ndarray:    
    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    P_ctypes = P.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    
    s1, s2 = utils.split(bas)
    nshells = utils.angl(bas)

    dR_c = libc.dR_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), P_ctypes, len(P.flatten()))
    dR = np.ctypeslib.as_array(dR_c, shape=(s2-s1, ))
    return dR # .reshape(2, 2, 2, 2, 6).transpose(4, 0, 1, 2, 3) #(4, 3, 2, 0, 1)

def danalyticalf(atm: np.ndarray, bas: np.ndarray, env: np.ndarray, P: np.ndarray) -> np.ndarray:    
    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    P_ctypes = P.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    
    s1, s2 = utils.split(bas)
    nshells = utils.angl(bas)

    dR_c = libc.danalytical_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), P_ctypes, len(P.flatten()))
    dR = np.ctypeslib.as_array(dR_c, shape=(s2-s1, ))
    return dR

def denergyf(atm: np.ndarray, bas: np.ndarray, env: np.ndarray, P: np.ndarray) -> np.ndarray:    
    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    P_ctypes = P.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    
    s1, s2 = utils.split(bas)
    nshells = utils.angl(bas)

    dR_c = libc.denergy_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), P_ctypes, len(P.flatten()))
    dR = np.ctypeslib.as_array(dR_c, shape=(s2-s1, ))
    return dR

if __name__ == '__main__':
    print("Hello world")