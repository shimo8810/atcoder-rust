use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        X: [[u32; W]; H],
        Q: usize
    }

    let mut cs = vec![vec![0; W + 1]; H + 1];

    for y in 1..=H {
        for x in 1..=W {
            cs[y][x] = X[y - 1][x - 1] + cs[y][x - 1];
        }
    }
    for y in 1..=H {
        for x in 1..=W {
            cs[y][x] += cs[y - 1][x];
        }
    }

    for _ in 0..Q {
        input! {A: usize, B: usize, C: usize, D: usize}
        let ans = cs[C][D] - cs[A - 1][D] - cs[C][B - 1] + cs[A - 1][B - 1];
        println!("{}", ans);
    }
}
