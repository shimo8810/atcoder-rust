use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        A: usize,
        B: usize
    }

    let ans = if A + B >= 15 && B >= 8 {
        1
    } else if A + B >= 10 && B >= 3 {
        2
    } else if A + B >= 3 {
        3
    } else {
        4
    };
    println!("{}", ans);
}
