use proconio::input;
use proconio::marker::Chars;

#[allow(non_snake_case)]
fn main() {
    input! {
        S: [String; 3],
        T: Chars,
    }

    for x in T {
        print!("{}", S[x as usize - 49]);
    }
}
