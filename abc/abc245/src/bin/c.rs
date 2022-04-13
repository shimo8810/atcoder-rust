use proconio::{fastout, input};
const YES: &str = "Yes";
const NO: &str = "No";
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: i32,
        A: [i32; N],
        B: [i32; N]
    }

    // 0 -> A, 1 -> B
    let mut dp = vec![vec![false; 2]; N];
    dp[0][0] = true;
    dp[0][1] = true;

    for i in 1..N {
        if dp[i - 1][0] {
            //
            if (A[i - 1] - A[i]).abs() <= K {
                dp[i][0] = true;
            }
            if (A[i - 1] - B[i]).abs() <= K {
                dp[i][1] = true;
            }
        }

        if dp[i - 1][1] {
            if (B[i - 1] - A[i]).abs() <= K {
                dp[i][0] = true;
            }
            if (B[i - 1] - B[i]).abs() <= K {
                dp[i][1] = true;
            }
        }
    }
    let ans = if dp[N - 1].iter().any(|&b| b) {
        YES
    } else {
        NO
    };
    println!("{}", ans);
}
