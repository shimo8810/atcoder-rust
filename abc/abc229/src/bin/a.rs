use proconio::{fastout, input, marker::Chars};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      s1: Chars,
      s2: Chars
    }
    let ans = if (s1[0] == '#' && s2[1] == '#' && s1[1] == '.' && s2[0] == '.')
        || (s1[1] == '#' && s2[0] == '#' && s1[0] == '.' && s2[1] == '.')
    {
        NO
    } else {
        YES
    };
    println!("{}", ans);
}
