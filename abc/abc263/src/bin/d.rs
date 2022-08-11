use proconio::{fastout, input};

type Int = i64;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        L: Int,
        R: Int,
        A: [Int; N]
    }

    let mut head = vec![0; N + 1];
    let mut tail = vec![0; N + 1];

    for (i, &a) in A.iter().enumerate() {
        head[i + 1] = (head[i] + a).min(L * (i as Int + 1));
    }
    for (i, &a) in A.iter().rev().enumerate() {
        tail[i + 1] = (tail[i] + a).min(R * (i as Int + 1));
    }

    let mut ans = L * N as Int;
    for (h, t) in head.into_iter().zip(tail.into_iter().rev()) {
        ans = ans.min(h + t);
    }

    println!("{}", ans);
}
