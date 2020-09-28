use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let res = (1000 - n % 1000) % 1000;
    println!("{:?}", res);
}
