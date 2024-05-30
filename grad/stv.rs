use std::io;
// use std::fs::File;
// use std::str::FromStr;
// use std::io::BufReader;
// use std::io::BufRead;

// use crate::cint_bas::CINTcgto_cart;
// use crate::cint1e::cint1e_ovlp_cart;

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

    let atm: [i32; natm * ATM_SLOTS] = [1, 20, 1, 23, 0, 0, 1, 24, 1, 27, 0, 0];
    let bas: [i32; nbas * BAS_SLOTS] = [0, 0, 3, 1, 0, 28, 31, 0, 1, 0, 3, 1, 0, 28, 31, 0];
    let env: [f32; 34] = [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 
        0., 0., 0., 0., 0., 0., -1.5117809, 0., 0., 0., 1.5117809, 0., 3.42525091, 0.62391373, 
        0.1688554, 0.98170675, 0.94946401, 0.29590645];
    
    for i in 0..(natm * ATM_SLOTS) {
        println!("{}", atm[i]);
    }

    for i in 0..(34) {
        println!("{}", env[i]);
    }

    let mut di: usize = 0;
    let mut dj: usize = 0;
	let mut shls: [usize; 4] = [0, 0, 0, 0];

	// double * buf; ?? or should I do array
    let mut buf: Box<[f32]>;
	
	println!("buf");
    for i in 0..nbas {
        for j in 0..nbas {
            shls[0] = i;
            shls[1] = j;

            // di = CINTcgto_cart(i, bas);
            // dj = CINTcgto_cart(j, bas);

            buf = vec![0.0; di * dj].into_boxed_slice();

            // cint1e_ovlp_cart(buf, shls, atm, natm, bas, nbas, env);
        }
    }

    Ok(())
}