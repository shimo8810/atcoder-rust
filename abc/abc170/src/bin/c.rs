#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        X: i32,
        N: usize,
        P: [i32; N],
    }

    for i in 0..=100 {
        // -i が含まれているか
        for s in &[-1, 1] {
            let mut ans = X + s * i;
            for &p in P.iter() {
                if ans == p {
                    ans = -1;
                    break;
                }
            }
            if ans >= 0 {
                println!("{}", ans);
                return;
            }
        }
    }
}
