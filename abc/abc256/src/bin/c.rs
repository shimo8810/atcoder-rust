use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        h: [i32; 3],
        w: [i32; 3],
    }

    let mut ans = 0;
    for x11 in 1..=30 {
        for x12 in 1..=30 {
            for x21 in 1..=30 {
                for x22 in 1..=30 {
                    let x13 = h[0] - x11 - x12;
                    let x23 = h[1] - x21 - x22;
                    let x31 = w[0] - x11 - x21;
                    let x32 = w[1] - x12 - x22;
                    let x33 = w[2] - x13 - x23;
                    if x13 > 0
                        && x23 > 0
                        && x31 > 0
                        && x32 > 0
                        && x33 > 0
                        && x33 == h[2] - x31 - x32
                    {
                        ans += 1;
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
