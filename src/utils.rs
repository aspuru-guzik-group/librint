use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

enum Token {
    Int(i32),
    Float(f64),
    Delimiter,
    Invalid,
}

pub fn read_basis(
    path: &str, // std::path::PathBuf,
    atm: &mut Vec<i32>,
    bas: &mut Vec<i32>,
    env: &mut Vec<f64>,
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

pub fn write_expected(path: &str, data: &[f64]) {
    if let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent).expect("Failed to create directories");
    }
    let file = File::create(path).expect("Failed to create file");
    let mut writer = BufWriter::new(file);

    for value in data {
        writeln!(writer, "{}", value).expect("Faile to write");
    }
}

pub fn load_expected(path: &str) -> Vec<f64> {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut vector = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        if let Ok(value) = line.trim().parse::<f64>() {
            vector.push(value);
        }
    }

    vector
}

pub fn save_arr(path: &str, a: &[f64]) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    for number in a {
        writeln!(writer, "{} ", number)?;
    }

    writer.flush()?;

    Ok(())
}

pub fn print_arr(n: usize, size: usize, a: &[f64]) {
    for i in 0..(n.pow(size as u32)) {
        print!("{:.6} ", a[i]);
        for p in 1..size {
            if (i + 1) % n.pow(p as u32) == 0 {
                println!();
            }
        }
    }
}

pub fn combine(env1: &[f64], env2: &[f64]) -> Vec<f64> {
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

    env
}

pub fn split(bas: &[i32]) -> (usize, usize) {
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

    (min as usize, max as usize)
}
