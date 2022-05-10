use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        S: Chars
    }
    println!("{}", S[N - 1]);
}