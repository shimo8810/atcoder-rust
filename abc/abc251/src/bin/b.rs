use proconio::{fastout, input};
use std::collections::HashSet;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        W: u32,
        A: [u32; N]
    }

    let mut set = HashSet::new();
    for (i, &a) in A.iter().enumerate() {
        if a <= W {
            set.insert(a);
        }
        for (j, &b) in A.iter().enumerate().skip(i + 1) {
            if a + b <= W {
                set.insert(a + b);
            }
            for &c in A.iter().skip(j + 1) {
                if a + b + c <= W {
                    set.insert(a + b + c);
                }
            }
        }
    }
    println!("{}", set.len());
    // println!("{}", ans);
}
