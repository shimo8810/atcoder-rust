use proconio::input;
use std::cmp;

#[allow(non_snake_case)]
fn main() {
    input! {
        A: i128,
        B: i128,
        N: i128,
    }

    let ans = cmp::min(B - 1, N) * A / B;
    println!("{}", ans);
}
