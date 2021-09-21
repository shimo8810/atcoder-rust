use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        X: i8,
        Y: i8
    }
    if (X - Y).abs() < 3 {
        println!("Yes");
    } else {
        println!("No");
    }
}
