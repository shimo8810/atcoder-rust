use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        H: u128,
    }
    let ans = (H * (12800000 + H)) as f64;
    let ans = ans.sqrt();
    println!("{}", ans);
}
