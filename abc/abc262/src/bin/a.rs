use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        Y: usize,
    }

    let ans = ((Y + 1) / 4) * 4 + 2;
    println!("{}", ans);
}
