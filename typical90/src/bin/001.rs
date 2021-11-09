use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      L: u64,
      K: usize,
      A: [u64; N]
    }

    let mut left = 0; // 条件を満たす
    let mut right = L; // 条件を満たさない

    while right - left > 1 {
        let x = (right + left) / 2;

        let mut k = 0;
        let mut p = 0;

        for &a in A.iter() {
            if a - p >= x && L - a >= x {
                p = a;
                k += 1;
            }
        }
        if k >= K {
            left = x;
        } else {
            right = x;
        }
    }
    println!("{}", left);
}
