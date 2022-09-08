use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [i64; N]
    }

    let mut cumsum = vec![0; N - M + 1];
    cumsum[0] = A.iter().take(M).sum();
    for i in 1..(N - M + 1) {
        cumsum[i] = cumsum[i - 1] - A[i - 1] + A[i + M - 1];
    }
    let mut prev = A
        .iter()
        .take(M)
        .enumerate()
        .fold(0, |p, (i, &x)| p + (i + 1) as i64 * x);
    let mut ans = prev;
    for i in 1..(N - M + 1) {
        // println!("{} {} {}", prev, A[i - 1 + M], cums)
        prev = prev + M as i64 * A[i - 1 + M] - cumsum[i - 1];
        ans = ans.max(prev);
    }
    println!("{}", ans);
}
