use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        X: i64,
        A: i64,
        D: i64,
        N: i64
    }

    if D == 0 {
        println!("{}", (X - A).abs());
        return;
    }

    let n1 = N.min(1.max((X - A) / D + 1));
    let n2 = N.min(1.max((X - A + D - 1) / D + 1));

    let ans = (A + D * (n1 - 1) - X)
        .abs()
        .min((A + D * (n2 - 1) - X).abs());

    println!("{}", ans);
}
