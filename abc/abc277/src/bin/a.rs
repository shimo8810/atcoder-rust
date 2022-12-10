use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        X: u8,
        P: [u8; N]
    }

    let ans = P.into_iter().position(|x| x == X).unwrap() + 1;
    println!("{}", ans);
}
