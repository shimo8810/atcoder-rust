use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        XY: [(Usize1, Usize1); N],
        Q: usize,
    }
    let S = 1500;

    let mut arr = vec![vec![0; S]; S];
    let mut cs = vec![vec![0; S + 1]; S + 1];

    for &(x, y) in &XY {
        arr[y][x] += 1;
    }

    for (y, row) in arr.iter().enumerate() {
        for (x, &v) in row.iter().enumerate() {
            cs[y + 1][x + 1] = v + cs[y + 1][x] + cs[y][x + 1] - cs[y][x];
        }
    }

    for _ in 0..Q {
        input! {a: usize, b: usize, c: usize, d: usize}
        let ans = cs[d][c] - cs[b - 1][c] - cs[d][a - 1] + cs[b - 1][a - 1];
        println!("{}", ans);
    }
}
