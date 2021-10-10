use proconio::input;
use std::cmp;
const M: usize = 998_244_353;
const MX: usize = 3_000;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        B: [usize; N]
    }

    // 前回の
    let mut dp = vec![vec![0usize; MX as usize + 1]; N + 1];

    for i in 0..N {
        let a = A[i];
        let b = B[i];
        for c in 0..=MX {
            let n = dp[i][c];
            let x = cmp::min(c, a as usize);
            for y in x..=b {
                dp[i + 1][y] = (n + dp[i + 1][y] + 1) % M;
            }
        }
    }

    let mut ans = 0;
    for x in dp[N - 1].iter() {
        ans = (ans + x) % M;
    }
    if N < 100 {
        println!("{:?}", dp);
    }
}
