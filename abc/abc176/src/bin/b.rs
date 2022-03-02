use proconio::input;
use proconio::marker::Chars;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: Chars,
    }

    let mut sum = 0;
    for c in N {
        sum += c as u32 - '0' as u32;
    }

    if sum % 9 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
