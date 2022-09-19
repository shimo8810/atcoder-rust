use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        K: u32,
    }

    let hh = 21 + K / 60;
    let mm = K % 60;
    println!("{:0>2}:{:0>2}", hh, mm);
}
