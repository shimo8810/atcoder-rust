use proconio::{fastout, input};
use std::cmp::Reverse;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
    }
    let mut list = vec![];
    for _ in 0..N {
        input! {S: String, T: u32}
        list.push((Reverse(T), S));
    }
    list.sort_unstable();
    println!("{}", list[1].1);
}
