use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: u64,
    }

    let mut ans = 0u64;

    let mut a = 1;
    while a * a * a <= N {
        let mut b = a;
        while b * b <= N / a {
            ans += N / a / b - b + 1;
            b += 1;
        }

        a += 1;
    }
    println!("{}", ans);
}
