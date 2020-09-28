use proconio::input;
use proconio::marker::Usize1;

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
        M: usize,
        AB: [(Usize1, Usize1); M]
    }

    let mut ans = 0;

    for i in 0..M {
        let mut uft = UFT::new(N);
        for j in 0..M {
            if i == j {
                continue;
            }
            let (a, b) = AB[j];
            uft.unite(a, b);
        }

        ans += if *uft.size.iter().max().unwrap() != N {
            1
        } else {
            0
        };
    }

    println!("{}", ans);
}
