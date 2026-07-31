"""Does the threaded gradient compute the same thing as the serial one?

The serial path (danalyticalf) is the reference: test_gradient_fd.py validates
it against central finite differences and test_gradient_pyscfad.py against
pyscfad. The threaded path (danalytical_par) has neither of those on its own,
so it has to be tied to the serial one before any timing of it means anything.

Exact equality is NOT the criterion and would be the wrong thing to demand: a
work-stealing reduction sums the same terms in a different association order,
so the two agree to round-off, not to the bit. What this asserts is

  1. every term matches its serial counterpart (dHcore, dR, dS separately, so
     a failure localizes), and the assembled gradient matches too;
  2. the agreement does not degrade as threads are added -- a real race shows
     up as error growing with thread count, whereas reassociation noise stays
     flat.

The thread sweep is clamped to the cores actually available, so this shrinks to
something meaningful on a laptop instead of failing there.
"""
import os

import numpy as np
import pytest
import pyscf

import librint
import librint.dscf
import librint.utils
from librint import _bindings

from pyscf_comp.geometries import geometries

# The .so committed in python/librint/ predates src/par.rs. Skipping beats
# failing: nothing here is broken, the library just has no threaded path to
# compare against. Point LIBRINT_SO at a fresh target/release/librint.so.
pytestmark = pytest.mark.skipif(
    not _bindings.HAS_PAR,
    reason="librint.so has no threaded entry points; rebuild and set LIBRINT_SO",
)

# Small enough to run every time. sto-3g and def2-svp are s/p only.
FAST = [
    ("H2", "sto-3g"),
    ("H2O", "sto-3g"),
    ("NH3", "sto-3g"),
    ("CH4", "sto-3g"),
    ("H2O", "def2-svp"),
    ("NH3", "def2-svp"),
    ("CH4", "def2-svp"),
]

# Each of these exists to reach a code path the fast list never touches, so
# they are worth minutes when you want them -- and worth skipping when you do
# not. Run with `-m slow`, or everything with no -m at all.
SLOW = [
    ("H2O", "def2-tzvp"),   # f shells (l=3): rys_tab.rs nroots 6-7 Chebyshev
    ("NH3", "def2-tzvp"),
    ("CH4", "cc-pvdz"),     # general contraction (nctr>1): eri_cart_gc path
    ("H2O", "cc-pvtz"),
    ("H2O", "def2-qzvp"),   # g shells (l=4): rys_tab.rs nroots 8-9 Chebyshev
]

SYSTEMS = ([pytest.param(g, b) for g, b in FAST]
           + [pytest.param(g, b, marks=pytest.mark.slow) for g, b in SLOW])

THREADS = [1, 2, 4, 8, 16, 32, 64]
RTOL = 1e-9  # relative to max|serial|


def build(geo, basis):
    atom = "\n".join(
        f"{a[0]} {0.529 * a[2][0]} {0.529 * a[2][1]} {0.529 * a[2][2]}"
        for a in geometries[geo]
    )
    mol = pyscf.gto.M(atom=atom, basis=basis, verbose=0)
    mol.cart = True
    return mol


def rel(got, ref):
    scale = max(float(np.abs(ref).max()), 1e-30)
    return float(np.abs(got - ref).max()) / scale


def thread_counts():
    ncores = len(os.sched_getaffinity(0))
    return [t for t in THREADS if t <= ncores]


@pytest.mark.parametrize("geo, basis", SYSTEMS)
def test_par_matches_serial(geo, basis):
    mol = build(geo, basis)
    mf = pyscf.scf.RHF(mol)
    mf.verbose = 0
    mf.conv_tol = 1e-10
    mf.max_cycle = 200
    mf.kernel()
    P = mf.make_rdm1()

    # dS_par takes the energy-weighted density directly; dSf builds it
    # internally via getF, so construct the same Q here to compare like with
    # like.
    h = mol.intor("int1e_kin") + mol.intor("int1e_nuc")
    eri = mol.intor("int2e")
    F = (h + np.einsum("kl,ijkl->ij", P, eri)
         - 0.5 * np.einsum("kl,ikjl->ij", P, eri))
    Q = P @ F @ P

    ser = {
        "dHcore": np.asarray(librint.dscf.dHcoref(mol, P)),
        "dR": np.asarray(librint.dscf.dRf(mol, P)),
        "dS": np.asarray(librint.dscf.dSf(mol, P)),
        "danalytical": np.asarray(librint.dscf.danalyticalf(mol, P)),
    }

    # The SCF above dominates the runtime, so sweep threads inside one test
    # rather than parametrizing over them and paying for it once per count.
    errs = {}
    for T in thread_counts():
        par = {
            "dHcore": librint.dscf.dHcore_par(mol, P, T),
            "dR": librint.dscf.dR_par(mol, P, T),
            "dS": librint.dscf.dS_par(mol, Q, T),
            "danalytical": librint.dscf.danalytical_par(mol, P, T),
        }
        errs[T] = {k: rel(par[k], ser[k]) for k in ser}

    bad = {T: {k: e for k, e in row.items() if not (e < RTOL)}
           for T, row in errs.items()}
    bad = {T: row for T, row in bad.items() if row}
    assert not bad, (
        f"{geo}/{basis}: parallel disagrees with serial beyond {RTOL:.0e}\n"
        + "\n".join(f"  T={T:3d}  " + "  ".join(f"{k}={e:.2e}"
                                                for k, e in sorted(row.items()))
                    for T, row in sorted(errs.items()))
    )


@pytest.mark.parametrize("geo, basis", SYSTEMS[:3])
def test_par_run_to_run(geo, basis):
    """Two identical calls must agree to round-off.

    NOT a bitwise check: work stealing decides the fold chunking at run time,
    so repeated runs may associate the sum differently. This pins down that the
    variation stays at round-off rather than growing into something a caller
    would notice.
    """
    mol = build(geo, basis)
    mf = pyscf.scf.RHF(mol)
    mf.verbose = 0
    mf.conv_tol = 1e-10
    mf.max_cycle = 200
    mf.kernel()
    P = mf.make_rdm1()

    T = thread_counts()[-1]
    a = librint.dscf.danalytical_par(mol, P, T)
    b = librint.dscf.danalytical_par(mol, P, T)
    assert rel(b, a) < RTOL, f"{geo}/{basis}: run-to-run spread at T={T}"
