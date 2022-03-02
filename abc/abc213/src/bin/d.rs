use proconio::{fastout, input};

fn dfs(u: usize, graph: &[Vec<usize>], checked: &mut [bool]) {
    checked[u] = true;
    print!("{} ", u);

    for &v in &graph[u] {
        if !checked[v] {
            dfs(v, graph, checked);
            print!("{} ", u);
        }
    }
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
    }

    let mut graph = vec![vec![]; N + 1];

    for _ in 1..N {
        input! {a: usize, b: usize}
        graph[a].push(b);
        graph[b].push(a);
    }

    for link in &mut graph {
        link.sort_unstable();
    }
    let mut checked = vec![false; N + 1];
    dfs(1, &graph, &mut checked);
}
