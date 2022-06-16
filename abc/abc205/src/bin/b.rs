use proconio::{fastout, input};
use std::collections::HashSet;

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        mut A: [u32; N]
    }
    let A: HashSet<_> = A.into_iter().collect();
    let ans = if A.len() == N { YES } else { NO };
    println!("{}", ans);
}
