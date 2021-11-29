use proconio::{fastout, input};
use std::cmp::min;
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      mut W: u64,
      mut AB: [(u64, u64); N]
    }
    AB.sort_unstable();

    let mut ans = 0;
    for &(a, b) in AB.iter().rev() {
        let k = min(W, b);
        ans += k * a;
        W -= k;
    }
    println!("{}", ans);
}
