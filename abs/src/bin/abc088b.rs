#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: u64,
        mut A: [u64; N],
    }

    A.sort_by(|a, b| b.cmp(a));
    let ans: u64 = A.iter().step_by(2).sum::<u64>() - A.iter().skip(1).step_by(2).sum::<u64>();
    println!("{:?}", ans);
}
