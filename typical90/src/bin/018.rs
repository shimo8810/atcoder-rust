use proconio::{fastout, input};
use std::f64::consts::PI;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      T: f64,
      L: f64,
      X: f64,
      Y: f64,
      Q: usize
    }

    for _ in 0..Q {
        input! {E: f64}
        let b = (X.powf(2.0) + (Y + L / 2.0 * (E / T * 2.0 * PI).sin()).powf(2.0)).sqrt();
        let t = L / 2.0 * ((E / T * 2.0 * PI).cos() * -1.0 + 1.0);
        // println!("{} / {} = {}", t, b, t / b);
        println!("{}", (t / b).atan() / PI * 180.0);
    }
}
