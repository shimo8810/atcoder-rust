#![allow(unused_imports)]

use proconio::input;
use proconio::marker::Usize1;
use std::cmp;
use std::collections::HashMap;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: i64,
    }

    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();
    for i in 1..(N + 1) {
        input! {
            A: i64,
        }

        let count = map1.entry(i - A).or_insert(0);
        *count += 1;
        let count = map2.entry(i + A).or_insert(0);
        *count += 1;
    }

    let mut ans = 0;
    for (&k, &v) in &map1 {
        ans += map2.get(&k).unwrap_or(&0) * v;
    }

    println!("{}", ans);
}
