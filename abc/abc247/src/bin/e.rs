use std::collections::VecDeque;

use proconio::{fastout, input};

fn shaku(s: &[u32], x: u32, y: u32) -> usize {
    let mut h = 0;
    let mut t = 0;
    let mut numx = 0;
    let mut numy = 0;
    let mut ans = 0;

    while h < s.len() {
        while t < s.len() && (numx == 0 || numy == 0) {
            if s[t] == x {
                numx += 1;
            }
            if s[t] == y {
                numy += 1;
            }
            t += 1;
        }

        if numx > 0 && numy > 0 {
            ans += s.len() + 1 - t;
        }

        if s[h] == x {
            numx -= 1;
        }
        if s[h] == y {
            numy -= 1;
        }
        h += 1;
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
    let mut seps: VecDeque<_> = A
        .iter()
        .enumerate()
        .filter(|(_, &a)| a > X || Y > a)
        .map(|(i, _)| i + 1)
        .collect();

    seps.push_front(0);
    seps.push_back(N + 1);
    let seps: Vec<_> = seps.into_iter().collect();
    // println!("{:?}", seps);
    let mut ans = 0;
    for sep in seps.windows(2) {
        let l = sep[0];
        let r = sep[1] - 1;
        ans += shaku(&A[l..r], X, Y);
    }
    println!("{}", ans);
}
