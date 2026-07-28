"""Integral-only primal vs reverse on the pyscfad/jax side. Times the 2e
integral TENSOR primal against its VJP w.r.t. basis parameters (exp +
ctr_coeff), unit seed, 1 core.
"""
import os
import time

os.environ.setdefault("OMP_NUM_THREADS", "1")
os.environ.setdefault("OPENBLAS_NUM_THREADS", "1")
os.environ.setdefault("MKL_NUM_THREADS", "1")
os.environ.setdefault("XLA_FLAGS", "--xla_cpu_multi_thread_eigen=false intra_op_parallelism_threads=1")
try:
    os.sched_setaffinity(0, {sorted(os.sched_getaffinity(0))[0]})
except OSError:
    pass

import numpy as np
import jax
jax.config.update("jax_enable_x64", True)
import jax.numpy as jnp
import pyscf
from pyscfad import gto as gtoad

from geometries import geometries

SYSTEMS = [
    ("H2", "sto-3g"),
    ("H2O", "sto-3g"),
    ("NH3", "sto-3g"),
    ("CH4", "sto-3g"),
    ("H2O", "def2-svp"),
    ("NH3", "def2-svp"),
    ("CH4", "def2-svp"),
]


def atom_str(geo):
    molecule = geometries[geo]
    return "\n".join(
        f"{a[0]} {0.529 * a[2][0]} {0.529 * a[2][1]} {0.529 * a[2][2]}"
        for a in molecule
    )


def build_ref(geo, basis):
    mol = pyscf.gto.M(atom=atom_str(geo), basis=basis, verbose=0)
    mol.cart = True
    return mol


def build_ad(geo, basis):
    mol = gtoad.Mole()
    mol.atom = atom_str(geo)
    mol.unit = "Angstrom"
    mol.basis = basis
    mol.charge = 0
    mol.verbose = 0
    mol.cart = True
    mol.build(trace_coords=False, trace_exp=True, trace_ctr_coeff=True)
    return mol


def block(x):
    for leaf in jax.tree_util.tree_leaves(x):
        try:
            leaf.block_until_ready()
        except AttributeError:
            pass
    return x


def med(f, n=5):
    ts = []
    for _ in range(n):
        t0 = time.perf_counter()
        f()
        ts.append(time.perf_counter() - t0)
    return sorted(ts)[n // 2]


def main():
    print(f"{'system':16s} {'libcint':>8s} {'ad-fwd':>8s} {'vjp-apply':>9s} "
          f"{'fwd+vjp':>8s} {'(f+v)/fwd':>9s} {'(f+v)/cint':>10s}")
    for geo, basis in SYSTEMS:
        mol_ref = build_ref(geo, basis)
        t_cint = med(lambda: mol_ref.intor("int2e"))

        mol = build_ad(geo, basis)

        def f(m):
            return m.intor("int2e")

        out, vjp_fn = jax.vjp(f, mol)
        seed = jnp.ones_like(out)
        block(vjp_fn(seed))  # warm

        t_fwd = med(lambda: block(f(mol)))
        t_apply = med(lambda: block(vjp_fn(seed)))
        t_total = med(lambda: block(jax.vjp(f, mol)[1](seed)))

        print(f"{geo + '/' + basis:16s} {t_cint:8.4f} {t_fwd:8.4f} {t_apply:9.4f} "
              f"{t_total:8.4f} {t_total / t_fwd:9.2f} {t_total / t_cint:10.2f}",
              flush=True)


if __name__ == "__main__":
    main()
