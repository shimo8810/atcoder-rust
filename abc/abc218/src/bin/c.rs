use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};

const YES: &str = "Yes";
const NO: &str = "No";

fn bb(m: &[Vec<char>]) -> (usize, usize, usize, usize) {
    let mut l = m.len();
    let mut t = m.len();
    let mut r = 0;
    let mut b = 0;

    for (y, row) in m.iter().enumerate() {
        for (x, v) in row.iter().enumerate() {
            if *v == '#' {
                l = min(l, x);
                t = min(t, y);
                r = max(r, x);
                b = max(b, y);
            }
        }
    }
    (l, t, r, b)
}
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      mut S: [Chars; N],
      T: [Chars; N],
    }

    let (tl, tt, tr, tb) = bb(&T);

    for _ in 0..4 {
        // 行列の回転
        let mut R = vec![vec!['\0'; N]; N];

        for (y, row) in S.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                R[N - x - 1][y] = *v;
            }
        }
        let (rl, rt, rr, rb) = bb(&R);

        if rr - rl == tr - tl && rb - rt == tb - tt {
            let w = rr - rl;
            let h = rb - rt;
            let mut f = true;
            for y in 0..h {
                for x in 0..w {
                    if R[rt + y][rl + x] != T[tt + y][tl + x] {
                        f = false;
                    }
                }
            }

            if f {
                println!("{}", YES);
                return;
            }
        }
        std::mem::swap(&mut S, &mut R);
    }

    println!("{}", NO);
}
