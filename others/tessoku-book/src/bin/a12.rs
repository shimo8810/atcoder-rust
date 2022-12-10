use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: u64,
        A: [u64; N]
    }

    let mut ng = 0;
    let mut ok = 1_000_000_000;

    while (ok - ng) > 1 {
        let md = (ok + ng) / 2;
        let k: u64 = A.iter().map(|&x| md / x).sum();
        if K <= k {
            ok = md;
        } else {
            ng = md;
        }
    }

    println!("{}", ok);
}
