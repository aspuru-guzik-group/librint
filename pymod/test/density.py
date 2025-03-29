import numpy as np
import pyscf

from librint import scf

mol = pyscf.gto.M(atom='''
                    H 0 0 -0.8
                    H 0 0 0.8''',
                    basis='sto-3g')

P = scf.density(mol)

print(P)