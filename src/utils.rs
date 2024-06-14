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
    Invalid,
}

pub fn read_basis(
    path: &str, 
    atm: &mut Vec<i32>, 
    bas: &mut Vec<i32>, 
    env: &mut Vec<f64>
) -> io::Result<()> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let mut tokens = contents.split_whitespace().map(|s| {
        if let Ok(int_val) = s.parse::<i32>() {
            Token::Int(int_val)
        } else if let Ok(float_val) = s.parse::<f64>() {
            Token::Float(float_val)
        } else {
            Token::Invalid
        }
    });

    for i in 0..(atm.len()) {
        if let Some(Token::Int(value)) = tokens.next() {
            atm[i] = value;
        } else {
            println!("Error: Expected integer in file.");
            break;
        }
    }

    for i in 0..(bas.len()) {
        if let Some(Token::Int(value)) = tokens.next() {
            bas[i] = value;
        } else {
            println!("Error: Expected integer in file.");
            break;
        }
    }

    let mut index = 0;
    while let Some(token) = tokens.next() {
        match token {
            Token::Float(value) => {
                env[index] = value;
                index += 1;
            }
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
        print!("{:.5} ", a[i]);
        for p in 1..size {
            if (i + 1) % n.pow(p as u32) == 0 {
                println!();
            }
        }
    }
}