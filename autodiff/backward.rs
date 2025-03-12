#![feature(autodiff)]
use std::autodiff::autodiff;
#[autodiff(df, Reverse, Duplicated, Duplicated)]
fn f(x: &[f32; 2], y: &mut f32) {
    *y = x[0] * x[0] + x[1] * x[0];
}

fn main() {
    let x = [2.0, 3.0];
    let mut bx = [0.0, 0.0];
    let mut y = 0.0;
    let mut by = 1.0; // seed
    df(&x, &mut bx, &mut y, &mut by);
    assert_eq!([7.0, 2.0], bx);
    assert_eq!(10.0, y);
    assert_eq!(0.0, by); // seed is zeroed
    println!("Runs successfully");
}
