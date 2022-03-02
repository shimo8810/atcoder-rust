use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        A: [[u32; W]; H]
    }

    let mut R = vec![0; H];
    let mut C = vec![0; W];

    for (i, r) in R.iter_mut().enumerate() {
        for (j, c) in C.iter_mut().enumerate() {
            *c += A[i][j];
            *r += A[i][j];
        }
    }

    for (i, row) in A.iter().enumerate() {
        for (j, &a) in row.iter().enumerate() {
            print!("{}{}", R[i] + C[j] - a, if j + 1 == W { "\n" } else { " " });
        }
    }
}
