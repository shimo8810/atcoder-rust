#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: u64,
        mut D: [u64; N],
    }
    D.dedup();
    println!("{:?}", D.len());
}
