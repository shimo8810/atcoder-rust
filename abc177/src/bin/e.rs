use num::integer::gcd;
use proconio::input;
use std::cmp;

#[allow(non_snake_case)]
fn main() {
    input! {

        N: usize,
        A:[u64; N]
    }

    for &a in A.iter() {
        let mut c = 1;
        let mut d = std::u64::MAX;
        for &b in A.iter() {
            if b * b > a {
                break;
            }
            let g = gcd(b, a);
            c = cmp::max(c, g);
            d = cmp::min(d, g);
        }
    }
}
