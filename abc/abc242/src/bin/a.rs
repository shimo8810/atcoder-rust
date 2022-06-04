use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        (A, B, C, X): (u32, u32, u32, u32)
    }

    let ans = if (..=A).contains(&X) {
        1.0
    } else if ((A + 1)..=B).contains(&X) {
        C as f64 / (B - A) as f64
    } else {
        0.0
    };

    println!("{}", ans);
}
