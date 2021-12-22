use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      X: usize,
    }
    let ans = match X {
        0..=39 => (40 - X).to_string(),
        40..=69 => (70 - X).to_string(),
        70..=89 => (90 - X).to_string(),
        _ => "expert".to_string(),
    };

    println!("{}", ans);
}
