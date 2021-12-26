use proconio::{fastout, input};
use std::cmp::min;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      H: [i32; N]
    }

    let mut dp = vec![0; N];
    dp[1] = (H[0] - H[1]).abs();

    for i in 2..N {
        dp[i] = min(
            dp[i - 1] + (H[i] - H[i - 1]).abs(),
            dp[i - 2] + (H[i] - H[i - 2]).abs(),
        );
    }

    println!("{}", dp[N - 1]);
}
