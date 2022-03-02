use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      X: f32,
    }

    let ans = X.round() as u32;
    println!("{}", ans);
}
