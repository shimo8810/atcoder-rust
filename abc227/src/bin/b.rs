use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      S: [u32; N]
    }

    let mut ans = N;
    for &s in S.iter() {
        for a in 1..=(s / 3) {
            let m = (s - 3 * a) % (4 * a + 3);
            let b = (s - 3 * a) / (4 * a + 3);
            if m == 0 && b > 0 {
                ans -= 1;
                break;
            }
        }
    }

    println!("{}", ans);
}
