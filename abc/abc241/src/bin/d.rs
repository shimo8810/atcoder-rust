use proconio::{fastout, input};
use std::collections::BTreeMap;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        Q: usize,
    }
    let mut map = BTreeMap::new();

    for _ in 0..Q {
        input! {t: u8, x: u64}
        if t == 1 {
            *map.entry(x).or_insert(0) += 1;
        } else if t == 2 {
            input! {mut k: u64}
            let mut range = map.range(..=x);
            let mut flag = true;
            while let Some((&v, &n)) = range.next_back() {
                if k <= n {
                    println!("{}", v);
                    flag = false;
                    break;
                }
                k -= n;
            }
            if flag {
                println!("-1");
            }
        } else {
            input! {mut k: u64}
            let mut range = map.range(x..);
            let mut flag = true;
            while let Some((&v, &n)) = range.next() {
                if k <= n {
                    println!("{}", v);
                    flag = false;
                    break;
                }
                k -= n;
            }
            if flag {
                println!("-1");
            }
        }
    }
}
