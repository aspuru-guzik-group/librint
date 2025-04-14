use std::io;
use std::ptr;
use std::time::Instant;

use librint::cint_bas::CINTcgto_cart;
use librint::cint1e::cint1e_ovlp_cart;
use librint::cint2e::cint2e_cart;
use librint::cint2e::cint2e_cart_optimizer;

use librint::cint::CINTOpt;

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

fn main() -> io::Result<()> {
    const natm: usize = 3;
    const nbas: usize = 5;

    let mut atm_arr: [i32; natm * ATM_SLOTS] = [
        8, 20, 1, 23, 0, 0,
        1, 24, 1, 27, 0, 0,
        1, 28, 1, 31, 0, 0
    ];
    let mut bas_arr: [i32; nbas * BAS_SLOTS] = [
        0, 0, 3, 1, 0, 32, 35, 0,
        0, 0, 3, 1, 0, 38, 41, 0,
        0, 1, 3, 1, 0, 44, 47, 0,
        1, 0, 3, 1, 0, 50, 53, 0,
        2, 0, 3, 1, 0, 50, 53, 0
    ];
    let mut env_arr: [f64; 56] = [
        0.0,           0.0,           0.0,           0.0,           0.0,
        0.0,           0.0,           0.0,           0.0,           0.0,
        0.0,           0.0,           0.0,           0.0,           0.0,
        0.0,           0.0,           0.0,           0.0,           0.0,
        0.0,          -0.21042327,   0.0,           0.0,           0.0,
        0.8416929,   -1.47972415,   0.0,           0.0,           0.8416929,
        1.47972415,   0.0,         130.70932,     23.808861,     6.4436083,
        15.07274649,  14.57770167,   4.54323359,   5.0331513,    1.1695961,
        0.380389,    -0.848697,     1.13520079,   0.85675304,   5.0331513,
        1.1695961,    0.380389,     3.42906571,   2.15628856,   0.34159239,
        3.42525091,   0.62391373,   0.1688554,    0.98170675,   0.94946401,
        0.29590645
    ];

    let mut shls_arr: [i32; 4] = [0, 0, 0, 0];

    let mut opt_ptr: *mut CINTOpt = ptr::null_mut();

    unsafe {
        cint2e_cart_optimizer(
            &mut opt_ptr as *mut *mut CINTOpt,  // double pointer
            atm_arr.as_mut_ptr(),
            natm as i32,
            bas_arr.as_mut_ptr(),
            nbas as i32,
            env_arr.as_mut_ptr(),
        );
    }

    // unsafe {
    //     if !opt_ptr.is_null() {
    //         let opt: &CINTOpt = &*opt_ptr;
    //         println!("nbas: {}", opt.nbas);
    //         println!("index_xyz_array: {:?}", opt.index_xyz_array);
    //         // ... same for other fields
    //     }
    // }


    let mut buf;

	println!("buf");
    for i in 0..nbas {
        for j in 0..nbas {
            for k in 0..nbas {
                for l in 0..nbas {
                    shls_arr[0] = i as i32;
                    shls_arr[1] = j as i32;
                    shls_arr[2] = k as i32;
                    shls_arr[3] = l as i32;
                    
                    let di = unsafe { CINTcgto_cart(i, &bas_arr) };
                    let dj = unsafe { CINTcgto_cart(j, &bas_arr) };
                    let dk = unsafe { CINTcgto_cart(k, &bas_arr) };
                    let dl = unsafe { CINTcgto_cart(l, &bas_arr) };

                    buf = vec![0.0; (di * dj * dk * dl) as usize];

                    
                    let start = Instant::now();
                    unsafe {
                        cint2e_cart(
                            buf.as_mut_ptr(),
                            shls_arr.as_mut_ptr(),
                            atm_arr.as_mut_ptr(),
                            natm as i32,
                            bas_arr.as_mut_ptr(),
                            nbas as i32,
                            env_arr.as_mut_ptr(),
                            opt_ptr,
                        );
                    }
                    let duration = start.elapsed();
                    
                    print!("time: {:?}        ", duration);
                    for i in 0..((di * dj * dk * dl) as usize) {
                        print!("{} ", buf[i]);
                    }
                    println!();
                }
            }
            println!();
        }
        println!();
    }
    Ok(())
}