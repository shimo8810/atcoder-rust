use proconio::{fastout, input};

const M: u64 = 1_000_000_007;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      K: u64
    }

    let mut buf = vec![0u64; 61]; // log2(1e18) < 60
    buf[1] = K - 2;
    // let mut n = 1;
    let mut k = 0;
    for i in 2..=60 {
        buf[i] = (buf[i - 1] * buf[i - 1]) % M;
        // n *= 2;
        k = i;
    }
    // println!("{}", n);
    println!("{:?}", &buf[..k]);
}
