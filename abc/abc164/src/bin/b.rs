use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        A: i32,
        B: i32,
        C: i32,
        D: i32,
    }

    let ans = if (A + D - 1) / D >= (C + B - 1) / B { "Yes" } else { "No" };
    println!("{}", ans);
}
