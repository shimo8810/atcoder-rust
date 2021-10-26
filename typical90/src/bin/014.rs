use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      mut A: [i64; N],
      mut B: [i64; N]
    }

    A.sort_unstable();
    B.sort_unstable();
    let ans = A
        .iter()
        .zip(B.iter())
        .fold(0, |acc, (a, b)| (a - b).abs() + acc);
    println!("{}", ans);
}
