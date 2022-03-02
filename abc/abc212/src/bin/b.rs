use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        X: Chars,
    }
    let X: Vec<_> = X.iter().map(|&x| x as u8 - b'0').collect();

    let ans = if X.iter().all(|x| X[0] == *x)
        || X.iter()
            .take(3)
            .zip(&X[1..])
            .all(|(&x, &y)| (x + 1) % 10 == y)
    {
        "Weak"
    } else {
        "Strong"
    };

    println!("{}", ans);
}
