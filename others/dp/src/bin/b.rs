use proconio::{fastout, input};
use std::cmp::min;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      K: usize,
      H: [i32; N]
    }

    let mut dp = vec![0; N];
    for i in 1..N {
        let l = i.saturating_sub(K);
        dp[i] = dp[i - 1] + (H[i] - H[i - 1]).abs();
        for j in l..i {
            dp[i] = min(dp[i], dp[j] + (H[i] - H[j]).abs());
        }
    }

    println!("{}", dp[N - 1]);
}
