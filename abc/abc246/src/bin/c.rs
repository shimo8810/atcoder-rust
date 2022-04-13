use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
        X: usize,
        A: [usize; N]
    }

    let mut ans: usize = A.iter().sum();
    let mut m = 0;
    let mut a: Vec<_> = A
        .iter()
        .map(|x| {
            m += x / X;
            x % X
        })
        .collect();
    a.sort_unstable();
    a.reverse();
    ans -= m.min(K) * X;

    let mut k = 0;
    while k < a.len() && (m + k) < K {
        ans -= a[k];
        k += 1;
    }
    println!("{}", ans);
}
