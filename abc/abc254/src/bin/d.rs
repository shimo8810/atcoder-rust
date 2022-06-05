use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: u64,
    }

    let mut ans = 0;
    for i in 1..=N {
        let mut k = i;
        let mut d = 2;
        while d * d <= k {
            while k % (d * d) == 0 {
                k /= d * d;
            }
            d += 1;
        }

        d = 1;
        while k * d * d <= N {
            ans += 1;
            d += 1;
        }
    }
    println!("{}", ans);
}
