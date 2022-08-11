use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        mut X: [u8; 5],
    }

    X.sort_unstable();
    let ans = if X[0] == X[2] && X[3] == X[4] || X[0] == X[1] && X[2] == X[4] {
        YES
    } else {
        NO
    };

    println!("{}", ans);
}
