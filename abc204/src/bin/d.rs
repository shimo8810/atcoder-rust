use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }
    let S: usize = A.iter().sum();
    let mut dp = vec![vec![0; S + 1]; N + 1];

    for i in 0..N {
        for t in 0..=S {
            dp[i + 1][t] = dp[i][t] + A[i];
            if t >= A[i] {
                dp[i + 1][t] = dp[i + 1][t].min(dp[i][t - A[i]]);
            }
        }
    }
    let mut ans = std::usize::MAX;
    for t in 0..=S {
        ans = dp[N][t].max(t).min(ans);
    }

    println!("{}", ans);
}
