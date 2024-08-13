import numpy as np
import pyscf

NORMALIZE_GTO = True

def pmat(A: np.ndarray):
    for i in range(len(A)):
        for j in range(len(A[0])):
            if (A[i,j] < 0):
                print("{:f} ".format(A[i,j]), end='')
            else:
                print(" {:f} ".format(A[i,j]), end='')
        print()

def angl(bas: np.ndarray, coord: int) -> int:
    nshells = 0
    for b in bas:
        if coord == 0:
            nshells += int((b[1] + 1) * (b[1] + 2) / 2)
        elif coord == 1:
            nshells += (2*b[1] + 1)
    return nshells

def nparams(atm, bas):
    natm = len(atm)
    nbas = len(bas)
    nshells = angl(bas)
    return (natm, nbas, nshells)

def split(bas: np.ndarray) -> tuple:
    min_c = -1
    max_c = -1

    for b in bas:
        ngto = b[2]
        exp = b[5]
        cont = b[6]

        if min_c == -1:
            min_c = exp
        elif min_c > exp:
            min_c = exp

        if max_c == -1:
            max_c = cont + ngto
        elif max_c < cont:
            max_c = cont + ngto

    return (min_c, max_c)

def norm(basis_add: list) -> np.ndarray:
    _env = []
    for b in basis_add:
        angl = b[0]
        b_coeff = np.array(sorted(list(b[1:]), reverse=True))
        es = b_coeff[:,0]
        cs = b_coeff[:,1:]
        nprim, nctr = cs.shape
        # cs = np.einsum('pi,p->pi', cs, pyscf.gto.gto_norm(angl, es))
        if NORMALIZE_GTO:
            cs = pyscf.gto.mole._nomalize_contracted_ao(angl, es, cs)

        _env.append(es)
        _env.append(cs.T.reshape(-1))

    return np.array(_env).flatten()


def basis_atm(bas: np.ndarray, env: np.ndarray):
    for b_orb in bas:
        s = b_orb[2]
        b1 = b_orb[5]
        b2 = b_orb[6]

        env_t = env[b1:b2+s].reshape(2,s).T.tolist()

        basis_add = [b_orb[1]]
        basis_add.extend(env_t)
        
        new_env = norm([basis_add])
        env[b1:b2+s] = new_env

if __name__ == '__main__':
    basi = ['sto-3g', 'def2-svp']
    mol = pyscf.gto.M(atom='''
                        O   -0.0000000   -0.1113512    0.0000000
                        H    0.0000000    0.4454047   -0.7830363
                        H   -0.0000000    0.4454047    0.7830363''',
                        basis=basi[0])

    atm = np.asarray(mol._atm, dtype=np.int32, order='C')
    bas = np.asarray(mol._bas, dtype=np.int32, order='C')
    env = np.asarray(mol._env, dtype=np.double, order='C')

    s = min(bas[:,5])
    e = len(env)

    # env[s:s+5] += 1e-4

    basis_atm(atm, bas, env)
