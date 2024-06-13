extern crate librint;

use librint::cint1e::cint1e_ovlp_sph;
use librint::cint_bas::CINTcgto_spheric;

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
            for nui in nu..(nu + dj) {
                for mui in mu..(mu + di) {
                    S[mui * nshells + nui] = buf[c];
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
	const nbas: usize = 12;
    const nshells: usize = 24;
    
    let mut atm: [i32; natm * ATM_SLOTS] = [8, 20, 1, 23, 0, 0, 1, 24, 1, 27, 0, 0, 1, 28, 1, 31, 0, 0];
    let mut bas: [i32; nbas * BAS_SLOTS] = [0, 0, 5, 1, 0, 42, 47, 0, 
                                            0, 0, 1, 1, 0, 52, 53, 0, 
                                            0, 0, 1, 1, 0, 54, 55, 0, 
                                            0, 1, 3, 1, 0, 56, 59, 0, 
                                            0, 1, 1, 1, 0, 62, 63, 0, 
                                            0, 2, 1, 1, 0, 64, 65, 0, 
                                            1, 0, 3, 1, 0, 32, 35, 0, 
                                            1, 0, 1, 1, 0, 38, 39, 0, 
                                            1, 1, 1, 1, 0, 40, 41, 0, 
                                            2, 0, 3, 1, 0, 32, 35, 0, 
                                            2, 0, 1, 1, 0, 38, 39, 0, 
                                            2, 1, 1, 1, 0, 40, 41, 0];
    let mut env: [f64; 66] = [0.000000, 
    0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 
    0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 
    -0.210423, 0.000000, 0.000000, 0.000000, 0.841693, -1.479724, 0.000000, 0.000000, 0.841693, 1.479724, 
    0.000000, 13.010701, 1.962257, 0.444538, 0.579558, 0.983186, 1.119305, 0.121950, 0.521375, 0.800000, 
    2.207226, 2266.176779, 340.870102, 77.363135, 21.479645, 6.658943, -4.472206, -8.064133, -11.868217, -11.804536, 
    -4.680635, 0.809760, 2.156662, 0.255308, 0.907430, 17.721504, 3.863551, 1.048092, 6.643459, 5.267054, 
    2.293965, 0.276415, 0.584706, 1.200000, 3.590018];

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