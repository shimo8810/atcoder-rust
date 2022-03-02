use num::integer::lcm;
use proconio::{fastout, input};

const L: u128 = 1_000_000_000_000_000_000;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      A: u128,
      B: u128
    }

    // let mut ans = 0;
    let ans = lcm(A, B);
    //
    if ans > L {
        println!("Large");
    } else {
        println!("{}", ans);
    }
}
