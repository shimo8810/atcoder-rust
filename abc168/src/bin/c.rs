use proconio::input;

#[allow(non_snake_case)]
fn main() {
    use std::f64::consts::PI;
    input! {
        A: f64,
        B: f64,
        H: f64,
        M: f64,
    }

    let t = (M / 60.0 - (H * 60.0 + M) / (12.0 * 60.0)) * 2.0 * PI;
    let ans = (A * A + B * B - 2.0 * A * B * t.cos()).sqrt();

    println!("{}", ans);
}
