#![allow(non_snake_case, non_upper_case_globals)]
#![feature(autodiff)]

use std::io;

use librint::utils::{read_basis, print_arr};

use librint::scf::{nparams, integral1e};

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
    const nelec: usize = 10;

    let mut atm = Vec::new();
    let mut bas = Vec::new();
    let mut env = Vec::new();

    let path = "/u/jpmedina/libcint/molecules/h2o/def2svp.txt";
    read_basis(path, &mut atm, &mut bas, &mut env)?;

    let (natm, nbas, nshells) = nparams(&mut atm, &mut bas);
    println!("{} {} {}", natm, nbas, nshells);

    const imax: i32 = 20;
    const conv: f64 = 0.000001;

    let mut S = integral1e(&mut atm, &mut bas, &mut env, 1, 0);
    print_arr(nshells, 2, &mut S);

    // let mut P = RHF(&mut atm, &mut bas, &mut env, nelec, imax, conv);
    // print_arr(nshells, 2, &mut P);

    // let Etot: f64 = energy(&mut atm, &mut bas, &mut env, &mut P);
    // println!("E: {}", Etot);

    // let (s1, s2) = split(&mut bas);
    // let mut env1: Vec<f64> = env[0..s1].to_vec();
    // let mut env2: Vec<f64> = env[s1..s2].to_vec();

    // let mut denv = vec![0.0; s2-s1];
    // let _dEtot = gradscf(&mut atm, &mut bas, &mut env1, &mut env2, &mut denv, &mut P, 1.0);
    // println!("denv: {:.6?}", denv);

    Ok(())
}