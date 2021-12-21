use proconio::{fastout, input, marker::Usize1};

const M: usize = 998244353;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        K: usize,
        x1: Usize1,
        y1: Usize1,
        x2: Usize1,
        y2: Usize1,
    }
    let mut dp = vec![vec![0; 4]; K + 1];
    let i = if x1 == x2 && y1 == y2 {
        0
    } else if x1 == x2 {
        1
    } else if y1 == y2 {
        2
    } else {
        3
    };
    dp[0][i] = 1;

    for k in 1..=K {
        dp[k][0] = (dp[k - 1][1] + dp[k - 1][2]) % M;
        dp[k][1] = (((W - 1) * dp[k - 1][0]) % M + dp[k - 1][3] + ((W - 2) * dp[k - 1][1]) % M) % M;
        dp[k][2] = (((H - 1) * dp[k - 1][0]) % M + dp[k - 1][3] + ((H - 2) * dp[k - 1][2]) % M) % M;
        dp[k][3] = (((W - 1) * dp[k - 1][2]) % M
            + ((H - 1) * dp[k - 1][1]) % M
            + ((W + H - 4) * dp[k - 1][3]) % M)
            % M;
    }

    println!("{}", dp[K][0]);
}
