use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        D: f64
    }

    println!("{}", D / 100.0);
}
