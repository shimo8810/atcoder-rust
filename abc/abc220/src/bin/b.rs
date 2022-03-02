use proconio::input;
use proconio::marker::Chars;

#[allow(non_snake_case)]
fn main() {
    input! {
        K: u64,
        mut A: Chars,
        mut B: Chars
    }

    let a: u64 = A
        .iter()
        .enumerate()
        .map(|(i, &x)| (x as u64 - 48) * K.pow((A.len() - i - 1) as u32))
        .sum();

    let b: u64 = B
        .iter()
        .enumerate()
        .map(|(i, &x)| (x as u64 - 48) * K.pow((B.len() - i - 1) as u32))
        .sum();

    let ans = a * b;

    println!("{}", ans);
}
