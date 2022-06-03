use proconio::{fastout, input, marker::Usize1};

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

    fn size(&mut self, a: usize) -> usize {
        let z = self.find(a);
        self.size[z]
    }
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
    }

    let mut edges = vec![];
    for _ in 1..N {
        input! {u: Usize1, v: Usize1, w: usize}
        edges.push((w, u, v));
    }
    let mut uft = UFT::new(N);

    let mut ans = 0usize;
    edges.sort_unstable();

    for &(w, u, v) in &edges {
        ans += w * uft.size(u) * uft.size(v);
        uft.unite(u, v);
    }
    println!("{}", ans);
}
