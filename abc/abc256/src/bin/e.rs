use proconio::{fastout, input, marker::Usize1};

fn dfs(v: usize, state: &mut [u32], x: &[usize], ans: &mut Vec<usize>) -> Option<usize> {
    if state[v] == 2 {
        return None;
    }
    if state[v] == 1 {
        return Some(v);
    }
    state[v] = 1;

    let r = dfs(x[v], state, x, ans);
    state[v] = 2;

    let r = r?;
    ans.push(v);
    if r == v {
        return None;
    }

    Some(r)
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        n: usize,
        x: [Usize1; n],
        c: [u64; n]
    }

    let mut state = vec![0u32; n];
    let mut ans = 0;
    for i in 0..n {
        if state[i] == 0 {
            let mut vs = vec![];
            dfs(i, &mut state, &x, &mut vs);
            if !vs.is_empty() {
                let mut cost = std::u64::MAX;
                for v in vs.into_iter() {
                    cost = cost.min(c[v]);
                }
                ans += cost;
            }
        }
    }

    println!("{}", ans);
}
