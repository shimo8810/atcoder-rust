use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [i64; N]
    }

    let mut dp = vec![vec![0; M + 1]; N + 1];
    dp[0][0] = 0;
    dp[0][1] = std::i64::MIN;


    for i in 1..=N {
        for k in 0..=M {
            if k == 0 {
                dp[i][0] = 0;
            } else if k > i {
                dp[i][k] = std::i64::MIN;
            } else {
                dp[i][k] = dp[i - 1][k].max(dp[i - 1][k - 1] + k as i64 * A[i - 1]);
            }
        }
    }
    println!("{}", dp[N][M]);
}
