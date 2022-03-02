use proconio::input;
use proconio::marker::Chars;
use std::cmp::Ordering;
use std::collections::HashMap;

#[allow(non_snake_case)]
fn main() {
    input! {
        X: Chars,
        N: usize,
        mut S: [String; N]
    }
    let dict: HashMap<_, _> = X.iter().enumerate().map(|(i, &x)| (x, i)).collect();

    S.sort_by(|s, t| {
        for (x, y) in s.chars().zip(t.chars()) {
            let ord = dict[&x].cmp(&dict[&y]);
            if ord != Ordering::Equal {
                return ord;
            }
        }
        s.len().cmp(&t.len())
    });

    for s in S {
        println!("{}", s);
    }
}
