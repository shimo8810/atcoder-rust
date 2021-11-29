use proconio::{fastout, input, marker::Usize1};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      M: usize,
      B: [[Usize1;M]; N]
    }

    let b = B[0][0];
    let x = b % 7;

    if x + M > 7 {
        println!("{}", NO);
        return;
    }

    for i in 0..N {
        for j in 0..M {
            if b + j + i * 7 != B[i][j] {
                println!("{}", NO);
                return;
            }
        }
    }

    println!("{}", YES);
}
