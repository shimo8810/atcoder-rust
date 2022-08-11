use proconio::{fastout, input};

fn dfs(v: &mut Vec<usize>, m: usize, n: usize) {
    if v.len() == n {
        for x in v.iter() {
            print!("{} ", x);
        }
    }

    let &x = v.last().unwrap_or(&0);
    for y in (x + 1)..=m {
        v.push(y);
        dfs(v, m, n);
        v.pop();
    }
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize
    }

    let mut v = vec![];
    dfs(&mut v, M, N);
}
