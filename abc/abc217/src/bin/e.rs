use proconio::input;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut heap = BinaryHeap::new();
    let mut que = VecDeque::new();
    for _ in 0..N {
        input! {q: u8}
        if q == 1 {
            // push buck
            input! {x: u32}
            que.push_back(x);
        } else if q == 2 {
            // pop front
            if let Some(Reverse(x)) = heap.pop() {
                println!("{}", x);
            } else if let Some(x) = que.pop_front() {
                println!("{}", x);
            }
        } else {
            // sort
            while let Some(x) = que.pop_front() {
                heap.push(Reverse(x));
            }
        }
    }
}
