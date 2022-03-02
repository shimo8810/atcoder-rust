use proconio::{fastout, input};
use std::collections::HashSet;

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: u32,
        X: u32
    }

    let mut set = HashSet::new();
    set.insert(0);

    for _ in 0..N {
        input! { a: u32, b: u32 }
        set = set
            .iter()
            .map(|&x| x + a)
            .filter(|&x| x <= X)
            .chain(set.iter().map(|&x| x + b).filter(|&x| x <= X))
            .collect();
    }

    let ans = if set.contains(&X) { YES } else { NO };
    println!("{}", ans);
}
