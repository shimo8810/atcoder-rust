#![allow(unused_imports)]

use proconio::input;
use proconio::marker::Usize1;
use std::cmp;

#[allow(non_snake_case)]
fn main() {
    input! {
        X: i64,
    }

    for a in -128..=128i64 {
        for b in -128..=128i64 {
            if a.pow(5) - b.pow(5) == X {
                println!("{} {}", a, b);
                return;
            }
        }
    }
}
