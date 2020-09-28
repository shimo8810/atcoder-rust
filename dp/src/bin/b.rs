#![allow(unused_imports)]
use proconio::input;
use std::cmp;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        H: [i64; N],
    }

    let mut dp = vec![0; N];
    for i in 1..N {
        let min = cmp::max(0, i as i64 - K as i64) as usize;

        dp[i] = dp[min..i]
            .iter()
            .enumerate()
            .map(|(j, &c)| c + (H[i] - H[j + min]).abs())
            .min()
            .unwrap();
    }

    println!("{}", dp[N - 1]);
}
