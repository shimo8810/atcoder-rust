use proconio::{fastout, input, marker::Chars};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      S: Chars,
      T: Chars
    }
    let S: Vec<_> = S.iter().map(|&x| x as i32).collect();
    let T: Vec<_> = T.iter().map(|&x| x as i32).collect();

    let d = (S[0] - T[0] + 26) % 26;
    for i in 0..S.len() {
        if d != (S[i] - T[i] + 26) % 26 {
            println!("{}", NO);
            return;
        }
    }
    println!("{}", YES);
}
