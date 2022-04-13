use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        (S1, S2, S3): (char, char, char),
        (T1, T2, T3): (char, char, char),
    }
    let ans = if (S1 == T1 && S2 == T2 && S3 == T3) || (S1 != T1 && S2 != T2 && S3 != T3) {
        YES
    } else {
        NO
    };
    println!("{}", ans);
}
