use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        X: u32,
        A: [u32; N]
    }

    let ans = if A.into_iter().any(|x| x == X) {
        YES
    } else {
        NO
    };

    println!("{}", ans);
}
