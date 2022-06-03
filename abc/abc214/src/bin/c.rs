use proconio::{fastout, input};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        S: [u32; N],
        T: [u32; N]
    }

    let mut heap = BinaryHeap::new();
    let mut set = HashSet::new();
    let mut ans = vec![std::u32::MAX; N];
    for (i, &t) in T.iter().enumerate() {
        heap.push((Reverse(t), i));
    }
    while let Some((Reverse(t), i)) = heap.pop() {
        ans[i] = ans[i].min(t);
        set.insert(i);
        if set.len() == N {
            break;
        }
        heap.push((Reverse(t + S[i]), (i + 1) % N));
    }
    for x in &ans {
        println!("{}", x);
    }
}
