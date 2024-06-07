#![feature(autodiff)]

#[autodiff(df, Forward, Dual, Dual)]
fn f(x: &[f32; 2], y: &mut f32) {
    *y = x[0] * x[0] + x[1] * x[0];
}

fn main() {
    let x = [2.0, 3.0];
    let dx = [1.0, 0.0];
    let mut y = 0.0;
    let mut dy = 0.0;
    df(&x, &dx, &mut y, &mut dy);
    assert_eq!(10.0, y);
    assert_eq!(7.0, dy);
    println!("Runs successfully");
}