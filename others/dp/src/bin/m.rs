use proconio::{fastout, input};

const M: u64 = 1_000_000_007;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N]
    }
    let mut dp = vec![vec![0; K + 1]; N + 1];
    dp[0][0] = 1;

    for i in 1..=N {
        let mut cum = vec![0; K + 2];
        for j in 1..(K + 2) {
            cum[j] = (cum[j - 1] + dp[i - 1][j - 1]) % M;
        }
        for j in 0..=K {
            dp[i][j] = (cum[j + 1] - cum[j.saturating_sub(A[i - 1])]) % M;
        }
    }

    println!("{}", dp[N][K]);
}
