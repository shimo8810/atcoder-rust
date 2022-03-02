use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let ans = s
        .iter()
        .fold(0, |acc, &c| if c == '1' { acc + 1 } else { acc });
    println!("{}", ans);
}
