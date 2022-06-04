use proconio::{fastout, input, marker::Chars};

const T: &str = "Takahashi";
const A: &str = "Aoki";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        _N: usize,
        S: Chars
    }
    let ans = if S.into_iter().position(|c| c == '1').unwrap() % 2 == 0 {
        T
    } else {
        A
    };
    println!("{}", ans);
}
