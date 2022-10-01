use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: u64,
    }

    let mut ans = 0;
    let mut c = 0;
    let mut l = 1;
    let mut r = 10;
    for i in 0..=16 {
        if r <= N {
            ans += (r - l) * c;
        } else if l <= N && N < r {
            ans += (N - l + 1) * c;
        }

        l *= 10;
        r *= 10;

        if i % 3 == 2 {
            c += 1;
        }
    }

    println!("{}", ans);
}
