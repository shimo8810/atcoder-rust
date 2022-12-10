use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: u64,
        A: [u64; N]
    }

    let mut ans = 0;

    let mut r = 0;
    for l in 0..(N - 1) {
        while r < N - 1 && A[r + 1] - A[l] <= K {
            r += 1;
        }

        ans += r - l;
    }
    println!("{}", ans);
}
