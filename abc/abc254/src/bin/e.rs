use proconio::{fastout, input};
use std::collections::VecDeque;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
    }

    let mut g = vec![vec![]; N + 1];
    for _ in 0..M {
        input! {a: usize, b: usize}
        g[a].push(b);
        g[b].push(a);
    }

    input! {Q: usize}

    for _ in 0..Q {
        input! {x: usize, k:usize}
        let mut check = vec![false; N + 1];
        let mut que = VecDeque::new();
        que.push_back((x, 0));

        let mut ans = 0;

        while let Some((u, d)) = que.pop_front() {
            if check[u] {
                continue;
            }
            check[u] = true;
            if d > k {
                continue;
            }
            ans += u;

            for &v in &g[u] {
                que.push_back((v, d + 1));
            }
        }

        println!("{}", ans);
    }
}
