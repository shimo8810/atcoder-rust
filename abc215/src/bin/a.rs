use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! { S: String }
    let T = "Hello,World!";

    println!("{}", if S == T { "AC" } else { "WA" });
}
