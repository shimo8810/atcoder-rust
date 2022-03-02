use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: Usize1
    }

    let ans = (K % N + N + A - 1) % N + 1;
    println!("{}", ans);
}
