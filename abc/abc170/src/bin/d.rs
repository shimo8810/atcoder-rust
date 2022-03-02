#![allow(non_snake_case)]

use proconio::input;

const MAX_A: u64 = 1e6 as u64;
fn main() {
    input! {
        N: usize,
        mut A: [u64; N],
    }
    let mut arr = [0; MAX_A as usize + 1];

    let mut ans: u64 = 0;

    for &a in A.iter() {
        if arr[a as usize] == 1 {
            arr[a as usize] = 2;
        } else if arr[a as usize] == 0 {
            arr[a as usize] = 1;
            for i in ((2 * a)..=MAX_A).step_by(a as usize) {
                arr[i as usize] += 2;
            }
        }
    }

    for &a in A.iter() {
        if arr[a as usize] == 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
