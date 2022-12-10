use proconio::{fastout, input};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
    }

    let mut graph = HashMap::new();

    for _ in 0..N {
        input! {A: usize, B: usize}
        graph.entry(A).or_insert_with(Vec::new).push(B);
        graph.entry(B).or_insert_with(Vec::new).push(A);
    }

    let mut ans = 0;

    let mut que = VecDeque::new();
    let mut set = HashSet::new();

    que.push_back(1);

    while let Some(u) = que.pop_front() {
        if set.contains(&u) {
            continue;
        }
        set.insert(u);
        ans = ans.max(u);

        for &v in graph.get(&u).unwrap_or(&Vec::new()) {
            que.push_back(v);
        }
    }
    println!("{}", ans);
}
