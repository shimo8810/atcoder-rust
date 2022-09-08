use proconio::{fastout, input};
use std::collections::HashMap;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
    }
    let mut tmax = 0;
    let mut A = HashMap::new();
    for _ in 0..N {
        input! {t: usize, x: usize, a: i64}
        A.insert((t, x), a);
        tmax = tmax.max(t);
    }

    let mut dp = vec![vec![std::i64::MIN; 5]; tmax + 1];
    dp[0][0] = 0;
    for t in 1..=tmax {
        for x in 0..5 {
            // 移動しないパターン
            let a = A.get(&(t, x)).unwrap_or(&0);
            dp[t][x] = dp[t - 1][x] + a;
            if x > 0 {
                dp[t][x] = dp[t][x].max(dp[t - 1][x - 1] + a);
            }
            if x < 4 {
                dp[t][x] = dp[t][x].max(dp[t - 1][x + 1] + a);
            }
        }
    }

    let ans = dp[tmax].iter().max().unwrap();
    println!("{}", ans);
}
