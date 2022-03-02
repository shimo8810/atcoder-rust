use proconio::{fastout, input};
const M: u64 = 1_000_000_007;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      A: [[u64; 6]; N]
    }

    let ans: u64 = A
        .iter()
        .map(|v| v.iter().sum())
        .fold(1, |acc, x: u64| (acc * x) % M);

    println!("{}", ans);
}
