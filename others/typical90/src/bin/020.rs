use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        a: u64,
        b: u32,
        c: u64
    }
    let ans = if a < c.pow(b) { YES } else { NO };
    println!("{}", ans);
}
