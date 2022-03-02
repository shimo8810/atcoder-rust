use proconio::input;

const M: usize = 998_244_353;
const MX: usize = 3_000;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        B: [usize; N]
    }

    // dp(i, j) := i番目までci = j 以下の総数の累積和
    let mut dp = vec![0; MX + 1];
    dp[0] = 1;
    for i in 0..N {
        let mut p = vec![0; MX + 1];
        std::mem::swap(&mut dp, &mut p);
        for j in 0..MX {
            p[j + 1] = (p[j + 1] + p[j]) % M;
        }

        for j in 0..=MX {
            if A[i] <= j && j <= B[i] {
                dp[j] = p[j] % M;
            }
        }
    }

    let mut ans = 0;
    for &x in dp.iter() {
        ans = (ans + x) % M;
    }
    println!("{}", ans);
}
