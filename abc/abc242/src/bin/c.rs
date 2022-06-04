use proconio::{fastout, input};

const M: i32 = 998244353;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
    }

    let mut dp = vec![vec![0i32; 10]; N];
    for i in 1..10 {
        dp[0][i] = 1;
    }
    for i in 1..N {
        for j in 1..10 {
            for k in &[-1isize, 0, 1] {
                if j + k >= 0 && j + k < 10 {
                    dp[i][j as usize] = (dp[i][j as usize] + dp[i - 1][(j + k) as usize]) % M;
                }
            }
        }
    }

    let mut ans = 0;
    for x in &dp[N - 1] {
        ans = (ans + x) % M;
    }
    println!("{}", ans);
}
