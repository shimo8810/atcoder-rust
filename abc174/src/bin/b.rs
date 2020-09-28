use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        D: i64,
        XY: [(i64, i64); N],
    }

    let mut ans = 0;
    for (x, y) in XY {
        if D * D >= x * x + y * y {
            ans += 1;
        }
    }

    println!("{}", ans);
}
