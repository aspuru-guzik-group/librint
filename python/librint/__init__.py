from ._bindings import library  # noqa: F401

from . import scf
from . import dscf
from . import utils


def sanity_check():
    """Run built-in sanity checks (H2O/6-31G energy & gradient)."""
    from .test_sanity import test_sanity_hf_energy, test_sanity_hf_gradient
    test_sanity_hf_energy()
    test_sanity_hf_gradient()
    print("All sanity checks passed.")
