use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

fn f(a: (i32, i32), b: (i32, i32), c: (i32, i32)) -> i32 {
    (c.1 - a.1) * (b.0 - a.0) - (b.1 - a.1) * (c.0 - a.0)
}
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        P: [(i32, i32); 4]
    }

    let ans = if (0..4).all(|i| f(P[i], P[(i + 2) % 4], P[(i + 1) % 4]) < 0) {
        YES
    } else {
        NO
    };
    println!("{}", ans);
}
