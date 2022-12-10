use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
fn bit_search(A: &[u64]) -> Vec<u64> {
    let mut B = vec![];

    for bit in 0..(1 << A.len()) {
        B.push(
            A.iter()
                .enumerate()
                .filter(|&(i, _)| (bit & (1 << i)) != 0)
                .map(|(_, x)| x)
                .sum(),
        );
    }

    B
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: u64,
        A: [u64; N]
    }

    let (A1, A2) = A.split_at(N / 2);
    let B1 = bit_search(A1);
    let mut B2 = bit_search(A2);
    B2.sort_unstable();

    for &b in &B1 {
        if B2.binary_search(&(K - b)).is_ok() {
            println!("{}", YES);
            return;
        }
    }

    println!("{}", NO);
}
