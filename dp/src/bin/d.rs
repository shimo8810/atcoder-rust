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

    let mut dp = vec![vec![0; W + 1]; N + 1];

    for i in 1..=N {
        let (w, v) = WV[i - 1];
        for j in 1..=W {
            //
            if j < w {
                //
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = cmp::max(dp[i - 1][j], dp[i - 1][j - w] + v);
            }
        }
    }
    println!("{}", dp[N][W]);
}
