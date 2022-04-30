use proconio::{fastout, input};
use std::collections::VecDeque;

#[allow(non_snake_case)]
fn shaku(A: &[u32], X: u32, Y: u32) -> usize {
    let mut deq = VecDeque::new();

    let mut cx = 0;
    let mut cy = 0;
    let mut ans = 0;
    for (i, &a) in A.iter().enumerate() {
        deq.push_back(a);
        if a == X {
            cx += 1;
        }
        if a == Y {
            cy += 1;
        }

        while !deq.is_empty() && cx > 0 && cy > 0 {
            ans += A.len() - i;
            let b = deq.pop_front().unwrap();
            if b == X {
                cx -= 1;
            }
            if b == Y {
                cy -= 1;
            }
        }
    }

    ans
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        X: u32,
        Y: u32,
        A: [u32; N]
    }
    let mut lrs = vec![];
    let mut tmp = 0;
    for (i, &a) in A.iter().enumerate() {
        if a < Y || X < a {
            lrs.push((tmp, i));
            tmp = i + 1;
        }
    }
    lrs.push((tmp, A.len()));

    let mut ans = 0;
    for &(l, r) in &lrs {
        ans += shaku(&A[l..r], X, Y);
    }

    println!("{}", ans);
}
