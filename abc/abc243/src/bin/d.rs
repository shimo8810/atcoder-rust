use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        _N: usize,
        mut X: u64,
        S: Chars
    }

    let mut T = vec![];
    for &c in S.iter() {
        if !T.is_empty() && (T[T.len() - 1] == 'L' || T[T.len() - 1] == 'R') && c == 'U' {
            T.pop().unwrap();
        } else {
            T.push(c);
        }
    }

    for &c in &T {
        X = match c {
            'L' => X * 2,
            'R' => X * 2 + 1,
            _ => X / 2,
        };
    }

    println!("{}", X);
}
