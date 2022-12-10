use proconio::{fastout, input};
use std::cmp::Ordering;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        Q: usize
    }

    let mut Z = vec![0; N + 1];
    for i in 0..N {
        Z[i + 1] = Z[i] + A[i];
    }

    for _ in 0..Q {
        input! {L: usize, R: usize}
        let s = R - L + 1;
        let a = Z[R] - Z[L - 1];

        let ans = match s.cmp(&(a * 2)) {
            Ordering::Equal => "draw",
            Ordering::Greater => "lose",
            Ordering::Less => "win",
        };

        println!("{}", ans);
    }
}
