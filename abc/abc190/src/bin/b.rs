use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        S: u32,
        D: u32,
        XY: [(u32,u32); N]
    }

    let mut ans = NO;
    for (x, y) in XY.into_iter() {
        if x < S && D < y {
            ans = YES;
            break;
        }
    }
    println!("{}", ans);
}
