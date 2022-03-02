use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        A: u128,
        B: f64,
    }
    println!("{}", (B * 100.0) as u128 * A / 100);
}
