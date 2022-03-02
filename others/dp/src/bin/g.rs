use petgraph::algo::toposort;
use petgraph::{Directed, Graph, Incoming};
use proconio::marker::Usize1;
use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      M: usize,
      xy: [(Usize1, Usize1); M]
    }
    // petgraph を使用してトポロジカルソート
    let g = Graph::<(), (), Directed, usize>::from_edges(&xy);
    let topo = toposort(&g, None).unwrap();

    let mut dp = vec![0; N];

    // トポロジカルソート順でDP
    // 最長経路で更新
    for &a in topo.iter() {
        let ns = g.neighbors_directed(a, Incoming);
        let mut len = 0;
        for n in ns {
            len = len.max(dp[n.index()] + 1);
        }
        dp[a.index()] = len;
    }
    let ans = dp.iter().max().unwrap();
    println!("{}", ans);
}
