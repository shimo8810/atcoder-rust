use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; K]
    }

    let mut dp = vec![0; N + 1];
    for i in 0..=N {
        dp[i] = A
            .iter()
            .take_while(|&&a| a <= i)
            .map(|&a| a + (i - a) - dp[i - a])
            .max()
            .unwrap_or(0);
    }

    println!("{}", dp[N]);
}
