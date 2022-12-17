use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        S: Chars,
        T: Chars
    }

    let N = S.len();
    let M = T.len();
    let mut dp = vec![vec![0; M + 1]; N + 1];

    for i in 1..=N {
        for j in 1..=M {
            dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            if S[i - 1] == T[j - 1] {
                dp[i][j] = dp[i][j].max(dp[i - 1][j - 1] + 1);
            }
        }
    }

    println!("{}", dp[N][M]);
}
