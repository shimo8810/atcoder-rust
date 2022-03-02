use proconio::{fastout, input};

const V: usize = 100_000;
const WMAX: usize = 1_000_000_001; // 最大値よりも大きい数値を指定

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      W: usize
    }

    let mut dp = vec![vec![WMAX; V + 1]; N + 1];
    dp[0][0] = 0;
    for i in 1..=N {
        input! {wi: usize, vi: usize}
        for v in 0..=V {
            if v < vi {
                // 大きすぎるときは入れない
                dp[i][v] = dp[i - 1][v];
            } else {
                // 入れる場合と入れない場合の最大
                dp[i][v] = dp[i - 1][v].min(dp[i - 1][v - vi] + wi);
            }
        }
    }

    let mut ans = 0;

    for (v, &w) in dp[N].iter().enumerate() {
        if w <= W {
            ans = ans.max(v);
        }
    }
    println!("{}", ans);
}
