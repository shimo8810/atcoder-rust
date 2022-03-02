use proconio::input;
use std::cmp;
#[allow(non_snake_case)]
fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64
    }

    let res = cmp::max(cmp::max(a * c, a * d), cmp::max(b * c, b * d));
    println!("{}", res);
}
