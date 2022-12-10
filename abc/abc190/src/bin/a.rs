use proconio::{fastout, input};

const YES: &str = "Takahashi";
const NO: &str = "Aoki";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        (A, B, C): (u32, u32, u32)
    }

    let ans = if A + C > B { YES } else { NO };
    let mut ans = 0;
    println!("{}", ans);
}
