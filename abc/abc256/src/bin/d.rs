use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        mut LR: [(u32, u32); N]
    }
    LR.sort_unstable();

    let mut h = LR[0].0;
    let mut t = LR[0].1;

    for (l, r) in LR.into_iter() {
        if t < l {
            println!("{} {}", h, t);
            h = l;
            t = r;
        } else if t < r {
            t = r;
        }
    }
    println!("{} {}", h, t);
}
