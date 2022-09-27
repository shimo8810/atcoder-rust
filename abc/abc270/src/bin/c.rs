use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        X: usize,
        Y: usize,
    }
    let mut g = vec![vec![]; N + 1];
    for _ in 1..N {
        input! {U: usize, V: usize}
        g[U].push(V);
        g[V].push(U);
    }

    let mut prev = vec![0; N + 1];
    let mut mrkd = vec![false; N + 1];
    let mut stack = vec![];
    stack.push(X);
    mrkd[X] = true;
    while let Some(u) = stack.pop() {
        for &v in &g[u] {
            if !mrkd[v] {
                mrkd[v] = true;
                prev[v] = u;
                stack.push(v);
            }
        }
    }

    let mut ans = vec![];
    let mut s = Y;
    while s != 0 {
        ans.push(s);
        s = prev[s];
    }
    for i in ans.into_iter().rev() {
        print!("{} ", i);
    }
}
