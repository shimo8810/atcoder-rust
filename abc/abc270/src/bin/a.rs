use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        (A, B): (u8, u8),
    }
    println!("{}", A | B);
}
