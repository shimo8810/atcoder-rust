use proconio::{fastout, input, marker::Usize1};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      K: Usize1
    }

    let mut P = vec![0; N];
    for p in &mut P {
        input! {ps: [u32; 3]}
        *p = ps.iter().sum();
    }

    let mut S = P.clone();
    S.sort_unstable();
    let S: Vec<_> = S.iter().rev().collect();
    let a = S[K];

    for p in P.iter() {
        let ans = if p + 300 >= *a { YES } else { NO };

        println!("{}", ans);
    }
}
