use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: u32,
        X: u32
    }
    let ans = (b'A' + ((X - 1) / N) as u8) as char;
    println!("{}", ans);
}
