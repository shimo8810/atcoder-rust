use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        T: usize,
    }

    for _ in 0..T {
        input! {a: u128, s: u128}

        let ans = if 2 * a <= s && (s - 2 * a) & a == 0 {
            YES
        } else {
            NO
        };
        println!("{}", ans);
    }
}
