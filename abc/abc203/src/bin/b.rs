use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,

    }

    let ans = K * (K + 1) * N / 2 + N * (N + 1) * K * 50;
    println!("{}", ans);
}
