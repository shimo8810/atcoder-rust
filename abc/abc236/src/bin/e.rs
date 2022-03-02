use proconio::{fastout, input};

fn dp(arr: &[i64]) -> i64 {
    let n = arr.len();
    let mut dp = vec![vec![0; 2]; n + 1];

    for (i, x) in arr.iter().enumerate() {
        dp[i + 1][0] = dp[i][0].max(dp[i][1]) + x;
        dp[i + 1][1] = dp[i][0];
    }
    dp[n][1].max(dp[n][0])
}

fn binary_search<F>(mut ng: i64, mut ok: i64, check: F) -> i64
where
    F: Fn(i64) -> bool,
{
    while (ok - ng).abs() > 1 {
        //
        let md = (ok + ng) / 2;
        if check(md) {
            ok = md;
        } else {
            ng = md;
        }
    }

    ok
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [i64; N]
    }

    let ok = 0;
    let ng = 1_000_000_001; // 1e9

    let mean = binary_search(ng, ok, |md| {
        let arr: Vec<_> = A.iter().map(|&x| x * 10_000 - md).collect();
        dp(&arr) >= 0
    });
    println!("{}", mean as f64 / 10_000.0);

    let median = binary_search(ng, ok, |md| {
        let arr: Vec<_> = A.iter().map(|&x| if x >= md { 1 } else { -1 }).collect();
        dp(&arr) > 0
    });
    println!("{}", median);
}
