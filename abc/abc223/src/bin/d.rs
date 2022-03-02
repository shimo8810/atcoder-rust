use proconio::{fastout, input, marker::Usize1};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

// トポロジカルソート(Topological Sort)

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      M: usize,
    }
    let mut list = vec![vec![]; N];
    let mut cnt = vec![0; N];

    for _ in 0..M {
        input! {a: Usize1, b: Usize1}
        list[a].push(b);
        cnt[b] += 1;
    }

    let mut heap = BinaryHeap::new();

    for (i, &c) in cnt.iter().enumerate() {
        if c == 0 {
            heap.push(Reverse(i));
        }
    }
    let mut ans = vec![];
    while let Some(Reverse(u)) = heap.pop() {
        ans.push((u + 1).to_string());
        for &v in list[u].iter() {
            cnt[v] -= 1;
            if cnt[v] == 0 {
                heap.push(Reverse(v));
            }
        }
    }
    let ans = if ans.len() != N {
        "-1".to_string()
    } else {
        ans.join(" ")
    };
    println!("{}", ans);
}
