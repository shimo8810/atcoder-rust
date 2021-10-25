use proconio::{fastout, input};
use std::collections::HashSet;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! { N: usize }

    let mut set = HashSet::new();
    for i in 1..=N {
        input! { s: String }
        if !set.contains(&s) {
            set.insert(s);
            println!("{}", i);
        }
    }
}
