use proconio::{fastout, input, marker::Usize1};
use std::collections::VecDeque;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut graph = vec![vec![]; N];
    for _ in 0..M {
        input! {A: Usize1, B: Usize1}
        graph[A].push(B);
    }

    let mut ans = 0;
    for s in 0..N {
        let mut check = vec![false; N];
        let mut que = VecDeque::new();
        que.push_back(s);

        while let Some(u) = que.pop_front() {
            check[u] = true;

            for &v in &graph[u] {
                if !check[v] {
                    que.push_back(v);
                }
            }
        }
        ans += check.into_iter().filter(|&f| f).count();
    }
    println!("{}", ans);
}
