use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      S: usize,
      T: usize,
      X: usize,
    }

    let mut ans = NO;
    if S < T {
        if S <= X && X < T {
            ans = YES;
        }
    } else {
        if S <= X || X < T {
            ans = YES;
        }
    }
    println!("{}", ans);
}
