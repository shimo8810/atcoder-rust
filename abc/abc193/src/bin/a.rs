use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        (A, B): (f64, f64),
    }

    let ans = (1.0 - B / A) * 100.0;
    println!("{}", ans);
}
