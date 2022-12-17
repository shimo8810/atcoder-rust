use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let mut W = vec![];
    let mut V = vec![];
    for _ in 0..N {
        input! {w: usize, v: usize}
        W.push(w);
        V.push(v);
    }
    let mut dp = vec![vec![0; K + 1]; N + 1];

    for i in 1..=N {
        for k in 0..=K {
            // 入れる場合
            if k >= W[i - 1] {
                dp[i][k] = dp[i - 1][k - W[i - 1]] + V[i - 1];
            }
            // 入れない場合
            dp[i][k] = dp[i][k].max(dp[i - 1][k]);
        }
    }

    let ans = dp[N].iter().max().unwrap();
    println!("{}", ans);
}
