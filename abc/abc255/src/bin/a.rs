use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        (R, C): (Usize1, Usize1),
        A: [[u32; 2]; 2]
    }

    let ans = A[R][C];
    println!("{}", ans);
}
