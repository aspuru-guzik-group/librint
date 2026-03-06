import pyscf

from librint import dscf

mol = pyscf.gto.M(atom='''
                    H 0 0 -0.4
                    H 0 0 0.4''',
                    basis='sto-3g')


dSu = dscf.dSu(mol)

print(dSu)