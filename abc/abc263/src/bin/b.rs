use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        P: [usize; N - 1]
    }
    let mut ans = 0;
    let mut i = N;
    while i != 1 {
        ans += 1;
        i = P[i - 2];
    }
    println!("{}", ans);
}
