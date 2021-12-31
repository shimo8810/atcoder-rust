use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [u32; N]
    }

    let mut dp = vec![vec![vec![0.0; N + 1]; N + 1]; N + 1];
    
}
