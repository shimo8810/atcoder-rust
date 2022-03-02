use num::integer::gcd;
use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        A: u64,
        B: u64,
        C: u64
    }
    let s = gcd(gcd(A, B), C);
    let ans = (A / s - 1) + (B / s - 1) + (C / s - 1);
    println!("{}", ans);
}
