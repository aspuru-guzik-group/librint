import ctypes
import numpy as np

from librint import _bindings
from librint import library
from librint import utils

# The 2e analytic gradient (Enzyme reverse) is memory-safe only on the eri.rs
# kernel: cartesian shells, l <= GRAD_LMAX, any contraction, no range
# separation. Reject out-of-domain molecules with a catchable error here so the
# Rust guard's panic never aborts the process.
GRAD_LMAX = 4


def _require_grad_domain(mol):
    lmax = max(int(mol._bas[i, 1]) for i in range(mol.nbas))
    omega = float(mol._env[8]) if len(mol._env) > 8 else 0.0
    if lmax > GRAD_LMAX or omega != 0.0:
        raise ValueError(
            f"librint analytic 2e gradient supports only cartesian shells with "
            f"l <= {GRAD_LMAX} and no range separation (got max l = {lmax}, "
            f"omega = {omega}); the primal integrals (scf.int2e etc.) cover the "
            f"full domain."
        )


def grad(mol, P: np.ndarray) -> np.ndarray:
    _require_grad_domain(mol)
    atm, bas, env, nelec = utils.prep(mol)

    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    P_ctypes = P.ctypes.data_as(ctypes.POINTER(ctypes.c_double))

    s1, s2 = utils.split(bas)
    denv_c = library.grad_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), P_ctypes, len(P.flatten()))
    return utils.take(denv_c, (1, s2 - s1)).flatten()

def dSu(mol) -> np.ndarray:
    atm, bas, env, nelec = utils.prep(mol)

    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    
    nshells = utils.angl(bas, 0)

    s1, s2 = utils.split(bas)

    dS_u = library.dS_u(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()))
    return utils.take(dS_u, (nshells, nshells, s2 - s1)).transpose(2, 0, 1)

def dSf(mol, P: np.ndarray) -> np.ndarray:
    atm, bas, env, nelec = utils.prep(mol)

    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    P_ctypes = P.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    
    s1, s2 = utils.split(bas)

    dS_c = library.dS_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), P_ctypes, len(P.flatten()))
    return utils.take(dS_c, (s2 - s1,))


def dHcoref(mol, P: np.ndarray) -> np.ndarray:
    atm, bas, env, nelec = utils.prep(mol)

    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    P_ctypes = P.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    
    s1, s2 = utils.split(bas)

    dH_c = library.dHcore_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), P_ctypes, len(P.flatten()))
    return utils.take(dH_c, (s2 - s1,))

def dRf(mol, P: np.ndarray) -> np.ndarray:
    _require_grad_domain(mol)
    atm, bas, env, nelec = utils.prep(mol)

    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    P_ctypes = P.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    
    s1, s2 = utils.split(bas)

    dR_c = library.dR_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), P_ctypes, len(P.flatten()))
    return utils.take(dR_c, (s2 - s1,))

def danalyticalf(mol, P: np.ndarray) -> np.ndarray:
    _require_grad_domain(mol)
    atm, bas, env, nelec = utils.prep(mol)

    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    P_ctypes = P.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    
    s1, s2 = utils.split(bas)

    dR_c = library.danalytical_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), P_ctypes, len(P.flatten()))
    return utils.take(dR_c, (s2 - s1,))

# ---------------------------------------------------------------------------
# Threaded entry points (src/par.rs).
#
# These are separate callables, not a flag on the serial ones: danalyticalf
# stays the finite-difference-validated reference path, byte-for-byte, so
# "parallel == serial" remains a statement about two independent things.
# ---------------------------------------------------------------------------

def _par(fn, mol, W: np.ndarray, nthreads: int) -> np.ndarray:
    if not _bindings.HAS_PAR:
        raise RuntimeError(
            "this librint.so has no threaded entry points -- it predates "
            "src/par.rs. Rebuild (cargo build --release) and point LIBRINT_SO "
            "at target/release/librint.so, or use the serial danalyticalf."
        )
    atm, bas, env, nelec = utils.prep(mol)
    W = np.ascontiguousarray(W, dtype=np.float64)
    s1, s2 = utils.split(bas)

    ptr = fn(
        atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int)), atm.size,
        bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int)), bas.size,
        env.ctypes.data_as(ctypes.POINTER(ctypes.c_double)), env.size,
        W.ctypes.data_as(ctypes.POINTER(ctypes.c_double)), W.size,
        int(nthreads),
    )
    return utils.take(ptr, (s2 - s1,))


def dS_par(mol, Q: np.ndarray, nthreads: int = 0) -> np.ndarray:
    """Overlap term seeded with the energy-weighted density Q = P F P.

    Unlike dSf this does NOT build F -- pass Q, not P.
    """
    return _par(library.dS_par_c, mol, Q, nthreads)


def dHcore_par(mol, P: np.ndarray, nthreads: int = 0) -> np.ndarray:
    return _par(library.dHcore_par_c, mol, P, nthreads)


def dR_par(mol, P: np.ndarray, nthreads: int = 0) -> np.ndarray:
    _require_grad_domain(mol)
    return _par(library.dR_par_c, mol, P, nthreads)


def danalytical_par(mol, P: np.ndarray, nthreads: int = 0) -> np.ndarray:
    """Threaded danalyticalf. nthreads=0 uses rayon's global pool, which reads
    RAYON_NUM_THREADS; any other value builds a pool of exactly that size."""
    _require_grad_domain(mol)
    return _par(library.danalytical_par_c, mol, P, nthreads)


def denergyf(mol, P: np.ndarray) -> np.ndarray:
    _require_grad_domain(mol)
    atm, bas, env, nelec = utils.prep(mol)

    atm_ctypes = atm.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    bas_ctypes = bas.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
    env_ctypes = env.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    P_ctypes = P.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
    
    s1, s2 = utils.split(bas)

    dR_c = library.denergy_c(atm_ctypes, len(atm.flatten()), bas_ctypes, len(bas.flatten()), env_ctypes, len(env.flatten()), P_ctypes, len(P.flatten()))
    return utils.take(dR_c, (s2 - s1,))