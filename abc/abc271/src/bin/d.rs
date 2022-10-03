use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        S: usize,
        AB: [(usize, usize); N]
    }

    let mut dp = vec![vec![false; S + 1]; N + 1];
    let mut memo = vec![vec![None; S + 1]; N + 1];
    dp[0][0] = true;

    for i in 0..N {
        let (a, b) = AB[i];

        if S >= a {
            for k in 0..=(S - a) {
                if dp[i][k] {
                    dp[i + 1][k + a] = true;
                    memo[i + 1][k + a] = Some((i, k, 'H'));
                }
            }
        }
        if S >= b {
            for k in 0..=(S - b) {
                if dp[i][k] {
                    dp[i + 1][k + b] = true;
                    memo[i + 1][k + b] = Some((i, k, 'T'));
                }
            }
        }
    }

    if dp[N][S] {
        let mut ans = vec![];

        let mut x = memo[N][S];
        while let Some((i, k, c)) = x {
            ans.push(c);
            x = memo[i][k];
        }
        let ans: String = ans.into_iter().rev().collect();
        println!("{}", YES);
        println!("{}", ans);
    } else {
        println!("{}", NO);
    }
}
