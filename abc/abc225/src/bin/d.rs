use proconio::{fastout, input};
use std::collections::HashMap;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      Q: usize
    }

    let mut h: HashMap<_, _> = (0..N).map(|x| (x + 1, None)).collect();
    let mut t: HashMap<_, _> = (0..N).map(|x| (x + 1, None)).collect();

    for _ in 0..Q {
        input! {q: u8}

        if q == 1 {
            input! {x: usize, y: usize}
            t.insert(x, Some(y));
            h.insert(y, Some(x));
        } else if q == 2 {
            input! {x: usize, y: usize}
            t.insert(x, None);
            h.insert(y, None);
        } else {
            input! {mut x: usize}
            while let Some(y) = *h.get(&x).unwrap() {
                x = y;
            }
            let mut v = vec![x.to_string()];
            while let Some(y) = *t.get(&x).unwrap() {
                v.push(y.to_string());
                x = y;
            }

            println!("{} {}", v.len(), v.join(" "));
        }
    }
}
