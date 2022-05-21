use proconio::{fastout, input, marker::Usize1};
use std::collections::HashMap;

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [u32; N],
        B: [Usize1; K]
    }
    let mut map = HashMap::new();

    let mut min = 0;
    for (i, &a) in A.iter().enumerate() {
        min = min.max(a);
        map.entry(a).or_insert_with(Vec::new).push(i);
    }

    for &x in map.get(&min).unwrap() {
        for &b in &B {
            if x == b {
                println!("{}", YES);
                return;
            }
        }
    }
    println!("{}", NO);
}
