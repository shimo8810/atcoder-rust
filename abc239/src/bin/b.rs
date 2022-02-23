use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        X: i64,
    }

    if X > 0 {
        println!("{}", X / 10);
    } else {
        let os = if X % 10 == 0 { 0 } else { -1 };
        println!("{}", X / 10 + os);
    }
}
