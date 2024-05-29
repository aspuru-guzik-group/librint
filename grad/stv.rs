use std::io;
use std::fs::File;
use std::str::FromStr;
use std::io::BufReader;
use std::io::BufRead;

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

fn read_arrays(file: &mut File, atm: &mut Box<[i32]>, bas: &mut Box<[i32]>, env: &mut Box<[f32]>) -> io::Result<()> {
    let mut reader = BufReader::new(file);

    println!("{}", atm.len());

    for i in 0..(atm.len()) {
        println!("{}", i);
        let mut line = String::new();
        reader.read_line(&mut line)?;
        println!("{}", line);
        let value = match i32::from_str(&line) {
            Ok(int) => int,
            Err(_) => {println!("could not read"); continue;}
        };
        atm[i] = value;
    }

    for i in 0..(bas.len()) {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let value = match i32::from_str(&line.trim()) {
            Ok(int) => int,
            Err(_) => {continue;}
        };
        bas[i] = value;
    }

    // Similar loop for env, commented out for now
    let mut index = 0;
    for line in reader.lines() {
        let line = line?;
        let value = match f32::from_str(&line.trim()) {
            Ok(float) => float,
            Err(_) => {continue;}
        };
        env[index] = value;
        index += 1;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let natm = 2;
    let nbas = 2;
    
    let mut atm: Box<[i32]> = vec![0; natm * ATM_SLOTS].into_boxed_slice();
    let mut bas: Box<[i32]> = vec![0; nbas * BAS_SLOTS].into_boxed_slice();
    let mut env: Box<[f32]> = vec![0.0; 10000].into_boxed_slice();

    let mut file = File::open("/home/juanpmg/enzyme/h2/basis.txt")?;

    read_arrays(&mut file, &mut atm, &mut bas, &mut env)?;

    // for i in 0..(natm * ATM_SLOTS) {
    //     println!("{}", atm[i]);
    // }

    Ok(())
}