use proconio::input;

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        A: [[u64; W]; H]
    }

    for i1 in 0..H {
        for i2 in (i1 + 1)..H {
            for j1 in 0..W {
                for j2 in (j1 + 1)..W {
                    let x = A[i1][j1] + A[i2][j2];
                    let y = A[i2][j1] + A[i1][j2];
                    if x > y {
                        println!("{}", NO);
                        return;
                    }
                }
            }
        }
    }

    println!("{}", YES);
}
