use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};
use std::collections::VecDeque;

fn bfs(n: usize, s: usize, list: &[Vec<usize>]) -> (usize, i64) {
    let mut dist = vec![-1; n];
    let mut que = VecDeque::new();
    que.push_back(s);
    dist[s] = 0;
    while let Some(u) = que.pop_front() {
        for &v in list[u].iter() {
            if dist[v] < 0 {
                que.push_back(v);
                dist[v] = dist[u] + 1;
            }
        }
    }
    let p = dist.iter().position_max().unwrap();
    (p, dist[p])
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! { N: usize }
    let mut list = vec![Vec::new(); N];

    for _ in 1..N {
        input! {a: Usize1, b: Usize1}
        list[a].push(b);
        list[b].push(a);
    }

    let (p, _) = bfs(N, 0, &list);
    let (_, ans) = bfs(N, p, &list);

    println!("{}", ans + 1);
}
