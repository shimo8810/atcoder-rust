#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: u64,
        Y: u64,
    }
    for a in 0..=N {
        if a * 10000 > Y {
            break;
        }
        for b in 0..=(N - a) {
            let c = N - a - b;
            match 10000 * a + 5000 * b + 1000 * c {
                y if y > Y => break,
                y if y == Y => {
                    println!("{} {} {}", a, b, c);
                    return;
                }
                _ => {}
            }
        }
    }

    println!("-1 -1 -1");
}
