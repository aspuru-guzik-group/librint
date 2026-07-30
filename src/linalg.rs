#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]

pub fn matmult(n: usize, A: &[f64], B: &[f64]) -> Vec<f64> {
    // cij = aik bkj
    let mut C = vec![0.0; n * n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                C[i * n + j] += A[i * n + k] * B[k * n + j];
            }
        }
    }
    C
}

pub fn transpose(n: usize, C: &[f64]) -> Vec<f64> {
    let mut Ct = vec![0.0; n * n];
    for i in 0..n {
        for j in 0..n {
            Ct[i * n + j] = C[j * n + i]
        }
    }
    Ct
}

pub fn dcopya(n: usize, A: &[f64]) -> Vec<f64> {
    let mut Ac = vec![0.0; n * n];
    for i in 0..n {
        for j in 0..n {
            Ac[i * n + j] = A[i * n + j];
        }
    }
    Ac
}

pub fn sort(n: usize, eig: &mut [f64], U: &mut [f64]) {
    for i in (0..n).rev() {
        for j in 0..i {
            if eig[j] > eig[j + 1] {
                let tmp = eig[j];
                eig[j] = eig[j + 1];
                eig[j + 1] = tmp;

                for k in 0..n {
                    let tmp = U[k * n + j];
                    U[k * n + j] = U[k * n + j + 1];
                    U[k * n + j + 1] = tmp;
                }
            }
        }
    }
}
