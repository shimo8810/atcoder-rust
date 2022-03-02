use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      W: usize,
    }

    // dp(i, w) := i番目まで選択, 重さがw以下ときの最大コスト
    let mut dp = vec![vec![0; W + 1]; N + 1];

    for i in 1..=N {
        input! {wi: usize, vi: usize}
        for w in 0..=W {
            if w < wi {
                // 大きすぎるときは入れない
                dp[i][w] = dp[i - 1][w];
            } else {
                // 入れる場合と入れない場合の最大
                dp[i][w] = dp[i - 1][w].max(dp[i - 1][w - wi] + vi);
            }
        }
    }
    println!("{}", dp[N][W]);
}
