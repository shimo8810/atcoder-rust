use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        N: usize,
    }

    let mut X = vec![vec![0i32; W + 1]; H + 1];
    let mut Z = vec![vec![0i32; W + 1]; H + 1];

    for _ in 0..N {
        input! {A: Usize1, B: Usize1, C: Usize1, D: Usize1}
        X[A][B] += 1;
        X[A][D + 1] -= 1;
        X[C + 1][B] -= 1;
        X[C + 1][D + 1] += 1;
    }

    for y in 1..=H {
        for x in 1..=W {
            Z[y][x] = Z[y][x - 1] + X[y - 1][x - 1];
        }
    }
    for y in 1..=H {
        for x in 1..=W {
            Z[y][x] += Z[y - 1][x];
        }
    }

    for y in 1..=H {
        for x in 1..=W {
            print!("{} ", Z[y][x]);
        }
        println!();
    }
}
