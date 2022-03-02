use proconio::input;
use proconio::marker::{Chars, Usize1};

#[allow(non_snake_case)]
fn main() {
    input! {
        N :Usize1,
        S: Chars
    }

    println!("{}", if S[N] == 'o' { "Yes" } else { "No" });
}
