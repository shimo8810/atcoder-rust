use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        A: u8,
        B: u8
    }

    let ans = if B == 0 {
        "Gold"
    } else if A == 0 {
        "Silver"
    } else {
        "Alloy"
    };
    println!("{}", ans);
}
