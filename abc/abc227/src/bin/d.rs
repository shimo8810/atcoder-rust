use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
        A:[usize; N]
    }

    let mut ok = 0;
    let mut ng = std::usize::MAX / K;
    while ng - ok > 1 {
        let p = (ok + ng) / 2;

        let s: usize = A.iter().map(|&a| a.min(p)).sum();
        if p * K <= s {
            ok = p;
        } else {
            ng = p;
        }
    }
    println!("{}", ok);
}
