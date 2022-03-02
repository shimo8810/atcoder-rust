use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        mut A: [u32; N]
    }
    let mut A: Vec<_> = A.iter().enumerate().map(|(i, &x)| (x, i + 1)).collect();
    A.sort_unstable();
    println!("{}", A[N - 2].1);
}
