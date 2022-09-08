use proconio::{fastout, input, marker::Chars};
use std::collections::{HashSet, VecDeque};
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        S: Chars,
    }
    let T: Vec<_> = "atcoder".chars().collect();
    let mut que = VecDeque::new();
    let mut set = HashSet::new();
    que.push_back((S, 0));

    while let Some((mut s, n)) = que.pop_front() {
        if set.contains(&s) {
            continue;
        } else {
            set.insert(s.clone());
        }
        if s == T {
            println!("{}", n);
            return;
        }
        for x in 0..(s.len() - 1) {
            s.swap(x, x + 1);
            que.push_back((s.clone(), n + 1));
            s.swap(x, x + 1);
        }
    }
}
