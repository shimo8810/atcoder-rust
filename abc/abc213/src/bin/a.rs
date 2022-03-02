use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        A: u8,
        B: u8
    }
    println!("{}", A ^ B);
}
