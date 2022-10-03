use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        R: f64,
        X: f64,
        Y: f64
    }

    if X + Y == 0.0 {
        println!("0");
        return;
    }

    let dist = (X * X + Y * Y).sqrt();

    let mut ans = (dist / R).ceil();
    if ans == 1.0 && dist != ans {
        ans += 1.0;
    }
    println!("{}", ans);
}
