import numpy as np
import timeit

import pyscf
from pyscf import gto

from librint import scf
from librint import dscf
from librint import test

# mol = pyscf.gto.M(atom='''
#                     H 0 0 -0.8
#                     H 0 0 0.8''',
#                     basis='sto-3g')



##### H2 #####

# mol = gto.Mole()
# mol.atom = '''
# H0 0 0 -0.8
# H1 0 0 0.8
# '''
# mol.basis = {
#     'H0': gto.basis.load('sto-3g', 'H'),
#     'H1': gto.basis.load('sto-3g', 'H'),
# }
# mol.build()

##### H2O #####

# mol = gto.Mole()
# mol.atom = '''
# O0 -0.0000000 -0.1113512  0.0000000
# H0  0.0000000  0.4454047 -0.7830363
# H1 -0.0000000  0.4454047  0.7830363
# '''
# mol.basis = {
#     'O0': gto.basis.load('sto-3g', 'O'),
#     'H0': gto.basis.load('sto-3g', 'H'),
#     'H1': gto.basis.load('sto-3g', 'H'),
# }
# mol.build()

#### 

mol = gto.Mole()
mol.atom = '''
C0 -1.8771137684 -1.8237635912  2.2841118024
C1 -4.5038328925 -1.7156712646  2.2625689261
C2 -5.7931929345 -1.5410605833 -0.0215428763
C3 -4.4564007702 -1.4743532559 -2.2843007750
C4 -1.8298706186 -1.5826345550 -2.2625689261
C5 -0.5403216040 -1.7572452364  0.0215428763
H0 -0.8625198851 -1.9613356431  4.0818081353
H1 -5.5562212955 -1.7682056471  4.0426908074
H2 -7.8601752208 -1.4562118864 -0.0385504102
H3 -5.4711836260 -1.3369701765 -4.0816191627
H4 -0.7780491334 -1.5304781177 -4.0430687526
H5  1.5264717097 -1.8422829059  0.0387393828
'''
mol.basis = {
    'C0': gto.basis.load('sto-3g', 'C'),
    'C1': gto.basis.load('sto-3g', 'C'),
    'C2': gto.basis.load('sto-3g', 'C'),
    'C3': gto.basis.load('sto-3g', 'C'),
    'C4': gto.basis.load('sto-3g', 'C'),
    'C5': gto.basis.load('sto-3g', 'C'),
    'H0': gto.basis.load('sto-3g', 'H'),
    'H1': gto.basis.load('sto-3g', 'H'),
    'H2': gto.basis.load('sto-3g', 'H'),
    'H3': gto.basis.load('sto-3g', 'H'),
    'H4': gto.basis.load('sto-3g', 'H'),
    'H5': gto.basis.load('sto-3g', 'H'),
}
mol.unit="Bohr"
mol.build()

print("mol built")

P = scf.density(mol, imax=4000)
# print(P)

# exit()

# print(mol._env)
# print(mol._bas)
# print(mol._env)

dS = dscf.dSf(mol, P)
print(dS)

dSo = test.dSof(mol, P)
print(dSo)

print(dS - dSo)

exit()

# dH = dscf.dHcoref(mol, P)
# print(dH)

# dHo = test.dHcoreof(mol, P)
# print(dHo)

# import gc

def dS_time(n_runs=1):
    # gc.disable()
    time_dS  = timeit.timeit(lambda: dscf.dSf(mol, P), number=n_runs)
    time_dSo = timeit.timeit(lambda: test.dSof(mol, P), number=n_runs)
    # time_dH  = timeit.timeit(lambda: dscf.dHcoref(mol, P), number=n_runs)
    # time_dHo = timeit.timeit(lambda: test.dHcoreof(mol, P), number=n_runs)
    # gc.enable()
    return (time_dS / n_runs, time_dSo / n_runs) #, time_dH / n_runs, time_dHo / n_runs)

t0, t1 = dS_time(n_runs=10)
print(f"dS ratio:  {t1 / t0:.4f}")
# print(f"dH ratio:  {t3 / t2:.4f}")


# dS

#   env1   env2   env3   env4   env5
# [______|______|______|______|______]
#          roi1          roi2


# dR
# env = split

#   env1   env2   env3   env4   env5   env6   env7   env8   env9
# [______|______|______|______|______|______|______|______|______]
#          roi1          roi2          roi3          roi4
