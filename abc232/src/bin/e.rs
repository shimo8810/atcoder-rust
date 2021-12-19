use proconio::{fastout, input, marker::Usize1};

const M: usize = 998244353;

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
        H: usize,
        W: usize,
        mut K: usize,
        x1: Usize1,
        y1: Usize1,
        x2: Usize1,
        y2: Usize1
    }

    let mut ans = 0;
    if K == 1 {
        if x1 == x2 || y1 == y2 {
            println!("{}", 1);
        } else {
            println!("{}", 0);
        }
        return;
    }

    let a = K / 2;
    let b = K % 2;
    
    println!("{}", ans);
}
