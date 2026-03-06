"""
Example of library usage. Contains some gradient comparisons.
"""
import numpy as np
import pyscf

from librint import scf, dscf, utils

geometries = {
    'HF': [
        ('H', 1, (0.0,0.0,-1.645509)),
        ('F', 9, (0.0,0.0,0.087291))
    ],
    'CH4': [
        ('C', 6, (0.00000000000000,0.000000000000000,0.000000000000000)),
        ('H', 1, (1.182181057825485, -1.182181057825485,1.182181057825485)),
        ('H', 1, (-1.182181057825485, 1.182181057825485,1.182181057825485)),
        ('H', 1, (1.182181057825485, 1.182181057825485,-1.182181057825485)),
        ('H', 1,(-1.182181057825485, -1.182181057825485,-1.182181057825485))
    ],
    'H2O': [
        ('O', 8, (0.0,0.0,0.091685801102911746)),
        ('H', 1, (1.4229678834888837,0.0,-0.98120954931681137)),
        ('H', 1, (-1.4229678834888837,0.0,-0.98120954931681137))
    ],
    'NH3': [
        ('N', 7, (0.0,0.0,0.127872)),
        ('H', 1, (1.77164,0.0,-0.592238)),
        ('H', 1, (0.88582,1.54288,-0.592238)),
        ('H', 1, (0.88582,-1.54288,-0.592238))
    ],
    'H2': [
        ('H', 1, (0.0,0.0,0.69217920969236535)),
        ('H', 1, (0.0,0.0,-0.69217920969236535))
    ],
    'LIH': [
        ('H', 1, (0.0,0.0,1.4624207)),
        ('Li', 3, (0.0,0.0,0.1))
    ],
    'C6H6': [
    ('C', 6,      ( -1.8771137684,     -1.8237635912,      2.2841118024)),
    ('C', 6,      ( -4.5038328925,     -1.7156712646,      2.2625689261)),
    ('C', 6,      ( -5.7931929345,     -1.5410605833,     -0.0215428763)),
    ('C', 6,      ( -4.4564007702,     -1.4743532559,     -2.2843007750)),
    ('C', 6,      ( -1.8298706186,     -1.5826345550,     -2.2625689261)),
    ('C', 6,      ( -0.5403216040,     -1.7572452364,      0.0215428763)),
    ('H', 1,      ( -0.8625198851,     -1.9613356431,      4.0818081353)),
    ('H', 1,      ( -5.5562212955,     -1.7682056471,      4.0426908074)),
    ('H', 1,      ( -7.8601752208,     -1.4562118864,     -0.0385504102)),
    ('H', 1,      ( -5.4711836260,     -1.3369701765,     -4.0816191627)),
    ('H', 1,      ( -0.7780491334,     -1.5304781177,     -4.0430687526)),
    ('H', 1,      (  1.5264717097,     -1.8422829059,      0.0387393828))
    ],
}

molecule = geometries[geo]

atom = '\n'.join([f"{atom[0]} {0.529*atom[2][0]} {0.529*atom[2][1]} {0.529*atom[2][2]}" for atom in molecule])

mol = pyscf.gto.M(atom=atom, 
                  basis=basis)


# Coordinates: cartesian vs spherical
S_crt = scf.int1e(mol, 'ovlp', 'cart')
S_sph = scf.int1e(mol, 'ovlp', 'sph')

print(S_crt.size)
print(S_sph.size)

atm = np.asarray(mol._atm, dtype=np.int32, order='C')
bas = np.asarray(mol._bas, dtype=np.int32, order='C')
env = np.asarray(mol._env, dtype=np.double, order='C')

# Energy + density calculations
P = density(mol)
E = energy(mol, P)

print("P:")
utils.pmat(P)
print("E:\n", E)

# Matrix integrals
S = int1e(mol, 'ovlp')
T = int1e(mol, 'kin')
V = scf.int1e(mol, 'nuc')

print("S:")
utils.pmat(S)
print("T:")
utils.pmat(T)
print("V:")
utils.pmat(V)

# Gradient Calculations
denv = dscf.grad(mol, P)

dS = dscf.dSf(mol, P)
dH = dscf.dHcoref(mol, P)
dR = dscf.dRf(mol, P)

denergy = dscf.denergyf(mol, P)
danalytical = dscf.danalyticalf(mol, P)

print("Analytical vs autodiff energy (should match)")
print(denergy)
print(danalytical)

np.set_printoptions(precision=5)

print("Autodiff required dS term")
print(denv)
print(dH + dR)

a, b = utils.split(bas)
h = 1e-6

fd = np.zeros(b-a)
for j in range(a, b):
    env[j] -= h
    P1 = scf.density(mol)
    E1 = scf.energy(mol, P1)
    env[j] += 2.0*h
    P2 = scf.density(mol)
    E2 = scf.energy(mol, P2)

    fd[j-a] = (E2 - E1)/(2.0*h)
    env[j] -= h

full = dH + dR - 0.5 * dS

print("Gradient calculation comparison")
print("Finite diff       : ", fd)
print("Analytical python : ", full)
print("Analytical  rust  : ", danalytical)
print("Autodiff energy   : ", denergy)