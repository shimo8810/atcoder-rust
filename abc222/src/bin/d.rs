use proconio::input;

const M: usize = 998_244_353;
const MX: usize = 3_000;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        B: [usize; N]
    }

    // dp(i, j) := i番目までci = j 以下の総数の累積和
    // 解答はdp(N, MX)
    let mut dp = vec![vec![0usize; MX as usize + 1]; N + 1];
    dp[0][0] = 1;
    for i in 0..N {
        for j in 0..MX {
            dp[i + 1][j + 1] = dp[i + 1][j] % M;
            if A[i] <= j && j <= B[i] {
                dp[i + 1][j + 1] = dp[i][j + 1] % M;
            }
        }
    }

    println!("{}", dp[N][MX]);
}
