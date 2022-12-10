use proconio::{fastout, input, marker::Chars};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        S: Chars,
    }

    let ans = if S.iter().skip(1).step_by(2).all(|c| c.is_ascii_uppercase())
        && S.iter().step_by(2).all(|c| c.is_ascii_lowercase())
    {
        YES
    } else {
        NO
    };
    println!("{}", ans);
}
