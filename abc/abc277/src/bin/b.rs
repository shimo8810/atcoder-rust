use proconio::{fastout, input, marker::Chars};
use std::collections::HashSet;

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        S: [Chars; N]
    }

    let suit: HashSet<_> = "HDCS".chars().collect();
    let nums: HashSet<_> = "A23456789TJQK".chars().collect();

    let set: HashSet<_> = S.iter().collect();
    let ans = if set.len() == S.len()
        && S.iter()
            .all(|c| suit.contains(&c[0]) && nums.contains(&c[1]))
    {
        YES
    } else {
        NO
    };

    println!("{}", ans);
}
