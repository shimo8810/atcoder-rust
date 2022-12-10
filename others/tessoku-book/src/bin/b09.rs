use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
    }
    let S = 1501;
    let mut ar = vec![vec![0; S]; S];
    let mut cs = vec![vec![0; S + 1]; S + 1];
    for _ in 0..N {
        input! {A: usize, B: usize, C: usize, D: usize}
        ar[A][B] += 1;
        ar[A][D] -= 1;
        ar[C][B] -= 1;
        ar[C][D] += 1;
    }

    for (y, row) in ar.iter().enumerate() {
        for (x, &v) in row.iter().enumerate() {
            cs[y + 1][x + 1] = v + cs[y + 1][x] + cs[y][x + 1] - cs[y][x];
        }
    }

    let mut ans = 0;
    for row in cs.iter() {
        for &v in row.iter() {
            if v > 0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
