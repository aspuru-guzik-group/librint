"""Elementwise (not sorted) librint-vs-pyscfad gradient check.

bench_fair.py and test_gradient_pyscfad.py compare np.sort(g) on both sides,
which is permutation-blind: it cannot tell whether gradient component k belongs
to the parameter librint thinks it does. This script builds the explicit
env-index map instead (pyscfad's setup_exp/setup_ctr_coeff both return
`env_of`, the mol._env index of every traced parameter) and compares
per-parameter.

Also reports:
  * whether the traced parameter set covers librint's env slice [s1, s2)
  * the primal energy on both sides
  * the permutation distance between the two vectors (how much sorting hid)

Usage: .venv/bin/python check_elementwise_jax.py
"""
import os

os.environ.setdefault("OMP_NUM_THREADS", "1")
os.environ.setdefault("OPENBLAS_NUM_THREADS", "1")
os.environ.setdefault("MKL_NUM_THREADS", "1")
os.environ.setdefault("PYTHONHASHSEED", "0")

import numpy as np
import pyscf
import jax
import jax.numpy as jnp

jax.config.update("jax_platform_name", "cpu")
jax.config.update("jax_enable_x64", True)

from pyscfad import gto as adgto
from pyscfad.gto._mole_helper import setup_exp, setup_ctr_coeff

import librint
import librint.dscf
import librint.utils

from geometries import geometries

SYSTEMS = [
    ("H2", "sto-3g"),
    ("H2O", "sto-3g"),
    ("NH3", "sto-3g"),
    ("CH4", "sto-3g"),
    ("H2O", "def2-svp"),
    ("CH4", "cc-pvdz"),   # general contraction: nctr > 1
]


def atom_string(geo):
    return "\n".join(
        f"{a[0]} {0.529 * a[2][0]} {0.529 * a[2][1]} {0.529 * a[2][2]}"
        for a in geometries[geo]
    )


def build_pyscf(geo, basis):
    mol = pyscf.gto.M(atom=atom_string(geo), basis=basis, verbose=0)
    mol.cart = True
    return mol


def build_ad(geo, basis):
    mol = adgto.Mole()
    mol.atom = atom_string(geo)
    mol.unit = "Angstrom"
    mol.basis = basis
    mol.charge = 0
    mol.verbose = 0
    mol.cart = True
    mol.build(trace_coords=False, trace_exp=True, trace_ctr_coeff=True)
    return mol


def jax_frozen_grad(mol_ad, P, Q):
    P, Q = jnp.asarray(P), jnp.asarray(Q)

    def e_frozen(m):
        h = m.intor("int1e_kin") + m.intor("int1e_nuc")
        eri = m.intor("int2e")
        S = m.intor("int1e_ovlp")
        e1 = jnp.einsum("ij,ji->", P, h)
        J = jnp.einsum("kl,ijkl->ij", P, eri)
        K = jnp.einsum("kl,ikjl->ij", P, eri)
        e2 = 0.5 * jnp.einsum("ij,ji->", P, J) - 0.25 * jnp.einsum("ij,ji->", P, K)
        pulay = -0.5 * jnp.einsum("ij,ji->", Q, S)
        return e1 + e2 + pulay

    E, grad = jax.value_and_grad(e_frozen)(mol_ad)
    return float(E), np.asarray(grad.exp).ravel(), np.asarray(grad.ctr_coeff).ravel()


def main():
    print(f"{'system':16s} {'np':>4s} {'cover':>7s} {'max|el|':>10s} {'max|sorted|':>12s} "
          f"{'nmis':>5s} {'perm':>5s} {'E_lib-E_jax':>12s}")
    bad = 0
    for geo, basis in SYSTEMS:
        mol = build_pyscf(geo, basis)
        mf = pyscf.scf.RHF(mol)
        mf.verbose = 0
        mf.conv_tol = 1e-12
        mf.max_cycle = 500
        mf.kernel()
        P = mf.make_rdm1()

        # frozen energy-weighted density, exactly the bench's convention
        h = mol.intor("int1e_kin") + mol.intor("int1e_nuc")
        eri = mol.intor("int2e")
        F = (h + np.einsum("kl,ijkl->ij", P, eri)
             - 0.5 * np.einsum("kl,ikjl->ij", P, eri))
        Q = P @ F @ P

        s1, s2 = librint.utils.split(mol._bas)
        g_lib = np.asarray(librint.dscf.danalyticalf(mol, P), dtype=float).copy()

        mol_ad = build_ad(geo, basis)
        same_env = (mol_ad._env.shape == mol._env.shape
                    and np.allclose(mol_ad._env, mol._env, rtol=0, atol=0))
        E_jax, g_exp, g_ctr = jax_frozen_grad(mol_ad, P, Q)

        _, _, env_exp = setup_exp(mol_ad)
        _, _, env_ctr = setup_ctr_coeff(mol_ad)

        n = s2 - s1
        g_map = np.full(n, np.nan)
        for gv, env_of in ((g_exp, env_exp), (g_ctr, env_ctr)):
            for val, j in zip(gv, env_of):
                if s1 <= j < s2:
                    g_map[j - s1] = val
        covered = int(np.count_nonzero(~np.isnan(g_map)))
        outside = int(np.count_nonzero((env_exp < s1) | (env_exp >= s2))) + \
                  int(np.count_nonzero((env_ctr < s1) | (env_ctr >= s2)))

        fin = ~np.isnan(g_map)
        el = np.abs(g_lib[fin] - g_map[fin])
        scale = max(np.abs(g_lib).max(), 1e-30)
        max_el = float(el.max()) if el.size else float("nan")
        nmis = int(np.count_nonzero(el > 1e-6 * scale))

        g_jax_full = np.concatenate([g_exp, g_ctr])
        if g_jax_full.size == n:
            max_sorted = float(np.abs(np.sort(g_lib) - np.sort(g_jax_full)).max())
            perm = int(np.count_nonzero(np.argsort(np.argsort(g_lib))
                                        != np.argsort(np.argsort(g_jax_full))))
        else:
            max_sorted, perm = float("nan"), -1

        # primal energies: librint's energy includes Enuc, jax's e_frozen has no
        # Enuc and carries the -1/2 tr(QS) pulay term -> undo both to compare
        E_lib = librint.scf.energy(mol, P)
        S = mol.intor("int1e_ovlp")
        E_jax_tot = E_jax + 0.5 * float(np.einsum("ij,ji->", Q, S)) + mol.energy_nuc()
        dE = E_lib - E_jax_tot

        ok = (nmis == 0 and covered == n and outside == 0 and same_env)
        flag = "" if ok else "  <-- CHECK"
        if not ok:
            bad += 1
        print(f"{geo + '/' + basis:16s} {n:>4d} {covered:>3d}/{n:<3d} {max_el:10.2e} "
              f"{max_sorted:12.2e} {nmis:>5d} {perm:>5d} {dE:12.2e}{flag}")
        if not same_env:
            print("      NOTE: pyscfad _env differs from pyscf _env (layout mismatch)")
        if outside:
            print(f"      NOTE: {outside} traced parameter(s) fall outside librint's "
                  f"env slice [{s1},{s2})")
        if covered != n:
            missing = np.where(np.isnan(g_map))[0] + s1
            print(f"      NOTE: env indices with no traced counterpart: {missing.tolist()}")
        if nmis:
            idx = np.where(el > 1e-6 * scale)[0]
            for k in idx[:8]:
                print(f"      env[{s1 + k}]  librint={g_lib[fin][k]: .8e}  "
                      f"jax={g_map[fin][k]: .8e}")
    raise SystemExit(1 if bad else 0)


if __name__ == "__main__":
    main()
