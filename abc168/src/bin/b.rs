use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
        mut S: String
    }

    if S.len() > K {
        S.truncate(K);
        S += "...";
    }

    println!("{}", S);
}
