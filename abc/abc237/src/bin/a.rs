use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: i64,
    }
    let r: i64 = 1 << 31;
    let l: i64 = -r;
    let ans = if (l..r).contains(&N) { YES } else { NO };
    println!("{}", ans);
}
