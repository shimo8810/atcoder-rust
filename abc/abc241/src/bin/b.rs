use proconio::{fastout, input};
use std::collections::HashMap;

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [u32; N],
        B: [u32; M]
    }
    let mut map = HashMap::new();
    for &a in &A {
        *map.entry(a).or_insert(0) += 1;
    }
    for &b in &B {
        if let Some(x) = map.get_mut(&b) {
            *x -= 1;
            if *x == 0 {
                map.remove(&b);
            }
        } else {
            println!("{}", NO);
            return;
        }
    }
    println!("{}", YES);
}
