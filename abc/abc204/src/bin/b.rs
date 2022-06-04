use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [i32; N]
    }

    let ans: i32 = A.into_iter().map(|a| (a - 10).max(0)).sum();
    println!("{}", ans);
}
