use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashMap;

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
fn main() {
    input! {
        N: usize,
        K: usize,
        L: usize,
        PQ: [(Usize1, Usize1); K],
        RS: [(Usize1, Usize1); L],
    }

    let mut uft1 = UFT::new(N);
    let mut uft2 = UFT::new(N);
    let mut map = HashMap::new();

    for &(a, b) in &PQ {
        uft1.unite(a, b);
    }
    for &(a, b) in &RS {
        uft2.unite(a, b);
    }

    for i in 0..N {
        let cnt = map.entry((uft1.find(i), uft2.find(i))).or_insert(0);
        *cnt += 1;
    }

    for i in 0..N {
        print!("{} ", map.get(&(uft1.find(i), uft2.find(i))).unwrap());
    }
}
