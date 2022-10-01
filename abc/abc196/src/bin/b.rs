use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        mut X: Chars,
    }
    X.push('.');
    let ans: String = X.iter().take_while(|&&x| x != '.').collect();
    println!("{}", ans);
}
