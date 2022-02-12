use proconio::{fastout, input, marker::Usize1};
use std::collections::BinaryHeap;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        H: [i64; N]
    }

    // ダイクストラ ポテンシャル
    let mut graph = vec![vec![]; N];
    let mut heap = BinaryHeap::new();
    let mut dist = vec![std::i64::MAX; N];

    for _ in 0..M {
        input! {U: Usize1, V: Usize1}
        graph[U].push((V, (H[V] - H[U]).max(0)));
        graph[V].push((U, (H[U] - H[V]).max(0)));
    }

    heap.push((0, 0));
    dist[0] = 0;

    while let Some((d, u)) = heap.pop() {
        if dist[u] < -d {
            continue;
        }

        for &(v, c) in &graph[u] {
            if dist[v] > dist[u] + c {
                dist[v] = dist[u] + c;
                heap.push((-dist[v], v));
            }
        }
    }

    let ans = dist
        .iter()
        .enumerate()
        .map(|(i, &x)| -x - H[i] + H[0])
        .max()
        .unwrap();
    println!("{}", ans);
}
