use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        mut X: i32,
        mut Y: i32,
        mut Z: i32
    }

    if X < 0 {
        X *= -1;
        Y *= -1;
        Z *= -1;
    }

    // スタートとゴールの間に壁がないとき
    if !(0..=X).contains(&Y) {
        println!("{}", X.abs());
        return;
    }

    // 間にあるが槌を回収できる場合
    if Z <= Y {
        println!("{}", Z.abs() + (X - Z).abs());
        return;
    }

    println!("-1");
}
