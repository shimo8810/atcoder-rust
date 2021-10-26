use proconio::{fastout, input};
use std::collections::VecDeque;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      Q: usize,
    }
    let mut deq = VecDeque::new();

    for _ in 0..N {
        input! {a: u32}
        deq.push_back(a);
    }

    for _ in 0..Q {
        input! {T: u8, x: usize, y: usize}

        if T == 1 {
            deq.swap(x - 1, y - 1);
        } else if T == 2 {
            deq.rotate_right(1);
        } else if T == 3 {
            println!("{}", deq[x - 1]);
        }
    }
}
