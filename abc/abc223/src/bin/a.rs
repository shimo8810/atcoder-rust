use proconio::input;

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize
    }

    let ans = if X > 0 && X % 100 == 0 { YES } else { NO };
    println!("{}", ans);
}
