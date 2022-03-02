use proconio::{fastout, input};

const M: usize = 1_000;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
    }

    // 二次元いもす
    let mut tile = vec![vec![0i64; M + 2]; M + 2];

    for _ in 0..N {
        input! {lx: usize, ly: usize, rx: usize, ry:usize}
        tile[ly][lx] += 1;
        tile[ly][rx] -= 1;
        tile[ry][lx] -= 1;
        tile[ry][rx] += 1;
    }

    for y in 0..=M {
        for x in 1..=M {
            tile[y][x] += tile[y][x - 1];
        }
    }

    for y in 1..=M {
        for x in 0..=M {
            tile[y][x] += tile[y - 1][x];
        }
    }
    let mut ans = vec![0; N + 1];

    for y in 0..=M {
        for x in 0..=M {
            ans[tile[y][x] as usize] += 1;
            // tile[y][x] += tile[y - 1][x];
        }
    }

    for x in ans[1..].iter() {
        println!("{}", x);
    }
}
