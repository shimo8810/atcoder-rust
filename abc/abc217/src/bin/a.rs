use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        T: String
    }

    let ans = if S < T { "Yes" } else { "No" };
    println!("{}", ans);
}
