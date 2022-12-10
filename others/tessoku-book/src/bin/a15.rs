use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [u32; N]
    }

    let mut B = A.clone();
    B.sort_unstable();
    B.dedup();

    for &a in &A {
        let a = B.binary_search(&a).unwrap();
        print!("{} ", a + 1);
    }
}
