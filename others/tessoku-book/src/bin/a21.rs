use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
    }
    let mut P = vec![];
    let mut A = vec![];
    for _ in 0..N {
        input! {p: Usize1, a: u32}
        P.push(p);
        A.push(a);
    }

    let mut dp = vec![vec![0; N]; N];

    // 長さでループ N, N - 1, ..., 0
    for d in (0..N).rev() {
        for l in 0..(N - d) {
            let r = l + d;

            // 左端を削るとき(取り出すのはl-1)
            if l > 0 {
                let s = if l - 1 <= P[l - 1] && P[l - 1] <= r {
                    A[l - 1]
                } else {
                    0
                };
                dp[l][r] = dp[l - 1][r] + s;
            }

            // 右端を削るとき(取り出すのはr+1)
            if r + 1 < N {
                let s = if l <= P[r + 1] && P[r + 1] <= r + 1 {
                    A[r + 1]
                } else {
                    0
                };

                dp[l][r] = dp[l][r].max(dp[l][r + 1] + s);
            }
            // 右端を削るとき
        }
    }


    let mut ans = 0;
    for i in 0..N {
        ans = ans.max(dp[i][i]);
    }
    println!("{}", ans);
}
