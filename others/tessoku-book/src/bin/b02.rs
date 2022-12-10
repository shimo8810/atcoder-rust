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

    for x in A..=B {
        if 100 % x == 0 {
            println!("{}", YES);
            return;
        }
    }

    println!("{}", NO);
}
