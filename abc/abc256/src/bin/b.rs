use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [u32; N]
    }

    let mut B = vec![0; N + 1];
    for (i, &a) in A.iter().rev().enumerate() {
        B[i + 1] = B[i] + a;
    }
    let ans = B.into_iter().filter(|&x| x >= 4).count();
    println!("{}", ans);
}
