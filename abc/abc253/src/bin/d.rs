use num::integer::lcm;
use proconio::{fastout, input};

fn s(a: u64, b: u64, n: u64) -> u64 {
    (a + b) * n / 2
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: u64,
        A: u64,
        B: u64
    }

    // aの倍数
    let p = if N >= A { (N - A) / A + 1 } else { 0 };
    let q = if N >= B { (N - B) / B + 1 } else { 0 };
    let C = lcm(A, B);
    let r = if N >= C { (N - C) / C + 1 } else { 0 };
    let ans = s(1, N, N) - s(A, A * p, p) - s(B, B * q, q) + s(C, C * r, r);
    println!("{}", ans);
}
