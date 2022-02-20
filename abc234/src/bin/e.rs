use proconio::{fastout, input};
use std::collections::HashSet;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        X: u64,
    }

    let mut sers = HashSet::new();
    for f in 1..=9 {
        for d in -9..9 {
            let mut s = "".to_string();
            let mut dg = f;
            for _ in 0..18 {
                s = format!("{}{}", s, dg);
                let a: u64 = s.parse().unwrap();
                sers.insert(a);
                dg += d;
                if !(0..=9).contains(&dg) {
                    break;
                }
            }
        }
    }

    let mut sers: Vec<_> = sers.iter().collect();
    sers.sort_unstable();
    let idx = sers.binary_search(&&X).unwrap_or_else(|x| x);
    println!("{}", sers[idx]);
}
