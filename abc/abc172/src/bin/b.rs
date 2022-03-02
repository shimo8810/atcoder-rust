use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    //
    input! {
        s: Chars,
        t: Chars,
    }

    let sum = s
        .iter()
        .zip(t.iter())
        .fold(0, |acc, (a, b)| if a == b { acc } else { acc + 1 });
    println!("{}", sum);
}
