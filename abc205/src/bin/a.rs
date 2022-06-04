use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        A: f64,
        B: f64
    }
    let ans = A / 100. * B;
    println!("{}", ans);
}
