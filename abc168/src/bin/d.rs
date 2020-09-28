#![allow(unused_imports)]

use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(usize, usize); M]
    }

    let mut graph = vec![Vec::<usize>::new(); N + 1];
    for i in 0..M {
        let (a, b) = AB[i];
        graph[a].push(b);
        graph[b].push(a);
    }

    // init dijkstra
    let mut dist = vec![std::usize::MAX; N + 1];
    dist[0] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, 1))); // cont, node

    let mut ans = vec![0; N + 1];

    while let Some(Reverse((cost, node))) = heap.pop() {
        if cost > dist[node] {
            continue;
        }

        for &node_ne in &graph[node] {
            let cost_ne = cost + 1;
            if cost_ne < dist[node_ne] {
                heap.push(Reverse((cost_ne, node_ne)));
                dist[node_ne] = cost_ne;
                ans[node_ne] = node;
            }
        }
    }

    println!("Yes");
    for i in &ans[2..] {
        println!("{}", i);
    }
}
