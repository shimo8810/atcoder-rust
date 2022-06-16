use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        A: [Chars; H]
    }

    let mut dp = vec![vec![0i64; W + 1]; H + 1];

    for i in 1..W {
        if i % 2 == 0 {
            // 青木moveのときは反転
            dp[0][i] = dp[0][i - 1] + if A[0][i] == '+' { -1 } else { 1 };
        } else {
            // 高橋move
            dp[0][i] = dp[0][i - 1] + if A[0][i] == '+' { 1 } else { -1 };
        }
    }

    for i in 1..H {
        if i % 2 == 0 {
            // 青木moveのときは反転
            dp[i][0] = dp[i - 1][0] + if A[i][0] == '+' { -1 } else { 1 };
        } else {
            // 高橋move
            dp[i][0] = dp[i - 1][0] + if A[i][0] == '+' { 1 } else { -1 };
        }
    }

    for i in 1..H {
        for j in 1..W {
            if i + j % 2 == 0 {
                // 青木moveのときは最小になるように点を取る
                let x = if A[0][i] == '+' { -1 } else { 1 };
                dp[i][j] = (dp[i - 1][j] + x).min(dp[i][j - 1] + x);
            } else {
                // 高橋moveのときは最大になるように点を取る
                let x = if A[0][i] == '+' { 1 } else { -1 };
                dp[i][j] = (dp[i - 1][j] + x).max(dp[i][j - 1] + x);
            }
        }
    }

    let ans = match dp[H - 1][W - 1].cmp(&0) {
        std::cmp::Ordering::Equal => "Draw",
        std::cmp::Ordering::Greater => "Aoki",
        std::cmp::Ordering::Less => "Takahashi",
    };
    println!("{}", ans);
}
