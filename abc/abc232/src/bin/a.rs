use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      S: Chars,
    }

    let a = S[0] as u8 - b'0';
    let b = S[2] as u8 - b'0';
    let ans = a * b;
    println!("{}", ans);
}
