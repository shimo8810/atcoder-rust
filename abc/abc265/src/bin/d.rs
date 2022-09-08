use proconio::{fastout, input};
use std::collections::HashSet;

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        (P, Q, R): (u64, u64, u64),
        A: [u64; N]
    }

    let mut cumsum = vec![0; N + 1];
    for i in 0..N {
        cumsum[i + 1] = cumsum[i] + A[i];
    }
    let set: HashSet<_> = cumsum.iter().collect();

    for x in 0..N {
        let mut z = cumsum[x] + P;
        if !set.contains(&z) {
            continue;
        }
        z += Q;
        if !set.contains(&z) {
            continue;
        }
        z += R;
        if !set.contains(&z) {
            continue;
        }

        println!("{}", YES);
        return;
    }

    println!("{}", NO);
    // for a in
}
