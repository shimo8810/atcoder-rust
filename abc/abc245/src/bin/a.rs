use proconio::{fastout, input};

const Y: &str = "Takahashi";
const N: &str = "Aoki";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        A: u32,
        B: u32,
        C: u32,
        D: u32,
    }

    let ans = if A * 3600 + B * 60 < C * 3600 + D * 60 + 1 {
        Y
    } else {
        N
    };
    println!("{}", ans);
}
