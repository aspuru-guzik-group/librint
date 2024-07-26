import librint
import ctypes
import numpy as np

def int1e(natm: int, nbas: int, nshells: int, atm: np.ndarray, bas: np.ndarray, env: np.ndarray, typei: str = 'ovlp', coord: str = 'cart') -> np.ndarray:
    atm_r = atm.flatten().tolist()
    bas_r = bas.flatten().tolist()
    env_r = env.flatten().tolist()

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

    
    R_c = librint.integral1e(natm, nbas, nshells, atm_r, bas_r, env_r, c, flag)
    R = np.array(R_c).reshape(nshells, nshells)
    return R

def int2e(natm: int, nbas: int, nshells: int, atm: np.ndarray, bas: np.ndarray, env: np.ndarray, coord: str = 'cart') -> np.ndarray:
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
    
    R_c = librint.integral2e(natm, nbas, nshells, atm_r, bas_r, env_r, c)
    R = np.array(R_c).reshape(nshells, nshells)
    return R

def RHF(natm: int, nbas: int, nelec: int, nshells: int, atm: np.ndarray, bas: np.ndarray, env: np.ndarray, imax: int = 200, conv: float = 1e-6) -> np.ndarray:   
    atm_r = atm.flatten().tolist()
    bas_r = bas.flatten().tolist()
    env_r = env.flatten().tolist()

    P_c = librint.RHFp(natm, nbas, nelec, nshells, atm_r, bas_r, env_r, imax, conv)
    P = np.array(P_c).reshape((nshells, nshells))
    return P

def energy(natm: int, nbas: int, nshells: int, atm: np.ndarray, bas: np.ndarray, env: np.ndarray, P: np.ndarray) -> float:
    atm_r = atm.flatten().tolist()
    bas_r = bas.flatten().tolist()
    env_r = env.flatten().tolist()
    P_r = P.flatten().tolist()
    return librint.energyp(natm, nbas, nshells, atm_r, bas_r, env_r, P_r)

def grad(natm: int, nbas: int, nshells: int, atm: np.ndarray, bas: np.ndarray, env: np.ndarray, P: np.ndarray) -> np.ndarray:
    atm_r = atm.flatten().tolist()
    bas_r = bas.flatten().tolist()
    env_r = env.flatten().tolist()
    denv_r = [0.0 for i in range(len(env_r))]

    P_r = P.flatten().tolist()
    librint.grad(natm, nbas, nshells, atm_r, bas_r, env_r, denv_r, P_r, 1.0)

    return denv_r


if __name__ == '__main__':
    print("Hello world")