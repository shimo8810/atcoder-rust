use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        K: i32,
        A: i32,
        B: i32,
    }

    for a in A..=B {
        if a % K == 0 {
            println!("OK");
            return;
        }
    }
    println!("NG");
}
