use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      K: usize,
    }

    let mut s = Vec::new();
    for _ in 0..N {
        input! {a: u64, b: u64}
        s.push(a - b);
        s.push(b)
    }
    s.sort_unstable();
    let ans: u64 = s.iter().rev().take(K).sum();
    println!("{}", ans);
}
