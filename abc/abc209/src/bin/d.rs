
use proconio::{fastout, input, marker::Usize1};
use std::collections::VecDeque;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
    }
    let mut graph = vec![vec![];N];
    for _ in  1..N {
        input! {a: Usize1, b: Usize1}
        graph[a].push(b);
        graph[b].push(a);
    }
    let mut even = vec![true; N];
    let mut que = VecDeque::new();
    let mut check = vec![false; N];
    que.push_back((0, true));

    while let Some((u, b)) = que.pop_front() {
        check[u] = true;
        even[u] = b;

        for &v in graph[u].iter() {
            if !check[v] {
                que.push_back((v, !b));
            }
        }
    }

    for _ in 0..Q {
        input! {c: Usize1, d:Usize1}
        let ans = if even[c] ^ even[d] {
            "Road"
        } else {
            "Town"
        };
        println!("{}", ans);
    }
}
