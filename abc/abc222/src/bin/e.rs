use proconio::{fastout, input, marker::Usize1};

fn _dfs(
    u: usize,
    g: usize,
    xs: &mut [i64],
    graph: &[Vec<(usize, usize)>],
    checked: &mut [bool],
) -> bool {
    if u == g {
        return true;
    }
    for &(v, i) in &graph[u] {
        if !checked[v] {
            checked[v] = true;
            xs[i] += 1;
            if _dfs(v, g, xs, graph, checked) {
                return true;
            }
            xs[i] -= 1;
        }
    }

    false
}

fn count_up(s: usize, g: usize, xs: &mut [i64], graph: &[Vec<(usize, usize)>]) {
    let n = graph.len();
    let mut checked = vec![false; n];
    checked[s] = true;
    _dfs(s, g, xs, graph, &mut checked);
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        K: i64,
        A: [Usize1; M]
    }

    let mut graph = vec![vec![]; N];
    for i in 0..(N - 1) {
        input! {u: Usize1, v: Usize1}
        graph[u].push((v, i));
        graph[v].push((u, i));
    }

    let mut xs = vec![0; N - 1];
    for aa in A.windows(2) {
        let a1 = aa[0];
        let a2 = aa[1];
        count_up(a1, a2, &mut xs, &graph);
    }

    let s: i64 = xs.iter().sum();

    if (s + K) % 2 != 0 || s + K < 0 {
        println!("0");
        return;
    }

    let mut dp = vec![0; 100_001];
    dp[0] = 1;
    for &x in &xs {
        let mut y = 100_000;
        while y >= x && y > 0 {
            dp[y as usize] += dp[(y - x) as usize];
            y -= 1;
        }
    }

    println!("{}", dp[(s + K) as usize / 2]);
}
