use proconio::{fastout, input};
use std::collections::HashMap;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
    }
    let mut map = HashMap::new();
    for _ in 0..N {
        input! {S: String}
        if map.contains_key(&S) {
            println!("{}({})", S, map.get(&S).unwrap());
        } else {
            println!("{}", S);
        }
        *map.entry(S).or_insert(0) += 1;
    }
}
