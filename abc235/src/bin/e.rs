use std::collections::HashSet;

use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        Q: usize,
    }
    let edges = vec![];
    let set = HashSet::new();
    for _ in 0..M {
        input! {u: Usize1, }
    }
    let mut uf = UnionFind::new(N);
    let mut ans = 0;
    println!("{}", ans);
}
