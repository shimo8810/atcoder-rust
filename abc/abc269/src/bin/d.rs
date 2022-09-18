use petgraph::unionfind::UnionFind;
use proconio::{fastout, input};
use std::collections::HashSet;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        XY: [(i32, i32); N]
    }

    let mut uft = UnionFind::new(N);

    for i in 0..N {
        for j in (i + 1)..N {
            // 隣接判定
            let (x1, y1) = XY[i];
            let (x2, y2) = XY[j];

            if (x1 == (x2 - 1) && y1 == (y2 - 1))
                || (x1 == (x2 - 1) && y1 == y2)
                || (x1 == x2 && y1 == (y2 - 1))
                || (x1 == (x2 + 1) && y1 == (y2 + 1))
                || (x1 == (x2 + 1) && y1 == y2)
                || (x1 == x2 && y1 == (y2 + 1))
            {
                uft.union(i, j);
            }
        }
    }

    let mut set = HashSet::new();
    for i in 0..N {
        set.insert(uft.find(i));
    }
    let ans = set.len();
    println!("{}", ans);
}
