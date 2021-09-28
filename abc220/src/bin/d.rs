use proconio::input;

const M: usize = 998244353;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let mut dp = vec![vec![0; 10]; N];

    dp[0][A[0]] = 1;

    for (i, &x) in A.iter().enumerate().skip(1) {
        for j in 0..10 {
            let s = (j + x) % 10;
            let p = (j * x) % 10;
            dp[i][s] = (dp[i][s] + dp[i - 1][j]) % M;
            dp[i][p] = (dp[i][p] + dp[i - 1][j]) % M;
        }
    }

    for i in 0..10 {
        println!("{}", dp[N - 1][i]);
    }
}
