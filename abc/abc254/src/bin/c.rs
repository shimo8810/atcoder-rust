use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
        mut A: [u32; N]
    }

    let mut bs = vec![vec![]; K];
    let mut C = vec![0; N];
    for (i, &a) in A.iter().enumerate() {
        bs[i % K].push(a);
    }
    A.sort_unstable();
    for (i, b) in bs.iter_mut().enumerate() {
        b.sort_unstable();
        for (j, x) in b.iter().enumerate() {
            C[i + j * K] = *x;
        }
    }
    let ans = if A == C { YES } else { NO };
    println!("{}", ans);
}
