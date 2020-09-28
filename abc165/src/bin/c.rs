use proconio::input;
use proconio::marker::Usize1;
use std::cmp;

#[allow(unused_variables)]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: u64,
        Q: usize,
        ABCD: [(Usize1, Usize1, u64, u64); Q],
    }

    let mut stack = Vec::new();
    stack.push(vec![0; 0]);

    let mut ans = 0;
    while let Some(arr) = stack.pop() {
        if arr.len() == N {
            let mut score = 0;
            for &(a, b, c, d) in ABCD.iter() {
                if arr[b] - arr[a] == c {
                    score += d;
                }
            }
            ans = cmp::max(ans, score);
            continue;
        }

        let min = if arr.len() > 0 { arr[arr.len() - 1] } else { 1 };

        for i in min..=M {
            let mut buf = arr.clone();
            buf.push(i);
            stack.push(buf);
        }
    }

    println!("{}", ans);
}
