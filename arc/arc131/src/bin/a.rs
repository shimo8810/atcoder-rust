use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      A: u128,
      B: u128
    }

    let d = format!("{}", A).len() as u32;

    let mut ans = B / 2 * 10u128.pow(d + 1) + A;
    if B % 2 != 0 {
        ans += 5 * 10u128.pow(d);
    }

    println!("{}", ans);
}
