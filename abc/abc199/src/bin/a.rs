use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        (A, B, C): (u32, u32, u32)
    }

    let ans = if A * A + B * B < C * C { YES } else { NO };
    println!("{}", ans);
}
