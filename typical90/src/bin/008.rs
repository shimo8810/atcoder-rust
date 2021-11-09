use proconio::{fastout, input, marker::Chars};

const M: u64 = 1_000_000_007;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      S: Chars
    }

    let T: Vec<_> = "atcoder".chars().collect();

    let mut dp = vec![vec![0; 8]; N + 1];
    dp[0][0] = 1;
    for (i, c) in S.iter().enumerate() {
        for j in 0..8 {
            dp[i + 1][j] = (dp[i + 1][j] + dp[i][j]) % M;
        }

        for (j, d) in T.iter().enumerate() {
            if c == d {
                dp[i + 1][j + 1] = (dp[i + 1][j + 1] + dp[i][j]) % M;
            }
        }
    }

    println!("{}", dp[N][7]);
}
