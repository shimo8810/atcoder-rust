use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        mut N: f32,
    }
    let ans = match (N * 1.08) as u32 {
        0..=205 => "Yay!",
        206 => "so-so",
        _ => ":(",
    };
    println!("{}", ans);
}
