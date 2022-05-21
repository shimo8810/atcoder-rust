use proconio::{fastout, input};

use std::collections::HashMap;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [u32; N]
    }

    let mut map = HashMap::new();

    for (i, &a) in A.iter().enumerate() {
        map.entry(a).or_insert_with(Vec::new).push(i);
    }

    let list: Vec<_> = map.into_iter().map(|(_, x)| x.len()).collect();
    let mut map = HashMap::new();
    for &x in &list {
        *map.entry(x).or_insert(0) += 1;
    }

    let mut ans = 0;
    let list: Vec<_> = map.into_iter().collect();
    for i in 0..list.len() {
        let (x, p) = list[i];
        if p >= 3 {
            ans += x * x * x * p * (p - 1) * (p - 2) / 6;
        }
        for j in (i + 1)..list.len() {
            let (y, q) = list[j];
            if p >= 2 {
                ans += x * x * y * p * (p - 1) * q / 2;
            }
            if q >= 2 {
                ans += x * y * y * q * (q - 1) * p / 2;
            }
            for k in (j + 1)..list.len() {
                let (z, r) = list[k];

                ans += x * y * z * p * q * r;
            }
        }
    }
    println!("{}", ans);
}
