use proconio::input;
use std::collections::HashSet;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: [String; N],
    }
    let mut set = HashSet::new();
    for s in S {
        set.insert(s);
    }

    println!("{}", set.len());
}
