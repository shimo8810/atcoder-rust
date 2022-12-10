use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        A: u32,
        B: u32
    }

    let ans = A + B;
    println!("{}", ans);
}
