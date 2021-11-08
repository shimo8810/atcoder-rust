use proconio::{fastout, input};
use std::collections::HashSet;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
    }
    let mut set = HashSet::new();

    for _ in 0..N {
        input! {
            L: usize,
            A: [u32; L]
        }
        set.insert(A);
    }
    println!("{}", set.len());
}
