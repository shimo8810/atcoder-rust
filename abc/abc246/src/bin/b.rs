use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        (A, B): (f64, f64),
    }
    let d = (A * A + B * B).sqrt();
    let x = A / d;
    let y = B / d;
    println!("{} {}", x, y);
}
