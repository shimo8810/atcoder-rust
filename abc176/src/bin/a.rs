use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: u32,
        X: u32,
        T: u32,
    }

    let ans = (N + X - 1) / X * T;

    println!("{}", ans);
}
