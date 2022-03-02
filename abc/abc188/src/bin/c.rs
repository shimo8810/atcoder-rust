use proconio::input;
use std::cmp;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [u32; 2usize.pow(N as u32)]
    }
    let c = 2usize.pow(N as u32 - 1);
    let ans = cmp::min(A[..c].iter().max().unwrap(), A[c..].iter().max().unwrap());
    println!("{}", ans);
}
