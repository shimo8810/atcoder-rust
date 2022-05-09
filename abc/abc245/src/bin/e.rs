use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        (N, M): (usize, usize),
        A: [u32; N],
        B: [u32; N],
        C: [u32; M],
        D: [u32; M]
    }

    let mut ans = 0;
    println!("{}", ans);
}
