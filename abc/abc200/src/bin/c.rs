use proconio::{fastout, input};
use std::collections::HashMap;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [i64; N]
    }
    let M = 200;
    let mut map = HashMap::new();
    for &a in &A {
        *map.entry(a % M).or_insert(0) += 1;
    }

    let ans: i64 = map.iter().map(|(_, &n)| n * (n - 1) / 2).sum();
    println!("{}", ans);
}
