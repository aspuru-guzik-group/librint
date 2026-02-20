import pyscf
from librint import dscf

mol = pyscf.gto.M(atom='''
                    H 0 0 -0.8
                    H 0 0 0.8''',
                    basis='sto-3g')

denv = dscf.dint1e_ovlp(mol, i, j)

print(denv)