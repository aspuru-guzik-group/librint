extern crate librint;

use librint::cint_bas::CINTcgto_spheric;
use librint::cint1e::cint1e_ovlp_sph;

pub const ATM_SLOTS: usize = 6;
pub const BAS_SLOTS: usize = 8;

fn int(
    natm: usize,
    nbas: usize,
    nshells: usize,
    atm: &mut [i32],
    bas: &mut [i32],
    env: &mut [f64],
) -> Vec<f64> {
    let mut buf: Vec<f64>;
    let mut shls: [i32; 4] = [0, 0, 0, 0];

    let mut S = vec![0.0; nshells * nshells];

    println!("ovlp");
    let mut mu: usize = 0;
    for i in 0..nbas {
        let mut nu: usize = 0;
        let mut di: usize = 0;
        for j in 0..nbas {
            shls[0] = i as i32;
            shls[1] = j as i32;
            
            di = CINTcgto_spheric(i, &bas) as usize;
            let dj = CINTcgto_spheric(j, &bas) as usize;

            buf = vec![0.0; di * dj];
            cint1e_ovlp_sph(&mut buf, &mut shls, atm, natm as i32, bas, nbas as i32, env);

            // Add the buf in the correct place
            let mut c: usize = 0;
            for mui in mu..(mu + di) {
                for nuj in nu..(nu + dj) {
                    S[mui * nshells + nuj] = buf[c];
                    c += 1;
                }
            }
            nu += dj;
        }
        mu += di;
    }

    return S;
}

fn main() {
	const natm: usize = 3;
	const nbas: usize = 5;
    const nshells: usize = 7;
    
    let mut atm: [i32; natm * ATM_SLOTS] = [8, 20, 1, 23, 0, 0, 
                                                1, 24, 1, 27, 0, 0, 
                                                1, 28, 1, 31, 0, 0];
    let mut bas: [i32; nbas * BAS_SLOTS] = [0, 0, 3, 1, 0, 38, 41, 0, 
                                                0, 0, 3, 1, 0, 44, 47, 0, 
                                                0, 1, 3, 1, 0, 50, 53, 0, 
                                                1, 0, 3, 1, 0, 32, 35, 0, 
                                                2, 0, 3, 1, 0, 32, 35, 0];
    let mut env: [f64; 56] = [0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
    -0.21042327, 0., 0., 0., 0.8416929, -1.47972415, 0., 0., 0.8416929, 1.47972415, 0., 3.42525091, 0.62391373, 0.1688554, 0.98170675,
    0.94946401, 0.29590645, 130.70932, 23.808861, 6.4436083, 15.07274649, 14.57770167, 4.54323359, 5.0331513, 1.1695961, 0.380389, -0.848697, 1.13520079,
    0.85675304, 5.0331513, 1.1695961, 0.380389, 3.42906571, 2.15628856, 0.34159239
    ];

    let mut S: Vec<f64> = int(natm, nbas, nshells, &mut atm, &mut bas, &mut env);

    print!("[");
    for i in 0..nshells {
        print!("[");
        for j in 0..nshells {
            print!("{:.5}, ", S[i*nshells + j]);
        }
        println!("], ");
    }
}