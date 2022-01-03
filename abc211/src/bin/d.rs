use proconio::{fastout, input, marker::Usize1};
use std::cmp::Ordering;
use std::collections::VecDeque;

const D: u64 = 1_000_000_007;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize
    }

    let mut graph = vec![vec![]; N];
    for _ in 0..M {
        input! {a: Usize1, b: Usize1}
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut dist = vec![N + 1; N];
    let mut dp = vec![0; N];
    let mut que = VecDeque::new();
    que.push_back(0);
    dp[0] = 1;
    dist[0] = 0;
    while let Some(u) = que.pop_front() {
        for &v in &graph[u] {
            match dist[v].cmp(&(dist[u] + 1)) {
                Ordering::Equal => {
                    dp[v] = (dp[v] + dp[u]) % D;
                }
                Ordering::Greater => {
                    dist[v] = dist[u] + 1;
                    dp[v] = dp[u];
                    que.push_back(v);
                }
                _ => {}
            }
        }
    }

    println!("{}", dp[N - 1]);
}
