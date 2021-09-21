use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
    X: u8
    }
    let ans = match X {
        x if x < 40 => (40 - X).to_string(),
        x if x < 70 => (70 - X).to_string(),
        x if x < 90 => (90 - X).to_string(),
        _ => "expert".to_string(),
    };
    println!("{}", ans);
}
