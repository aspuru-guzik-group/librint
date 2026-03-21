#![allow(
    non_snake_case,
    non_upper_case_globals,
    unused_variables,
    improper_ctypes_definitions,
    static_mut_refs
)]
#![feature(autodiff)]

use std::env;

use std::time::Instant;

use librint::scf::{angl, nmol};
use librint::utils::{combine, read_basis, split};
use std::autodiff::*;

use librint::cint1e::cint1e_ovlp_cart;
use librint::cint_bas::CINTcgto_cart;

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

#[no_mangle]
#[autodiff_reverse(dovlp_rev, Duplicated, Const, Const, Const, Const, Const, Duplicated)]
#[autodiff_forward(dovlp_for, Dual, Const, Const, Const, Const, Const, Dual)]
fn ovlpp(
    out: &mut [f64],
    shls: &mut [i32],
    atm: &mut [i32],
    natm: usize,
    bas: &mut [i32],
    nbas: usize,
    env: &mut [f64],
) {
    cint1e_ovlp_cart(
        out,
        shls,
        atm,
        natm as i32,
        bas,
        nbas as i32,
        env,
        std::ptr::null_mut(),
    );
}

#[no_mangle]
//#[autodiff_reverse(dS_rev, Duplicated, Const, Const, Const, Duplicated)]
//#[autodiff_forward(dS_for, Dual, Const, Const, Const, Dual)]
#[autodiff_reverse(dS_rev, Duplicated, Const, Const, Duplicated)]
#[autodiff_forward(dS_for, Dual, Const, Const, Dual)]
pub fn S(
    out: &mut Vec<f64>,
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    //env1: &mut Vec<f64>,
    //env2: &mut Vec<f64>,
) {
    let (natm, nbas) = nmol(atm, bas);
    let nshells = angl(bas, 0);
    //let mut env: Vec<f64> = combine(&env1, &env2);
    let mut shls_buf = vec![0i32; 4];

    let mut mu = 0;
    for i in 0..nbas {
        shls_buf[0] = i as i32;
        let di = CINTcgto_cart(i, &bas) as usize;
        let mut nu = 0;
        for j in 0..nbas {
            shls_buf[1] = j as i32;
            let dj = CINTcgto_cart(j, &bas) as usize;

            let mut buf = vec![0.0; di * dj];
            cint1e_ovlp_cart(
                &mut buf,
                &mut shls_buf,
                atm,
                natm as i32,
                bas,
                nbas as i32,
                env,
                std::ptr::null_mut(),
            );

            let mut c: usize = 0;
            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    out[nuj * nshells + mui] = buf[c];
                    c += 1;
                }
            }
            nu += dj;
        }
        mu += di;
    }
}

fn time_reverse_element_mode(
    atm: &mut Vec<i32>,
    natm: usize,
    bas: &mut Vec<i32>,
    nbas: usize,
    env: &mut Vec<f64>,
    env2_len: usize,
    dS: &mut [f64],
) {
    println!("--- Timing Reverse Mode (element) dS ---");
    let mut shls: [i32; 4] = [0, 0, 0, 0];

    let nshells = angl(&bas, 0);

    let max_size = (nbas * nbas) as usize;
    let mut buf = vec![0.0; max_size];
    let mut dbuf = vec![0.0; max_size];
    let mut denv = vec![0.0f64; env.len()];

    let mut total_ovlp_time = 0.0;
    let mut total_dovlp_time = 0.0;
    let mut count = 0;

    let mut di = 0;
    let mut dj = 0;

    let mut mu: usize;
    let mut nu: usize;
    mu = 0;
    for i in 0..nbas {
        nu = 0;
        for j in 0..nbas {
            shls[0] = i as i32;
            shls[1] = j as i32;

            di = CINTcgto_cart(i, bas) as usize;
            dj = CINTcgto_cart(j, bas) as usize;

            let size = (di * dj) as usize;

            let start_ovlp = std::time::Instant::now();
            ovlpp(&mut buf, &mut shls, atm, natm, bas, nbas, env);
            let duration_ovlp = start_ovlp.elapsed().as_secs_f64();
            total_ovlp_time += duration_ovlp;

            let mut c: usize = 0;
            for nuj in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    denv.fill(0.0);
                    dbuf[c] = 1.0;

                    let start_dovlp = std::time::Instant::now();
                    dovlp_rev(
                        &mut buf, &mut dbuf, &mut shls, atm, natm, bas, nbas, env, &mut denv,
                    );

                    let duration_dovlp = start_dovlp.elapsed().as_secs_f64();
                    total_dovlp_time += duration_dovlp;

                    for l in 0..env2_len {
                        let idx = (nuj * nshells + mui) * env2_len + l;
                        if idx < dS.len() {
                            dS[idx] = denv[l];
                        }
                    }

                    dbuf[c] = 0.0;
                    c += 1;
                }
            }

            count += 1;

            nu += dj;
        }
        mu += di;
    }

    println!("count {}", count);
    println!("total ovlp time:    {:.6} sec", total_ovlp_time);
    println!("total dovlp time:   {:.6} sec", total_dovlp_time);
    println!(
        "average ovlp time:  {:.6} sec",
        total_ovlp_time / count as f64
    );
    println!(
        "average dovlp time: {:.6} sec",
        total_dovlp_time / count as f64
    );
    println!(
        "avg overhead:       {:.6}",
        total_dovlp_time / total_ovlp_time
    );
}

fn time_forward_element_mode(
    atm: &mut Vec<i32>,
    natm: usize,
    bas: &mut Vec<i32>,
    nbas: usize,
    env: &mut Vec<f64>,
    env2_len: usize,
    dS_for: &mut [f64],
) {
    println!("--- Timing Forward Mode (element) dS ---");
    let mut shls: [i32; 4] = [0, 0, 0, 0];

    let nshells = angl(bas, 0);

    let max_size = (nbas * nbas) as usize;
    let mut buf = vec![0.0; max_size];
    let mut dbuf = vec![0.0; max_size];
    let mut denv = vec![0.0f64; env.len()];

    let mut total_ovlp_time_for = 0.0;
    let mut total_dovlp_time_for = 0.0;
    let mut count = 0;

    let mut di = 0;
    let mut dj = 0;

    let mut mu: usize;
    let mut nu: usize;
    mu = 0;
    for i in 0..nbas {
        nu = 0;
        for j in 0..nbas {
            shls[0] = i as i32;
            shls[1] = j as i32;

            di = CINTcgto_cart(i, bas) as usize;
            dj = CINTcgto_cart(j, bas) as usize;

            let size = (di * dj) as usize;

            let start_ovlp = std::time::Instant::now();
            ovlpp(&mut buf, &mut shls, atm, natm, bas, nbas, env);
            let duration_ovlp = start_ovlp.elapsed().as_secs_f64();
            total_ovlp_time_for += duration_ovlp;

            for l in 0..env2_len {
                dbuf.fill(0.0);
                denv[l] = 1.0;

                let start_dovlp = std::time::Instant::now();
                dovlp_for(
                    &mut buf, &mut dbuf, &mut shls, atm, natm, bas, nbas, env, &mut denv,
                );
                let duration_dovlp = start_dovlp.elapsed().as_secs_f64();
                total_dovlp_time_for += duration_dovlp;

                let mut c: usize = 0;
                for nuj in nu..(nu + dj) {
                    for mui in mu..(mu + di) {
                        let idx = (nuj * nshells + mui) * env2_len + l;
                        if idx < dS_for.len() {
                            dS_for[idx] = dbuf[c];
                        }
                        c += 1;
                    }
                }

                denv[l] = 0.0;
            }

            count += 1;

            nu += dj;
        }
        mu += di;
    }

    println!("count {}", count);
    println!("total ovlp time:    {:.6} sec", total_ovlp_time_for);
    println!("total dovlp time:   {:.6} sec", total_dovlp_time_for);
    println!(
        "average ovlp time:  {:.6} sec",
        total_ovlp_time_for / count as f64
    );
    println!(
        "average dovlp time: {:.6} sec",
        total_dovlp_time_for / count as f64
    );
    println!(
        "avg overhead:       {:.6}",
        total_dovlp_time_for / total_ovlp_time_for
    );
}

fn time_forward_matrix_mode(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    env2_len: usize,
    dS: &mut [f64],
) {
    println!("--- Timing Matrix Forward Mode dS ---");
    let nshells = angl(&bas, 0);

    //let (s1, s2) = split(bas);
    //let mut env1: Vec<f64> = env[0..s1].to_vec();
    //let mut env2: Vec<f64> = env[s1..s2].to_vec();

    // Time the primal (building the full matrix once)
    let start_primal = std::time::Instant::now();
    let mut out_primal = vec![0.0; nshells * nshells];
    S(&mut out_primal, atm, bas, env);
    //S(&mut out_primal, atm, bas, &mut env1, &mut env2);
    let primal_time = start_primal.elapsed().as_secs_f64();

    // Time the full Jacobian computation: env2_len forward calls
    let mut out = vec![0.0; nshells * nshells];
    let mut dout = vec![0.0; nshells * nshells];
    let mut denv2 = vec![0.0; env.len()];

    let start_total = std::time::Instant::now();
    for l in 0..env2_len {
        dout.fill(0.0);
        denv2[l] = 1.0;

        dS_for(
            &mut out, &mut dout, atm, bas, env, &mut denv2,
            //&mut out, &mut dout, atm, bas, &mut env1, &mut env2, &mut denv2,
        );

        denv2[l] = 0.0;

        for idx in 0..(nshells * nshells) {
            dS[idx * env2_len + l] = dout[idx];
        }
    }

    let total_dovlp_time = start_total.elapsed().as_secs_f64();

    println!("AD calls:           {}", env2_len);
    println!("primal time:        {:.6} sec", primal_time);
    println!("total dovlp time:   {:.6} sec", total_dovlp_time);
    println!(
        "avg dovlp time:     {:.6} sec",
        total_dovlp_time / env2_len as f64
    );
    println!("avg overhead:       {:.6}", total_dovlp_time / primal_time);
}

fn time_reverse_matrix_mode(
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
    env2_len: usize,
    dS: &mut [f64],
) {
    println!("--- Timing Row Reverse Mode dS ---");
    let (_, nbas) = nmol(&atm, &bas);
    let nshells = angl(&bas, 0);

    //let (s1, s2) = split(bas);
    //let mut env1: Vec<f64> = env[0..s1].to_vec();
    //let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let start_primal = std::time::Instant::now();
    let mut out_primal = vec![0.0; nshells * nshells];
    S(&mut out_primal, atm, bas, env);
    //S(&mut out_primal, atm, bas, &mut env1, &mut env2);
    let primal_time = start_primal.elapsed().as_secs_f64();

    // Time the full Jacobian computation
    let mut out = vec![0.0; nshells * nshells];
    let mut dout = vec![0.0; nshells * nshells];
    let mut denv2 = vec![0.0; env.len()];
    //let mut denv2 = vec![0.0; env2_len];

    let start_total = std::time::Instant::now();

    for k in 0..(nshells * nshells) {
        denv2.fill(0.0);
        dout[k] = 1.0;

        dS_rev(
            &mut out, &mut dout, atm, bas, env, &mut denv2,
            //&mut out, &mut dout, atm, bas, &mut env1, &mut env2, &mut denv2,
        );

        dout[k] = 0.0;

        for l in (env.len() - env2_len)..(env.len()) {
            let idx = k * env2_len + l;
            if idx < dS.len() {
                dS[idx] = denv2[l];
            }
        }
    }

    let total_dovlp_time = start_total.elapsed().as_secs_f64();

    println!("primal time:        {:.6} sec", primal_time);
    println!("total dovlp time:   {:.6} sec", total_dovlp_time);
    println!(
        "avg dovlp time:     {:.6} sec",
        total_dovlp_time / (nshells * nshells) as f64
    );
    println!("avg overhead:       {:.6}", total_dovlp_time / primal_time);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} file", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];

    let mut atm: Vec<i32> = Vec::new();
    let mut bas: Vec<i32> = Vec::new();
    let mut env: Vec<f64> = Vec::new();

    _ = read_basis(&path, &mut atm, &mut bas, &mut env);

    let (natm, nbas) = nmol(&mut atm, &mut bas);

    let (s1, s2) = split(&mut bas);
    let nshells = angl(&bas, 0);

    let mut env1: Vec<f64> = env[0..s1].to_vec();
    let mut env2: Vec<f64> = env[s1..s2].to_vec();

    let mut dS_rev = vec![0.0; nbas * nbas * env2.len()];
    let mut dS_matrix_rev = vec![0.0; nshells * nshells * env2.len()];

    let mut dS_for = vec![0.0; nbas * nbas * env2.len()];
    let mut dS_matrix_for = vec![0.0; nshells * nshells * env2.len()];

    time_reverse_element_mode(
        &mut atm,
        natm,
        &mut bas,
        nbas,
        &mut env,
        env2.len(),
        &mut dS_rev,
    );
    //time_reverse_matrix_mode(&mut atm, &mut bas, &mut env, env2.len(), &mut dS_matrix_rev);

    time_forward_element_mode(
        &mut atm,
        natm,
        &mut bas,
        nbas,
        &mut env,
        env2.len(),
        &mut dS_for,
    );
    time_forward_matrix_mode(&mut atm, &mut bas, &mut env, env2.len(), &mut dS_matrix_for);

    // Compare dS and dS_for
    let mut mismatches = 0;
    for i in 0..dS_rev.len() {
        if mismatches > 10 {
            break;
        }
        if (dS_rev[i] - dS_for[i]).abs() > 1e-10 {
            println!(
                "Mismatch at index {}: dS_rev = {}, dS_for = {}",
                i, dS_rev[i], dS_for[i]
            );
            mismatches += 1;
        }
    }
    if mismatches == 0 {
        println!("dS_rev and dS_for match");
    }

    println!("env2.len() {}", env2.len());
}
