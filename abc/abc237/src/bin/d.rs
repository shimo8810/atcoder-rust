use proconio::{fastout, input, marker::Chars};

fn dfs(n: usize, tree: &[(Option<usize>, Option<usize>)]) {
    let (l, r) = tree[n];

    if let Some(m) = l {
        dfs(m, tree);
    }
    println!("{} ", n);

    if let Some(m) = r {
        dfs(m, tree);
    }
}
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        S: Chars
    }

    let mut tree = vec![(None, None); N + 1];
    for (i, s) in S.iter().enumerate() {
        tree[i] = match s {
            'L' => (Some(i + 1), None),
            _ => (None, Some(i + 1)),
        };
    }

    dfs(0, &tree);
}
