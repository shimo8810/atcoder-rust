use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        S: Chars,
        T: Chars
    }
    // dp(i, j) := Sをi文字までTをj文字まで見たときのLCS
    let mut dp = vec![vec![0; T.len() + 1]; S.len() + 1];

    for (i, &s) in S.iter().enumerate() {
        for (j, &t) in T.iter().enumerate() {
            if s == t {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }

    let mut len = dp[S.len()][T.len()];
    let mut ans = vec![];
    let mut i = S.len();
    let mut j = T.len();

    while len > 0 {
        if S[i - 1] == T[j - 1] {
            ans.push(S[i - 1]);
            i -= 1;
            j -= 1;
            len -= 1;
        } else if dp[i][j] == dp[i][j - 1] {
            j -= 1;
        } else {
            i -= 1;
        }
    }

    let ans: String = ans.iter().rev().collect();
    println!("{}", ans);
}
