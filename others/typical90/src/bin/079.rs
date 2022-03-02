use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      H: usize,
      W: usize,
      mut A: [[i32; W]; H],
      B: [[i32; W]; H]
    }

    let mut ans = 0;
    for y in 0..(H - 1) {
        for x in 0..(W - 1) {
            let d = B[y][x] - A[y][x];
            ans += d.abs() as u64;
            A[y][x] += d;
            A[y][x + 1] += d;
            A[y + 1][x] += d;
            A[y + 1][x + 1] += d;
        }
    }
    if A == B {
        println!("{}\n{}", YES, ans);
    } else {
        println!("{}", NO);
    }
}
