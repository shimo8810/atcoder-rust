use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        mut N: usize,
    }

    let mut ans = 0;
    let mut k = 0;
    while N > 0 {
        ans += (N & 1) * (1 << k);
        k += 1;
        N /= 10;
    }

    println!("{}", ans);
}
