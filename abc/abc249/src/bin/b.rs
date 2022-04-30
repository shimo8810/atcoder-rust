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
    
    let smalls: HashSet<_> = (b'a'..b'z').map(|x| x as char).collect();
    let capitals: HashSet<_> = (b'A'..b'Z').map(|x| x as char).collect();

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
