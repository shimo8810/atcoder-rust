use proconio::{fastout, input};

const M: usize = 46;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! { N: usize }

    let mut A = vec![0u64; M];
    let mut B = vec![0u64; M];
    let mut C = vec![0u64; M];

    for _ in 0..N {
        input! {x: usize}
        A[x % M] += 1;
    }
    for _ in 0..N {
        input! {x: usize}
        B[x % M] += 1;
    }
    for _ in 0..N {
        input! {x: usize}
        C[x % M] += 1;
    }

    let mut ans = 0;
    for (i, a) in A.iter().enumerate() {
        for (j, b) in B.iter().enumerate() {
            for (k, c) in C.iter().enumerate() {
                if (i + j + k) % M == 0 {
                    ans += a * b * c;
                }
            }
        }
    }

    println!("{}", ans);
}
