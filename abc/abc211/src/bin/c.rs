use proconio::{fastout, input, marker::Chars};
use std::collections::HashMap;

const M: u64 = 1_000_000_007;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        S: Chars,
    }

    let map: HashMap<_, _> = "chokudai"
        .chars()
        .enumerate()
        .map(|(i, x)| (x, i + 1))
        .collect();
    let mut dp = vec![[0; 9]; S.len() + 1];
    dp[0][0] = 1;

    for i in 1..=S.len() {
        let c = S[i - 1];
        for j in 0..=8 {
            dp[i][j] = dp[i - 1][j] % M;
            if let Some(&n) = map.get(&c) {
                if j > 0 && n == j {
                    dp[i][j] = (dp[i][j] + dp[i - 1][j - 1]) % M;
                }
            }
        }
    }

    println!("{}", dp[S.len()][8]);
}
