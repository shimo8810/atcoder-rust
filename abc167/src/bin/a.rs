use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        mut T: String,
    }
    T.truncate(S.len());
    let ans = if T == S { "Yes" } else { "No" };

    println!("{}", ans);
}
