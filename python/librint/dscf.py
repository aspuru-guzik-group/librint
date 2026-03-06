import ctypes
import numpy as np

from librint import library
from librint import utils

def grad(mol, P: np.ndarray) -> np.ndarray:
    atm, bas, env, nelec = utils.prep(mol)

    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    P_ctypes = P.ctypes.data_as(ctypes.POINTER(ctypes.c_double))

    s1, s2 = utils.split(bas)
    denv_c = library.grad_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), P_ctypes, len(P.flatten()))
    denv = np.ctypeslib.as_array(denv_c, shape=(1, s2-s1))
    return denv.flatten()

def dSu(mol) -> np.ndarray:
    atm, bas, env, nelec = utils.prep(mol)

    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    
    nshells = utils.angl(bas, 0)

    s1, s2 = utils.split(bas)

    dS_u = library.dS_u(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()))
    dS = np.ctypeslib.as_array(dS_u, shape=(nshells, nshells, s2-s1))
    return dS.transpose(2, 0, 1)

def dSf(mol, P: np.ndarray) -> np.ndarray:
    atm, bas, env, nelec = utils.prep(mol)

    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    P_ctypes = P.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    
    s1, s2 = utils.split(bas)

    dS_c = library.dS_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), P_ctypes, len(P.flatten()))
    dS = np.ctypeslib.as_array(dS_c, shape=(s2-s1, ))
    return dS


def dHcoref(mol, P: np.ndarray) -> np.ndarray:
    atm, bas, env, nelec = utils.prep(mol)

    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    P_ctypes = P.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    
    s1, s2 = utils.split(bas)

    dH_c = library.dHcore_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), P_ctypes, len(P.flatten()))
    dH = np.ctypeslib.as_array(dH_c, shape=(s2-s1, ))
    return dH # return dH.reshape(2, 2, 6).transpose(2, 0, 1)

def dRf(mol, P: np.ndarray) -> np.ndarray:
    atm, bas, env, nelec = utils.prep(mol)

    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    P_ctypes = P.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    
    s1, s2 = utils.split(bas)

    dR_c = library.dR_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), P_ctypes, len(P.flatten()))
    dR = np.ctypeslib.as_array(dR_c, shape=(s2-s1, ))
    return dR # .reshape(2, 2, 2, 2, 6).transpose(4, 0, 1, 2, 3) #(4, 3, 2, 0, 1)

def danalyticalf(mol, P: np.ndarray) -> np.ndarray:
    atm, bas, env, nelec = utils.prep(mol)

    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    P_ctypes = P.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    
    s1, s2 = utils.split(bas)

    dR_c = library.danalytical_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), P_ctypes, len(P.flatten()))
    dR = np.ctypeslib.as_array(dR_c, shape=(s2-s1, ))
    return dR

def denergyf(mol, P: np.ndarray) -> np.ndarray:
    atm, bas, env, nelec = utils.prep(mol)

    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    P_ctypes = P.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    
    s1, s2 = utils.split(bas)

    dR_c = library.denergy_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), P_ctypes, len(P.flatten()))
    dR = np.ctypeslib.as_array(dR_c, shape=(s2-s1, ))
    return dR