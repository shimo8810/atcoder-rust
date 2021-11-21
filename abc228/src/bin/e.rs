use proconio::{fastout, input};

const Q: u128 = 998244353;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: u128,
      K: u128,
      M: u128
    }
    let N = N % Q;
    let K = K % Q;
    let M = M % Q;

    let L = ((K * M) % Q) as u128;
    let mut buf = vec![0u128; 1000];
    buf[1] = N;
    for i in 2..1000 {
        buf[i] = (buf[i - 1] * buf[i - 1]) % Q;
    }

    let mut ans = 1;
    for (i, c) in format!("{:b}", L).chars().rev().enumerate() {
        if c == '1' {
            ans = (ans * buf[i + 1]) % Q;
        }
    }
    println!("{}", ans);
    // println!("{}", );
}
