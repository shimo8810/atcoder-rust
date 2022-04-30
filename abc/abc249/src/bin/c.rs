use proconio::{fastout, input, marker::Chars};
use std::collections::HashMap;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
        S: [Chars; N]
    }

    let mut ans = 0;
    for bits in 0..(1 << N) {
        let mut map = HashMap::new();
        for t in (0..N).filter(|i| (bits >> i) & 0x1 == 0x1).map(|i| &S[i]) {
            for &c in t {
                *map.entry(c).or_insert(0usize) += 1;
            }
        }
        ans = ans.max(map.iter().filter(|(_, &v)| v == K).count());
    }
    println!("{}", ans);
}
