use proconio::{fastout, input, marker::Chars};

const M: u64 = 1_000_000_007;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      H: usize,
      W: usize,
      A: [Chars; H]
    }
    let mut dp = vec![vec![0; W]; H];
    dp[0][0] = 1;

    // グリッド上のDP
    for h in 0..H {
        for w in 0..W {
            if A[h][w] == '#' {
                continue;
            }

            if h >= 1 && A[h - 1][w] == '.' {
                dp[h][w] = (dp[h][w] + dp[h - 1][w]) % M;
            }
            if w >= 1 && A[h][w - 1] == '.' {
                dp[h][w] = (dp[h][w] + dp[h][w - 1]) % M;
            }
        }
    }

    println!("{}", dp[H - 1][W - 1]);
}
