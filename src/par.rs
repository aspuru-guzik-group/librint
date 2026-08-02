#![allow(non_snake_case)]
//! Shared-memory parallel gradient accumulation: rayon work-stealing over
//! shell PAIRS.
//!
//! Two design points, both measured rather than assumed (see
//! python/pyscf_comp/bench_par_loops.py):
//!
//! 1. **Granularity: pairs, not the bra index.** Splitting the 2e loop over `i`
//!    alone leaves one shell holding 7-11% of the total work, so makespan >=
//!    that item and no scheduler helps beyond ~9-14 threads -- block, stride
//!    and dynamic all collapse to the same efficiency there. Over `(i, j)`
//!    pairs the largest item is 0.4-1.4%, lifting the ceiling to ~70-250.
//!
//! 2. **Dynamic, not static.** At pair granularity on C4H10/def2-svp, T=64:
//!    block 0.40, stride 0.71, work-stealing 1.00. A static stride is only
//!    worth using where work stealing is unavailable (e.g. across MPI ranks).
//!
//! Per task the closure carries its own scratch and its own clones of
//! atm/bas/env: `#[autodiff_reverse]` demands `&mut` on arguments the loop only
//! reads, and those arrays are kilobytes, so cloning is far cheaper than
//! reworking every signature in the call chain. rayon's `fold` runs the init
//! closure once per work chunk rather than once per pair, so the clones are
//! amortized. The reduction is a sum of `env2.len()` doubles (24-84).
//!
//! ## Pool ownership
//!
//! The `*_in` functions do no pool management -- they run on whatever pool the
//! caller is already inside. The public wrappers add `in_pool`, and
//! `danalytical_par` builds ONE pool for all four loops rather than one each.
//! `nthreads = 0` skips the build entirely and uses rayon's global pool, which
//! honours `RAYON_NUM_THREADS`; that is the preferred path for repeated calls.

use rayon::prelude::*;

use crate::cint::CINTOpt;
use crate::cint2e::{cint2e_cart, cint2e_cart_optimizer};
use crate::cint_bas::CINTcgto_cart;
use crate::dscf::{dkin, dnuc, dovlp, dtwo_ad};
use crate::linalg::matmult;
use crate::optimizer::CINTdel_optimizer;
use crate::p2c::leak_vec;
use crate::scf::{angl, integral1e, nmol};
use crate::utils::split;

/// Per-task state: the mutable copies the autodiff wrappers require, plus the
/// running adjoint accumulator.
struct Task {
    atm: Vec<i32>,
    bas: Vec<i32>,
    env1: Vec<f64>,
    env2: Vec<f64>,
    shls: Vec<i32>,
    acc: Vec<f64>,
}

fn task_init(atm: &[i32], bas: &[i32], env1: &[f64], env2: &[f64]) -> Task {
    Task {
        atm: atm.to_vec(),
        bas: bas.to_vec(),
        env1: env1.to_vec(),
        env2: env2.to_vec(),
        shls: vec![0i32; 4],
        acc: vec![0.0; env2.len()],
    }
}

fn shell_offsets(nbas: usize, bas: &[i32]) -> Vec<usize> {
    let mut offs = vec![0usize; nbas + 1];
    for s in 0..nbas {
        offs[s + 1] = offs[s] + CINTcgto_cart(s, bas) as usize;
    }
    offs
}

fn in_pool<R: Send>(nthreads: usize, f: impl FnOnce() -> R + Send) -> R {
    if nthreads == 0 {
        return f(); // rayon's global pool, sized by RAYON_NUM_THREADS
    }
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(nthreads)
        .build()
        .expect("rayon pool");
    pool.install(f)
}

fn add_into(acc: &mut [f64], x: &[f64]) {
    for t in 0..acc.len() {
        acc[t] += x[t];
    }
}

/// The shape shared by every 1e term: one Enzyme reverse per shell pair over
/// the full `nbas x nbas` loop (no permutational reduction), each seeded with
/// the caller's weight matrix `W`.
///
/// `rev` is one of the generated reverses `dovlp` / `dkin` / `dnuc`, which have
/// identical signatures. It is taken as a generic rather than a `fn` pointer so
/// each instantiation monomorphizes to a direct call -- an indirect call into
/// an Enzyme-generated body is exactly the kind of thing that type-checks and
/// then misbehaves under fat LTO.
fn pair_1e_in<F>(
    atm: &[i32],
    bas: &[i32],
    env1: &[f64],
    env2: &[f64],
    W: &[f64],
    rev: F,
) -> Vec<f64>
where
    F: Fn(
            &mut [f64],
            &mut [f64],
            &mut [i32],
            &mut [i32],
            &mut [i32],
            &mut [f64],
            &mut [f64],
            &mut [f64],
        ) + Sync
        + Send,
{
    let (_, nbas) = nmol(atm, bas);
    let nshells = angl(bas, 0);
    let nparam = env2.len();
    let offs = shell_offsets(nbas, bas);

    let pairs: Vec<(usize, usize)> =
        (0..nbas).flat_map(|i| (0..nbas).map(move |j| (i, j))).collect();

    pairs
        .par_iter()
        .fold(
            || task_init(atm, bas, env1, env2),
            |mut st, &(i, j)| {
                let (di, dj) = (offs[i + 1] - offs[i], offs[j + 1] - offs[j]);
                st.shls[0] = i as i32;
                st.shls[1] = j as i32;

                let mut buf = vec![0.0; di * dj];
                let mut dbuf = vec![0.0; di * dj];
                let mut c: usize = 0;
                for nuj in offs[j]..offs[j + 1] {
                    for mui in offs[i]..offs[i + 1] {
                        dbuf[c] = W[nuj * nshells + mui];
                        c += 1;
                    }
                }

                let mut denv = vec![0.0; nparam];
                rev(
                    &mut buf, &mut dbuf, &mut st.shls, &mut st.atm, &mut st.bas,
                    &mut st.env1, &mut st.env2, &mut denv,
                );
                add_into(&mut st.acc, &denv);
                st
            },
        )
        .map(|st| st.acc)
        .reduce(
            || vec![0.0; nparam],
            |mut a, b| {
                add_into(&mut a, &b);
                a
            },
        )
}

/// Threaded `dscf::dSf`: the overlap term, seeded with the energy-weighted
/// density `Q = P F P`. Note this is `dSf`, not `dSg` -- it does NOT build `F`.
pub fn dS_par(
    atm: &[i32],
    bas: &[i32],
    env1: &[f64],
    env2: &[f64],
    Q: &[f64],
    nthreads: usize,
) -> Vec<f64> {
    in_pool(nthreads, || pair_1e_in(atm, bas, env1, env2, Q, dovlp))
}

fn dHcore_par_in(
    atm: &[i32],
    bas: &[i32],
    env1: &[f64],
    env2: &[f64],
    P: &[f64],
) -> Vec<f64> {
    // Same term order as dscf::dHcoreg: dT then dV, summed afterwards.
    let mut dH = pair_1e_in(atm, bas, env1, env2, P, dkin);
    let dV = pair_1e_in(atm, bas, env1, env2, P, dnuc);
    add_into(&mut dH, &dV);
    dH
}

/// Threaded `dscf::dHcoreg`.
pub fn dHcore_par(
    atm: &[i32],
    bas: &[i32],
    env1: &[f64],
    env2: &[f64],
    P: &[f64],
    nthreads: usize,
) -> Vec<f64> {
    in_pool(nthreads, || dHcore_par_in(atm, bas, env1, env2, P))
}

fn dR_par_in(
    atm: &[i32],
    bas: &[i32],
    env1: &[f64],
    env2: &[f64],
    P: &[f64],
) -> Vec<f64> {
    let (_, nbas) = nmol(atm, bas);
    let nshells = angl(bas, 0);
    let nparam = env2.len();
    let offs = shell_offsets(nbas, bas);

    let w = |a: usize, b: usize, c: usize, d: usize| -> f64 {
        0.5 * (P[a * nshells + b] * P[c * nshells + d]
            - 0.5 * P[a * nshells + c] * P[b * nshells + d])
    };

    // canonical pairs i >= j. Largest-first: rayon splits the range from the
    // front, and cost grows with i, so handing out the expensive pairs early
    // leaves the cheap ones as filler for whoever finishes first.
    let mut pairs: Vec<(usize, usize)> =
        (0..nbas).flat_map(|i| (0..=i).map(move |j| (i, j))).collect();
    pairs.reverse();

    pairs
        .par_iter()
        .fold(
            || task_init(atm, bas, env1, env2),
            |mut st, &(i, j)| {
                let (di, dj) = (offs[i + 1] - offs[i], offs[j + 1] - offs[j]);
                let (mu, nu) = (offs[i], offs[j]);
                st.shls[0] = i as i32;
                st.shls[1] = j as i32;

                for k in 0..=i {
                    let dk = offs[k + 1] - offs[k];
                    let sig = offs[k];
                    st.shls[2] = k as i32;
                    let lmax = if k == i { j } else { k };
                    for l in 0..=lmax {
                        let dl = offs[l + 1] - offs[l];
                        let lam = offs[l];
                        st.shls[3] = l as i32;

                        let imgs = [
                            (i, j, k, l), (j, i, k, l), (i, j, l, k), (j, i, l, k),
                            (k, l, i, j), (l, k, i, j), (k, l, j, i), (l, k, j, i),
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

                        let mut buf = vec![0.0; di * dj * dk * dl];
                        let mut dbuf = vec![0.0; di * dj * dk * dl];
                        let mut c: usize = 0;
                        for laml in lam..(lam + dl) {
                            for sigk in sig..(sig + dk) {
                                for nuj in nu..(nu + dj) {
                                    for mui in mu..(mu + di) {
                                        let mut ws = 0.0;
                                        if keep[0] { ws += w(mui, nuj, sigk, laml); }
                                        if keep[1] { ws += w(nuj, mui, sigk, laml); }
                                        if keep[2] { ws += w(mui, nuj, laml, sigk); }
                                        if keep[3] { ws += w(nuj, mui, laml, sigk); }
                                        if keep[4] { ws += w(sigk, laml, mui, nuj); }
                                        if keep[5] { ws += w(laml, sigk, mui, nuj); }
                                        if keep[6] { ws += w(sigk, laml, nuj, mui); }
                                        if keep[7] { ws += w(laml, sigk, nuj, mui); }
                                        dbuf[c] = ws;
                                        c += 1;
                                    }
                                }
                            }
                        }

                        let lmax_sh = (st.bas[8 * i + 1])
                            .max(st.bas[8 * j + 1])
                            .max(st.bas[8 * k + 1])
                            .max(st.bas[8 * l + 1]);
                        if lmax_sh as usize > crate::eri::LMAX || st.env1[8] != 0.0 {
                            panic!(
                                "2e gradient unsupported for this quartet (max l = \
                                 {}, omega = {}): the memory-safe reverse (eri.rs) \
                                 covers only cartesian l <= {}, no range separation.",
                                lmax_sh, st.env1[8], crate::eri::LMAX,
                            );
                        }

                        let mut denv = vec![0.0; nparam];
                        dtwo_ad(
                            &mut buf, &mut dbuf, &mut st.shls, &mut st.atm,
                            &mut st.bas, &mut st.env1, &mut st.env2, &mut denv,
                        );
                        add_into(&mut st.acc, &denv);
                    }
                }
                st
            },
        )
        .map(|st| st.acc)
        .reduce(
            || vec![0.0; nparam],
            |mut a, b| {
                add_into(&mut a, &b);
                a
            },
        )
}

/// Threaded `dscf::dRf`: the 2e basis-parameter gradient. Keeps the serial
/// 8-fold permutational reduction exactly -- only canonical quartets are
/// differentiated, and each element seed sums the energy weight over the
/// quartet's distinct permutation images. Work is distributed over the
/// canonical `(i, j)` pairs; the `k`/`l` bounds depend only on `i` and `j`, so a
/// pair is a self-contained unit.
pub fn dR_par(
    atm: &[i32],
    bas: &[i32],
    env1: &[f64],
    env2: &[f64],
    P: &[f64],
    nthreads: usize,
) -> Vec<f64> {
    in_pool(nthreads, || dR_par_in(atm, bas, env1, env2, P))
}

/// libcint's shell-quartet optimizer, shared read-only across worker threads.
///
/// `CINTOpt` is built once and then only read while integrals are evaluated:
/// every `(*opt).` access in the cint2e evaluation path loads into a local,
/// none store back. This is the same contract that lets pyscf hand a single
/// `CINTOpt` to every OpenMP thread. Per-task optimizers would be the
/// conservative alternative, but its tables are O(nbas^2) and would be rebuilt
/// once per work chunk, which at 64 threads costs more memory than the whole
/// rest of the gradient.
struct SharedOpt(*mut CINTOpt);
unsafe impl Send for SharedOpt {}
unsafe impl Sync for SharedOpt {}

impl SharedOpt {
    /// Read the pointer through a method, not the field. Closures capture
    /// disjoint fields, so `shared.0` inside one would capture the bare
    /// `*mut CINTOpt` -- which is not `Sync` -- and lose this wrapper entirely.
    fn get(&self) -> *mut CINTOpt {
        self.0
    }
}

/// Per-task state for the primal Fock build.
struct FockTask {
    atm: Vec<i32>,
    bas: Vec<i32>,
    env: Vec<f64>,
    shls: Vec<i32>,
    G: Vec<f64>,
}

/// Threaded `scf::integral2e_fock` (cartesian only, which is what `getF` uses).
///
/// Each task accumulates into its own `G`: the Coulomb term writes
/// `G[mui, nuj]`, which is disjoint across `j`, but the exchange term writes
/// `G[mui, laml]` with `l` running over every shell, so two `(i, j)` tasks
/// sharing an `i` collide. Private accumulators plus a sum reduction avoid
/// that without atomics or locking; the cost is one `nao^2` buffer per work
/// chunk.
fn fock2e_par_in(
    atm: &[i32],
    bas: &[i32],
    env: &[f64],
    P: &[f64],
) -> Vec<f64> {
    let (natm, nbas) = nmol(atm, bas);
    let n = angl(bas, 0);
    let offs = shell_offsets(nbas, bas);

    // The optimizer's tables may reference these, so they outlive the loop.
    let mut atm_o = atm.to_vec();
    let mut bas_o = bas.to_vec();
    let mut env_o = env.to_vec();
    let mut opt: *mut CINTOpt = std::ptr::null_mut();
    unsafe {
        cint2e_cart_optimizer(
            &mut opt, atm_o.as_mut_ptr(), natm as i32,
            bas_o.as_mut_ptr(), nbas as i32, env_o.as_mut_ptr(),
        );
    }
    let shared = SharedOpt(opt);

    // Full nbas^2 pair list (no permutational reduction here, matching the
    // serial build), heaviest pairs first so the tail is cheap filler.
    let mut pairs: Vec<(usize, usize)> =
        (0..nbas).flat_map(|i| (0..nbas).map(move |j| (i, j))).collect();
    pairs.sort_by_key(|&(i, j)| {
        std::cmp::Reverse((offs[i + 1] - offs[i]) * (offs[j + 1] - offs[j]))
    });

    let G = pairs
        .par_iter()
        .fold(
            || FockTask {
                atm: atm.to_vec(),
                bas: bas.to_vec(),
                env: env.to_vec(),
                shls: vec![0i32; 4],
                G: vec![0.0; n * n],
            },
            |mut st, &(i, j)| {
                let (di, dj) = (offs[i + 1] - offs[i], offs[j + 1] - offs[j]);
                let (mu, nu) = (offs[i], offs[j]);
                st.shls[0] = i as i32;
                st.shls[1] = j as i32;

                for k in 0..nbas {
                    let dk = offs[k + 1] - offs[k];
                    let sig = offs[k];
                    st.shls[2] = k as i32;
                    for l in 0..nbas {
                        let dl = offs[l + 1] - offs[l];
                        let lam = offs[l];
                        st.shls[3] = l as i32;

                        let mut buf = vec![0.0; di * dj * dk * dl];
                        cint2e_cart(
                            &mut buf, &mut st.shls, &mut st.atm, natm as i32,
                            &mut st.bas, nbas as i32, &mut st.env, shared.get(),
                        );

                        let mut c: usize = 0;
                        for laml in lam..(lam + dl) {
                            for sigk in sig..(sig + dk) {
                                for nuj in nu..(nu + dj) {
                                    for mui in mu..(mu + di) {
                                        let v = buf[c];
                                        c += 1;
                                        st.G[mui * n + nuj] += P[laml * n + sigk] * v;
                                        st.G[mui * n + laml] +=
                                            -0.5 * P[nuj * n + sigk] * v;
                                    }
                                }
                            }
                        }
                    }
                }
                st
            },
        )
        .map(|st| st.G)
        .reduce(
            || vec![0.0; n * n],
            |mut a, b| {
                for t in 0..a.len() {
                    a[t] += b[t];
                }
                a
            },
        );

    unsafe {
        CINTdel_optimizer(&mut opt);
    }
    G
}

/// Threaded `dscf::getF`. The two 1e builds stay serial: they are `nbas^2`
/// primals next to an `nbas^4` one.
fn getF_par_in(
    atm: &mut [i32],
    bas: &mut [i32],
    env: &mut [f64],
    P: &[f64],
) -> Vec<f64> {
    let T = integral1e(atm, bas, env, 0, 1);
    let V = integral1e(atm, bas, env, 0, 2);
    let G = fock2e_par_in(atm, bas, env, P);

    let mut F = vec![0.0; T.len()];
    for i in 0..F.len() {
        F[i] = T[i] + V[i] + G[i];
    }
    F
}

/// Threaded `dscf::danalyticalg`: the whole frozen-P basis-parameter gradient,
/// `dHcore + dR - 0.5 dS`, on one pool.
///
/// Every `nbas^4` term is threaded, including the primal Fock build that `dSg`
/// hides inside `getF` -- that one is not an Enzyme reverse, but it is the same
/// order of work and leaving it serial caps the whole gradient by Amdahl. See
/// python/pyscf_comp/bench_grad_breakdown.py for the measured split.
pub fn danalytical_par(
    atm: &mut [i32],
    bas: &mut [i32],
    env: &mut [f64],
    P: &[f64],
    nthreads: usize,
) -> Vec<f64> {
    let nshells = angl(bas, 0);

    let (s1, s2) = split(bas);
    let env1: Vec<f64> = env[0..s1].to_vec();
    let env2: Vec<f64> = env[s1..s2].to_vec();
    let atm_r: Vec<i32> = atm.to_vec();
    let bas_r: Vec<i32> = bas.to_vec();

    let (dH, dR, dS) = in_pool(nthreads, || {
        // Q = P F P, the energy-weighted density that seeds the overlap term.
        // dscf::dSg builds this internally; hoisted here so F is built once
        // and so the Fock build shares this pool instead of running serially.
        let F = getF_par_in(atm, bas, env, P);
        let pf = matmult(nshells, P, &F);
        let Q = matmult(nshells, &pf, P);

        let dH = dHcore_par_in(&atm_r, &bas_r, &env1, &env2, P);
        let dR = dR_par_in(&atm_r, &bas_r, &env1, &env2, P);
        let dS = pair_1e_in(&atm_r, &bas_r, &env1, &env2, &Q, dovlp);
        (dH, dR, dS)
    });

    let mut dtotal = vec![0.0; dH.len()];
    for i in 0..dtotal.len() {
        dtotal[i] = dH[i] + dR[i] - 0.5 * dS[i];
    }
    dtotal
}

fn c_args(
    atm_p: *mut i32, atm_l: usize,
    bas_p: *mut i32, bas_l: usize,
    env_p: *mut f64, env_l: usize,
    W_p: *mut f64, W_l: usize,
) -> (Vec<i32>, Vec<i32>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let atm: Vec<i32> = unsafe { std::slice::from_raw_parts(atm_p, atm_l) }.to_vec();
    let bas: Vec<i32> = unsafe { std::slice::from_raw_parts(bas_p, bas_l) }.to_vec();
    let env: Vec<f64> = unsafe { std::slice::from_raw_parts(env_p, env_l) }.to_vec();
    let W: Vec<f64> = unsafe { std::slice::from_raw_parts(W_p, W_l) }.to_vec();
    let (s1, s2) = split(&bas);
    let env1 = env[0..s1].to_vec();
    let env2 = env[s1..s2].to_vec();
    (atm, bas, env1, env2, W)
}

/// `dS_par` over the C ABI. W is the nshells x nshells adjoint seed (Q = P F P
/// for the overlap term -- `dscf::dSg` builds it internally, this takes it).
/// nthreads = 0 uses rayon's global pool.
#[no_mangle]
pub extern "C" fn dS_par_c(
    atm_p: *mut i32, atm_l: usize,
    bas_p: *mut i32, bas_l: usize,
    env_p: *mut f64, env_l: usize,
    W_p: *mut f64, W_l: usize,
    nthreads: usize,
) -> *mut f64 {
    let (atm, bas, env1, env2, W) =
        c_args(atm_p, atm_l, bas_p, bas_l, env_p, env_l, W_p, W_l);
    leak_vec(dS_par(&atm, &bas, &env1, &env2, &W, nthreads))
}

/// `dR_par` over the C ABI. W is the density matrix P.
#[no_mangle]
pub extern "C" fn dR_par_c(
    atm_p: *mut i32, atm_l: usize,
    bas_p: *mut i32, bas_l: usize,
    env_p: *mut f64, env_l: usize,
    W_p: *mut f64, W_l: usize,
    nthreads: usize,
) -> *mut f64 {
    let (atm, bas, env1, env2, W) =
        c_args(atm_p, atm_l, bas_p, bas_l, env_p, env_l, W_p, W_l);
    leak_vec(dR_par(&atm, &bas, &env1, &env2, &W, nthreads))
}

/// `dHcore_par` over the C ABI. W is the density matrix P.
#[no_mangle]
pub extern "C" fn dHcore_par_c(
    atm_p: *mut i32, atm_l: usize,
    bas_p: *mut i32, bas_l: usize,
    env_p: *mut f64, env_l: usize,
    W_p: *mut f64, W_l: usize,
    nthreads: usize,
) -> *mut f64 {
    let (atm, bas, env1, env2, W) =
        c_args(atm_p, atm_l, bas_p, bas_l, env_p, env_l, W_p, W_l);
    leak_vec(dHcore_par(&atm, &bas, &env1, &env2, &W, nthreads))
}

/// `danalytical_par` over the C ABI: the threaded counterpart of
/// `danalytical_c`. W is the density matrix P; nthreads = 0 uses rayon's
/// global pool.
#[no_mangle]
pub extern "C" fn danalytical_par_c(
    atm_p: *mut i32, atm_l: usize,
    bas_p: *mut i32, bas_l: usize,
    env_p: *mut f64, env_l: usize,
    W_p: *mut f64, W_l: usize,
    nthreads: usize,
) -> *mut f64 {
    let mut atm: Vec<i32> = unsafe { std::slice::from_raw_parts(atm_p, atm_l) }.to_vec();
    let mut bas: Vec<i32> = unsafe { std::slice::from_raw_parts(bas_p, bas_l) }.to_vec();
    let mut env: Vec<f64> = unsafe { std::slice::from_raw_parts(env_p, env_l) }.to_vec();
    let P: Vec<f64> = unsafe { std::slice::from_raw_parts(W_p, W_l) }.to_vec();
    leak_vec(danalytical_par(&mut atm, &mut bas, &mut env, &P, nthreads))
}
