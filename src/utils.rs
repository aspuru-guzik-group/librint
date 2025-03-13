use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

enum Token {
    Int(i32),
    Float(f64),
    Delimiter,
    Invalid,
}

pub fn read_basis(
    path: &std::path::PathBuf,
    atm: &mut Vec<i32>, 
    bas: &mut Vec<i32>, 
    env: &mut Vec<f64>
) -> io::Result<()> {
    assert!(path.exists());
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let mut tokens = contents.split_whitespace().map(|s| {
        if s == "|" {
            Token::Delimiter
        } else if let Ok(int_val) = s.parse::<i32>() {
            Token::Int(int_val)
        } else if let Ok(float_val) = s.parse::<f64>() {
            Token::Float(float_val)
        } else {
            Token::Invalid
        }
    });

    while let Some(token) = tokens.next() {
        match token {
            Token::Int(value) => {
                atm.push(value);
            }
            Token::Delimiter => {
                break;
            }
            Token::Float(_) | Token::Invalid => {
                println!("Error: Expected int in file.");
            }
        }
    }

    while let Some(token) = tokens.next() {
        match token {
            Token::Int(value) => {
                bas.push(value);
            }
            Token::Delimiter => {
                break;
            }
            Token::Float(_) | Token::Invalid => {
                println!("Error: Expected int in file.");
            }
        }
    }

    while let Some(token) = tokens.next() {
        match token {
            Token::Float(value) => {
                env.push(value);
            }
            Token::Delimiter => (),
            Token::Int(_) | Token::Invalid => {
                println!("Error: Expected float in file.");
            }
        }
    }
    Ok(())
}

pub fn save_arr(
    path: &str,
    a: &mut [f64],
) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    for number in a {
        writeln!(writer, "{} ", number)?;
    }

    writer.flush()?;

    Ok(())
}

pub fn print_arr(
    n: usize,
    size: usize,
    a: &mut [f64],
) {
    for i in 0..(n.pow(size as u32)) {
        print!("{:.6} ", a[i]);
        for p in 1..size {
            if (i + 1) % n.pow(p as u32) == 0 {
                println!();
            }
        }
    }
}

#[no_mangle]
pub fn combine(
    env1: &Vec<f64>,
    env2: &Vec<f64>
) -> Vec<f64> {
    let mut env: Vec<f64> = vec![0.0; env1.len() + env2.len()];

    let mut c = 0;
    for i in 0..env1.len() {
        env[c] = env1[i];
        c += 1;
    }
    for j in 0..env2.len() {
        env[c] = env2[j];
        c += 1;
    }

    return env;
}

#[no_mangle]
pub fn split(
    bas: &mut Vec<i32>,
) -> (usize, usize) {
    let mut min = -1;
    let mut max = -1;

    for b in (0..bas.len()).step_by(BAS_SLOTS) {
        let ngto = bas[b + 2];
        let exp = bas[b + 5];
        let cont = bas[b + 6];

        if min == -1 {
            min = exp;
        } else if min > exp {
            min = exp;
        }

        if max == -1 {
            max = cont + ngto;
        } else if max < cont {
            max = cont + ngto;
        }
    }

    return (min as usize, max as usize);
}
