use proconio::{fastout, input};

const F: &str = "First";
const S: &str = "Second";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N]
    }
    // コマの取り合いゲーム
    // dp(i) := 自分が選択する前の勝敗判定(trueで勝利)
    // ans = dp(K), 先手が選択する前の勝敗判定
    // dp(i) = dp(i - a)の中で1つでも敗北があれば勝利
    // dp(i) = Sum(!dp(i - a))
    let mut dp = vec![false; K + 1];

    let &min = A.iter().min().unwrap();
    for i in 0..=K {
        if i < min {
            dp[i] = false;
        } else {
            for &a in A.iter() {
                if i >= a {
                    dp[i] |= !dp[i - a];
                }
            }
        }
    }

    let ans = if dp[K] { F } else { S };
    println!("{}", ans);
}
