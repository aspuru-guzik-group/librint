#![allow(non_snake_case, non_upper_case_globals)]
#![feature(autodiff)]
use std::autodiff::autodiff;
use std::io;
use std::time::Instant;

use librint::utils::read_basis;
use librint::utils::print_arr;

use librint::scf::{angl, density, energy, energycont};
use librint::utils::split;

// #[no_mangle]
// #[autodiff(gradscf, Reverse, Const, Const, Const, Duplicated, Const, Active)]
// pub fn energyscf(
//     atm: &mut Vec<i32>,
//     bas: &mut Vec<i32>,
//     env1: &mut Vec<f64>,
//     env2: &mut Vec<f64>,
//     P: &mut Vec<f64>,
// ) -> f64 {
//     let mut env = vec![0.0; env1.len() + env2.len()];

//     let mut c = 0;
//     for i in 0..env1.len() {
//         env[c] = env1[i];
//         c += 1;
//     }
//     for j in 0..env2.len() {
//         env[c] = env2[j];
//         c += 1;
//     }

//     return energy(atm, bas, &mut env, P);
// }


fn main() -> io::Result<()> {
    let mut atm = Vec::new();
    let mut bas = Vec::new();
    let mut env = Vec::new();

    let path = "/u/jpmedina/libcint/librint/molecules/h2/sto3g.txt";
    read_basis(path, &mut atm, &mut bas, &mut env)?;

    let nshells = angl(&bas, 0);

    const imax: i32 = 20;
    const conv: f64 = 0.000001;

    const nelec: usize = 2;

    let mut P = density(&mut atm, &mut bas, &mut env, nelec, imax, conv);

    let mut Etot = energy(&mut atm, &mut bas, &mut env, &mut P);
    println!("energy: {}", Etot);
    Etot = energycont(&mut atm, &mut bas, &mut env, &mut P);
    println!("energy: {}", Etot);

    // let now = Instant::now();
    // let Etot: f64 = energy(&mut atm, &mut bas, &mut env, &mut P);
    // let elapsed_time = now.elapsed();
    // println!("E: {}", Etot);
    // println!("E time: {}", elapsed_time.as_micros());

    // let (s1, s2) = split(&mut bas);

    // let mut env1: Vec<f64> = env[0..s1].to_vec();
    // let mut env2: Vec<f64> = env[s1..s2].to_vec();
    // println!("{:?}", env1);
    // println!("{:?}", env2);

    // let mut denv = vec![0.0; env2.len()];
    // let _ = gradscf(&mut atm, &mut bas, &mut env1, &mut env2, &mut denv, &mut P, 1.0);
    // println!("{:.6?}", denv);

    // let mut denv: Vec<f64> = vec![0.0; env2.len()];

    // let dnow = Instant::now();
    // let _Etot = gradscf(&mut atm, &mut bas, &mut env1, &mut env2, &mut denv, &mut P, 1.0);
    // let delapsed_time = dnow.elapsed();

    // println!("denv:");
    // println!("{:.6?}", denv);
    // println!("dE time: {}", delapsed_time.as_micros());

    Ok(())
}
