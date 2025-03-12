#![feature(autodiff)]
use std::autodiff::autodiff;
#[autodiff(df, Reverse, Active, Active, Active)]
fn f(x: f32, y: f32) -> f32 {
    x * x + 3.0 * y
}

fn main() {
    let (x, y) = (5.0, 7.0);
    let (z, bx, by) = df(x, y, 1.0);
    assert_eq!(46.0, z);
    assert_eq!(10.0, bx);
    assert_eq!(3.0, by);
}
