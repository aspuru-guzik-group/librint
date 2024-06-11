extern crate librint;

use faer::mat;
use faer::linalg::solvers::Eigendecomposition;
use faer::complex_native::c64;

fn main() {
    // Construct a real matrix
    let a: [f64; 4] = [1.000000, 0.222082, 0.222082, 1.000000];
    let S = mat::from_column_major_slice::<f64>(&a, 2, 2);

    // Perform eigendecomposition
    let eig_decomp: Eigendecomposition<c64> = Eigendecomposition::new_from_real(S);

    // Extract eigenvalues and eigenvectors
    let eigenvalues = eig_decomp.s();
    let eigenvectors = eig_decomp.u();

    println!("Eigenvalues");
    let eign = eigenvalues.column_vector();
    for i in 0..eign.nrows() {
        print!("{} ", eign.read(i).re());
    }
    println!();


    // Print the eigenvectors
    println!("Eigenvectors:");
    for i in 0..eigenvectors.nrows() {
        for j in 0..eigenvectors.ncols() {
            print!("{:>8.3} ", eigenvectors.read(i, j).re());
        }
        println!();
    }
}