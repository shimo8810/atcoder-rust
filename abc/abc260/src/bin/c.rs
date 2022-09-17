use proconio::{fastout, input};

fn rec(lv: usize, n: usize, m: usize, x: usize, y: usize) -> usize {
    if lv == 1 {
        m
    } else {
        rec(lv - 1, n * (x + 1) + m, (n * x + m) * y, x, y)
    }
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        (N, X, Y): (usize, usize, usize)
    }

    let ans = rec(N, 1, 0, X, Y);
    println!("{}", ans);
}
