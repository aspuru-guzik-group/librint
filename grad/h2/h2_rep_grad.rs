#![allow(non_snake_case, non_upper_case_globals)]
#![feature(autodiff)]

use librint::cint_bas::CINTcgto_cart;
use librint::cint2e::cint2e_cart;

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

#[autodiff(cint_diff, Reverse, Duplicated, Const, Const, Const, Const, Const, Duplicated)]
fn cint_wrap(
    mut out: &mut [f64], 
    shls: &mut [i32], 
    atm: &mut [i32],
    natm: i32, 
    bas: &mut [i32], 
    nbas: i32, 
    env: &mut [f64]
) {
    cint2e_cart(&mut out, shls, atm, natm, bas, nbas, env);
}

fn main() {
    const natm: usize = 2;
    const nbas: usize = 2;
    
    let mut atm_arr: [i32; natm * ATM_SLOTS] = [1, 20, 1, 23, 0, 0, 1, 24, 1, 27, 0, 0];
    let mut bas_arr: [i32; nbas * BAS_SLOTS] = [0, 0, 3, 1, 0, 28, 31, 0, 1, 0, 3, 1, 0, 28, 31, 0];
    let mut env_arr: [f64; 34] = [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 
        -1.5117809, 0., 0., 0., 1.5117809, 0., 3.42525091, 0.62391373, 0.1688554, 0.98170675, 0.94946401, 0.29590645];
    let mut denv_arr: [f64; 34] = [0.0; 34];

    let mut shls_arr: [i32; 4] = [0, 0, 0, 0];

    let mut buf;
    let mut dbuf;

	println!("buf");
    for i in 0..nbas {
        for j in 0..nbas {
            for k in 0..nbas {
                for l in 0..nbas {
                    shls_arr[0] = i as i32;
                    shls_arr[1] = j as i32;
                    shls_arr[2] = k as i32;
                    shls_arr[3] = l as i32;
                    
                    let di = CINTcgto_cart(i, &bas_arr);
                    let dj = CINTcgto_cart(j, &bas_arr);
                    let dk = CINTcgto_cart(k, &bas_arr);
                    let dl = CINTcgto_cart(l, &bas_arr);
        
                    buf = vec![0.0; (di * dj * dk * dl) as usize];
                    dbuf = vec![0.0; (di * dj) as usize];
                    dbuf[0] = 1.0;
        
                    // cint2e_cart(&mut buf, &mut shls_arr, &mut atm_arr, natm as i32, &mut bas_arr, nbas as i32, &mut env_arr);
                    cint_diff(&mut buf, &mut dbuf, &mut shls_arr, &mut atm_arr, natm as i32, &mut bas_arr, nbas as i32, &mut env_arr, &mut denv_arr);
        
                    for m in 0..((di*dj*dk*dl) as usize) {
                        print!("{} ", buf[m]);
                    }
                }
                println!();
            }
            println!();
        }
        println!();
    }
}