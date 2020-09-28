use proconio::input;

#[allow(unused_assignments)]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: i64,
        B: i64,
        _: i64,
        K: i64,
    }

    if K < A {
        println!("{}", K);
    } else if K < A + B {
        println!("{}", A);
    } else {
        println!("{}", A - (K - A - B));
    }
}
