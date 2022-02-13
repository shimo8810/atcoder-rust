use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        XY: [(i64, i64); N]
    }

    let mut ans = 0;
    for (x1, y1) in &XY {
        for (x2, y2) in &XY {
            ans = ans.max((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1))
        }
    }
    println!("{}", (ans as f64).sqrt());
}
