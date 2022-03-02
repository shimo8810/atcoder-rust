use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      S: String,
    }
    let T = "oxxoxxoxxoxx";
    for i in 0..3 {
        if T[i..(i + S.len())] == S[..] {
            println!("{}", YES);
            return;
        }
    }

    println!("{}", NO);
}
