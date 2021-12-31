use proconio::{fastout, input};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! { Q: usize }

    let mut heap = BinaryHeap::new();
    let mut sum = 0;
    for _ in 0..Q {
        input! {t: u8}
        if t == 1 {
            input! {x: i64}
            heap.push(Reverse(x - sum));
        } else if t == 2 {
            input! {x: i64}
            sum += x;
        } else {
            let Reverse(x) = heap.pop().unwrap();
            println!("{}", x + sum);
        }
    }
}
