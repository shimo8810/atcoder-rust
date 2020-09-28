use proconio::input;

struct UFT {
    par: Vec<isize>,
    n: usize,
}

impl UFT {
    fn new(n: usize) -> Self {
        let par = vec![-1; n];
        Self { par, n }
    }

    fn merge(&mut self, x: usize, y: usize) {}

    fn find(&mut self, x: usize) -> usize {
        if self.par[x] < 0 {
            x
        } else {
            self.par[x] = self.find(self.par[x] as usize) as isize;
            self.par[x] as usize
        }
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut xy: [(i64, i64); N]
    }

    xy.sort();
    let mut uft = UFT::new(N);

    for i in 0..N {
        for j in 0..i {
            if xy[i].1 > xy[j].1 {
                uft.unite(i, j);
                println!("{:?}, {:?}", xy[i], xy[j]);
            }
        }
    }

    println!("{:?}", uft.par);
}
