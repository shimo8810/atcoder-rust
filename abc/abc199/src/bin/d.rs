use proconio::{fastout, input, marker::Usize1};

fn dfs(u: usize, idx: &mut [i32], c: &mut [i32], g: &[Vec<usize>]) -> u64 {
    for &v in &g[u] {
        if c[u] == c[v] {
            return 0;
        }
    }
    let mut res = 1;
    for &v in &g[u] {
        if idx[v] == -1 {
            idx[v] = idx[u] + 1;
        }
        if idx[v] == idx[u] + 1 {
            let mut tmp = 0;
            c[v] = (c[u] + 1) % 3;
            tmp += dfs(v, idx, c, g);
            c[v] = (c[u] + 2) % 3;
            tmp += dfs(v, idx, c, g);
            c[v] = -1;
            res *= tmp;
            if tmp == 0 {
                break;
            }
        }
    }

    res
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut g = vec![vec![]; N];
    for _ in 0..M {
        input! {A: Usize1, B: Usize1}
        g[A].push(B);
        g[B].push(A);
    }

    let mut idx = vec![-1; N];
    let mut c = vec![-1; N];
    let mut ans = 1;

    for i in 0..N {
        // すでに探索済みのノードはスキップ
        if idx[i] != -1 {
            continue;
        }
        idx[i] = 0;
        c[i] = 0;
        ans *= dfs(i, &mut idx, &mut c, &g);
    }
    
    println!("{}", ans);
}
