use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        (a, b, c): (u8, u8, u8),
    }
    let ans = if (a <= b && b <= c) || (c <= b && b <= a) {
        YES
    } else {
        NO
    };
    println!("{}", ans);
}
