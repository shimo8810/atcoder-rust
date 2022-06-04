use proconio::{fastout, input, marker::Chars};

fn rec(t: u64, k: u64, s: &[u8]) -> u64 {
    if t == 0 {
        s[k as usize] as u64
    } else if k == 0 {
        s[0] as u64 + t
    } else {
        rec(t - 1, k / 2, s) + k % 2 + 1
    }
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        S: Chars,
        Q: usize,
    }
    let S: Vec<_> = S.into_iter().map(|c| c as u8 - b'A').collect();
    for _ in 0..Q {
        input! {t: u64, k: u64}
        let k = k - 1;
        let ans = ((rec(t, k, &S) % 3) as u8 + b'A') as char;
        println!("{}", ans);
    }
}
