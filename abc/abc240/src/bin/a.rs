use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        a: u8,
        b: u8
    }

    let ans = if b - a == 1 || b - a == 9 { YES } else { NO };
    println!("{}", ans);
}
