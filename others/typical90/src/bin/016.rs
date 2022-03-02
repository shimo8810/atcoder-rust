use proconio::{fastout, input};
use std::cmp::min;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
        c: i64
    }

    let mut ans = 10000;
    for i in 0..=9999 {
        for j in 0..=(9999 - i) {
            let d = n - i * a - j * b;
            let k = i + j + d / c;
            if d % c != 0 || d < 0 || k > 9999 {
                continue;
            }
            ans = min(ans, k);
        }
    }
    // for i in 0..=(n / a) {
    //     for j in 0..=((n - a * i) / b) {
    //         let d = n - a * i - b * j;
    //         let k = i + j + d / c;
    //         if d % c != 0 || k > 9999 {
    //             continue;
    //         }
    //         ans = min(ans, i + j + d / c);
    //     }
    // }

    println!("{}", ans);
}
