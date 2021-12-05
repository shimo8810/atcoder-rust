use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      _N: usize,
      A: usize,
      B: usize,
      P: usize,
      Q: usize,
      R: usize,
      S: usize
    }

    for i in P..=Q {
        for j in R..=S {
            let d = if i + B == j + A || i + j == A + B {
                '#'
            } else {
                '.'
            };
            print!("{}", d);
        }
        println!();
    }
}
