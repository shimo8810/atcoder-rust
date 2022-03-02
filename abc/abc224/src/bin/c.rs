use itertools::Itertools;
use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        XY: [(i64, i64); N]
    }

    let mut ans = 0;

    for n in (0..N).combinations(3) {
        let a = XY[n[0]];
        let b = XY[n[1]];
        let c = XY[n[2]];

        let dx1 = b.0 - a.0;
        let dy1 = b.1 - a.1;
        let dx2 = c.0 - a.0;
        let dy2 = c.1 - a.1;

        if dx2 * dy1 != dx1 * dy2 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
