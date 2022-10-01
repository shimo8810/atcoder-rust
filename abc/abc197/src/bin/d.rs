use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: f64,
        (x0, y0): (f64, f64),
        (x1, y1): (f64, f64),
    }

    let t = 2.0 * std::f64::consts::PI / N;

    let xc = (x0 + x1) / 2.0;
    let yc = (y0 + y1) / 2.0;
    let xp = x0 - xc;
    let yp = y0 - yc;
    let x = xp * t.cos() - yp * t.sin();
    let y = xp * t.sin() + yp * t.cos();
    println!("{} {}", xc + x, yc + y);
}
