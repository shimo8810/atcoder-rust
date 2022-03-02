use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      a: Chars,
      b: Chars
    }
    let mut a: Vec<_> = a.iter().map(|&c| c as u8 - 48).collect();
    let mut b: Vec<_> = b.iter().map(|&c| c as u8 - 48).collect();
    let a
    dbg!(a);
    let ans = 0;
    println!("{}", ans);
}
