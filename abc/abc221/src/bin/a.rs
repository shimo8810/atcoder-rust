use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        A: u32,
        B: u32
    }

    let ans = 32u64.pow(A - B);
    println!("{}", ans);
}
