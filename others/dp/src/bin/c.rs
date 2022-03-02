use proconio::{fastout, input};
use std::cmp::max;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      ABC: [[u32; 3]; N]
    }

    let mut dp = vec![[0; 3]; N];

    for i in 0..3 {
        dp[0][i] = ABC[0][i];
    }

    for i in 1..N {
        for j in 0..3 {
            dp[i][j] = max(
                dp[i - 1][(j + 1) % 3] + ABC[i][j],
                dp[i - 1][(j + 2) % 3] + ABC[i][j],
            );
        }
    }

    let ans = dp[N - 1].iter().max().unwrap();
    println!("{}", ans);
}
