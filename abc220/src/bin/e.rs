use proconio::input;

const M: u64 = 998244353;

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
fn main() {
    input! {
        N: u64,
        D: u64
    }

    let mut ans = 0;

    for i in 0..(N - 1) {
        if i + D <= N - 1 {
            ans += powmod(2, D, M);
        }
    }
    println!("{}", ans);
}
