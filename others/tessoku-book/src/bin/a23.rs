use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize
    }
    let mut T = vec![0];
    for _ in 0..M {
        input! {A: [usize; N]}
        let mut t = 0;
        for (i, a) in A.iter().enumerate() {
            t += a << i;
        }
        T.push(t);
    }
    let mut dp = vec![vec![std::u64::MAX; 1 << N]; M + 1];

    // 初期値 クーポン0枚で0個の商品を買える
    dp[0][0] = 0;

    for i in 0..(M - 1) {
        for S in 0..(1 << N) {
            // クーポンを使わない場合
            dp[i + 1][S] = dp[i + 1][S].min(dp[i][S]);

            // クーポンを使う場合
            println!(
                "{:010b} {} {}, {}",
                S | T[i + 1],
                i,
                S,
                dp[i][S].overflowing_add(1).0
            );
            dp[i + 1][S | T[i + 1]] = dp[i + 1][S | T[i + 1]].min(dp[i][S].overflowing_add(1).0);
        }
        // クーポンを使わない場合
    }

    for row in &dp {
        println!("{:?}", row);
    }
}
