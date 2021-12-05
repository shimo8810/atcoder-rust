use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      mut N: usize,
    }
    if N >= 42 {
        N += 1;
    }
    println!("AGC{:03}", N);
}
