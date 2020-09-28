use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u32,
        b: u32,
    }

    if (a % 2) * (b % 2) == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
