use proconio::{fastout, input};

#[fastout]
fn main() {
    //
    input! {
        n: usize,
        m: usize,
        k: u32,
        a: [u64; n],
        b: [u64; m],
    }
    let mut acum = a[0];
    let mut bcum: Vec<u64> = Vec::with_capacity(m);

    bcum[0] = b[0];
    for i in 1..m {
        bcum[i] = bcum[i - 1] + b[i];
    }

    for i in 1..n {}
}
