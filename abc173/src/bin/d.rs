use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u64; n]
    }

    a.sort_by(|x, y| y.cmp(x));
    let mut res = 0;
    for i in 0..(a.len() - 1) {
        res += a[(i + 1) / 2];
    }
    println!("{}", res);
}
