use proconio::{fastout, input};
use std::cmp::{max, min};

const M: u64 = 1_000_000_007;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      L: u64,
      R: u64
    }

    // let ln = ;
    // let rn = R.to_string().len();
    let mut ans = 0;
    for n in L.to_string().len()..=R.to_string().len() {
        // 同じ桁のやつを全部足す, n 桁数
        let l = max(L, 10u64.pow((n - 1) as u32));
        let r = min(R, 10u64.pow(n as u32) - 1);

        ans = (ans + ((r - l + 1) % M) * ((l + r) % M) * n as u64 / 2) % M;
    }

    println!("{}", ans);
}
