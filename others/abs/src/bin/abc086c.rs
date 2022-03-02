#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: u64,
        D: [[i64;3]; N],
    }

    let mut t = 0;
    let mut x = 0;
    let mut y = 0;

    for i in 0..N {
        let dt = (&D[i as usize])[0] - t;
        let dx = (&D[i as usize])[1] - x;
        let dy = (&D[i as usize])[2] - y;

        let dist = dx.abs() + dy.abs();
        if !(dist <= dt && dt % 2 == dist % 2) {
            println!("No");
            return;
        }

        // 更新
        t = (&D[i as usize])[0];
        x = (&D[i as usize])[1];
        y = (&D[i as usize])[2];
    }

    println!("Yes");
}
