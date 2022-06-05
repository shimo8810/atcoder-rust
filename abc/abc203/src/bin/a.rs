use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        (a, b, c): (u8, u8, u8)
    }

    let ans = if a == b {
        c
    } else if b == c {
        a
    } else if c == a {
        b
    } else {
        0
    };
    println!("{}", ans);
}
