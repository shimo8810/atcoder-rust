use proconio::{fastout, input, marker::Usize1};
use std::cmp::Reverse;

#[allow(non_snake_case)]
fn dfs(u: usize, g: &[Vec<usize>], c: &mut [bool], vecs: &mut [Vec<u32>], X: &[u32]) {
    let mut chs = vec![];
    for &v in g[u].iter() {
        if !c[v] {
            chs.push(v);
            c[v] = true;
            dfs(v, g, c, vecs, X);
        }
    }

    let mut arr = vec![Reverse(X[u])];
    for &ch in &chs {
        for &y in &vecs[ch] {
            arr.push(Reverse(y));
        }
    }
    arr.sort_unstable();
    let mx = 20.min(arr.len());
    vecs[u] = arr[..mx].iter().map(|&Reverse(x)| x).collect();
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        X: [u32; N]
    }

    let mut g = vec![vec![]; N];
    for _ in 1..N {
        input! {A: Usize1, B: Usize1}
        g[A].push(B);
        g[B].push(A);
    }

    let mut c = vec![false; N];
    let mut vecs = vec![vec![]; N];
    c[0] = true;
    dfs(0, &g, &mut c, &mut vecs, &X);

    for _ in 0..Q {
        input! {V: Usize1, K: Usize1}
        let ans = vecs[V][K];
        println!("{}", ans);
    }
}
