use proconio::{fastout, input};
use std::collections::VecDeque;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        Q: u32,
    }

    let mut queue = VecDeque::new();

    for _ in 0..Q {
        input! {t: u8}

        if t == 1 {
            input! {x: u64, c: u64}
            queue.push_back((x, c));
        } else {
            input! {mut c: u64}
            let mut v = 0;
            loop {
                let (x, k) = queue.pop_front().unwrap();

                if c > k {
                    v += x * k;
                    c -= k;
                } else {
                    v += x * c;
                    queue.push_front((x, k - c));
                    break;
                }
            }
            println!("{}", v);
            //
        }
    }
}
