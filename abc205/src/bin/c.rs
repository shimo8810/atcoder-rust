use proconio::{fastout, input};
use std::cmp::Ordering::*;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        mut A: i32,
        mut B: i32,
        C: u32

    }

    if C % 2 == 0 {
        A = A.abs();
        B = B.abs();
    }
    let ans = match A.cmp(&B) {
        Less => "<",
        Equal => "=",
        Greater => ">",
    };

    println!("{}", ans);
}
