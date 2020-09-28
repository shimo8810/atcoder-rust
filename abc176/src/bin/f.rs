#![allow(non_snake_case)]

use proconio::input;
use proconio::marker::Usize1;
use std::cmp;

fn upd(i: usize, j: usize, x: i64, dp: &mut Vec<Vec<i64>>, N: usize) {
    dp[i][j] = cmp::max(dp[i][j], x);
    dp[j][i] = cmp::max(dp[j][i], x);
    dp[N][i] = cmp::max(dp[N][j], x);
    dp[i][N] = cmp::max(dp[i][N], x);
    dp[N][j] = cmp::max(dp[N][j], x);
    dp[j][N] = cmp::max(dp[j][N], x);
    dp[N][N] = cmp::max(dp[N][N], x);
}
fn main() {
    input! {
        N: usize,
        A: [Usize1; 3 * N]
    }

    let mut ans = 0;
    let mut dp = vec![vec![std::i64::MIN; N + 1]; N + 1];
    upd(A[0], A[1], 0, &mut dp, N);
    for i in (2..(3 * N - 1)).step_by(3) {
        println!("{}, {}, {}", A[i] + 1, A[i + 1] + 1, A[i + 2] + 1);
        if A[i] == A[i + 1] && A[i + 1] == A[i + 2] {
            ans += 1;
            continue;
        }

        let mut q = Vec::new();
        for j in 0..3 {
            let x = A[i + j % 3];
            let y = A[i + (1 + j) % 3];
            let z = A[i + (2 + j) % 3];

            for a in 0..=N {
                let mut now = dp[a][N];
                if y == z {
                    now = cmp::max(now, dp[a][y] + 1);
                }
                q.push((a, x, now));
            }

            let now = cmp::max(dp[N][N], dp[z][z] + 1);
            q.push((x, y, now));
        }

        for (a, b, x) in q {
            upd(a, b, x, &mut dp, N);
        }
        println!("{:?}", dp);
    }


    let l = A[3 * N - 1];
    ans += cmp::max(dp[N][N], dp[l][l] + 1);
    println!("{}", ans);
}
