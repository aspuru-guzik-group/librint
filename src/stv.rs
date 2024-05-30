use std::io;
// use std::fs::File;
// use std::str::FromStr;
// use std::io::BufReader;
// use std::io::BufRead;

use crate::cint_bas::CINTcgto_cart;
use crate::cint1e::cint1e_ovlp_cart;
use crate::cint1e::CINTOpt;

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

// fn read_arrays(file: &mut File, atm: &mut Box<[i32]>, bas: &mut Box<[i32]>, env: &mut Box<[f32]>) -> io::Result<()> {
//     let mut reader = BufReader::new(file);

//     println!("{}", atm.len());

//     for i in 0..(atm.len()) {
//         println!("{}", i);
//         let mut line = String::new();
//         reader.read_line(&mut line)?;
//         println!("{}", line);
//         let value = match i32::from_str(&line) {
//             Ok(int) => int,
//             Err(_) => {println!("could not read"); continue;}
//         };
//         atm[i] = value;
//     }

//     for i in 0..(bas.len()) {
//         let mut line = String::new();
//         reader.read_line(&mut line)?;
//         let value = match i32::from_str(&line.trim()) {
//             Ok(int) => int,
//             Err(_) => {continue;}
//         };
//         bas[i] = value;
//     }

//     // Similar loop for env, commented out for now
//     let mut index = 0;
//     for line in reader.lines() {
//         let line = line?;
//         let value = match f32::from_str(&line.trim()) {
//             Ok(float) => float,
//             Err(_) => {continue;}
//         };
//         env[index] = value;
//         index += 1;
//     }

//     Ok(())
// }

fn main() -> io::Result<()> {
    const natm: usize = 2;
    const nbas: usize = 2;
    
    // let mut atm: Box<[i32]> = vec![0; natm * ATM_SLOTS].into_boxed_slice();
    // let mut bas: Box<[i32]> = vec![0; nbas * BAS_SLOTS].into_boxed_slice();
    // let mut env: Box<[f32]> = vec![0.0; 10000].into_boxed_slice();

    // let mut file = File::open("/home/juanpmg/enzyme/h2/basis.txt")?;

    // read_arrays(&mut file, &mut atm, &mut bas, &mut env)?;

    let atm_arr: [i32; natm * ATM_SLOTS] = [1, 20, 1, 23, 0, 0, 1, 24, 1, 27, 0, 0];
    let bas_arr: [i32; nbas * BAS_SLOTS] = [0, 0, 3, 1, 0, 28, 31, 0, 1, 0, 3, 1, 0, 28, 31, 0];
    let env_arr: [f64; 34] = [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 
        -1.5117809, 0., 0., 0., 1.5117809, 0., 3.42525091, 0.62391373, 0.1688554, 0.98170675, 0.94946401, 0.29590645];

    let atm: *mut i32 = atm_arr.as_mut_ptr();
    let bas: *mut i32 = bas_arr.as_mut_ptr();
    let env: *mut f64 = env_arr.as_mut_ptr();
    
    for i in 0..(natm * ATM_SLOTS) {
        println!("{}", *atm.add(i));
    }

    for i in 0..(34) {
        println!("{}", *env.add(i));
    }

    let mut di: usize = 0;
    let mut dj: usize = 0;
    let shls_arr: [i32; 4] = [0, 0, 0, 0];
	let mut shls: *mut i32 = shls_arr.as_mut_ptr();

    let mut buf: *mut f64;

    let mut opt: *mut CINTOpt;
	
	println!("buf");
    for i in 0..nbas {
        for j in 0..nbas {
            let i32_i = i as i32;
            let i32_j = j as i32;
            *shls.add(0) = i32_i;
            *shls.add(1) = i32_j;

            di = CINTcgto_cart(i32_i, bas) as usize;
            dj = CINTcgto_cart(i32_j, bas) as usize;

            // buf = vec![0.0; di * dj].into_boxed_slice();

            cint1e_ovlp_cart(buf, shls, atm, natm as i32, bas, nbas as i32, env, opt);
        }
    }

    Ok(())
}