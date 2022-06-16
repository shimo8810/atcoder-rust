use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [Usize1; N],
        B: [Usize1; N],
        C: [Usize1; N]
    }

    let mut X = vec![0; N];

    for &c in &C {
        X[B[c]] += 1;
    }
    let ans: usize = A.into_iter().map(|a| X[a]).sum();

    println!("{}", ans);
}
