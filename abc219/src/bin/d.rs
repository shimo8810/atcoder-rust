use proconio::input;
use std::cmp::min;

const INF: u32 = 301;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
        Y: usize,
        AB: [(usize, usize); N]
    }
    // dp[i, x, y]
    let mut dp = vec![vec![vec![INF; Y + 1]; X + 1]; N + 1];

    for i in 0..=N {
        for x in 0..=X {
            for y in 0..=Y {
                if i == 0 {
                    if x == 0 && y == 0 {
                        dp[i][x][y] = 0;
                    }
                } else {
                    let (a, b) = AB[i - 1];
                    let xa = if x > a { x - a } else { 0 };
                    let yb = if y > b { y - b } else { 0 };
                    dp[i][x][y] = min(dp[i - 1][x][y], dp[i - 1][xa][yb] + 1);
                }
            }
        }
    }

    if dp[N][X][Y] >= INF {
        println!("{}", -1);
    } else {
        println!("{}", dp[N][X][Y]);
    }
}
