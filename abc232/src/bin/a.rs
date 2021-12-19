use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      S: Chars,
    }

    let a = S[0] as u32 - 48;
    let b = S[2] as u32 - 48;
    let ans = a * b;
    println!("{}", ans);
}
