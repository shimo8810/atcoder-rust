use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::collections::BTreeSet;

#[allow(non_snake_case)]
fn main() {
    input! {
        mut S: Chars,
        K: Usize1
    }

    let a: BTreeSet<String> = (0..S.len())
        .permutations(S.len())
        .map(|x| x.iter().map(|&i| S[i]).collect())
        .collect();
    println!("{}", a.iter().nth(K).unwrap());
}
