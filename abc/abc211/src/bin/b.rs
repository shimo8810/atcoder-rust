use proconio::{fastout, input};
use std::collections::HashSet;

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        S: [String; 4],
    }
    let S: HashSet<_> = S.into_iter().collect();
    let ans = if S.contains(&"H".to_string())
        && S.contains(&"2B".to_string())
        && S.contains(&"3B".to_string())
        && S.contains(&"HR".to_string())
    {
        YES
    } else {
        NO
    };
    println!("{}", ans);
}
