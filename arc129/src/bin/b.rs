use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {N: usize}

    let mut l = 0u32;
    let mut r = 1_000_000_001u32;

    for _ in 0..N {
        input! {L: u32, R: u32}
        l = l.max(L);
        r = r.min(R);
        let ans = if l <= r { 0 } else { (l - r) / 2 + (l - r) % 2 };
        println!("{}", ans);
    }
}
