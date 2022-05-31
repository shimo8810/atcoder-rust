use proconio::{fastout, input, marker::Usize1};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize
    }
    let mut graph = vec![vec![]; N];
    for i in 0..M {
        input! {A: Usize1, B: Usize1, C: u64}
        graph[A].push((B, C, i));
        graph[B].push((A, C, i));
    }

    let mut dist = vec![std::u64::MAX; N];
    dist[0] = 0;
    let mut prev = vec![0; N];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0u64), 0));

    while let Some((Reverse(d), u)) = heap.pop() {
        if dist[u] != d {
            continue;
        }
        for &(v, c, i) in &graph[u] {
            let alt = d.saturating_add(c);
            if dist[v] > alt {
                dist[v] = alt;
                prev[v] = i;
                heap.push((Reverse(alt), v));
            }
        }
    }
    for x in &prev[1..] {
        print!("{} ", x + 1);
    }
}
