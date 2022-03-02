use proconio::{fastout, input, marker::Chars};
use std::collections::VecDeque;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        S: [Chars; H]
    }
    let inf = H * W + 1;
    let mut deq = VecDeque::new();
    let mut dist = vec![vec![inf; W]; H];
    deq.push_back((0, 0));

    while let Some((x, y)) = deq.pop_front() {
        
    }

    println!("{}", dist[H - 1][W - 1]);
}
