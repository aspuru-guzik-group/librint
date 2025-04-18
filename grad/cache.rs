use std::io;
use std::time::{Instant, Duration};

use librint::cint_bas::CINTcgto_cart;
use librint::cint1e::cint1e_ovlp_cart;
use librint::cint1e::cint1e_nuc_cart;
use librint::cint1e::int1e_nuc_cart;
use librint::cint::CINTOpt;

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

fn max_cache_size(
    nbas: usize,
    atm: &mut [i32],
    natm: usize,
    bas: &mut [i32],
    env: &mut [f64],
) -> i32 {
    let mut di;
    let mut dj;

    let mut shls: [i32; 4] = [0, 0, 0, 0];

    let mut tmp;

    let mut cache_size = 0;

    for i in 0..nbas {
        for j in 0..nbas {
            shls[0] = i as i32;
            shls[1] = j as i32;
            
            di = CINTcgto_cart(i, &bas);
            dj = CINTcgto_cart(j, &bas);

            tmp = unsafe {
                int1e_nuc_cart(
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    shls.as_mut_ptr(),
                    atm.as_mut_ptr(),
                    natm as i32,
                    bas.as_mut_ptr(),
                    nbas as i32,
                    env.as_mut_ptr(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                )
            };

            if (tmp > cache_size) {
                cache_size = tmp;
                print!("{} ", cache_size);
            }
        }
    }

    return cache_size;
}

fn main() -> io::Result<()> {
    // const natm: usize = 2;
    // const nbas: usize = 2;

    // let mut atm: [i32; natm * ATM_SLOTS] = [
    //     1, 20, 1, 23, 0, 0, 
    //     1, 24, 1, 27, 0, 0
    // ];
    // let mut bas: [i32; nbas * BAS_SLOTS] = [
    //     0, 0, 3, 1, 0, 28, 31, 0, 
    //     1, 0, 3, 1, 0, 28, 31, 0
    // ];
    // let mut env: [f64; 34] = [
    //     0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 
    //     0., -1.5117809, 0., 0., 0., 1.5117809, 0., 3.42525091, 0.62391373, 0.1688554, 0.98170675, 0.94946401, 0.29590645
    // ];

    const natm: usize = 3;
    const nbas: usize = 12;

    let mut atm: [i32; natm * ATM_SLOTS] = [
        8, 20, 1, 23, 0, 0,
        1, 24, 1, 27, 0, 0,
        1, 28, 1, 31, 0, 0
    ];
    let mut bas: [i32; nbas * BAS_SLOTS] = [
        0,  0,  5,  1,  0, 32, 37,  0,
        0,  0,  1,  1,  0, 42, 43,  0,
        0,  0,  1,  1,  0, 44, 45,  0,
        0,  1,  3,  1,  0, 46, 49,  0,
        0,  1,  1,  1,  0, 52, 53,  0,
        0,  2,  1,  1,  0, 54, 55,  0,
        1,  0,  3,  1,  0, 56, 59,  0,
        1,  0,  1,  1,  0, 62, 63,  0,
        1,  1,  1,  1,  0, 64, 65,  0,
        2,  0,  3,  1,  0, 56, 59,  0,
        2,  0,  1,  1,  0, 62, 63,  0,
        2,  1,  1,  1,  0, 64, 65,  0
    ];
    let mut env: [f64; 66] = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.2104232716416691, 0.0, 0.0, 0.0, 0.841692897594064, -1.4797241525927651, 0.0, 0.0, 0.841692897594064, 1.4797241525927651, 0.0, 2266.1767785, 340.87010191, 77.363135167, 21.47964494, 6.6589433124, -4.472205830377161, -8.064133436980633, -11.86821744420372, -11.804535745439507, -4.680635259503801, 0.80975975668, 2.1566623979211927, 0.25530772234, 0.907429696013769, 17.721504317, 3.863550544, 1.0480920883, 6.643459018549882, 5.2670542098593485, 2.293965466701481, 0.27641544411, 0.5847056269797676, 1.2, 3.590017508385001, 13.010701, 1.9622572, 0.44453796, 0.5795583105047568, 0.9831856491001469, 1.1193051215867884, 0.12194962, 0.5213751919783473, 0.8, 2.207226371076266
    ];

    let mut shls: [i32; 4] = [0, 0, 0, 0];

    let cache_size = max_cache_size(nbas, &mut atm, natm, &mut bas, &mut env);

    let mut cache = vec![0.0f64; cache_size as usize];

    let mut buf;
    let mut di;
    let mut dj;

	println!("buf");

    let mut total: Duration = Default::default();

    for l in 0..10 {
        let start = Instant::now();
        for i in 0..nbas {
            for j in 0..nbas {
                shls[0] = i as i32;
                shls[1] = j as i32;
                
                di = CINTcgto_cart(i, &bas);
                dj = CINTcgto_cart(j, &bas);

                buf = vec![0.0; (di * dj) as usize];

                // unsafe {
                //     int1e_nuc_cart(
                //         buf.as_mut_ptr(),
                //         std::ptr::null_mut(),
                //         shls.as_mut_ptr(),
                //         atm.as_mut_ptr(),
                //         natm as i32,
                //         bas.as_mut_ptr(),
                //         nbas as i32,
                //         env.as_mut_ptr(),
                //         std::ptr::null_mut(),
                //         cache.as_mut_ptr(),
                //     );
                // }

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

                // for i in 0..(di * dj) {
                //     println!("{} ", buf[i as usize]);
                // }
            }
        }

        let duration = start.elapsed();

        total += duration;
    }

    println!("total duration (10 runs): {:?}", total / 10);

    Ok(())
}
