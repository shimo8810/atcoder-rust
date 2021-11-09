use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";
/// Union-Find 木
#[allow(dead_code)]
struct Uft {
    /// 親node番号
    par: Vec<usize>,
    /// 深さ
    rank: Vec<usize>,
}

#[allow(dead_code)]
impl Uft {
    fn new(n: usize) -> Self {
        let par = (0..n).collect();
        let rank = vec![1; n];
        Self { par, rank }
    }

    /// UFTのrootを探索
    fn find(&mut self, a: usize) -> usize {
        if self.par[a] == a {
            a
        } else {
            self.par[a] = self.find(self.par[a]);
            self.par[a]
        }
    }

    /// aとbが属する集合を併合
    fn unite(&mut self, a: usize, b: usize) -> usize {
        let mut x = self.find(a);
        let mut y = self.find(b);
        if x != y {
            if self.rank[x] < self.rank[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.rank[x] += self.rank[y];
            self.par[y] = x;
        }
        x
    }

    /// aとbが同じ集合に属するか判定
    fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}

fn p2i(r: usize, c: usize, w: usize) -> usize {
    (r - 1) * w + c - 1
}

#[allow(clippy::branches_sharing_code)]
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        Q: usize
    }

    let mut utf = Uft::new(H * W);
    let mut A = vec![vec![false; W + 2]; H + 2];

    for _ in 0..Q {
        input! {t: u8}

        if t == 1 {
            input! {
                r: usize,
                c: usize
            }
            A[r][c] = true;
            let i = p2i(r, c, W);
            if A[r - 1][c] {
                utf.unite(i, p2i(r - 1, c, W));
            }
            if A[r + 1][c] {
                utf.unite(i, p2i(r + 1, c, W));
            }
            if A[r][c - 1] {
                utf.unite(i, p2i(r, c - 1, W));
            }
            if A[r][c + 1] {
                utf.unite(i, p2i(r, c + 1, W));
            }
        } else {
            input! {
                ra: usize,
                ca: usize,
                rb: usize,
                cb: usize
            }
            let a = p2i(ra, ca, W);
            let b = p2i(rb, cb, W);

            let ans = if A[ra][ca] && A[rb][cb] && utf.same(a, b) {
                YES
            } else {
                NO
            };
            println!("{}", ans);
        }
    }
}
