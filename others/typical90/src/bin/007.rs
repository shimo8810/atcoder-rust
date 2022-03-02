use proconio::{fastout, input};
use std::cmp::min;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      mut A: [i32; N],
      Q: usize,
      B: [i32; Q]
    }
    A.sort_unstable();

    for &b in B.iter() {
        let mut left = 0;
        let mut right = N;
        let mut c = N / 2;
        loop {
            if left == c {
                break;
            }

            if A[c] < b {
                left = c;
                c = (c + right) / 2;
            } else {
                right = c;
                c = (c + left) / 2;
            }
        }

        let ans = min((A[c] - b).abs(), (A[min(c + 1, N - 1)] - b).abs());
        println!("{}", ans);
    }
}
