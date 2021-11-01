use proconio::{fastout, input};

const M: u64 = 1_000_000_007;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      L: usize
    }

    let mut dp = vec![0; N + 1];
    dp[0] = 1;
    for i in 1..=N {
        dp[i] = (dp[i - 1] + if L <= i { dp[i - L] } else { 0 }) % M;
    }
    println!("{}", dp[N]);
}
