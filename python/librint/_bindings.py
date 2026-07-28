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
