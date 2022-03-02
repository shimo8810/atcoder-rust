use proconio::input;
use proconio::marker::Chars;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: Chars
    }

    let b = N
        .iter()
        .rev()
        .skip_while(|&x| *x == '0')
        .collect::<Vec<_>>();
    let mut c = b.clone();
    c.reverse();
    if b == c {
        println!("Yes");
    } else {
        println!("No");
    }
}
