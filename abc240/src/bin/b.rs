use proconio::{fastout, input};
use std::collections::HashSet;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [u32; N]
    }
    let A: HashSet<_> = A.iter().collect();
    println!("{}", A.len());
}
