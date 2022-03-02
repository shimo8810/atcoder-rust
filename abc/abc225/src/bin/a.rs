use proconio::{fastout, input, marker::Chars};
use std::collections::HashSet;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      S: Chars,
    }

    let ans = match S.iter().collect::<HashSet<_>>().len() {
        1 => 1,
        2 => 3,
        _ => 6,
    };
    println!("{}", ans);
}
