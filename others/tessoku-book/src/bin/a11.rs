use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        X: u32,
        A: [u32; N]
    }

    let ans = A.binary_search(&X).unwrap();
    println!("{}", ans + 1);
}
