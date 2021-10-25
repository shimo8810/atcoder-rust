use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: u64,
        A: [i32; N],
        B: [i32; N]
    }
    let s: u64 = A
        .iter()
        .zip(B.iter())
        .map(|(&a, &b)| (a - b).abs() as u64)
        .sum();
    let ans = if s > K || (K - s) % 2 != 0 { NO } else { YES };
    println!("{}", ans);
}
