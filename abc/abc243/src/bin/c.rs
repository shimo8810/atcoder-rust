use proconio::{fastout, input, marker::Chars};
use std::collections::HashMap;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        XY: [(u32, u32); N],
        S: Chars
    }
    let mut lmax = HashMap::new();
    let mut rmin = HashMap::new();

    for (i, &(x, y)) in XY.iter().enumerate() {
        if S[i] == 'R' {
            if let Some(&l) = lmax.get(&y) {
                if x < l {
                    println!("Yes");
                    return;
                }
            }
        } else {
            if let Some(&r) = rmin.get(&y) {
                if r < x {
                    println!("Yes");
                    return;
                }
            }
        }

        if S[i] == 'R' {
            let z = rmin.entry(y).or_insert(x);
            *z = (*z).min(x);
        } else {
            let z = lmax.entry(y).or_insert(x);
            *z = (*z).max(x);
        }
    }

    println!("No");
}
