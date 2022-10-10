use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [u32; N]
    }

    let ans: u32 = A.into_iter().sum();
    println!("{}", ans);
}
