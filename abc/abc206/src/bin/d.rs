use proconio::{fastout, input};

const MAXN: usize = 200_000;

struct UnionFind {
    par: Vec<usize>,
    size: Vec<usize>,
}

#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> Self {
        let par = (0..n).collect();
        let size = vec![1; n];
        Self { par, size }
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
        let a = self.find(a);
        self.size[a]
    }
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }
    let mut uft = UnionFind::new(MAXN + 1);
    for (i, &a) in A.iter().enumerate().take(N / 2) {
        uft.unite(a, A[N - 1 - i]);
    }
    let mut ans = 0;
    for i in 0..=MAXN {
        if i == uft.find(i) {
            ans += uft.size(i) - 1;
        }
    }

    println!("{}", ans);
}
