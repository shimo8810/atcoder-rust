use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        D: usize,
        T: usize,
        S: usize,
    }

    if (D + S - 1) / S > T {
        println!("No");
    } else {
        println!("Yes");
    }
}
