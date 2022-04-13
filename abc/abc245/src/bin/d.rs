use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [i32; N + 1],
        C: [i32; N + M + 1]
    }

    let mut B = vec![0i32; M + 1];

    B[M] = C[N + M] / A[N];

    for k in (0..M).rev() {
        let mut ab = C[N + k];
        for i in (k + 1)..=M {
            if N + k >= i {
                ab -= B[i] * A[N + k - i];
            }
        }
        B[k] = ab / A[N];
    }

    for b in &B {
        print!("{} ", b);
    }
}
