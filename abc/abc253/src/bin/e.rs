use proconio::{fastout, input};

const D: u64 = 998244353;

/// モジュロ演算付き繰り返し二乗法
fn powmod(x: u64, mut n: u64, m: u64) -> u64 {
    let mut ret = 1;
    let mut x = x % m;
    while n > 0 {
        if n & 1 == 1 {
            ret = (ret * x) % m;
        }
        x = (x * x) % m;
        n >>= 1;
    }
    ret
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize
    }

    if K == 0 {
        println!("{}", powmod(M as u64, N as u64 / 2, D));
        return;
    }
    let mut dp = vec![vec![0; M + 1]; N + 1];

    for i in 1..=M {
        dp[1][i] = i as u64;
    }
    for i in 2..=N {
        for x in 1..=M {
            dp[i][x] = (dp[i][x] + dp[i][x - 1]) % D;
            if x >= K {
                dp[i][x] = (dp[i][x] + dp[i - 1][x - K]) % D;
            }
            dp[i][x] = (dp[i][x] + dp[i - 1][M] + D - dp[i - 1][M.min(x - 1 + K)]) % D;
        }
    }
    println!("{}", dp[N][M]);
}
