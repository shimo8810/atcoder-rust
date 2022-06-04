#![allow(dead_code, unused_imports)]

use itertools::Itertools;
use proconio::{fastout, input};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

enum Query {
    Add(u64),
    Upper(u64, usize),
    Lower(u64, usize),
}
use Query::*;

struct SegmentTree<F, T> {
    size: usize,
    tree: Vec<T>,
    element: T,
    eval: F,
}
impl<F: Fn(T, T) -> T, T: Copy + Eq + std::fmt::Debug> SegmentTree<F, T> {
    fn new(max: usize, element: T, eval: F) -> Self {
        let size = max.next_power_of_two();
        Self {
            size,
            tree: vec![element; size * 2],
            element,
            eval,
        }
    }
    fn get(&self, left: usize, right: usize) -> T {
        self._get(left, right + 1, 1, 0, self.size)
    }
    fn _get(&self, left: usize, right: usize, now_pos: usize, l: usize, r: usize) -> T {
        if r <= left || right <= l {
            self.element
        } else if left <= l && r <= right {
            self.tree[now_pos]
        } else {
            (self.eval)(
                self._get(left, right, now_pos * 2, l, (l + r) / 2),
                self._get(left, right, now_pos * 2 + 1, (l + r) / 2, r),
            )
        }
    }
    pub fn update(&mut self, index: usize, value: T) {
        let mut i = self.size + index;
        self.tree[i] = value;
        while i > 0 {
            i /= 2;
            self.tree[i] = (self.eval)(self.tree[i * 2], self.tree[i * 2 + 1]);
        }
    }
}
impl<F, T: std::fmt::Debug> std::fmt::Debug for SegmentTree<F, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SegmentTree{:?}", self.tree)
    }
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        Q: usize,
    }
    let mut qs = vec![];
    let mut cc = vec![];
    for _ in 0..Q {
        input! {t: u8}
        if t == 1 {
            input! {x: u64}
            qs.push(Add(x));
            cc.push(x);
            //
        } else if t == 2 {
            input! {x: u64, k: usize}
            qs.push(Upper(x, k));
            cc.push(x);
        } else {
            input! {x: u64, k: usize}
            qs.push(Lower(x, k));
            cc.push(x);
        }
    }

    let mut cc = cc
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect_vec();
    cc.sort_unstable();
    let mut tree = SegmentTree::new(cc.len(), 0, |a, b| a + b);

    for q in &qs {
        match q {
            Add(x) => {
                let i = cc.binary_search(x).unwrap();
                let a = tree.get(i, i);
                tree.update(i, a + 1);
            }
            Upper(x, k) => {
                let i = cc.binary_search(x).unwrap();
                let mut ok = -1;
                let mut ng = i as isize;
                while (ng - ok).abs() > 1 {
                    let mid = (ok + ng) / 2;
                    if tree.get(mid as usize, i) >= *k {
                        ok = mid;
                    } else {
                        ng = mid;
                    }
                }
                if ok == -1 {
                    println!("-1");
                } else {
                    println!("{}", cc[ok as usize]);
                }
            }
            Lower(x, k) => {
                let i = cc.binary_search(x).unwrap();
                let mut ok = cc.len();
                let mut ng = i;
                while ok - ng > 1 {
                    let mid = (ok + ng) / 2;
                    if tree.get(i, mid) >= *k {
                        ok = mid;
                    } else {
                        ng = mid;
                    }
                }
                if ok == cc.len() {
                    println!("-1");
                } else {
                    println!("{}", cc[ok as usize]);
                }
            }
        }
    }
}
