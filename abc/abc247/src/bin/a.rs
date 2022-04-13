use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        S: String,
    }

    let ans = format!("0{}", &S[..3]);
    println!("{}", ans);
}
