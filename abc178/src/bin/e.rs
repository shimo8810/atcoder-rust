use proconio::input;
use std::cmp;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        xy: [(i64, i64); N]
    }

    let mut zmin = std::i64::MAX;
    let mut zmax = std::i64::MIN;
    let mut wmin = std::i64::MAX;
    let mut wmax = std::i64::MIN;

    for (x, y) in xy {
        let z = x + y;
        let w = x - y;

        zmin = cmp::min(z, zmin);
        zmax = cmp::max(z, zmax);
        wmin = cmp::min(w, wmin);
        wmax = cmp::max(w, wmax);
    }

    let ans = cmp::max(zmax - zmin, wmax - wmin);
    println!("{}", ans);
}
