use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        mut A: [u8; 3],
    }

    A.sort_unstable();
    let ans = if A[2] - A[1] == A[1] - A[0] { YES } else { NO };
    println!("{}", ans);
}
