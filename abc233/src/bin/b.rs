use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      L: Usize1,
      R: Usize1,
      S: Chars
    }
    let a: String = S[..L].iter().collect();
    let b: String = S[L..=R].iter().rev().collect();
    let c: String = S[(R + 1)..].iter().collect();
    print!("{}{}{}", a, b, c);
}
