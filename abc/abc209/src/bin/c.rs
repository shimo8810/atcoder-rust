use proconio::{fastout, input};

const D: i64 = 1_000_000_007;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        mut C: [i64; N]
    }
    C.sort_unstable();
    let mut ans = 1;

    for (i, c) in C.into_iter().enumerate() {
        ans = (ans * (c - i as i64).max(0)) % D;
    }
    println!("{}", ans);
}
