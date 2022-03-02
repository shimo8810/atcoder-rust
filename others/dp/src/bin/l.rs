use proconio::{fastout, input};
use std::cmp::max;
use std::collections::HashMap;

fn dfs(s: usize, t: usize, memo: &mut HashMap<(usize, usize), i64>, a: &[i64]) -> i64 {
    if let Some(&ret) = memo.get(&(s, t)) {
        // メモ上にあればそれを使用する
        ret
    } else {
        // メモになければ計算
        let ret = if t == s {
            a[s]
        } else {
            max(a[s] - dfs(s + 1, t, memo, a), a[t] - dfs(s, t - 1, memo, a))
        };
        memo.insert((s, t), ret);
        ret
    }
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [i64; N]
    }

    // メモ再帰
    let mut memo = HashMap::new();
    let ans = dfs(0, N - 1, &mut memo, &A);
    println!("{}", ans);
}
