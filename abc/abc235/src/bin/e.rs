use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Usize1};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        Q: usize,
    }
    let mut edges = vec![];
    let mut uf = UnionFind::<usize>::new(N);
    let mut ans = vec![NO; Q];

    for _ in 0..M {
        input! {a: Usize1, b: Usize1, c: u32}
        edges.push((c, a, b, None));
    }

    for i in 0..Q {
        input! {u: Usize1, v: Usize1, w: u32}
        edges.push((w, u, v, Some(i)));
    }

    edges.sort_unstable();

    for &(_, x, y, i) in &edges {
        if !uf.equiv(x, y) {
            if let Some(i) = i {
                ans[i] = YES;
            } else {
                uf.union(x, y);
            }
        }
    }

    for a in &ans {
        println!("{}", a);
    }
}
