use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        (x,y) : (u8, u8)
    }
    let ans = if x == y { x } else { 0b11 - (x ^ y) };
    println!("{}", ans);
}
