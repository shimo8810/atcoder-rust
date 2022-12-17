use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        S: usize,
        A: [usize; N]
    }
    let mut dp = vec![vec![false; S + 1]; N + 1];

    dp[0][0] = true;

    for i in 1..=N {
        for k in 0..=S {
            // カードを追加する場合
            if k >= A[i - 1] && dp[i - 1][k - A[i - 1]] {
                dp[i][k] = true;
            }

            // カードを追加しない場合
            if dp[i - 1][k] {
                dp[i][k] = true;
            }
        }
    }

    let ans = if dp[N][S] { YES } else { NO };
    println!("{}", ans);
}
