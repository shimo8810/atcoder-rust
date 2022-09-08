use itertools::Itertools;
use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        (H1, W1): (usize, usize),
        A: [[u32; W1]; H1],
        (H2, W2): (usize, usize),
        B: [[u32; W2]; H2],
    }

    for h in (0..H1).combinations(H2) {
        for w in (0..W1).combinations(W2) {
            let mut flag = true;
            for (i, &y) in h.iter().enumerate() {
                for (j, &x) in w.iter().enumerate() {
                    if B[i][j] != A[y][x] {
                        flag = false;
                        break;
                    }
                }
            }
            if flag {
                println!("{}", YES);
                return;
            }
        }
    }
    println!("{}", NO);
}
