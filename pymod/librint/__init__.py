import os
import ctypes

current_dir = os.path.dirname(os.path.abspath(__file__))

so_path = os.path.join(current_dir, "librint.so")

library = None

try:
    library = ctypes.CDLL(so_path)
    # print(f"Successfully loaded librint.so from {so_path}")
except OSError as e:
    print(f"Error loading librint.so from {so_path}: {e}")

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

# import librint.scf
# import librint.dscf