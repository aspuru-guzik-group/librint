import ctypes
import numpy as np

from librint import library
from librint import utils

library.dSo_c.argtypes =(
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
)
library.dSo_c.restype = ctypes.POINTER(ctypes.c_double)

library.dHcoreo_c.argtypes =(
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
)
library.dHcoreo_c.restype = ctypes.POINTER(ctypes.c_double)

def dSof(mol, P: np.ndarray) -> np.ndarray:
    atm, bas, env, nelec = utils.prep(mol)

    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    P_ctypes = P.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    
    s1, s2 = utils.split(bas)

    dS_c = library.dSo_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), P_ctypes, len(P.flatten()))
    dS = np.ctypeslib.as_array(dS_c, shape=(s2-s1, ))
    return dS

def dHcoreof(mol, P: np.ndarray) -> np.ndarray:
    atm, bas, env, nelec = utils.prep(mol)

    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    P_ctypes = P.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    
    s1, s2 = utils.split(bas)

    dH_c = library.dHcoreo_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), P_ctypes, len(P.flatten()))
    dH = np.ctypeslib.as_array(dH_c, shape=(s2-s1, ))
    return dH