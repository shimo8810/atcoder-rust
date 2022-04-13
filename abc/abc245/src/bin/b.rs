use proconio::{fastout, input};
use std::collections::HashSet;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [u32; N]
    }

    let mut set = HashSet::new();
    for &a in &A {
        set.insert(a);
    }
    for ans in 0..=2000 {
        if !set.contains(&ans) {
            println!("{}", ans);
            break;
        }
    }
}
