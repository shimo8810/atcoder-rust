use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        A: i32,
        B: i32,
        C: i32,
        D: i32
    }

    let x = C * D - B;
    let ans = if x > 0 { (A + x - 1) / x } else { -1 };
    println!("{}", ans);
}
