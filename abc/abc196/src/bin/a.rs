use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        (_, b): (i32, i32),
        (c, _): (i32, i32),
    }

    let ans = b - c;
    println!("{}", ans);
}
