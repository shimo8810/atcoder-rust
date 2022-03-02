use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};
use std::collections::HashSet;

const YES: &str = "Yes";
const NO: &str = "No";
use std::cmp::{max, min};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(Usize1, Usize1); M],
        CD: [(Usize1, Usize1); M]
    }
    let CD: HashSet<_> = CD.iter().collect();

    for P in (0..N).permutations(N) {
        let mut f = true;
        for &(a, b) in AB.iter() {
            let c = min(P[a], P[b]);
            let d = max(P[a], P[b]);

            if !CD.contains(&(c, d)) {
                f = false;
                break;
            }
        }
        if f {
            println!("{}", YES);
            return;
        }
    }
    println!("{}", NO);
}
