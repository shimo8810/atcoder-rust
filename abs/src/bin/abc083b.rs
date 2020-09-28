#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: u64,
        A: u64,
        B: u64,
    }

    let mut ans = 0;

    for mut n in 1..=N {
        let n_ = n;
        let mut sum = 0;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }
        if A <= sum && sum <= B {
            ans += n_;
        }
    }
    println!("{}", ans);
}
