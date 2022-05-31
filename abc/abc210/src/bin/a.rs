use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: i32,
        A: i32,
        X: i32,
        Y: i32
    }

    let ans = A.min(N) * X + 0.max(N - A) * Y;
    println!("{}", ans);
}
