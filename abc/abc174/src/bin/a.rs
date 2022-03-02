use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        X: i32,
    }

    if X >= 30 {
        println!("Yes");
    } else {
        println!("No");
    }
}
