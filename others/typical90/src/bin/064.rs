use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      Q: usize,
      A: [i32; N]
    }

    // println!("{:?}", A);

    let mut diff: Vec<_> = A.iter().tuple_windows().map(|(a, b)| b - a).collect();
    let mut ans = diff.iter().fold(0, |acc, d| acc + d.abs());

    for _ in 0..Q {
        input! {l: usize, r: usize, v: i32}

        if l > 0 && l + 1 < N {
            diff[l] += v;
        }
        // if r + 1 < N {
        //     diff[r] += v;
        // }

        println!("{:?}", diff);
    }
}
