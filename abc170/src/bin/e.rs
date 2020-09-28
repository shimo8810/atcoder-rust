#![allow(non_snake_case)]
#![allow(unused_imports)]

use proconio::input;
use std::cmp::Reverse;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;

fn main() {
    input! {
        N: usize,
        Q: usize,
        mut AB: [(u64, usize); N + 1],
        mut CD: [(usize, usize); Q],
    }

    // score, id の集合のベクトル
    let mut s = vec![BTreeSet::new(); 200_001];
    for (i, &(a, b)) in AB.iter().enumerate() {
        //
        s[b].insert((a, i));
    }

    // 各集合の最大値の集合
    let mut maxs = BinaryHeap::new();
    for i in 0..200_001 {
        //
        if let Some(&(a, _)) = s[i].iter().last() {
            maxs.push(Reverse((a, i)));
        }
    }

    println!("{:?}", AB);
    for &(c, d) in CD.iter() {
        // c をdへ移動させる
        let (a, b) = AB[c];
        AB[c].1 = d;
        s[b].remove(&(a, c));
        s[d].insert((a, c));
        // maxs へ追加
        if let Some(&(a, _)) = s[b].iter().last() {
            maxs.push(Reverse((a, b)));
        }
        if let Some(&(a, _)) = s[b].iter().last() {
            maxs.push(Reverse((a, b)));
        }
    }
    println!("{:?}", maxs);

    println!("{:?}", maxs.peek());
}
