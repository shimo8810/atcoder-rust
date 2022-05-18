use proconio::{fastout, input};
use std::cmp::Reverse;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [u64; N]
    }

    let mut list: Vec<_> = A
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, Reverse(i)))
        .collect();
    list.sort_unstable();

    let mut j = -1;
    let mut ans = std::isize::MAX;
    for (_, Reverse(i)) in &list {
        let i = *i as isize;
        if i < K as isize {
            j = j.max(i);
        } else if j != -1 {
            ans = ans.min(i - j);
        }
    }
    let ans = if ans == std::isize::MAX { -1 } else { ans };
    println!("{}", ans);
}
