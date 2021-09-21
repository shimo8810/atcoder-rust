// use proconio::input;
// use proconio::marker::Usize1;

// const D: u64 = 998244353;

// #[allow(non_snake_case)]
// fn main() {
//     input! {
//         N: usize,
//         M: usize
//     }
//     // i -> i + 2j
//     let mut dp = vec![vec![0; 2 * N + 1]; 2 * N + 1];
//     let mut rel = vec![vec![false; 2 * N]; 2 * N];
//     for _ in 0..M {
//         input! {a: Usize1, b: Usize1}
//         rel[a][b] = true;
//         rel[b][a] = true;
//     }
//     for i in 0..=(2 * N) {
//         dp[i][0] = 1;
//     }

//     for j in 1..=N {
//         for i in 0..=2 * (N - j) {
//             dp[i][j] = 0;
//             for k in 0..j {
//                 if rel[i][i + (2 * k) + 1] {
//                     //
//                 }
//             }
//         }
//     }

//     let ans = dp[0][N] % D;
//     println!("{}", ans);
// }
