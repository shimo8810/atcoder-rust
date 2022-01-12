use proconio::{fastout, input};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
        P: [u32; N]
    }
    let mut heap = BinaryHeap::from_iter(P[..K].iter().map(|&x| Reverse(x)));

    println!("{}", heap.peek().unwrap().0);
    for i in K..N {
        let p = heap.peek().unwrap().0;
        if p < P[i] {
            heap.pop();
            heap.push(Reverse(P[i]));
        }
        println!("{}", heap.peek().unwrap().0);
    }
}
