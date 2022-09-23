use proconio::{fastout, input, marker::Usize1};
use std::collections::HashSet;
use std::collections::VecDeque;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [i64; N],
    }

    let mut graph = vec![vec![]; N];
    for _ in 0..M {
        input! {U: Usize1, V: Usize1}
        graph[U].push(V);
        graph[V].push(U);
    }

    let mut ng = -1i64;
    let mut ok = A.iter().sum::<i64>();

    while ok - ng > 1 {
        let mid = (ok + ng) / 2;

        let mut s = vec![0; N];
        let mut que = VecDeque::new();
        for u in 0..N {
            for &v in &graph[u] {
                s[u] += A[v];
            }
            if s[u] <= mid {
                que.push_back(u);
            }
        }

        if true {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
