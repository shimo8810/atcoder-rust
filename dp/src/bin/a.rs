#![allow(unused_imports)]
use proconio::input;
use std::cmp;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        H: [i64; N],
    }

    let mut dp = vec![0; N];
    dp[1] = (H[0] - H[1]).abs();

    for i in 2..N {
        dp[i] = cmp::min(
            dp[i - 2] + (H[i] - H[i - 2]).abs(),
            dp[i - 1] + (H[i] - H[i - 1]).abs(),
        );
    }

    println!("{}", dp[N - 1]);
}
