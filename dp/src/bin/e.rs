#![allow(unused_imports)]
use proconio::input;
use std::cmp;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        W: usize,
        WV: [(usize, usize); N],
    }

    let mut dp = vec![vec![1_000_000_000; 1_000 * 100 + 1]; N + 1];
    dp[0][0] = 0;
    for i in 0..N {
        let (w, v) = WV[i];
        for j in 0..=(1_000 * 100) {
            //
            if j < v {
                //
                dp[i + 1][j] = dp[i][j];
            } else {
                dp[i + 1][j] = cmp::min(dp[i][j], dp[i][j - v] + w);
            }
        }
    }
    println!("{:?}", dp[N].iter().rev().find(|&&x| x <= W).unwrap());
}
