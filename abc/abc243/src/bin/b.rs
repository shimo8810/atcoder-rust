use proconio::{fastout, input};
use std::collections::HashMap;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [u32; N],
        B: [u32; N]
    }

    let ans = A.iter().zip(B.iter()).filter(|(&a, &b)| a == b).count();
    println!("{}", ans);
    let map: HashMap<_, _> = B.into_iter().enumerate().map(|(i, x)| (x, i)).collect();

    let ans = A
        .iter()
        .enumerate()
        .filter(|(i, &x)| map.get(&x).unwrap_or(i) != i)
        .count();
    println!("{}", ans);
}
