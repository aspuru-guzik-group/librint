#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]

use crate::cint1e::{cint1e_nuc_cart, cint1e_ovlp_cart};
use crate::cint_bas::CINTcgto_cart;
use crate::intor1::cint1e_kin_cart;
use std::autodiff::*;

use crate::linalg::matmult;
use crate::scf::{angl, integral1e, integral2e_fock, nmol};
use crate::utils::{combine, split};

#[autodiff_reverse(dovlp, Duplicated, Const, Const, Const, Const, Duplicated)]
pub fn ovlp(
    out: &mut [f64],
    shls: &mut [i32],
    atm: &mut [i32],
    bas: &mut [i32],
    env1: &mut [f64],
    env2: &mut [f64],
) {
    let (natm, nbas) = nmol(atm, bas);
    let mut env: Vec<f64> = combine(env1, env2);
    cint1e_ovlp_cart(
        out,
        shls,
        atm,
        natm as i32,
        bas,
        nbas as i32,
        &mut env,
        std::ptr::null_mut(),
    );
}

#[autodiff_reverse(dkin, Duplicated, Const, Const, Const, Const, Duplicated)]
pub fn kin(
    out: &mut [f64],
    shls: &mut [i32],
    atm: &mut [i32],
    bas: &mut [i32],
    env1: &mut [f64],
    env2: &mut [f64],
) {
    let (natm, nbas) = nmol(atm, bas);
    let mut env: Vec<f64> = combine(env1, env2);
    cint1e_kin_cart(
        out,
        shls,
        atm,
        natm as i32,
        bas,
        nbas as i32,
        &mut env,
        std::ptr::null_mut(),
    );
}

#[autodiff_reverse(dnuc, Duplicated, Const, Const, Const, Const, Duplicated)]
pub fn nuc(
    out: &mut [f64],
    shls: &mut [i32],
    atm: &mut [i32],
    bas: &mut [i32],
    env1: &mut [f64],
    env2: &mut [f64],
) {
    let (natm, nbas) = nmol(atm, bas);
    let mut env: Vec<f64> = combine(env1, env2);
    cint1e_nuc_cart(
        out,
        shls,
        atm,
        natm as i32,
        bas,
        nbas as i32,
        &mut env,
        std::ptr::null_mut(),
    );
}

// 2e integral block on the AD-friendly rys kernel (src/eri.rs) -- the memory-
// safe path for the Enzyme reverse (dtwo_ad): one shell quartet per reverse.
#[autodiff_reverse(dtwo_ad, Duplicated, Const, Const, Const, Const, Duplicated)]
pub fn two_ad(
    out: &mut [f64],
    shls: &mut [i32],
    atm: &mut [i32],
    bas: &mut [i32],
    env1: &mut [f64],
    env2: &mut [f64],
) {
    let env: Vec<f64> = combine(env1, env2);
    crate::eri::eri_cart(out, shls, atm, bas, &env);
}

pub fn dS_uncontracted(atm: &mut [i32], bas: &mut [i32], env: &mut [f64]) -> Vec<f64> {
    let (_, nbas) = nmol(atm, bas);
    let nshells = angl(bas, 0);

    let (s1, s2) = split(bas);

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let mut dS = vec![0.0; nshells * nshells * env2.len()];

    let mut buf;
    let mut dbuf;
    let mut denv;
    let mut shls = vec![0; 4];

    let mut mu;
    let mut nu;

    mu = 0;
    for i in 0..nbas {
        shls[0] = i as i32;
        let di = CINTcgto_cart(i, bas) as usize;
        nu = 0;
        for j in 0..nbas {
            shls[1] = j as i32;
            let dj = CINTcgto_cart(j, bas) as usize;

            buf = vec![0.0; di * dj];
            dbuf = vec![0.0; di * dj];

            let mut c: usize = 0;
            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    dbuf[c] = 1.0;

                    denv = vec![0.0; env2.len()];
                    dovlp(
                        &mut buf, &mut dbuf, &mut shls, atm, bas, &mut env1, &mut env2, &mut denv,
                    );
                    for l in 0..env2.len() {
                        dS[(nuj * nshells + mui) * env2.len() + l] = denv[l];
                        // dS[l * nshells * nshells + nuj * nshells + mui] = denv[l];
                    }

                    dbuf[c] = 0.0;
                    c += 1;
                }
            }
            nu += dj;
        }
        mu += di;
    }

    dS
}

fn dSf(
    atm: &mut [i32],
    bas: &mut [i32],
    env1: &mut [f64],
    env2: &mut [f64],
    Q: &[f64],
) -> Vec<f64> {
    let (_, nbas) = nmol(atm, bas);
    let nshells = angl(bas, 0);

    let mut dS = vec![0.0; env2.len()];

    let mut buf;
    let mut dbuf;
    let mut denv;
    let mut shls = vec![0; 4];

    let mut mu;
    let mut nu;

    mu = 0;
    for i in 0..nbas {
        shls[0] = i as i32;
        let di = CINTcgto_cart(i, bas) as usize;
        nu = 0;
        for j in 0..nbas {
            shls[1] = j as i32;
            let dj = CINTcgto_cart(j, bas) as usize;

            buf = vec![0.0; di * dj];
            dbuf = vec![0.0; di * dj];

            // batched adjoint seed with Q-weights (see dTf)
            let mut c: usize = 0;
            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    dbuf[c] = Q[nuj * nshells + mui];
                    c += 1;
                }
            }
            denv = vec![0.0; env2.len()];
            dovlp(
                &mut buf, &mut dbuf, &mut shls, atm, bas, env1, env2, &mut denv,
            );
            for l in 0..env2.len() {
                dS[l] += denv[l];
            }
            nu += dj;
        }
        mu += di;
    }

    dS
}

fn dTf(
    atm: &mut [i32],
    bas: &mut [i32],
    env1: &mut [f64],
    env2: &mut [f64],
    P: &[f64],
) -> Vec<f64> {
    let (_, nbas) = nmol(atm, bas);
    let nshells = angl(bas, 0);

    let mut dT = vec![0.0; env2.len()];

    let mut buf;
    let mut dbuf;
    let mut denv;
    let mut shls = vec![0; 4];

    let mut mu;
    let mut nu;

    mu = 0;
    for i in 0..nbas {
        shls[0] = i as i32;
        let di = CINTcgto_cart(i, bas) as usize;
        nu = 0;
        for j in 0..nbas {
            shls[1] = j as i32;
            let dj = CINTcgto_cart(j, bas) as usize;

            buf = vec![0.0; di * dj];
            dbuf = vec![0.0; di * dj];

            // batched adjoint: seed the whole shell-pair block with its
            // P-weights and run ONE reverse pass; by linearity denv is the
            // already-contracted sum_c P_c dT_c/denv
            let mut c: usize = 0;
            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    dbuf[c] = P[nuj * nshells + mui];
                    c += 1;
                }
            }
            denv = vec![0.0; env2.len()];
            dkin(
                &mut buf, &mut dbuf, &mut shls, atm, bas, env1, env2, &mut denv,
            );
            for l in 0..env2.len() {
                dT[l] += denv[l];
            }
            nu += dj;
        }
        mu += di;
    }

    dT
}

fn dVf(
    atm: &mut [i32],
    bas: &mut [i32],
    env1: &mut [f64],
    env2: &mut [f64],
    P: &[f64],
) -> Vec<f64> {
    let (_, nbas) = nmol(atm, bas);
    let nshells = angl(bas, 0);

    let mut dV = vec![0.0; env2.len()];

    let mut buf;
    let mut dbuf;
    let mut denv;
    let mut shls = vec![0; 4];

    let mut mu;
    let mut nu;

    mu = 0;
    for i in 0..nbas {
        shls[0] = i as i32;
        let di = CINTcgto_cart(i, bas) as usize;
        nu = 0;
        for j in 0..nbas {
            shls[1] = j as i32;
            let dj = CINTcgto_cart(j, bas) as usize;

            buf = vec![0.0; di * dj];
            dbuf = vec![0.0; di * dj];

            // batched adjoint seed (see dTf)
            let mut c: usize = 0;
            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    dbuf[c] = P[nuj * nshells + mui];
                    c += 1;
                }
            }
            denv = vec![0.0; env2.len()];
            dnuc(
                &mut buf, &mut dbuf, &mut shls, atm, bas, env1, env2, &mut denv,
            );
            for l in 0..env2.len() {
                dV[l] += denv[l];
            }
            nu += dj;
        }
        mu += di;
    }

    dV
}

pub fn dHcoreg(
    atm: &mut [i32],
    bas: &mut [i32],
    env: &mut [f64],
    P: &[f64],
) -> Vec<f64> {
    let (s1, s2) = split(bas);

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let mut dHcore = vec![0.0; env2.len()];

    let dT = dTf(atm, bas, &mut env1, &mut env2, P);
    let dV = dVf(atm, bas, &mut env1, &mut env2, P);

    for i in 0..env2.len() {
        dHcore[i] = dT[i] + dV[i];
    }

    dHcore
}

pub fn getF(atm: &mut [i32], bas: &mut [i32], env: &mut [f64], P: &[f64]) -> Vec<f64> {
    // F = H + G, with G the 2e Fock part accumulated directly in O(n^2) by
    // integral2e_fock. The frozen-P gradient needs F only to form Q=PFP, and
    // building the full n^4 ERI tensor for that was the old memory wall.
    let T = integral1e(atm, bas, env, 0, 1);
    let V = integral1e(atm, bas, env, 0, 2);
    let G = integral2e_fock(atm, bas, env, P, 0);

    let mut F = vec![0.0; T.len()];
    for i in 0..F.len() {
        F[i] = T[i] + V[i] + G[i];
    }

    F
}

pub fn dSg(atm: &mut [i32], bas: &mut [i32], env: &mut [f64], P: &[f64]) -> Vec<f64> {
    let nshells = angl(bas, 0);

    let F = getF(atm, bas, env, P);

    let (s1, s2) = split(bas);

    let pf = matmult(nshells, P, &F);
    let Q = matmult(nshells, &pf, P); // PFP

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let dS = dSf(atm, bas, &mut env1, &mut env2, &Q);

    dS
}

pub fn dRf(
    atm: &mut [i32],
    bas: &mut [i32],
    env1: &mut [f64],
    env2: &mut [f64],
    P: &[f64],
) -> Vec<f64> {
    // 8-fold permutational symmetry: only canonical quartets (i>=j, k>=l,
    // (i,j)>=(k,l)) are differentiated; each element seed sums the energy-
    // expression weight over the quartet's permutation images, so by adjoint
    // linearity this equals the full nbas^4 loop with ~8x fewer reverse passes.
    // Seeds stay OUTSIDE the reverse -- one cint call per reverse is the only
    // shape Enzyme compiles correctly.
    let (_, nbas) = nmol(atm, bas);
    let nshells = angl(bas, 0);

    let mut dR = vec![0.0; env2.len()];

    let mut offs = vec![0usize; nbas + 1];
    for s in 0..nbas {
        offs[s + 1] = offs[s] + CINTcgto_cart(s, bas) as usize;
    }

    let w = |a: usize, b: usize, c: usize, d: usize| -> f64 {
        0.5 * (P[a * nshells + b] * P[c * nshells + d]
            - 0.5 * P[a * nshells + c] * P[b * nshells + d])
    };

    let mut shls = vec![0; 4];
    let mut buf;
    let mut dbuf;
    let mut denv;

    for i in 0..nbas {
        let di = offs[i + 1] - offs[i];
        let mu = offs[i];
        shls[0] = i as i32;
        for j in 0..=i {
            let dj = offs[j + 1] - offs[j];
            let nu = offs[j];
            shls[1] = j as i32;
            for k in 0..=i {
                let dk = offs[k + 1] - offs[k];
                let sig = offs[k];
                shls[2] = k as i32;
                let lmax = if k == i { j } else { k };
                for l in 0..=lmax {
                    let dl = offs[l + 1] - offs[l];
                    let lam = offs[l];
                    shls[3] = l as i32;

                    // the 8 permutation images of (i, j, k, l); keep only the
                    // first occurrence of each distinct ordered tuple so
                    // degenerate quartets (i==j, k==l, ij==kl) are not
                    // double-counted
                    let imgs = [
                        (i, j, k, l),
                        (j, i, k, l),
                        (i, j, l, k),
                        (j, i, l, k),
                        (k, l, i, j),
                        (l, k, i, j),
                        (k, l, j, i),
                        (l, k, j, i),
                    ];
                    let mut keep = [true; 8];
                    for a in 1..8 {
                        for b in 0..a {
                            if imgs[a] == imgs[b] {
                                keep[a] = false;
                                break;
                            }
                        }
                    }

                    buf = vec![0.0; di * dj * dk * dl];
                    dbuf = vec![0.0; di * dj * dk * dl];

                    let mut c: usize = 0;
                    for laml in lam..(lam + dl) {
                        for sigk in sig..(sig + dk) {
                            for nuj in nu..(nu + dj) {
                                for mui in mu..(mu + di) {
                                    let mut ws = 0.0;
                                    if keep[0] {
                                        ws += w(mui, nuj, sigk, laml);
                                    }
                                    if keep[1] {
                                        ws += w(nuj, mui, sigk, laml);
                                    }
                                    if keep[2] {
                                        ws += w(mui, nuj, laml, sigk);
                                    }
                                    if keep[3] {
                                        ws += w(nuj, mui, laml, sigk);
                                    }
                                    if keep[4] {
                                        ws += w(sigk, laml, mui, nuj);
                                    }
                                    if keep[5] {
                                        ws += w(laml, sigk, mui, nuj);
                                    }
                                    if keep[6] {
                                        ws += w(sigk, laml, nuj, mui);
                                    }
                                    if keep[7] {
                                        ws += w(laml, sigk, nuj, mui);
                                    }
                                    dbuf[c] = ws;
                                    c += 1;
                                }
                            }
                        }
                    }
                    denv = vec![0.0; env2.len()];
                    // The only memory-safe 2e reverse is the eri.rs rys kernel
                    // (cartesian, l <= LMAX, any nctr, omega == 0). Outside that
                    // domain the c2rust reverse corrupts memory, so fail loud
                    // rather than smash the heap (primal keeps the full domain).
                    let lmax_sh = (bas[8 * i + 1])
                        .max(bas[8 * j + 1])
                        .max(bas[8 * k + 1])
                        .max(bas[8 * l + 1]);
                    if lmax_sh as usize > crate::eri::LMAX || env1[8] != 0.0 {
                        panic!(
                            "2e gradient unsupported for this quartet (max l = {}, \
                             omega = {}): the memory-safe reverse (eri.rs) covers \
                             only cartesian l <= {}, no range separation.",
                            lmax_sh,
                            env1[8],
                            crate::eri::LMAX,
                        );
                    }
                    dtwo_ad(
                        &mut buf, &mut dbuf, &mut shls, atm, bas, env1, env2, &mut denv,
                    );
                    for t in 0..env2.len() {
                        dR[t] += denv[t];
                    }
                }
            }
        }
    }

    dR
}

pub fn dRg(atm: &mut [i32], bas: &mut [i32], env: &mut [f64], P: &[f64]) -> Vec<f64> {
    let (s1, s2) = split(bas);

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let dR = dRf(atm, bas, &mut env1, &mut env2, P);
    dR
}

pub fn danalyticalg(
    atm: &mut [i32],
    bas: &mut [i32],
    env: &mut [f64],
    P: &[f64],
) -> Vec<f64> {
    let dH = dHcoreg(atm, bas, env, P);
    let dR = dRg(atm, bas, env, P);
    let dS = dSg(atm, bas, env, P);

    let mut dtotal = vec![0.0; dH.len()];
    for i in 0..dtotal.len() {
        dtotal[i] = dH[i] + dR[i] - 0.5 * dS[i];
    }

    dtotal
}

// The fused whole-energy Enzyme reverses (of `energywrap` and `energyf`) used
// to live here. They were miscompiled on nightly-2026-05-13 -- wrong gradients
// for any multi-shell system -- so every caller was moved to the part-wise
// batched adjoints below, and the fused primals were left behind with no
// callers while Enzyme kept differentiating them on every build. They are gone
// now; `python -c "librint.dscf.denergyf"` is denergy_c -> denergyfast, an
// assembled path, and shares nothing but a name with the old reverse.

pub fn gradenergy(
    atm: &mut [i32],
    bas: &mut [i32],
    env: &mut [f64],
    P: &[f64],
) -> Vec<f64> {
    // 1/2 tr(P(H+F)) = tr(PH) + 1/2 tr(PGP), assembled from the batched
    // part-wise adjoints (each of which IS an Enzyme reverse).
    let dH = dHcoreg(atm, bas, env, P);
    let dR = dRg(atm, bas, env, P);
    let mut denv = vec![0.0; dH.len()];
    for i in 0..denv.len() {
        denv[i] = dH[i] + dR[i];
    }
    denv
}

pub fn denergyfast(
    atm: &mut [i32],
    bas: &mut [i32],
    env: &mut [f64],
    P: &[f64],
) -> Vec<f64> {
    // Kept as a separate C entry point (denergy_c) for compatibility only: it
    // had inlined the same dH + dR - 0.5 dS expression as danalyticalg, in the
    // same order, so the two returned bitwise-identical values. Delegate rather
    // than maintain the duplicate.
    danalyticalg(atm, bas, env, P)
}
