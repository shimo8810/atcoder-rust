use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        mut A: [i32; N],
        mut B: [i32; M]
    }

    A.sort_unstable();
    B.sort_unstable();

    let mut ans = std::i32::MAX;
    for &a in &A {
        let i = B.binary_search(&a).unwrap_or_else(|x| x);
        if i < M {
            ans = ans.min((B[i] - a).abs());
        }
        if i > 0 {
            ans = ans.min((a - B[i - 1]).abs());
        }
    }

    println!("{}", ans);
}
