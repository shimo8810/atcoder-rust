use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      P: u64,
      Q: u64,
      A: [u64; N]
    }

    let mut ans = 0;
    for i in 0..N {
        for j in 0..i {
            for k in 0..j {
                for l in 0..k {
                    for m in 0..l {
                        if A[i] * A[j] % P * A[k] % P * A[l] % P * A[m] % P == Q {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
