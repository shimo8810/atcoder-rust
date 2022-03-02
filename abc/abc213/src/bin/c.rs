use proconio::{fastout, input};
use std::collections::{HashMap, HashSet};
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        _H: usize,
        _W: usize,
        N: usize,
        AB: [(usize, usize);N ]
    }

    // 2次元座標圧縮
    let mut xs = HashSet::new();
    let mut ys = HashSet::new();

    for &(x, y) in AB.iter() {
        xs.insert(x);
        ys.insert(y);
    }
    let mut xs: Vec<_> = xs.iter().collect();
    let mut ys: Vec<_> = ys.iter().collect();
    xs.sort_unstable();
    ys.sort_unstable();
    let xs: HashMap<_, _> = xs.iter().enumerate().map(|(i, &x)| (x, i + 1)).collect();
    let ys: HashMap<_, _> = ys.iter().enumerate().map(|(i, &x)| (x, i + 1)).collect();

    for (x, y) in &AB {
        println!("{} {}", xs.get(x).unwrap(), ys.get(y).unwrap());
    }
}
