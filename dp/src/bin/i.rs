use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      P: [f64; N]
    }

    // 確率DP
    let mut dp = vec![vec![0.0; N + 1]; N + 1];
    dp[0][0] = 1.0;

    for i in 1..=N {
        for k in 0..=N {
            // 裏を出すとき
            dp[i][k] = dp[i - 1][k] * (1.0 - P[i - 1]);
            if k > 0 {
                // 表を出すとき
                dp[i][k] += dp[i - 1][k - 1] * P[i - 1];
            }
        }
    }
    let ans: f64 = dp[N][(N / 2 + 1)..].iter().sum();
    println!("{}", ans);
}
