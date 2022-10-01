use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        M: u32,
        H: u32
    }

    let ans = if H % M == 0 { YES } else { NO };
    println!("{}", ans);
}
