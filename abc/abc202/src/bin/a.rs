use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        (a, b, c): (u8, u8, u8)
    }

    let mut ans = 21 - a - b - c;
    println!("{}", ans);
}
