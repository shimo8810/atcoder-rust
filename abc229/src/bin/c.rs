use proconio::{fastout, input};

use std::cmp::max;
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      W: usize,
      AB: [(usize, usize); N]
    }

    

    println!("{}", dp[N][W]);
}
