use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
        d: f64
    }

    let d = d / 180.0 * std::f64::consts::PI;
    let x = a * d.cos() - b * d.sin();
    let y = a * d.sin() + b * d.cos();
    println!("{} {}", x, y);
}
