use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      X: Usize1,
      A: [Usize1; N]
    }

    let mut c = vec![false; N];
    c[X] = true;

    let mut i = X;
    let mut cnt = 1;
    loop {
        let j = A[i];
        if c[j] {
            break;
        } else {
            i = j;
            c[i] = true;
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
