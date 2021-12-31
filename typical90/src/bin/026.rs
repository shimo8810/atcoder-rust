use proconio::{fastout, input, marker::Usize1};
use std::collections::HashSet;
use std::collections::VecDeque;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
    }
    let mut link = vec![vec![]; N];

    for _ in 1..N {
        input! {a: Usize1, b: Usize1}
        link[a].push(b);
        link[b].push(a);
    }

    let mut checked = vec![HashSet::new(); 2];
    // let mut left = HashSet::new();
    // let mut right = HashSet::new();
    let mut que = VecDeque::new();
    que.push_back((0, 0));

    while let Some((u, t)) = que.pop_front() {
        checked[t].insert(u);
        for &v in &link[u] {
            if !checked[0].contains(&v) && !checked[1].contains(&v) {
                que.push_back((v, 1 - t));
            }
        }
    }
    let idx = if checked[0].len() >= checked[1].len() {
        0
    } else {
        1
    };
    for (i, x) in checked[idx].iter().enumerate() {
        if i >= N / 2 {
            break;
        }
        print!("{} ", x + 1);
    }
}
