use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      A: [u32; N]
    }

    for a in A.iter() {
        println!("0b {:04b}", a);
    }

    let ans = 0;
    println!("{}", ans);
}
