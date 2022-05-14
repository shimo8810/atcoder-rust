use proconio::{fastout, input};
use std::cmp::Reverse;
use std::collections::HashSet;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
    }

    let mut list = vec![];
    let mut set = HashSet::new();

    for i in 0..N {
        input! {s: String, t: u32}
        if !set.contains(&s) {
            list.push((Reverse(t), i + 1));
            set.insert(s);
        }
    }
    list.sort_unstable();
    let (_, ans) = list[0];

    println!("{}", ans);
}
