use proconio::input;
use proconio::marker::Chars;

const YES : &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
fn main() {
    input!{
        mut S: Chars,
        T: Chars
    }
    let mut ans = NO;

    if S == T {
        ans = YES;
    }
    for i in 1..S.len() {
        S.swap(i - 1, i);
            if S == T {
                ans = YES;
            }
        S.swap(i - 1, i);
    }

    println!("{}", ans);
}