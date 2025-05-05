use std::io;
use std::time::{Instant, Duration};

use librint::cint_bas::CINTcgto_cart;
use librint::cint1e::cint1e_ovlp_cart;
use librint::cint1e::cint1e_nuc_cart;
use librint::cint1e::int1e_nuc_cart;
use librint::cint::CINTOpt;
use librint::utils::{nmol, read_basis};

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

fn main() -> io::Result<()> {
    let mut atm: Vec<i32> = Vec::new();
    let mut bas: Vec<i32> = Vec::new();
    let mut env: Vec<f64> = Vec::new();

    let path = "/h/332/jpmedina/librint/molecules/h2/sto3g.txt";
    read_basis(&path, &mut atm, &mut bas, &mut env);

    let (natm, nbas) = nmol(&mut atm, &mut bas);

    let mut shls: [i32; 4] = [0, 0, 0, 0];

    let mut buf;
    let mut di;
    let mut dj;

	println!("buf");

    let mut duration = Duration::default();
    let count = 30;

    for l in 0..count {
        let start = Instant::now();
        for i in 0..nbas {
            for j in 0..nbas {
                shls[0] = i as i32;
                shls[1] = j as i32;
                
                di = CINTcgto_cart(i, &bas);
                dj = CINTcgto_cart(j, &bas);

                buf = vec![0.0; (di * dj) as usize];

                unsafe {
                    int1e_nuc_cart(
                        buf.as_mut_ptr(),
                        std::ptr::null_mut(),
                        shls.as_mut_ptr(),
                        atm.as_mut_ptr(),
                        natm as i32,
                        bas.as_mut_ptr(),
                        nbas as i32,
                        env.as_mut_ptr(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                    );
                }
            }
        }
        duration += start.elapsed();
    }

    println!("{:?}", duration / count);

    Ok(())
}
