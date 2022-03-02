use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        A: u32,
        B: u32,
        C: u32
    }
    let ans = B / C * C;
    if A <= ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
