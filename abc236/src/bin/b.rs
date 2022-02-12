use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [Usize1; N * 4 - 1]
    }
    let mut counts = vec![0; N];
    for &a in &A {
        counts[a] += 1usize;
    }
    let ans = counts.iter().position(|&x| x != 4).unwrap();

    println!("{}", ans + 1);
}
