use proconio::{fastout, input};
use std::collections::HashMap;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        X: u64,
    }

    let mut set = HashMap::new();
    set.insert(X, 1u64);

    for _ in 0..N {
        // set.remove(&1);
        input! {
            L: usize,
            A: [u64; L]
        }
        let mut next = HashMap::new();

        for (x, n) in set.iter() {
            for a in A.iter() {
                let p = x / a;
                let q = x % a;
                if q == 0 && p != 0 {
                    *next.entry(p).or_default() += n;
                }
            }
        }

        std::mem::swap(&mut set, &mut next);
        // println!("{:?}", set);
    }

    if let Some(ans) = set.get(&1) {
        println!("{}", ans);
    } else {
        println!("0");
    }
}
