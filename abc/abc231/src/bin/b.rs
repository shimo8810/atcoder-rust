use proconio::{fastout, input};
use std::collections::HashMap;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! { N: usize }

    let mut map = HashMap::new();
    for _ in 0..N {
        input! {s: String}
        *map.entry(s).or_insert(0) += 1;
    }
    let (ans, _) = map.iter().max_by_key(|(_, v)| *v).unwrap();
    println!("{}", ans);
}
