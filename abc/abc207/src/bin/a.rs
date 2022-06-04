use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        X: [u32; 3]
    }

    let mut ans = 0;
    for i in 0..3 {
        ans = ans.max(X[i] + X[(i + 1) % 3]);
    }
    println!("{}", ans);
}
