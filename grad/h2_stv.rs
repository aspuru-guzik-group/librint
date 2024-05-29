use std::fs::File;

// fn cint1e_ovlp_cart(
//     mut out: *mut libc::c_double,
//     mut shls: *mut libc::c_int,
//     mut atm: *mut libc::c_int,
//     mut natm: libc::c_int,
//     mut bas: *mut libc::c_int,
//     mut nbas: libc::c_int,
//     mut env: *mut libc::c_double,
//     mut opt: *mut CINTOpt,
// )

const i32: ATM_SLOTS = 6;
const i32: BAS_SLOTS = 8;

fn read_arrays(file: File, atm: &mut Box<[i32]>, bas: &mut Box<[i32]>, env: &mut Box<[f32]>) -> io::Result<()>
{
    let reader = BufReader::new(file);

    for i in 0..(natm * ATM_SLOTS) {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let value = i32::from_str(&line);
        atm[i] = value;
    }

    for i in 0..(nbas * BAS_SLOTS) {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let value = i32::from_str(&line);
        bas[i] = value;
    }

    // for i in 0..(natm * ATM_SLOTS) {
    //     let mut line = String::new();
    //     reader.read_line(&mut line)?;
    //     let value = f32::from_str(&line);
    //     env[i] = value;
    // }

    Ok(())
}

fn main() 
{
    let natm = 2;
    let nbas = 2;
    
    let mut atm: Box<i32> = vec![0; natm * ATM_SLOTS].into_boxed_slice();
    let mut bas: Box<i32> = vec![0; nbas * BAS_SLOTS].into_boxed_slice();
    let mut env: Box<f32> = vec![0; 10000].into_boxed_slice();

    let file = File::open("/u/jpmedina/libcint/molecules/h2/basis.txt")?;

    for i in 0..(natm * ATM_SLOTS) {
        println!("{}", atm[i]);
    }
    
}