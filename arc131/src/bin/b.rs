use proconio::{fastout, input, marker::Chars};
use std::collections::HashSet;
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      H: usize,
      W: usize,
      mut D: [Chars; H]
    }

    for h in 0..H {
        for w in 0..W {
            if D[h][w] == '.' {
                let mut cs: HashSet<_> = ['1', '2', '3', '4', '5'].iter().collect();
                if h > 0 {
                    cs.remove(&D[h - 1][w]);
                }
                if w > 0 {
                    cs.remove(&D[h][w - 1]);
                }
                if h + 1 < H {
                    cs.remove(&D[h + 1][w]);
                }
                if w + 1 < W {
                    cs.remove(&D[h][w + 1]);
                }
                let a = **cs.iter().next().unwrap();
                D[h][w] = a;
            }
            print!("{}", D[h][w]);
        }
        println!();
    }
}
