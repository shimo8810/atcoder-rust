#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [u64; N],
    }

    let sum = A.iter().fold(0, |sum, i| sum ^ i);

    for a in A.iter() {
        print!("{} ", a ^ sum);
    }
}
