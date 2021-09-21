use proconio::input;
use proconio::marker::Usize1;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: [Usize1; N]
    }
    // let ans =
    let mut Q = vec!["".to_string(); N];
    for (i, &x) in P.iter().enumerate() {
        Q[x] = (i + 1).to_string();
    }
    let ans = Q.join(" ");
    println!("{}", ans);
}
