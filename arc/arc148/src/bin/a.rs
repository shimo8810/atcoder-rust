use num::integer::gcd;
use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        mut A: [i32; N]
    }

    println!(
        "{}",
        if A.windows(2)
            .fold(0, |prev, a| gcd(prev, (a[0] - a[1]).abs()))
            == 1
        {
            2
        } else {
            1
        }
    );
}
