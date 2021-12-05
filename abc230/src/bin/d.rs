use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      D: usize,
      mut LR: [(usize, usize); N]
    }

    LR.sort_unstable_by_key(|(_, r)| *r);
    let mut ans = 0;
    let mut x = 0;
    for &(l, r) in LR.iter() {
        if x < l {
            ans += 1;
            x = r + D - 1;
        }
    }
    println!("{}", ans);
}
