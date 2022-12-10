use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [u32; N]
    }

    for i in 0..N {
        for j in (i + 1)..N {
            for k in (j + 1)..N {
                if A[i] + A[j] + A[k] == 1_000 {
                    println!("{}", YES);
                    return;
                }
            }
        }
    }

    println!("{}", NO);
}
