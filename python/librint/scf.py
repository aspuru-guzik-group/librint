import ctypes
import numpy as np

from librint import library
from librint import utils

def int1e(mol, typei: str = 'ovlp', coord: str = 'cart') -> np.ndarray:
    atm, bas, env, nelec = utils.prep(mol)

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

    nshells = utils.angl(bas, c)

    R_c = library.int1e_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), c, flag)
    R = np.ctypeslib.as_array(R_c, shape=(nshells, nshells))
    return R


def int2e(mol, coord: str = 'cart') -> np.ndarray:
    atm, bas, env, nelec = utils.prep(mol)

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

    nshells = utils.angl(bas, c)

    R_c = library.int2e_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), c)
    R = np.ctypeslib.as_array(R_c, shape=(nshells, nshells, nshells, nshells))
    return R


def density(mol, imax: int = 200, conv: float = 1e-6) -> np.ndarray:
    atm, bas, env, nelec = utils.prep(mol)

    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))

    nshells = utils.angl(bas, 0)

    P_c = library.density_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), nelec, imax, conv)
    P = np.ctypeslib.as_array(P_c, shape=(nshells, nshells))
    return P


def energy(mol, P: np.ndarray) -> float:
    atm, bas, env, nelec = utils.prep(mol)

    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    P_ctypes = P.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    return library.energy_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), P_ctypes, len(P.flatten()))


def scf(mol, imax: int = 200, conv: float = 1e-6) -> float:
    atm, bas, env, nelec = utils.prep(mol)

    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    return library.scf_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), nelec, imax, conv)