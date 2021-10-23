use proconio::{input, marker::Chars};

#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars
    }

    let &c = S.last().unwrap();
    let ans = if c == 'r' { "er" } else { "ist" };
    println!("{}", ans);
}
