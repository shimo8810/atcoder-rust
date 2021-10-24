use proconio::input;
use std::cmp;

#[allow(non_snake_case)]
fn main() {
    input! {
        S: String
    }

    let mut max = S.clone();
    let mut min = S.clone();

    for i in 0..S.len() {
        let a = S[i..].to_string() + &S[..i];
        max = cmp::max(a.clone(), max);
        min = cmp::min(a, min);
    }
    println!("{}", min);
    println!("{}", max);
}
