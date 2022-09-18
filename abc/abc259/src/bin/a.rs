use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        _N: usize,
        M: usize,
        X: usize,
        T: usize,
        D: usize,
    }

    let ans = if M > X { T } else { T - (X - M) * D };
    println!("{}", ans);
}
