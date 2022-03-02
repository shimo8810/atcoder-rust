use proconio::{fastout, input};
use std::collections::{BTreeSet, HashMap};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! { Q: usize }
    let N = 1u64 << 20;
    let mut s: BTreeSet<_> = (0..N).collect(); // A == -1
    let mut m = HashMap::new(); // A != -1

    for _ in 0..Q {
        input! {t: u8, mut x: u64}
        let h = x % N;
        if t == 1 {
            let i = if let Some(&i) = s.range(h..).next() {
                i
            } else {
                *s.iter().next().unwrap()
            };
            m.insert(i, x);
            s.remove(&i);
        } else {
            let ans = if let Some(z) = m.get(&h) {
                format!("{}", z)
            } else {
                "-1".to_string()
            };
            println!("{}", ans);
        }
    }
}
