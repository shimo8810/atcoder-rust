use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        S: u8,
        W: u8,
    }

    let ans = if S > W { "safe" } else { "unsafe" };
    println!("{}", ans);
}
