use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [Usize1; N - 1],
        B: [Usize1; N - 1]
    }

    let mut dp = vec![0; N];
    dp[0] = 0;
    for i in 0..(N - 1) {
        dp[A[i]] = dp[A[i]].max(dp[i] + 100);
        dp[B[i]] = dp[B[i]].max(dp[i] + 150);
    }
    let ans = dp[N - 1];
    println!("{}", ans);
}
