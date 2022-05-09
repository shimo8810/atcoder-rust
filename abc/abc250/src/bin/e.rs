use proconio::{fastout, input, marker::Usize1};
use std::collections::HashSet;

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [u32; N],
        B: [u32; N],
        Q: usize,
    }
    let mut c = vec![];
    let mut set = HashSet::new();
    let mut alens = vec![];
    // let d = vec![];
    for &a in &A {
        if !set.contains(&a) {
            set.insert(a);
            c.push(a);
        }
        alens.push(set.len());
    }
    let mut d = vec![];
    let mut set = HashSet::new();
    let mut blens = vec![];
    for &b in &B {
        if !set.contains(&b) {
            set.insert(b);
            d.push(b);
        }
        blens.push(set.len());
    }

    let n = c.len().min(d.len());

    let mut check = vec![false; n];
    let mut set = HashSet::new();
    for i in 0..n {
        if set.contains(&c[i]) {
            set.remove(&c[i]);
        } else {
            set.insert(c[i]);
        }

        if set.contains(&d[i]) {
            set.remove(&d[i]);
        } else {
            set.insert(d[i]);
        }

        if set.is_empty() {
            check[i] = true;
        }
    }

    for _ in 0..Q {
        input! {x: Usize1, y: Usize1}

        let ans = if alens[x] == blens[y] && check[alens[x] - 1] {
            YES
        } else {
            NO
        };

        println!("{}", ans);
    }
}
