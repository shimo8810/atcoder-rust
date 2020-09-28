#![allow(unused_imports)]
use proconio::input;
use std::cmp;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        ABC: [[i64; 3]; N],
    }

    let mut dp = vec![[0; 3]; N + 1];

    for i in 1..=N {
        let a = &ABC[i - 1];
        for j in 0..3 {
            dp[i][j] = cmp::max(dp[i - 1][(j + 1) % 3] + a[j], dp[i - 1][(j + 2) % 3] + a[j]);
        }
    }
    println!("{}", dp[N].iter().max().unwrap());
}
