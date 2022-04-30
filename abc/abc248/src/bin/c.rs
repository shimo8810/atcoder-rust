use proconio::{fastout, input};

const P: usize = 998244353;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize
    }

    let mut dp = vec![vec![0usize; K + 1]; N + 1];
    dp[0][0] = 1;
    for i in 0..N {
        for j in 0..K {
            for k in 1..=M {
                if j + k <= K {
                    dp[i + 1][j + k] = (dp[i + 1][j + k] + dp[i][j]) % P;
                }
            }
        }
    }
    let ans: usize = dp[N].iter().fold(0, |acc, x| (acc + x) % P);
    println!("{}", ans);
}

// 1 <= Ai <= M
// Sum{Ai} <= K
// 1 < N, M < 50
// N < K < NM
// dp(i, j) := i番目まで読み込んだときの(1< i < M)値がj(0 < j < K)のときの個数
// dp(i + 1, j) = Sum {dp(i, x)} i <= x < j
//
