use proconio::{fastout, input};

const D: usize = 998244353;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let mut ans = 0;
    // x個選択するのを全通りで試す
    for x in 1..=N {
        // i番目まで見たとき、k個を選択したとき、余りがrであるような組
        let mut dp = vec![vec![vec![0usize; x]; x + 1]; N + 1];
        dp[0][0][0] = 1;
        for (i, &a) in A.iter().enumerate() {
            for k in 0..=x {
                for r in 0..x {
                    // i項目を入れない場合
                    dp[i + 1][k][r] = (dp[i + 1][k][r] + dp[i][k][r]) % D;
                    // i項目を入れる場合
                    if k != x {
                        dp[i + 1][k + 1][(r + a) % x] =
                            (dp[i + 1][k + 1][(r + a) % x] + dp[i][k][r]) % D;
                    }
                }
            }
        }
        ans = (ans + dp[N][x][0]) % D;
    }

    println!("{}", ans);
}
