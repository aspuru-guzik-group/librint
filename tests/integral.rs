use std::env;
use std::io;
use std::time::{Duration, Instant};

use librint::utils::{nmol, read_basis};

use librint::cint1e::cint1e_nuc_cart;
use librint::cint2e::cint2e_cart;
use librint::cint_bas::CINTcgto_cart;

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

fn nuc(
    atm: &mut Vec<i32>,
    natm: usize,
    bas: &mut Vec<i32>,
    nbas: usize,
    env: &mut Vec<f64>,
) -> Duration {
    let mut shls: [i32; 4] = [0, 0, 0, 0];

    let mut buf;
    let mut di;
    let mut dj;

    let start_total = Instant::now();
    for i in 0..nbas {
        for j in 0..nbas {
            shls[0] = i as i32;
            shls[1] = j as i32;

            unsafe {
                di = CINTcgto_cart(i as i32, bas.as_mut_ptr());
                dj = CINTcgto_cart(j as i32, bas.as_mut_ptr());
            }

            buf = vec![0.0; (di * dj) as usize];

            // let start = Instant::now();
            unsafe {
                cint1e_nuc_cart(
                    buf.as_mut_ptr(),
                    shls.as_mut_ptr(),
                    atm.as_mut_ptr(),
                    natm as i32,
                    bas.as_mut_ptr(),
                    nbas as i32,
                    env.as_mut_ptr(),
                    std::ptr::null_mut(),
                );
            }
            // println!("{} {} {:?}", i, j, start.elapsed());

            for p in 0..(di * dj) {
                print!("{} ", buf[p as usize]);
            }
        }
    }
    println!();

    let duration_total = start_total.elapsed();

    return duration_total;
}

fn rep(
    atm: &mut Vec<i32>,
    natm: usize,
    bas: &mut Vec<i32>,
    nbas: usize,
    env: &mut Vec<f64>,
) -> Duration {
    let mut shls: [i32; 4] = [0, 0, 0, 0];

    let mut buf;
    let mut di;
    let mut dj;
    let mut dk;
    let mut dl;

    let start_total = Instant::now();
    for i in 0..nbas {
        for j in 0..nbas {
            for k in 0..nbas {
                for l in 0..nbas {
                    shls[0] = i as i32;
                    shls[1] = j as i32;
                    shls[2] = k as i32;
                    shls[3] = l as i32;

                    unsafe {
                        di = CINTcgto_cart(i as i32, bas.as_mut_ptr());
                        dj = CINTcgto_cart(j as i32, bas.as_mut_ptr());
                        dk = CINTcgto_cart(k as i32, bas.as_mut_ptr());
                        dl = CINTcgto_cart(l as i32, bas.as_mut_ptr());
                    }

                    buf = vec![0.0; (di * dj * dk * dl) as usize];

                    unsafe {
                        cint2e_cart(
                            buf.as_mut_ptr(),
                            shls.as_mut_ptr(),
                            atm.as_mut_ptr(),
                            natm as i32,
                            bas.as_mut_ptr(),
                            nbas as i32,
                            env.as_mut_ptr(),
                            std::ptr::null_mut(),
                        );
                    }
                }
            }
        }
    }
    let duration_total = start_total.elapsed();
    return duration_total;
}

fn main() -> io::Result<()> {
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

    let mut time;

    // first call is warmup

    // Nuc integral
    _ = nuc(&mut atm, natm, &mut bas, nbas, &mut env);
    time = nuc(&mut atm, natm, &mut bas, nbas, &mut env);
    println!("{:?} ns", time.as_nanos());

    // Repulsion integral
    _ = rep(&mut atm, natm, &mut bas, nbas, &mut env);
    time = rep(&mut atm, natm, &mut bas, nbas, &mut env);
    println!("{:?} ns", time.as_nanos());

    Ok(())
}
