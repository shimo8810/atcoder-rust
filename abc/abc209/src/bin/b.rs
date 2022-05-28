use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        X: usize,
        A: [usize; N]
        }
    let ans = if X >= A.into_iter().sum::<usize>() - N / 2 {
        YES
    } else {
        NO
    };

    println!("{}", ans);

}
