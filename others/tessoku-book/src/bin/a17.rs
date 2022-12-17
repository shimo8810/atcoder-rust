use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
    }
    let mut A = vec![0; N + 1];
    for i in 2..=N {
        input! {a: u32}
        A[i] = a;
    }
    let mut B = vec![0; N + 1];
    for i in 3..=N {
        input! {b: u32}
        B[i] = b;
    }

    let mut dp = vec![std::u32::MAX; N + 1];
    dp[1] = 0;
    dp[2] = A[2];

    for i in 3..=N {
        dp[i] = (dp[i - 1] + A[i]).min(dp[i - 2] + B[i]);
    }

    let mut p = N;

    let mut ans = vec![];

    loop {
        ans.push(p);
        if p == 1 {
            break;
        }

        if dp[p - 1] + A[p] == dp[p] {
            p -= 1;
        } else {
            p -= 2;
        }
    }

    println!("{}", ans.len());
    for x in ans.iter().rev() {
        print!("{} ", x);
    }
}
