use proconio::{fastout, input};
use std::collections::VecDeque;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        mut A: [u32; N]
    }
    A.sort_unstable();
    let mut deq: VecDeque<_> = A.into_iter().collect();
    let mut ans = 0;
    while deq.len() > 1 {
        let mx = deq.pop_back().unwrap();
        let mn = deq[0];
        if mx % mn > 0 {
            deq.push_front(mx % mn);
        }
        ans += 1;
    }
    println!("{}", ans);
}
