use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize
    }

    let ans = if (2..=4).contains(&N) { NO } else { YES };

    println!("{}", ans);
}
