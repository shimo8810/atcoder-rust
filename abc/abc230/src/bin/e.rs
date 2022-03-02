use proconio::{fastout, input};

// fn solve_simple(n: u64) -> u64 {
//     (1..=n).map(|x| n / x).sum()
// }
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: u64,
    }

    let mut i = 1;
    let mut ans = 0;
    while i <= N {
        let r = N / i;
        let q = N % i;
        let k = q / r + 1;
        ans += r * k;
        i += k;
    }
    // assert_eq!(ans, solve_simple(N));
    println!("{}", ans);
}
