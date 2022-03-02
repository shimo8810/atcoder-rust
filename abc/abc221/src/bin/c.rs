use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: Chars
    }
    let mut ans = 0;
    for perm in N.iter().permutations(N.len()).unique() {
        for i in 1..perm.len() {
            let a = &perm[..i];
            let b = &perm[i..];
            if *a[0] == '0' || *b[0] == '0' {
                break;
            }
            let a = a.iter().enumerate().fold(0, |acc, (i, &&x)| {
                acc + 10u32.pow((a.len() - i - 1) as u32) * (x as u32 - 48)
            });
            let b = b.iter().enumerate().fold(0, |acc, (i, &&x)| {
                acc + 10u32.pow((b.len() - i - 1) as u32) * (x as u32 - 48)
            });
            ans = cmp::max(ans, a * b);
        }
    }
    println!("{}", ans);
}
