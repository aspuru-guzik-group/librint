"""
Internal ctypes bindings for the librint shared library.
This module loads the compiled Rust library and declares all C function signatures.
"""
import os
import ctypes
import sys

current_dir = os.path.dirname(os.path.abspath(__file__))

def _dylib_suffix():
    if sys.platform == "darwin":
        return ".dylib"
    if sys.platform.startswith("win"):
        return ".dll"
    return ".so"

# LIBRINT_SO overrides the bundled library (e.g. a fresh target/release/librint.so)
so_path = os.environ.get(
    "LIBRINT_SO", os.path.join(current_dir, "librint" + _dylib_suffix())
)

library = None

try:
    library = ctypes.CDLL(so_path)
except OSError as e:
    print(f"Error loading librint from {so_path}: {e}")

library.int1e_c.argtypes = (
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
    ctypes.c_int,
    ctypes.c_int,
)
library.int1e_c.restype = ctypes.POINTER(ctypes.c_double)

library.int2e_c.argtypes = (
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
    ctypes.c_int,
)
library.int2e_c.restype = ctypes.POINTER(ctypes.c_double)

library.density_c.argtypes = (
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
library.density_c.restype = ctypes.POINTER(ctypes.c_double)

library.energy_c.argtypes =(
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
)
library.energy_c.restype = ctypes.c_double

library.scf_c.argtypes =(
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
library.scf_c.restype = ctypes.c_double

library.grad_c.argtypes =(
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
)
library.grad_c.restype = ctypes.POINTER(ctypes.c_double)

library.dS_u.argtypes =(
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
)
library.dS_u.restype = ctypes.POINTER(ctypes.c_double)

library.dS_c.argtypes =(
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
)
library.dS_c.restype = ctypes.POINTER(ctypes.c_double)

library.dHcore_c.argtypes =(
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
)
library.dHcore_c.restype = ctypes.POINTER(ctypes.c_double)

library.dR_c.argtypes =(
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
)
library.dR_c.restype = ctypes.POINTER(ctypes.c_double)

library.danalytical_c.argtypes =(
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
)
library.danalytical_c.restype = ctypes.POINTER(ctypes.c_double)

library.denergy_c.argtypes =(
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
)
library.denergy_c.restype = ctypes.POINTER(ctypes.c_double)

# Threaded counterparts (src/par.rs). Same arguments as above plus a trailing
# thread count; 0 means rayon's global pool, sized by RAYON_NUM_THREADS.
_PAR_ARGS = (
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_int),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
    ctypes.c_size_t,
)

# Bound defensively: any .so built before src/par.rs existed -- including the
# one committed in this directory -- has none of these symbols, and reaching
# for them eagerly would make `import librint` fail for everyone rather than
# just for the caller who wants a threaded gradient. HAS_PAR lets dscf.py raise
# something a human can act on, and lets the test suite skip instead of error.
HAS_PAR = True
for _name in ("dS_par_c", "dR_par_c", "dHcore_par_c", "danalytical_par_c"):
    try:
        _fn = getattr(library, _name)
    except AttributeError:
        HAS_PAR = False
        break
    _fn.argtypes = _PAR_ARGS
    _fn.restype = ctypes.POINTER(ctypes.c_double)

# Releases any buffer returned by the entry points above; len is the element
# count that call produced. utils.take() copies then calls this.
library.free_c.argtypes = (
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
)
library.free_c.restype = None
