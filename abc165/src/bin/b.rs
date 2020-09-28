use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        mut X: u128,
    }
    let mut a = 100u128;
    for i in 1..4000 {
        a = a * 101 / 100;
        if a >= X {
            println!("{}", i);
            return;
        }
    }
}
