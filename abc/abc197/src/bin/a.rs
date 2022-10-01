
use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        S: String,
    }

    let ans = format!("{}{}", &S[1..], &S[..1]);
    println!("{}", ans);
}
