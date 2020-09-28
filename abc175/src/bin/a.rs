use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
    S: String
    }

    let ans = if S.contains("RRR") {
        3
    } else if S.contains("RR") {
        2
    } else if S.contains("R") {
        1
    } else {
        0
    };

    println!("{}", ans);
}
