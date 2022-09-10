use proconio::{fastout, input};
use std::collections::HashMap;

// 6 3
// 2 7 1 8 2 8
// 2 10
// 3 1
// 5 5

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        X: [u64; N],
    }
    let mut bonus = HashMap::new();
    for _ in 0..M {
        input! {C: usize, Y: u64}
        bonus.insert(C, Y);
    }
    // dp(i, c) := i回コインを投げたとき、連続回数がcである最大値
    let mut dp = vec![vec![0u64; N + 1]; N + 1];

    for i in 1..=N {
        // コインが裏のとき
        dp[i][0] = *dp[i - 1].iter().max().unwrap();
        for c in 1..=i {
            dp[i][c] = dp[i - 1][c - 1] + X[i - 1] + bonus.get(&c).unwrap_or(&0);
        }
    }

    let ans = dp[N].iter().max().unwrap();
    println!("{}", ans);
}
