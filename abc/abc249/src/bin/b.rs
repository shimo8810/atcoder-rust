use proconio::{fastout, input, marker::Chars};
use std::collections::HashSet;

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        S: Chars
    }
    let capitals: HashSet<_> = ('A'..'Z').collect();
    let smalls: HashSet<_> = ('a'..'z').collect();

    let ans = if S.iter().collect::<HashSet<&char>>().len() == S.len()
        && S.iter().any(|c| capitals.contains(c))
        && S.iter().any(|c| smalls.contains(c))
    {
        YES
    } else {
        NO
    };

    println!("{}", ans);
}
