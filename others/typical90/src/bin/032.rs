use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};
use std::cmp::min;
// use std::collections::{HashMap, HashSet};

const MAX: u32 = 100001;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      A: [[u32; N]; N],
      M: usize
    }
    let mut rel = vec![vec![false; N]; N];

    for _ in 0..M {
        input! {x: Usize1, y: Usize1}
        rel[x][y] = true;
        rel[y][x] = true;
    }

    let mut ans = MAX;

    'perm: for p in (0..N).permutations(N) {
        // let perm = perm;
        for i in 0..N {
            if i < N - 1 && rel[p[i]][p[i + 1]] {
                continue 'perm;
            }
        }
        let t = p.iter().enumerate().fold(0, |acc, (i, &j)| acc + A[j][i]);
        ans = min(t, ans);
    }
    if ans == MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
