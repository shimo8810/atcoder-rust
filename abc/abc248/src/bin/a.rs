use proconio::{fastout, input, marker::Chars};
use std::collections::HashSet;
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        S: Chars,
    }
    let set: HashSet<_> = S.into_iter().collect();
    for c in 0..10 {
        let c = (b'0' + c) as char;
        if !set.contains(&c) {
            println!("{}", c);
        }
    }
}
