#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        A: u64,
        B: u64,
        C: u64,
        X: u64,
    }

    let mut ans = 0;
    for a in 0..=A {
        for b in 0..=B {
            if 500 * a + 100 * b > X {
                break;
            }
            for c in 0..=C {
                match 500 * a + 100 * b + 50 * c {
                    x if x == X => ans += 1,
                    x if x > X => break,
                    _ => {}
                }
            }
        }
    }

    println!("{}", ans);
}
