use proconio::{fastout, input};
use std::collections::HashSet;

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize
    }


    // Graf作成
    let mut graph = vec![vec![]; N + 1];
    for _ in 0..Q {
        input! {l: usize, r: usize}
        graph[r].push(l - 1);
        graph[l - 1].push(r);
    }

    let mut check = HashSet::new();

    let mut que = vec![0];
    while let Some(v) = que.pop() {
        check.insert(v);
        for u in &graph[v] {
            if !check.contains(u) {
                que.push(*u);
            }
        }
    }

    let ans = if check.contains(&N) { YES } else { NO };

    println!("{}", ans);
}
