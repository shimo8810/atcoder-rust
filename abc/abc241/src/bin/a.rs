use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        A: [usize; 10]
    }

    let mut ans = 0;
    for _ in 0..3 {
        ans = A[ans];
    }
    println!("{}", ans);
}
