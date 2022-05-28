use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        A: u32,
        B: u32
    }

    let ans = if (A..=A * 6).contains(&B) { YES } else { NO };
    println!("{}", ans);
}
