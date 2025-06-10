use std::io;
use std::fs::File;
use std::io::BufReader;
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
    path: &str, //std::path::PathBuf,
    atm: &mut Vec<i32>, 
    bas: &mut Vec<i32>, 
    env: &mut Vec<f64>
) -> io::Result<()> {
    // assert!(path.exists());
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

pub fn nmol(
    atm: &Vec<i32>,
    bas: &Vec<i32>,
) 
-> (usize, usize) {
    let natm: usize = atm.len() / ATM_SLOTS;
    let nbas: usize = bas.len() / BAS_SLOTS;
    return (natm, nbas);
}