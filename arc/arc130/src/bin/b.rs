use std::collections::HashSet;

use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        C: usize,
        Q: usize,
        query: [(u8, Usize1, Usize1); Q]
    }


    let mut nc = vec![0; C];
    let mut hs= HashSet::new();
    let mut ws= HashSet::new();

    for &(t, n, c) in query.iter().rev() {
         if t == 1 {
             if !hs.contains(&n) {
                hs.insert(n);
                nc[c] += W - ws.len();
             }
         } else {
            if !ws.contains(&n) {
                ws.insert(n);
                nc[c] += H - hs.len();
             }
         }
    }

    let ans = nc.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ");
    println!("{}", ans);
}
