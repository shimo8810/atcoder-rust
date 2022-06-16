use proconio::{fastout, input};

const INF: u32 = 1_000_000_001;

struct Cumsum2d {
    arr: Vec<Vec<usize>>,
    cs: Vec<Vec<usize>>,
}

impl Cumsum2d {
    fn new(h: usize, w: usize) -> Self {
        let arr = vec![vec![0; w]; h];
        let cs = vec![vec![0; w + 1]; h + 1];

        Self { arr, cs }
    }

    fn add(&mut self, x: usize, y: usize, v: usize) {
        self.arr[y][x] = v;
    }

    fn build(&mut self) {
        for (y, row) in self.arr.iter().enumerate() {
            for (x, &v) in row.iter().enumerate() {
                self.cs[y + 1][x + 1] = v + self.cs[y + 1][x] + self.cs[y][x + 1] - self.cs[y][x];
            }
        }
    }

    fn get(&self, x1: usize, x2: usize, y1: usize, y2: usize) -> usize {
        self.cs[y2][x2] + self.cs[y1][x1] - self.cs[y2][x1] - self.cs[y1][x2]
    }
}

#[allow(non_snake_case)]
fn check(x: u32, A: &[Vec<u32>], K: usize) -> bool {
    // 二次元累積和
    let n = A.len();
    let mut cs = Cumsum2d::new(n, n);
    for (i, row) in A.iter().enumerate() {
        for (j, &v) in row.iter().enumerate() {
            if x <= v {
                cs.add(j, i, 1);
            }
        }
    }

    cs.build();

    for i in 0..=(n - K) {
        for j in 0..=(n - K) {
            let s = cs.get(j, j + K, i, i + K);

            if s < K * K / 2 + 1 {
                return false;
            }
        }
    }

    true
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [[u32; N]; N]
    }

    let mut ok = 0;
    let mut ng = INF;
    while ng - ok > 1 {
        let md = (ok + ng) / 2;

        let f = check(md, &A, K);
        if f {
            ok = md;
        } else {
            ng = md;
        }
    }

    println!("{}", ok);
}
