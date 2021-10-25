use proconio::{fastout, input};
use std::collections::VecDeque;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
    }

    let mut deq = VecDeque::new();

    for _ in 0..N {
        input! {t: u8, x: usize}
        if t == 1 {
            deq.push_front(x);
        } else if t == 2 {
            deq.push_back(x);
        } else {
            println!("{}", deq[x - 1]);
        }
    }
}
