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
            self.par[y]
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
      M: usize,
    }

    let mut list = vec![vec![]; N];
    for _ in 0..M {
        input! {a: Usize1, b: Usize1}
        list[a].push(b);
    }

    let mut uft = UFT::new(N);
    let mut res = vec![0];

    let mut ans = 0;
    for u in (1..N).rev() {
        // dbg!(u);
        ans += 1;
        for &v in list[u].iter() {
            if !uft.same(u, v) {
                uft.unite(u, v);
                ans -= 1;
            }
        }
        res.push(ans);
    }

    for i in res.iter().rev() {
        println!("{}", i);
    }
}
