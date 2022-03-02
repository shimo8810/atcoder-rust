// use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Usize1};
use std::collections::HashMap;

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(dead_code)]
struct UFT {
    par: Vec<usize>,
    size: Vec<usize>,
    n: usize,
}

#[allow(dead_code)]
impl UFT {
    fn new(n: usize) -> Self {
        let par = (0..n).collect();
        let size = vec![1; n];
        Self { par, size, n }
    }
    fn find(&mut self, a: usize) -> usize {
        if self.par[a] == a {
            a
        } else {
            self.par[a] = self.find(self.par[a]);
            self.par[a]
        }
    }
    fn unite(&mut self, a: usize, b: usize) -> usize {
        let mut x = self.find(a);
        let mut y = self.find(b);
        if x == y {
            x
        } else {
            if self.size[x] < self.size[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.size[x] += self.size[y];
            self.par[y] = x;
            x
        }
    }
    fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      M: usize
    }

    let mut uf = UFT::new(N);
    let mut map = HashMap::new();
    for _ in 0..M {
        input! {a: Usize1, b: Usize1}
        if uf.same(a, b) {
            println!("{}", NO);
            return;
        } else {
            uf.unite(a, b);
            *map.entry(a).or_insert(0) += 1;
            *map.entry(b).or_insert(0) += 1;
        }
    }
    let ans = match map.iter().find(|&(_, &v)| v > 2) {
        Some(_) => NO,
        _ => YES,
    };
    println!("{}", ans);
}
