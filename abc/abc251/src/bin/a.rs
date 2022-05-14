use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        S: String,
    }
    let ans = if S.len() == 1 {
        format!("{}{}{}{}{}{}", S, S, S, S, S, S)
    } else if S.len() == 2 {
        format!("{}{}{}", S, S, S)
    } else {
        format!("{}{}", S, S)
    };

    println!("{}", ans);
}
