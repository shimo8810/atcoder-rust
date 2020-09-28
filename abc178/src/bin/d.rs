use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        S:usize,
    }

    if S < 3 {
        println!("0");
        return;
    }

    let m = 1000_000_000 + 7u64;
    let mut dp = vec![0; S + 1];

    for i in 3..=S {
        dp[i] += 1;
        for j in 3..=i {
            dp[i] = (dp[i] + dp[i - j]) % m;
        }
    }

    println!("{}", dp[S] % m);
}
