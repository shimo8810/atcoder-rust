use proconio::{fastout, input, marker::Usize1};

fn dfs(u: usize, mut i: u32, g: &[Vec<usize>], lr: &mut [(u32, u32)], check: &mut [bool]) -> u32 {
    check[u] = true;
    if g[u].len() == 1 && u != 0 {
        lr[u] = (i, i);
        i + 1
    } else {
        let mut l = i;
        let mut r = i;

        for &v in g[u].iter() {
            if !check[v] {
                i = dfs(v, i, g, lr, check);
                l = l.min(lr[v].0);
                r = r.max(lr[v].1);
            }
        }
        lr[u] = (l, r);
        i
    }
}
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
    }
    let mut g = vec![vec![]; N];
    for _ in 1..N {
        input! {u: Usize1, v: Usize1}
        g[u].push(v);
        g[v].push(u);
    }

    let mut lr = vec![(0, 0); N];
    let mut check = vec![false; N];

    dfs(0, 1, &g, &mut lr, &mut check);

    for (l, r) in &lr {
        println!("{} {}", l, r);
    }
}
