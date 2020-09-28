#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        X: i64,
        Y: i64,
    }
    let m = (Y - 2 * X) / 2;
    if Y % 2 == 0 && 0 <= m && m <= X {
        println!("Yes");
    } else {
        println!("No");
    }
}
