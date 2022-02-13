use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        mut N: u32,
    }
    let mut ans = 0;
    while N > 0 {
        ans += N % 10;
        N /= 10;
    }
    println!("{}", ans * 111);
}
