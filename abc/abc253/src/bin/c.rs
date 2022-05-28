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
        input! {t: u8}

        if t == 1 {
            input! {x: u32}
            *map.entry(x).or_insert(0) += 1;
        } else if t == 2 {
            input! {x: u32, c: u32}
            if let Some(&n) = map.get(&x) {
                if n > c {
                    *map.get_mut(&x).unwrap() = n - c;
                } else {
                    map.remove(&x);
                }
            }
        } else {
            let (a, _) = map.iter().next().unwrap();
            let (b, _) = map.iter().next_back().unwrap();
            println!("{}", b - a);
        }

        // println!("{}: {:?}", t, map);
    }
}
