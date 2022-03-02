use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        S: [String; N],
        T: [String; M]
    }

    let mut k = 0;
    for s in &S {
        let ans = if &T[k] == s {
            k += 1;
            YES
        } else {
            NO
        };

        println!("{}", ans);
    }
}
