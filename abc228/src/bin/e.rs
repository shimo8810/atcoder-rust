use proconio::{fastout, input};

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
      N: u64,
      K: u64,
      M: u64
    }

    const P: u64 = 998244353;

    let r = powmod(K, N, P - 1);
    let ans = if M % P == 0 { 0 } else { powmod(M, r, P) };
    println!("{}", ans);
}
