use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        X: usize,
    }

    let ans = 100 - X % 100;
    println!("{}", ans);
}
