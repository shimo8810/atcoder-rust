use proconio::input;

#[allow(unused_mut)]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: usize,
        A: [usize; N]
    }

    let mut dp: Vec<Vec<u64>> = vec![vec![0; S + 1]; N + 1];
    dp[0][0] = 1;
    let m = 998244353;

    for i in 1..=N {
        for j in 0..=S {
            // 入れないグループ
            dp[i][j] = (dp[i][j] + dp[i - 1][j] * 2) % m;

            // A[i]を追加するパターン
            if j + A[i - 1] <= S {
                dp[i][j + A[i - 1]] = (dp[i][j + A[i - 1]] + dp[i - 1][j]) % m;
            }
        }
    }

    println!("{}", dp[N][S]);
}
