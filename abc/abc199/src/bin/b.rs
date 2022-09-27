use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [i32; N],
        B: [i32; N]
    }

    let l = A.into_iter().max().unwrap();
    let r = B.into_iter().min().unwrap();
    let ans = (r - l + 1).max(0);
    println!("{}", ans);
}
