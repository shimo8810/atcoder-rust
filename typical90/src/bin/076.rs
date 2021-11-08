use proconio::{fastout, input};
use std::iter;

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      A: [u64; N]
    }

    let S = A.iter().sum::<u64>() / 10;

    if S == 0 {
        println!("{}", NO);
        return;
    }

    let A: Vec<_> = iter::repeat(A).take(2).flatten().collect();

    let mut sum = 0;
    let mut h = 0;
    let mut t = 0;
    let ans = loop {
        while t < 2 * N && sum < S {
            sum += A[t];
            // println!("{}-{}", h, t);
            t += 1;
        }

        if sum == S {
            break YES;
        } else {
            sum -= A[h];
            h += 1;
        }

        if h == A.len() - 1 {
            break NO;
        }
    };

    println!("{}", ans);
}
