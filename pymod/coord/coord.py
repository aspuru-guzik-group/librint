from librint import scf
import pyscf

# mol = pyscf.gto.M(atom='''
#                     H 0 0 -0.8
#                     H 0 0 0.8''',
#                     basis='sto-3g')

mol = pyscf.gto.M(atom='''
                        O   -0.0000000   -0.1113512    0.0000000
                        H    0.0000000    0.4454047   -0.7830363
                        H   -0.0000000    0.4454047    0.7830363''',
                        basis='def2-svp')

S = scf.int1e(mol, 'ovlp', 'cart')

print(S.size)

S = scf.int1e(mol, 'ovlp', 'sph')

print(S.size)
