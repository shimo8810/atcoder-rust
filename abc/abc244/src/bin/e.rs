use proconio::{fastout, input, marker::Usize1};

const D: usize = 998244353;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
        S: Usize1,
        T: Usize1,
        X: Usize1,
    }

    let mut g = vec![vec![]; N];
    for _ in 0..M {
        input! {u: Usize1, v: Usize1}
        g[u].push(v);
        g[v].push(u);
    }
    let mut dp = vec![vec![vec![0; 2]; N]; K + 1];

    for j in 0..N {
        dp[0][j][0] = if j == S { 1 } else { 0 };
        dp[0][j][1] = 0;
    }

    for i in 1..=K {
        for j in 0..N {
            for k in 0..2 {
                for &l in g[j].iter() {
                    dp[i][j][k] = if j == X {
                        (dp[i][j][k] + dp[i - 1][l][1 - k]) % D
                    } else {
                        (dp[i][j][k] + dp[i - 1][l][k]) % D
                    };
                }
            }
        }
    }

    println!("{}", dp[K][T][0]);
}
