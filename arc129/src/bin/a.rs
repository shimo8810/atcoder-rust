use proconio::{fastout, input};
use std::cmp::{max, min};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      mut N: usize,
      L: usize,
      R: usize
    }

    let mut z = 1;
    let mut ans = 0;
    while N > 0 {
        if N & 0x1 == 0x1 {
            if min(R, z * 2 - 1) + 1 >= max(L, z) {
                ans += min(R, z * 2 - 1) - max(L, z) + 1;
            }
        }
        z <<= 1;
        N >>= 1;
    }
    println!("{}", ans);
}
